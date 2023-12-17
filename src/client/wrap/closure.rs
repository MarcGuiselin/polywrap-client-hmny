use crate::InvokeError;
use polywrap_msgpack_serde::{from_slice, to_vec};
use serde::{de::DeserializeOwned, Serialize};
use std::collections::HashMap;

pub struct ClosureWrap {
    closure: HashMap<String, Box<dyn Fn(&[u8]) -> Result<Vec<u8>, InvokeError> + Send + Sync>>,
}

impl ClosureWrap {
    pub fn new() -> Self {
        Self {
            closure: HashMap::new(),
        }
    }

    pub fn add_method<Input: DeserializeOwned, Output: Serialize>(
        mut self,
        method: &str,
        callback: impl Fn(&Input) -> Result<Output, InvokeError> + Send + Sync + 'static,
    ) -> Self {
        self.closure.insert(
            method.to_string(),
            Box::new(move |args| {
                let args = from_slice(args).map_err(InvokeError::MsgpackDeserialize)?;
                let result = callback(&args)?;
                let result = to_vec(&result).map_err(InvokeError::MsgpackSerialize)?;
                Ok(result)
            }),
        );
        self
    }

    pub async fn invoke(&self, method: &str, args: &[u8]) -> Result<Vec<u8>, InvokeError> {
        let closure = self
            .closure
            .get(method)
            .ok_or(InvokeError::MethodNotFound)?;

        let result = closure(&args)?;

        Ok(result)
    }
}
