const FUNCTION_CALLS: u32 = 2000;

fn get_context() -> wasmtime_wasi::WasiCtx {
    let mut builder = wasmtime_wasi::WasiCtxBuilder::new()
        .inherit_stdout()
        .inherit_stderr()
        .inherit_stdin()
        .inherit_args().expect("Could not inherit args");

    if cfg!(feature = "leak") {
        builder = builder.inherit_env().expect("Could not inherit env");
    }

    builder.build()
}

fn benchmark(instance: wasmtime::Instance, mut store: wasmtime::Store<wasmtime_wasi::WasiCtx>) -> wasmtime::Store<wasmtime_wasi::WasiCtx> {
    let call = instance.get_typed_func::<(), (), _>(&mut store, "do_something").expect("Could not find print function");

    for _ in 0..FUNCTION_CALLS {
        call.call(&mut store, ())
            .expect("Could not call allocate method");
    }

    store
}

fn main() {
    let start = std::time::Instant::now();
    eprintln!("[{:.8}] Initializing wasm runtime", start.elapsed().as_secs_f32());
    let engine = wasmtime::Engine::default();
    let mut linker = wasmtime::Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s).unwrap();
    let module = wasmtime::Module::from_file(&engine, "target/wasm32-wasi/debug/wasmtime_lib.wasm").expect("Could not load module");
    let mut store = wasmtime::Store::new(&engine, get_context());
    linker.module(&mut store, "", &module).expect("Could not add module");
    let instance = linker.instantiate(&mut store, &module).expect("Failed to create instance");
    let memory = instance.get_memory(&mut store, "memory").expect("Could not find memory");

    eprintln!("[{:.8}] Calling a wasm function {} times", start.elapsed().as_secs_f32(), FUNCTION_CALLS);
    let start_vars = (start.elapsed(), memory.data_size(&store));
    let store = benchmark(instance, store);
    eprintln!("[{:.8}] Memory size start: {}", start_vars.0.as_secs_f32(), start_vars.1);
    eprintln!("[{:.8}] Memory size end  : {}", start.elapsed().as_secs_f32(), memory.data_size(&store));
}
