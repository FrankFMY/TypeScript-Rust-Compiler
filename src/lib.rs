//! # TS2RS - TypeScript to Rust Compiler
//! 
//! A high-performance compiler that transforms TypeScript code into idiomatic Rust.
//! Supports all TypeScript features including advanced types, generics, decorators,
//! and async/await patterns.

pub mod ast;
pub mod compiler;
pub mod error;
pub mod generator;
pub mod lexer;
pub mod parser;
pub mod semantic;
pub mod test_lexer;
pub mod types;

use error::Result;

/// Main compiler interface
pub struct Compiler {
    optimize: bool,
    runtime: bool,
}

impl Compiler {
    /// Create a new compiler instance
    pub fn new() -> Self {
        Self {
            optimize: false,
            runtime: false,
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

    /// Compile TypeScript code to Rust
    pub fn compile(&mut self, _input: &std::path::Path, _output: &std::path::Path) -> Result<()> {
        // TODO: Implement compilation pipeline
        todo!("Implement compilation pipeline")
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}
