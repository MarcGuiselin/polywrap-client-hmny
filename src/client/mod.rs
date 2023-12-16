use std::{collections::HashMap, path::PathBuf, sync::Arc};

pub use polywrap_core_macros::uri;
use polywrap_msgpack_serde::{from_slice, to_vec};
pub use polywrap_uri::Uri;
use serde::{de::DeserializeOwned, Serialize};
use tokio::sync::Mutex;

mod error;
pub use error::*;
mod wrap;
pub use wrap::*;

pub struct Client {
    inner: Arc<Mutex<ClientInner>>,
}

impl Clone for Client {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

struct ClientInner {
    pub loaded_wraps: HashMap<Uri, LoadedWrap>,
}

impl Client {
    /// Ignoring env for now
    pub async fn invoke<Input: Serialize, Output: DeserializeOwned>(
        &self,
        uri: &Uri,
        method: &str,
        args: Input,
    ) -> Result<Output, InvokeError> {
        let args = to_vec(&args).map_err(InvokeError::MsgpackSerialize)?;

        let (mut wasm_instance, execution_context) = self.obtain_instance(uri).await?;

        let result = wasm_instance
            .invoke(method, args, &execution_context)
            .await?;

        self.recycle_instance(uri, wasm_instance).await;

        let result = from_slice(&result).map_err(InvokeError::MsgpackDeserialize)?;

        Ok(result)
    }

    async fn obtain_instance(
        &self,
        uri: &Uri,
    ) -> Result<(WrapInstance, Arc<ExecutionContext>), InvokeError> {
        let mut inner = self.inner.lock().await;
        let loaded_wrap = inner
            .loaded_wraps
            .get_mut(uri)
            .ok_or(InvokeError::WrapNotLoaded)?;

        Ok((
            loaded_wrap
                .cached_instances
                .pop()
                .unwrap_or_else(|| WrapInstance::new()),
            loaded_wrap.execution_context.clone(),
        ))
    }

    async fn recycle_instance(&self, uri: &Uri, instance: WrapInstance) {
        let mut inner = self.inner.lock().await;

        // When recycling the instace, we don't care if the wrap is loaded or not. We can just throw away the instance if no longer loaded.
        if let Some(loaded_wrap) = inner.loaded_wraps.get_mut(uri) {
            loaded_wrap.cached_instances.push(instance);
        }
    }
}

pub struct ClientBuilder {
    wraps_to_load: Vec<(Uri, PathBuf)>,
}

impl ClientBuilder {
    pub fn new() -> Self {
        Self {
            wraps_to_load: vec![],
        }
    }

    pub fn add_fs_wrap<P: Into<PathBuf>>(&mut self, uri: Uri, path: P) -> &mut Self {
        self.wraps_to_load.push((uri, path.into()));
        self
    }

    pub async fn load(&self) -> Client {
        // TODO: Loading logic
        Client {
            inner: Arc::new(Mutex::new(ClientInner {
                loaded_wraps: self
                    .wraps_to_load
                    .iter()
                    .map(|(uri, path)| (uri.clone(), LoadedWrap::new_from_local(path)))
                    .collect(),
            })),
        }
    }
}
