# Contributing to PlotScript Engine

Thank you for your interest in contributing to PlotScript Engine! This document provides guidelines and instructions for contributing.

## Code of Conduct

By participating in this project, you agree to abide by our Code of Conduct:

- Be respectful and inclusive
- Welcome newcomers and help them get started
- Focus on constructive criticism
- Assume good intentions
- No harassment or discrimination of any kind

## How to Contribute

### Reporting Issues

1. Check if the issue already exists in the [issue tracker](https://github.com/plotscript/engine.plotscri.pt/issues)
2. If not, create a new issue with:
   - Clear, descriptive title
   - Detailed description of the problem
   - Steps to reproduce
   - Expected vs actual behavior
   - System information (OS, Rust version, etc.)
   - Minimal code example if applicable

### Suggesting Features

1. Check the [discussions](https://github.com/plotscript/engine.plotscri.pt/discussions) for similar ideas
2. Open a new discussion with:
   - Clear description of the feature
   - Use cases and benefits
   - Potential implementation approach
   - Examples from other engines if relevant

### Contributing Code

#### Setup

1. Fork the repository
2. Clone your fork:
   ```bash
   git clone https://github.com/YOUR_USERNAME/engine.plotscri.pt
   cd engine.plotscri.pt
   ```
3. Add upstream remote:
   ```bash
   git remote add upstream https://github.com/plotscript/engine.plotscri.pt
   ```
4. Create a feature branch:
   ```bash
   git checkout -b feature/your-feature-name
   ```

#### Development Process

1. **Write tests first** - We practice TDD when possible
2. **Make your changes** - Follow the coding standards below
3. **Run tests** - Ensure all tests pass
4. **Run formatting** - `cargo fmt`
5. **Run linting** - `cargo clippy -- -D warnings`
6. **Update documentation** - Both code docs and README if needed
7. **Commit your changes** - Use conventional commits (see below)

#### Conventional Commits

We use [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation changes
- `style:` Code style changes (formatting, etc.)
- `refactor:` Code refactoring
- `perf:` Performance improvements
- `test:` Test additions or changes
- `chore:` Build process or auxiliary tool changes
- `ci:` CI/CD changes

Examples:
```
feat: add fuzzy matching to parser
fix: correct inventory weight calculation
docs: update RON format examples
perf: optimize world state queries
```

#### Pull Request Process

1. Update your branch with latest upstream:
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```
2. Push to your fork:
   ```bash
   git push origin feature/your-feature-name
   ```
3. Create a Pull Request with:
   - Clear title following conventional commits
   - Description of changes
   - Link to related issue(s)
   - Screenshots/examples if applicable
4. Address review feedback
5. Squash commits if requested

### Coding Standards

#### Rust Style

- Follow standard Rust conventions
- Use `rustfmt` for formatting
- Use `clippy` for linting
- Prefer `&str` over `String` for function parameters
- Use `Result<T, Error>` for fallible operations
- Document all public APIs with examples

#### Documentation

- All public items must have documentation
- Include examples in doc comments
- Use `///` for item documentation
- Use `//!` for module documentation
- Keep comments up to date with code

Example:
```rust
/// Processes player input and returns a response.
/// 
/// # Arguments
/// 
/// * `input` - The player's command
/// 
/// # Examples
/// 
/// ```
/// # use plotscript::Engine;
/// # let mut engine = Engine::new();
/// let response = engine.process_input("go north")?;
/// println!("{}", response.text);
/// ```
/// 
/// # Errors
/// 
/// Returns an error if the game hasn't been started.
pub fn process_input(&mut self, input: &str) -> Result<Response> {
    // Implementation
}
```

#### Testing

- Write unit tests for all new functionality
- Write integration tests for complex features
- Aim for >80% code coverage
- Use property-based testing where appropriate
- Test error cases, not just happy paths

#### Performance

- Benchmark performance-critical code
- Document performance characteristics
- Avoid premature optimization
- Profile before optimizing

### Areas for Contribution

We especially welcome contributions in these areas:

#### Parser Improvements
- Grammar enhancements
- Better error messages
- Additional language constructs
- Performance optimizations

#### Game Format Support
- Additional script formats
- Format converters
- Validation tools
- Editor integrations

#### Platform Support
- Mobile platforms
- Additional WASM targets
- Native GUI frontends
- Cloud save support

#### Documentation
- Tutorials
- Example games
- API documentation
- Translation to other languages

#### Testing
- Additional test coverage
- Fuzzing
- Property-based tests
- Performance benchmarks

### Getting Help

- Join our [Discord](https://discord.gg/plotscript)
- Ask in [GitHub Discussions](https://github.com/plotscript/engine.plotscri.pt/discussions)
- Check the [documentation](https://docs.plotscri.pt)
- Email maintainers (for sensitive issues only)

### Recognition

Contributors will be:
- Listed in the CONTRIBUTORS file
- Mentioned in release notes
- Given credit in documentation

### License

By contributing, you agree that your contributions will be licensed under the same terms as the project (MIT/Apache-2.0 dual license).

Thank you for contributing to PlotScript Engine!