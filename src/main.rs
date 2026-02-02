use cli::Cli;
use clap::Parser;

mod cli;
mod executor;
mod limiter;

fn main() -> anyhow::Result<()> {
    // 1. We call parse to get the data from the user
    let cli = Cli::parse();

    // 2. We match on the command to see what the user wants to do
    match cli.command {
        cli::Commands::Run { path, memory, fuel } => {
            // Here, 'path' is a String, 'memory' is a usize, etc.

            // At xAI, we care about units!
            // Our CLI takes 'memory' in MB, but our executor wants Bytes.
            let memory_in_bytes = memory * 1024 * 1024;

            // Now we call our executor with the REAL data
            executor::run_wasm(&path, "add", (5, 10), memory_in_bytes, fuel)?;
        }
    }

    Ok(())
}
