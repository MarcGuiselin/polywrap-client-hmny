use crate::{ExecutionContext, InvokeError, LoadedWrap};
use std::time::Instant;

mod imports;
mod state;
pub use state::State;

pub struct WrapInstance {
    last_used: Instant,
    // TODO: hold all the wasmer instance stuff
    store: wasmer::Store,
    env: wasmer::FunctionEnv<State>,
    invoke: wasmer::TypedFunction<(i32, i32, i32), i32>,
}

impl WrapInstance {
    pub fn new(loaded_wrap: &LoadedWrap) -> Self {
        // Create a Store.
        let mut store = wasmer::Store::default();

        // Initiate shared memory pool
        let memory = wasmer::Memory::new(&mut store, wasmer::MemoryType::new(2, None, false))
            .expect("wasm memory allocation failed");

        let state = State::new(memory.clone());
        let env = wasmer::FunctionEnv::new(&mut store, state);
        let imports = imports::create(memory, &mut store, &env);

        let instance = wasmer::Instance::new(&mut store, &loaded_wrap.module, &imports)
            .expect("wasm instantiation failed");

        let invoke = instance
            .exports
            .get_typed_function(&store, "_wrap_invoke")
            .expect("wasm invoke function not found");

        Self {
            last_used: Instant::now(),
            store,
            env,
            invoke,
        }
    }

    pub async fn invoke(
        &mut self,
        method: &str,
        args: Vec<u8>,
        _execution_context: &ExecutionContext,
    ) -> Result<Vec<u8>, InvokeError> {
        let len = args.len();
        self.last_used = Instant::now();
        self.env
            .as_mut(&mut self.store)
            .init(method.as_bytes().to_vec(), args);

        match self.invoke.call(
            &mut self.store,
            method.len() as _,
            len as _,
            0, // env.len()
        ) {
            Ok(_) => match self.env.as_mut(&mut self.store).invoke.take() {
                Some(Ok(result)) => Ok(result),
                Some(Err(error)) => Err(InvokeError::from_runtime_error(error)),
                None => Err(InvokeError::from_runtime_error(
                    "invoke function did not return a result".to_string(),
                )),
            },
            Err(e) => Err(InvokeError::RuntimeError(e)),
        }
    }
}
