//! Native I/O operations (not available in WASM)

use std::fs;
use std::path::Path;
use std::io::{self, BufRead, Write};

use crate::error::{Error, Result};
use crate::engine::{SaveGame, SaveStorage, SaveInfo};

/// File-based save storage implementation
pub struct FileSaveStorage {
    save_dir: std::path::PathBuf,
}

impl FileSaveStorage {
    /// Create a new file storage
    pub fn new<P: AsRef<Path>>(save_dir: P) -> Result<Self> {
        let save_dir = save_dir.as_ref().to_path_buf();
        fs::create_dir_all(&save_dir)?;
        
        Ok(Self { save_dir })
    }
    
    fn save_path(&self, slot: u8) -> std::path::PathBuf {
        self.save_dir.join(format!("save_{:03}.json", slot))
    }
}

impl SaveStorage for FileSaveStorage {
    fn save(&mut self, slot: u8, save: &SaveGame) -> Result<()> {
        let path = self.save_path(slot);
        let json = serde_json::to_string_pretty(save)?;
        
        // Write with compression
        let compressed = compress_save_data(&json)?;
        fs::write(path, compressed)?;
        
        Ok(())
    }
    
    fn load(&self, slot: u8) -> Result<SaveGame> {
        let path = self.save_path(slot);
        let compressed = fs::read(path)?;
        
        // Decompress and parse
        let json = decompress_save_data(&compressed)?;
        let save = serde_json::from_str(&json)?;
        
        Ok(save)
    }
    
    fn list(&self) -> Vec<(u8, SaveInfo)> {
        let mut saves = Vec::new();
        
        for slot in 0..=255 {
            let path = self.save_path(slot);
            if path.exists() {
                if let Ok(save) = self.load(slot) {
                    let info = SaveInfo {
                        timestamp: save.timestamp,
                        description: save.description,
                        metadata: save.metadata,
                    };
                    saves.push((slot, info));
                }
            }
        }
        
        saves
    }
    
    fn delete(&mut self, slot: u8) -> Result<()> {
        let path = self.save_path(slot);
        if path.exists() {
            fs::remove_file(path)?;
        }
        Ok(())
    }
    
    fn exists(&self, slot: u8) -> bool {
        self.save_path(slot).exists()
    }
}

/// Compress save data using LZ4
#[cfg(not(target_arch = "wasm32"))]
fn compress_save_data(data: &str) -> Result<Vec<u8>> {
    Ok(lz4::block::compress(data.as_bytes(), None, false)?)
}

/// Compress save data (no compression for WASM)
#[cfg(target_arch = "wasm32")]
fn compress_save_data(data: &str) -> Result<Vec<u8>> {
    Ok(data.as_bytes().to_vec())
}

/// Decompress save data
#[cfg(not(target_arch = "wasm32"))]
fn decompress_save_data(compressed: &[u8]) -> Result<String> {
    let decompressed = lz4::block::decompress(compressed, None)?;
    String::from_utf8(decompressed)
        .map_err(|e| Error::RuntimeError(format!("Invalid UTF-8 in save data: {}", e)))
}

/// Decompress save data (no compression for WASM)
#[cfg(target_arch = "wasm32")]
fn decompress_save_data(compressed: &[u8]) -> Result<String> {
    String::from_utf8(compressed.to_vec())
        .map_err(|e| Error::RuntimeError(format!("Invalid UTF-8 in save data: {}", e)))
}

/// Terminal I/O for CLI
pub struct TerminalIO {
    stdin: io::Stdin,
    stdout: io::Stdout,
}

impl TerminalIO {
    /// Create new terminal I/O
    pub fn new() -> Self {
        Self {
            stdin: io::stdin(),
            stdout: io::stdout(),
        }
    }
    
    /// Read a line from stdin
    pub fn read_line(&self) -> Result<String> {
        let mut buffer = String::new();
        self.stdin.lock().read_line(&mut buffer)?;
        Ok(buffer.trim().to_string())
    }
    
    /// Write output
    pub fn write(&mut self, text: &str) -> Result<()> {
        write!(self.stdout, "{}", text)?;
        self.stdout.flush()?;
        Ok(())
    }
    
    /// Write line
    pub fn writeln(&mut self, text: &str) -> Result<()> {
        writeln!(self.stdout, "{}", text)?;
        self.stdout.flush()?;
        Ok(())
    }
    
    /// Clear screen (ANSI escape code)
    pub fn clear_screen(&mut self) -> Result<()> {
        write!(self.stdout, "\x1B[2J\x1B[1;1H")?;
        self.stdout.flush()?;
        Ok(())
    }
}

impl Default for TerminalIO {
    fn default() -> Self {
        Self::new()
    }
}

/// Load a script from file
pub fn load_script_file<P: AsRef<Path>>(path: P) -> Result<String> {
    fs::read_to_string(path)
        .map_err(|e| Error::IoError(e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[test]
    fn test_save_compression() {
        let data = "Test save data with some repetition repetition repetition";
        let compressed = compress_save_data(data).unwrap();
        let decompressed = decompress_save_data(&compressed).unwrap();
        assert_eq!(data, decompressed);
        assert!(compressed.len() < data.len());
    }
    
    #[test]
    fn test_file_save_storage() {
        let temp_dir = TempDir::new().unwrap();
        let mut storage = FileSaveStorage::new(temp_dir.path()).unwrap();
        
        // Create a test save
        let save = SaveGame {
            version: 1,
            timestamp: chrono::Utc::now(),
            description: "Test save".to_string(),
            world: crate::world::World::new(),
            runtime: crate::runtime::Runtime::new(),
            metadata: crate::engine::SaveMetadata {
                play_time: 100,
                chapter: None,
                completion: 50.0,
                thumbnail: None,
            },
        };
        
        // Test save and load
        storage.save(1, &save).unwrap();
        let loaded = storage.load(1).unwrap();
        assert_eq!(loaded.description, save.description);
        
        // Test list
        let saves = storage.list();
        assert_eq!(saves.len(), 1);
        assert_eq!(saves[0].0, 1);
        
        // Test delete
        storage.delete(1).unwrap();
        assert!(storage.list().is_empty());
    }
}