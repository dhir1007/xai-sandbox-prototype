use anyhow::Result;
use wasmtime::*;

fn main() -> Result<()> {
    let engine = Engine::default(); //Engine is a global config object that decides how the WASM module will run.
    let module = Module::from_file(&engine, "add.wasm")?; // Recipe to compile the WASM module. 'Compiled code, Read Only'
    let mut store = Store::new(&engine, ()); // Kitchen to prepare the WASM module to run. ' Actual state of the WASM module'
    let instance = Instance::new(&mut store, &module, &[])?; // Instance is the actual running WASM module.
    // Instance takes the module(recipe) and the store(kitchen) and returns the actual running WASM module.
    // &[]: This represents "imports." WebAssembly is highly sandboxed; it can't even print to the screen or access the internet unless you explicitly give it "tools" (imports). For our simple add function, we don't need any tools, so we pass an empty list.

    // dish is cooked, now we need to find the specific part we want to eat.
    let add = instance.get_typed_func::<(i32, i32), i32>(&mut store, "add")?; // get function from the instance.
    let result = add.call(&mut store, (5, 10))?; // call the function with the arguments.
    println!("Result: {}", result);
    Ok(())
}
