mod executor;
mod limiter;

fn main() -> anyhow::Result<()> {
    println!("--- xAI Sandbox Prototype ---");

    // Hardcoded values for testing before we finish cli.rs
    let file_path = "add.wasm";
    let function = "add";
    let input = (10, 20);
    let memory_limit = 10 * 1024 * 1024; // 10MB
    let fuel_limit = 10_000;

    match executor::run_wasm(file_path, function, input, memory_limit, fuel_limit) {
        Ok(res) => println!("Success! Result: {}", res),
        Err(e) => eprintln!("Sandbox Error: {}", e),
    }

    Ok(())
}
