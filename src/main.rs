use clap::Parser;
use std::path::PathBuf;
use ts2rs::compiler::Compiler;
use ts2rs::error::Result;

#[derive(Parser)]
#[command(name = "ts2rs")]
#[command(about = "High-performance TypeScript to Rust compiler")]
#[command(version)]
struct Cli {
    /// Input TypeScript file or directory
    #[arg(short, long)]
    input: PathBuf,

    /// Output directory for generated Rust code
    #[arg(short, long)]
    output: PathBuf,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Enable debug mode
    #[arg(short, long)]
    debug: bool,

    /// Optimize generated Rust code
    #[arg(long)]
    optimize: bool,

    /// Generate runtime for TypeScript semantics
    #[arg(short, long)]
    runtime: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    let log_level = if cli.debug {
        tracing::Level::DEBUG
    } else if cli.verbose {
        tracing::Level::INFO
    } else {
        tracing::Level::WARN
    };

    tracing_subscriber::fmt()
        .with_max_level(log_level)
        .init();

    // Create compiler instance
    let mut compiler = Compiler::new()
        .with_optimization(cli.optimize)
        .with_runtime(cli.runtime);

            // Test lexer (only in debug mode)
            if cli.debug {
                ts2rs::test_lexer::test_lexer();
            }
    
    // Compile TypeScript to Rust
    compiler.compile(&cli.input, &cli.output)?;

    println!("✅ Compilation completed successfully!");
    println!("📁 Output directory: {}", cli.output.display());

    Ok(())
}
