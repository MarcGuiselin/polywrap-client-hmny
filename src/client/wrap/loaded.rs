use super::WrapInstance;
use crate::LoadError;
use polywrap_uri::Uri;
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use tokio::{fs, sync::Mutex};

/// This struct contains all the information needed to execute a wasm module (besides the instance itself).
pub struct ExecutionContext {
    /// Manifest must declare all uris it wants to use. It can't use something not in the manifest.
    /// These uris map directly to a pre-loaded wrap uri, and in theory can be configured by user.
    pub subinvoke_uri_resolution: HashMap<Uri, Uri>,
}

pub struct LoadedWrap {
    pub execution_context: Arc<ExecutionContext>,
    pub store: wasmer::Store,
    pub module: wasmer::Module,
    pub cached_instances: Mutex<Vec<WrapInstance>>,
}

impl LoadedWrap {
    pub async fn new_from_file(path: PathBuf) -> Result<Self, LoadError> {
        let path = path.join("wrap.wasm");

        let bytes = fs::read(&path)
            .await
            .map_err(|_| LoadError::WrapNotFound(path))?;
        Self::new_from_bytes(&bytes)
    }

    pub fn new_from_bytes(bytes: &[u8]) -> Result<Self, LoadError> {
        // Create a Store.
        let store = wasmer::Store::default();

        // We then use our store and Wasm bytes to compile a `Module`.
        // A `Module` is a compiled WebAssembly module that isn't ready to execute yet.
        let module = wasmer::Module::new(&store, bytes).map_err(LoadError::InvalidWasm)?;

        Ok(Self {
            execution_context: Arc::new(ExecutionContext {
                subinvoke_uri_resolution: HashMap::new(),
            }),
            store,
            module,
            cached_instances: Mutex::new(vec![]),
        })
    }
}
