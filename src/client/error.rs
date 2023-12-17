use std::path::PathBuf;

#[derive(Debug)]
pub enum LoadError {
    WrapNotFound(PathBuf),
    InvalidWasm(wasmer::CompileError),
}

#[derive(Debug)]
pub enum InvokeError {
    // MemoryTooSmall(usize),
    // CallFailed(wasmer::RuntimeError),
    // WrapError(WrapError),
    // DecodeFailed(String),
    // EncodeFailed(String),
    MethodNotFound,
    MsgpackSerialize(polywrap_msgpack_serde::Error),
    MsgpackDeserialize(polywrap_msgpack_serde::Error),
    WrapNotLoaded,
    RuntimeError(wasmer::RuntimeError),
}

impl InvokeError {
    pub fn from_runtime_error(err: String) -> Self {
        Self::RuntimeError(wasmer::RuntimeError::new(err))
    }
}
