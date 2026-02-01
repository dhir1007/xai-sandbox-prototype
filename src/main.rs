use anyhow::Result;
use wasmtime::*;


struct MyRequirements {
    max_memory: usize, // Max memory in bytes.
}

impl ResourceLimiter for MyRequirements {
    fn memory_growing(&mut self, current: usize, desired: usize, max: Option<usize>) -> Result<bool> {
        Ok(desired <= self.max_memory) // if the desired memory is less than the max memory, return true.
    }

    fn table_growing(&mut self, current: usize, desired: usize, max: Option<usize>) -> Result<bool> {
        Ok(desired <= 1000) // if the desired table is less than 1000, return true.
    }
}
fn main() -> Result<()> {

    let mut config = Config::new();

    // This will consume the fuel from the store, so the WASM module will run out of fuel and stop.
    // Fuel handles CPU time i.e FUEL -> CPU TIME. infinite loop problem handled by this.
    config.consume_fuel(true); 

    //Engine is a global config object that decides how the WASM module will run.
    let engine = Engine::new(&config)?;
    
    // Recipe to compile the WASM module. 'Compiled code, Read Only'
    let module = Module::from_file(&engine, "add.wasm")?; 

    let reqs = MyRequirements {
        max_memory: 10 * 1024 * 1024, // 10 MB limit.
    };
    // Kitchen to prepare the WASM module to run. ' Actual state of the WASM module'
    let mut store = Store::new(&engine, reqs); // we didnt pass &reqs because if we do so, the store doesnt have full ownership of the limiter.

    // tell the store to use the limiter.
    // This looks a bit like a tiny function (a Closure in Rust). 
    // It says: "Given the internal state of the store, return the part that implements ResourceLimiter." 
    // Since our whole state is the limiter, we just return it.
    store .limiter(|state| state); // state is the current state of the limiter.

    // give store 10000 units of gas.
    store.set_fuel(10_000)?;

    // Instance is the actual running WASM module.
    // Instance takes the module(recipe) and the store(kitchen) and returns the actual running WASM module.
    // &[]: This represents "imports." WebAssembly is highly sandboxed; it can't even print to the screen or access the internet unless you explicitly give it "tools" (imports). 
    //For our simple add function, we don't need any tools, so we pass an empty list.
    let instance = Instance::new(&mut store, &module, &[])?; 


    // dish is cooked, now we need to find the specific part we want to eat.

    // get function from the instance.
    let add = instance.get_typed_func::<(i32, i32), i32>(&mut store, "add")?; 

    // call the function with the arguments.
    let result = add.call(&mut store, (5, 10))?; 

    // Check the fuel
    let remaining = store.get_fuel()?;
    let consumed = 10_000 - remaining;
    println!("Fuel consumed: {}", consumed);

    // print the result.
    println!("Result: {}", result);
    Ok(())
}
