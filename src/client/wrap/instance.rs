use std::time::Instant;

use crate::{ExecutionContext, InvokeError};

pub struct WrapInstance {
    last_used: Instant,
    // TODO: hold all the wasmer instance stuff
}

impl WrapInstance {
    pub fn new() -> Self {
        Self {
            last_used: Instant::now(),
        }
    }

    pub async fn invoke(
        &mut self,
        _method: &str,
        _args: Vec<u8>,
        _execution_context: &ExecutionContext,
    ) -> Result<Vec<u8>, InvokeError> {
        self.last_used = Instant::now();

        // TODO: Invoke wasm
        Ok(vec![
            129, 166, 114, 101, 115, 117, 108, 116, 164, 116, 101, 115, 116,
        ])
    }
}
