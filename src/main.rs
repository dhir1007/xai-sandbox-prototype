use cli::Cli;
use clap::Parser;

mod cli;
mod executor;
mod limiter;

fn main() -> anyhow::Result<()> {

    // env_logger::init();
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp_millis()
        .init();
    // 1. We call parse to get the data from the user
    let cli = Cli::parse();

    // 2. We match on the command to see what the user wants to do
    match cli.command {
        cli::Commands::Run { path, function, memory, fuel, args } => {
            // Here, 'path' is a String, 'memory' is a usize, etc.

            // At xAI, we care about units!
            // Our CLI takes 'memory' in MB, but our executor wants Bytes.
            let memory_in_bytes = memory * 1024 * 1024;

            // Now we call our executor with the REAL data
            match executor::run_wasm(&path, &function, args, memory_in_bytes, fuel) {
                Ok(report ) => {
                    log::info!("Sandbox execution successful. Result: {}", report.result);
                    log::info!("Fuel consumed: {}", report.fuel_consumed);
                    log::info!("Memory consumed: {}", report.memory_consumed);
                    log::info!("Time consumed: {}", report.time_consumed);
                    Ok(())
                }
                Err(e) => {
                    log::error!("Sandbox execution failed: {}", e);
                    Err(e)
                }
            }
        }
    }

    
}
