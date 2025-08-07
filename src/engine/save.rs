//! Save game management

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use crate::error::{Error, Result};
use crate::world::World;
use crate::runtime::Runtime;

/// Save game data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveGame {
    /// Save version for compatibility
    pub version: u32,
    /// When the save was created
    pub timestamp: DateTime<Utc>,
    /// Save description
    pub description: String,
    /// World state
    pub world: World,
    /// Runtime state
    pub runtime: Runtime,
    /// Metadata
    pub metadata: SaveMetadata,
}

/// Save game metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveMetadata {
    /// Play time in seconds
    pub play_time: u64,
    /// Chapter or section
    pub chapter: Option<String>,
    /// Completion percentage
    pub completion: f32,
    /// Thumbnail data (base64)
    pub thumbnail: Option<String>,
}

/// Save game manager
pub struct SaveManager {
    /// Storage backend
    storage: Box<dyn SaveStorage>,
    /// Current save version
    version: u32,
}

impl SaveManager {
    /// Create a new save manager
    pub fn new() -> Self {
        Self {
            storage: Box::new(MemorySaveStorage::new()),
            version: 1,
        }
    }
    
    /// Save the game
    pub fn save(&mut self, world: &World, runtime: &Runtime, slot: u8) -> Result<()> {
        let save = SaveGame {
            version: self.version,
            timestamp: Utc::now(),
            description: self.generate_description(world),
            world: world.clone(),
            runtime: runtime.clone(),
            metadata: self.generate_metadata(world),
        };
        
        self.storage.save(slot, &save)
    }
    
    /// Load a saved game
    pub fn load(&self, slot: u8) -> Result<(World, Runtime)> {
        let save = self.storage.load(slot)?;
        
        // Check version compatibility
        if save.version > self.version {
            return Err(Error::InvalidScript(
                "Save game is from a newer version".to_string()
            ));
        }
        
        Ok((save.world, save.runtime))
    }
    
    /// Auto-save the game
    pub fn auto_save(&mut self, world: &World, runtime: &Runtime) -> Result<()> {
        self.save(world, runtime, 0) // Slot 0 for auto-save
    }
    
    /// List available saves
    pub fn list_saves(&self) -> Vec<(u8, SaveInfo)> {
        self.storage.list()
    }
    
    /// Delete a save
    pub fn delete(&mut self, slot: u8) -> Result<()> {
        self.storage.delete(slot)
    }
    
    /// Check if a save exists
    pub fn has_save(&self, save_name: &str) -> bool {
        // Convert save name to slot number
        let slot = match save_name {
            "quicksave" => 254,
            "autosave" => 255,
            "checkpoint" => 253,
            s if s.starts_with("save_") => {
                s.strip_prefix("save_")
                    .and_then(|n| n.parse::<u8>().ok())
                    .unwrap_or(0)
            }
            _ => 0,
        };
        
        self.storage.exists(slot)
    }
    
    /// Generate save description
    fn generate_description(&self, world: &World) -> String {
        format!(
            "{} - Turn {} - Score {}",
            world.player_location,
            world.turn_count,
            world.score
        )
    }
    
    /// Generate save metadata
    fn generate_metadata(&self, world: &World) -> SaveMetadata {
        let stats = world.stats();
        
        SaveMetadata {
            play_time: 0, // TODO: Track play time
            chapter: None,
            completion: stats.completion,
            thumbnail: None,
        }
    }
}

/// Save storage trait
pub trait SaveStorage: Send + Sync {
    /// Save a game
    fn save(&mut self, slot: u8, save: &SaveGame) -> Result<()>;
    
    /// Load a game
    fn load(&self, slot: u8) -> Result<SaveGame>;
    
    /// List available saves
    fn list(&self) -> Vec<(u8, SaveInfo)>;
    
    /// Delete a save
    fn delete(&mut self, slot: u8) -> Result<()>;
    
    /// Check if a save exists
    fn exists(&self, slot: u8) -> bool;
}

/// Save info for listing
#[derive(Debug, Clone)]
pub struct SaveInfo {
    /// When saved
    pub timestamp: DateTime<Utc>,
    /// Description
    pub description: String,
    /// Metadata
    pub metadata: SaveMetadata,
}

/// In-memory save storage (for testing/WASM)
struct MemorySaveStorage {
    saves: HashMap<u8, SaveGame>,
}

impl MemorySaveStorage {
    fn new() -> Self {
        Self {
            saves: HashMap::new(),
        }
    }
}

impl SaveStorage for MemorySaveStorage {
    fn save(&mut self, slot: u8, save: &SaveGame) -> Result<()> {
        self.saves.insert(slot, save.clone());
        Ok(())
    }
    
    fn load(&self, slot: u8) -> Result<SaveGame> {
        self.saves.get(&slot)
            .cloned()
            .ok_or_else(|| Error::NotFound(format!("Save slot {}", slot)))
    }
    
    fn list(&self) -> Vec<(u8, SaveInfo)> {
        self.saves.iter()
            .map(|(&slot, save)| {
                let info = SaveInfo {
                    timestamp: save.timestamp,
                    description: save.description.clone(),
                    metadata: save.metadata.clone(),
                };
                (slot, info)
            })
            .collect()
    }
    
    fn delete(&mut self, slot: u8) -> Result<()> {
        self.saves.remove(&slot);
        Ok(())
    }
    
    fn exists(&self, slot: u8) -> bool {
        self.saves.contains_key(&slot)
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod file_storage {
    use super::*;
    use std::fs;
    use std::path::{Path, PathBuf};
    
    /// File-based save storage
    pub struct FileSaveStorage {
        save_dir: PathBuf,
    }
    
    impl FileSaveStorage {
        pub fn new<P: AsRef<Path>>(save_dir: P) -> Result<Self> {
            let save_dir = save_dir.as_ref().to_path_buf();
            fs::create_dir_all(&save_dir)?;
            
            Ok(Self { save_dir })
        }
        
        fn save_path(&self, slot: u8) -> PathBuf {
            self.save_dir.join(format!("save_{:03}.json", slot))
        }
    }
    
    impl SaveStorage for FileSaveStorage {
        fn save(&mut self, slot: u8, save: &SaveGame) -> Result<()> {
            let path = self.save_path(slot);
            let json = serde_json::to_string_pretty(save)?;
            fs::write(path, json)?;
            Ok(())
        }
        
        fn load(&self, slot: u8) -> Result<SaveGame> {
            let path = self.save_path(slot);
            let json = fs::read_to_string(path)?;
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
}