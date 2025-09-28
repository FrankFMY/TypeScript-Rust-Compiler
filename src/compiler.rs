//! Main compiler implementation

use crate::error::{CompilerError, Result};
use crate::generator::CodeGenerator;
use crate::lexer::Lexer;
use crate::parser::Parser;
use std::fs;
use std::path::{Path, PathBuf};

/// Main compiler struct
pub struct Compiler {
    optimize: bool,
    runtime: bool,
    output_dir: Option<PathBuf>,
}

impl Compiler {
    /// Create a new compiler instance
    pub fn new() -> Self {
        Self {
            optimize: false,
            runtime: false,
            output_dir: None,
        }
    }

    /// Enable code optimization
    pub fn with_optimization(mut self, optimize: bool) -> Self {
        self.optimize = optimize;
        self
    }

    /// Enable runtime for TypeScript semantics
    pub fn with_runtime(mut self, runtime: bool) -> Self {
        self.runtime = runtime;
        self
    }

    /// Set output directory
    pub fn with_output_dir(mut self, output_dir: PathBuf) -> Self {
        self.output_dir = Some(output_dir);
        self
    }

    /// Compile TypeScript code to Rust
    pub fn compile(&mut self, input: &Path, output: &Path) -> Result<()> {
        // Read input file
        let input_content = fs::read_to_string(input).map_err(CompilerError::Io)?;

        // Create lexer and tokenize
        let mut lexer = Lexer::new(input_content);
        let tokens = lexer.tokenize()?;

        // Create parser and parse AST
        let mut parser = Parser::new(tokens);
        let program = parser.parse()?;

        // Create code generator
        let mut generator = CodeGenerator::new(self.runtime);
        let rust_code = generator.generate(&program)?;

        // Write output
        self.write_output(output, &rust_code)?;

        Ok(())
    }

    /// Write output to file or directory
    fn write_output(&self, output: &Path, rust_code: &str) -> Result<()> {
        if output.is_dir() {
            // Generate multiple files
            self.write_multiple_files(output, rust_code)?;
        } else {
            // Create parent directory if it doesn't exist
            if let Some(parent) = output.parent() {
                fs::create_dir_all(parent).map_err(CompilerError::Io)?;
            }
            // Write single file
            fs::write(output, rust_code).map_err(CompilerError::Io)?;
        }

        Ok(())
    }

    /// Write multiple files for a project
    fn write_multiple_files(&self, output_dir: &Path, rust_code: &str) -> Result<()> {
        // Create output directory if it doesn't exist
        fs::create_dir_all(output_dir).map_err(CompilerError::Io)?;

        // Write main.rs
        let main_rs_path = output_dir.join("src").join("main.rs");
        fs::create_dir_all(main_rs_path.parent().unwrap()).map_err(CompilerError::Io)?;
        fs::write(&main_rs_path, rust_code).map_err(CompilerError::Io)?;

        // Write Cargo.toml
        let cargo_toml = self.generate_cargo_toml();
        let cargo_toml_path = output_dir.join("Cargo.toml");
        fs::write(&cargo_toml_path, cargo_toml).map_err(CompilerError::Io)?;

        // Write lib.rs if needed
        let lib_rs_path = output_dir.join("src").join("lib.rs");
        let lib_rs_content = self.generate_lib_rs();
        fs::write(&lib_rs_path, lib_rs_content).map_err(CompilerError::Io)?;

        Ok(())
    }

    /// Generate Cargo.toml for the output project
    fn generate_cargo_toml(&self) -> String {
        let mut dependencies = vec![
            "serde = { version = \"1.0\", features = [\"derive\"] }".to_string(),
            "serde_json = \"1.0\"".to_string(),
        ];

        if self.runtime {
            dependencies.push("anyhow = \"1.0\"".to_string());
            dependencies.push("thiserror = \"1.0\"".to_string());
        }

        format!(
            r#"[package]
name = "generated_rust_project"
version = "0.1.6"
edition = "2021"

[dependencies]
{}

[profile.release]
opt-level = 3
lto = true
"#,
            dependencies.join("\n")
        )
    }

