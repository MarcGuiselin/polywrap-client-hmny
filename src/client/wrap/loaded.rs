use super::WrapInstance;
use polywrap_uri::Uri;
use std::{collections::HashMap, path::Path, sync::Arc};

/// This struct contains all the information needed to execute a wasm module (besides the instance itself).
pub struct ExecutionContext {
    /// Manifest must declare all uris it wants to use. It can't use something not in the manifest.
    /// These uris map directly to a pre-loaded wrap uri, and in theory can be configured by user.
    pub subinvoke_uri_resolution: HashMap<Uri, Uri>,
}

pub struct LoadedWrap {
    pub execution_context: Arc<ExecutionContext>,
    pub cached_instances: Vec<WrapInstance>,
    // TODO: Hold wasmer module. Used to instantiate new instances
    // TODO: Hold a cache of wasmer instances
}

impl LoadedWrap {
    pub fn new_from_local(_path: &Path) -> Self {
        // TODO: Load wasmer module
        Self {
            execution_context: Arc::new(ExecutionContext {
                subinvoke_uri_resolution: HashMap::new(),
            }),
            cached_instances: vec![],
        }
    }
}
