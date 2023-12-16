#[derive(Debug)]
pub enum InvokeError {
    // MemoryTooSmall(usize),
    // CallFailed(wasmer::RuntimeError),
    // WrapError(WrapError),
    // DecodeFailed(String),
    // EncodeFailed(String),
    MsgpackSerialize(polywrap_msgpack_serde::Error),
    MsgpackDeserialize(polywrap_msgpack_serde::Error),
    WrapNotLoaded,
}
