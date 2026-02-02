use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "xai-sandbox")]
#[command(about = "A secure WASM sandbox for xAI", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run a WASM module
    Run {
        /// Path to the .wasm file (Required)
        path: String,

        #[arg(short, long, default_value = "add")]
        function: String,

        /// Memory limit in MB (Defaults to 10 if not provided)
        #[arg(short, long, default_value_t = 10)]
        memory: usize,

        /// Fuel limit (Defaults to 10,000 if not provided)
        #[arg(long, default_value_t = 10000)]
        fuel: u64,

        #[arg(num_args = 1..)]
        args: Vec<i32>,
    },
}