    /// Generate lib.rs for the output project
    fn generate_lib_rs(&self) -> String {
        if self.runtime {
            r#"
pub mod runtime;
pub mod types;

use runtime::*;
use types::*;

// Re-export commonly used types
pub use runtime::{Any, Unknown, TypeScriptObject};
"#
            .to_string()
        } else {
            r#"
// Generated Rust code
"#
            .to_string()
        }
    }

    /// Compile multiple files
    pub fn compile_project(&mut self, input_dir: &Path, output_dir: &Path) -> Result<()> {
        // Find all TypeScript files
        let ts_files = self.find_typescript_files(input_dir)?;

        if ts_files.is_empty() {
            return Err(CompilerError::internal_error(
                "No TypeScript files found in input directory",
            ));
        }

        // Create output directory
        fs::create_dir_all(output_dir).map_err(CompilerError::Io)?;

        // Compile each file
        for ts_file in ts_files {
            let relative_path = ts_file
                .strip_prefix(input_dir)
                .map_err(|_| CompilerError::internal_error("Failed to strip prefix"))?;

            let rust_file = output_dir.join(relative_path).with_extension("rs");

            // Create directory for output file
            if let Some(parent) = rust_file.parent() {
                fs::create_dir_all(parent).map_err(CompilerError::Io)?;
            }

            // Compile single file
            self.compile(&ts_file, &rust_file)?;
        }

        // Generate project files
        self.generate_project_files(output_dir)?;

        Ok(())
    }

    /// Find all TypeScript files in directory
    fn find_typescript_files(&self, dir: &Path) -> Result<Vec<PathBuf>> {
        let mut ts_files = Vec::new();

        for entry in fs::read_dir(dir).map_err(CompilerError::Io)? {
            let entry = entry.map_err(CompilerError::Io)?;
            let path = entry.path();

            if path.is_dir() {
                // Recursively search subdirectories
                let sub_files = self.find_typescript_files(&path)?;
                ts_files.extend(sub_files);
            } else if path.extension().and_then(|s| s.to_str()) == Some("ts") {
                ts_files.push(path);
            }
        }

        Ok(ts_files)
    }

    /// Generate project files
    fn generate_project_files(&self, output_dir: &Path) -> Result<()> {
        // Generate Cargo.toml
        let cargo_toml = self.generate_cargo_toml();
        let cargo_toml_path = output_dir.join("Cargo.toml");
        fs::write(&cargo_toml_path, cargo_toml).map_err(CompilerError::Io)?;

        // Generate README.md
        let readme = self.generate_readme();
        let readme_path = output_dir.join("README.md");
        fs::write(&readme_path, readme).map_err(CompilerError::Io)?;

        // Generate .gitignore
        let gitignore = self.generate_gitignore();
        let gitignore_path = output_dir.join(".gitignore");
        fs::write(&gitignore_path, gitignore).map_err(CompilerError::Io)?;

        Ok(())
    }

    /// Generate README.md
    fn generate_readme(&self) -> String {
        r#"# Generated Rust Project

This project was generated from TypeScript code using the TypeScript-Rust-Compiler.

## Building

```bash
cargo build
```

## Running

```bash
cargo run
```

## Testing

```bash
cargo test
```

## Features

- Generated from TypeScript source code
- Full Rust type safety
- Serde serialization support
"#
        .to_string()
    }

    /// Generate .gitignore
    fn generate_gitignore(&self) -> String {
        r#"# Rust
/target/
Cargo.lock

# IDE
.vscode/
.idea/
*.swp
*.swo

# OS
.DS_Store
Thumbs.db
"#
        .to_string()
    }

    /// Get compiler version
    pub fn version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    /// Get compiler information
    pub fn info(&self) -> String {
        format!(
            "TypeScript-Rust-Compiler v{}\nOptimization: {}\nRuntime: {}",
            Self::version(),
            self.optimize,
            self.runtime
        )
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}
