use crate::limiter::SandboxState;
use wasmtime::*;

pub struct ExectionReport {
    pub result: i32,
    pub fuel_consumed: u64,
    pub memory_consumed: usize,
    pub time_consumed: u64,
}
pub fn run_wasm(
    file_path: &str,
    func_name: &str,
    args: Vec<i32>,
    mem_limit: usize,
    fuel_limit: u64,
) -> anyhow::Result<ExectionReport> {
    let mut config = Config::new();

    // This will consume the fuel from the store, so the WASM module will run out of fuel and stop.
    // Fuel handles CPU time i.e FUEL -> CPU TIME. infinite loop problem handled by this.
    config.consume_fuel(true);

    //Engine is a global config object that decides how the WASM module will run.
    let engine = Engine::new(&config)?;

    // Recipe to compile the WASM module. 'Compiled code, Read Only'
    let module = Module::from_file(&engine, file_path)?;

    // let reqs = limiter::SandboxState {
    //     max_memory: 10 * 1024 * 1024, // 10 MB limit.
    // };

    let reqs = SandboxState {
        max_memory_bytes: mem_limit,
    };

    // Kitchen to prepare the WASM module to run. ' Actual state of the WASM module'
    let mut store = Store::new(&engine, reqs); // we didnt pass &reqs because if we do so, the store doesnt have full ownership of the limiter.

    // tell the store to use the limiter.
    // This looks a bit like a tiny function (a Closure in Rust).
    // It says: "Given the internal state of the store, return the part that implements ResourceLimiter."
    // Since our whole state is the limiter, we just return it.
    store.limiter(|state| state); // state is the current state of the limiter.

    // give store 10000 units of gas.
    store.set_fuel(fuel_limit)?;

    // Instance is the actual running WASM module.
    // Instance takes the module(recipe) and the store(kitchen) and returns the actual running WASM module.
    // &[]: This represents "imports." WebAssembly is highly sandboxed; it can't even print to the screen or access the internet unless you explicitly give it "tools" (imports).
    //For our simple add function, we don't need any tools, so we pass an empty list.
    let instance = Instance::new(&mut store, &module, &[])?;

    // dish is cooked, now we need to find the specific part we want to eat.

    
// ------------------------------------------------------------ START MEASURING TIME ------------------------------------------------------------
    let start_time = std::time::Instant::now();
    // get function from the instance.
    let func = instance.get_func(&mut store, func_name).ok_or(anyhow::anyhow!("Function not found"))?;

    // ------------------------------------------------------------ END MEASURING TIME ------------------------------------------------------------
    let duration = start_time.elapsed();


    let wasm_args: Vec<Val> = args.iter().map(|&arg| Val::I32(arg)).collect();

    let mut result = [Val::I32(0)];

    func.call(&mut store, &wasm_args, &mut result)?;
    // call the function with the arguments.
    // let result = add.call(&mut store, (5, 10))?;
    
    //In WebAssembly, a function can technically return more than one value (though most, like add, 
    //only return one). By using an array [Val::I32(0)], we're telling the engine: "I expect exactly one 32-bit integer back."
    let final_val = result[0].i32().ok_or(anyhow::anyhow!("Expected i32 result"))?;
    let remaining_fuel = store.get_fuel().unwrap_or(0);
    let fuel_consumed = fuel_limit - remaining_fuel;
    // // Check the fuel
    // let remaining = store.get_fuel()?;
    // let consumed = 10_000 - remaining;
    // println!("Fuel consumed: {}", consumed);

    // // print the result.
    // println!("Result: {}", result);
    Ok(ExectionReport {
        result: final_val,
        fuel_consumed: fuel_consumed,
        memory_consumed: mem_limit,
        time_consumed: duration.as_nanos() as u64,
    })
}
