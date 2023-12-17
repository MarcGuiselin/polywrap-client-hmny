pub use polywrap_core_macros::uri;
use polywrap_msgpack_serde::{from_slice, to_vec};
pub use polywrap_uri::Uri;
use serde::{de::DeserializeOwned, Serialize};
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use tokio::sync::Mutex;

mod error;
pub use error::*;
mod wrap;
pub use wrap::*;

pub struct Client {
    inner: Arc<ClientInner>,
}

impl Clone for Client {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

struct ClientInner {
    pub loaded_wraps: HashMap<Uri, Wrap>,
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
        let result = self.invoke_raw(uri, method, &args).await?;
        let result = from_slice(&result).map_err(InvokeError::MsgpackDeserialize)?;

        Ok(result)
    }

    async fn invoke_raw(
        &self,
        uri: &Uri,
        method: &str,
        args: &[u8],
    ) -> Result<Vec<u8>, InvokeError> {
        let wrap = self
            .inner
            .loaded_wraps
            .get(uri)
            .ok_or(InvokeError::WrapNotLoaded)?;

        let result = match wrap {
            Wrap::Loaded(loaded_wrap) => {
                // Get an instance from the cache, or create a new one if none are available.
                let mut loaded_wrap_locked = loaded_wrap.lock().await;
                let mut instance = loaded_wrap_locked
                    .cached_instances
                    .pop()
                    .unwrap_or_else(|| WrapInstance::new());
                let execution_context = loaded_wrap_locked.execution_context.clone();

                // The lock should be dropped here so other threads can use other WrapInstances while this one is invoking.
                drop(loaded_wrap_locked);

                // Invoke the method on the instance.
                let result = instance.invoke(method, &args, &execution_context).await?;

                // Put the instance back in the cache.
                let mut loaded_wrap_locked = loaded_wrap.lock().await;
                loaded_wrap_locked.cached_instances.push(instance);

                result
            }
            Wrap::Closure(closure_wrap) => closure_wrap.invoke(method, &args).await?,
        };

        Ok(result)
    }
}

pub struct ClientBuilder {
    wraps_to_load: Vec<(Uri, LoadWrapRequest)>,
}

enum LoadWrapRequest {
    Fs(PathBuf),
    Closure(ClosureWrap),
}

impl ClientBuilder {
    pub fn new() -> Self {
        Self {
            wraps_to_load: vec![],
        }
    }

    pub fn add_file<P: Into<PathBuf>>(mut self, uri: Uri, path: P) -> Self {
        self.wraps_to_load
            .push((uri, LoadWrapRequest::Fs(path.into())));
        self
    }

    pub fn add_closure(mut self, uri: Uri, closure_wrap: ClosureWrap) -> Self {
        self.wraps_to_load
            .push((uri, LoadWrapRequest::Closure(closure_wrap)));
        self
    }

    pub async fn load(self) -> Client {
        // TODO: Loading logic

        Client {
            inner: Arc::new(ClientInner {
                loaded_wraps: self
                    .wraps_to_load
                    .into_iter()
                    .map(|(uri, load_wrap_request)| {
                        let wrap = match load_wrap_request {
                            LoadWrapRequest::Fs(path) => {
                                Wrap::Loaded(Mutex::new(LoadedWrap::new_from_local(&path)))
                            }
                            LoadWrapRequest::Closure(closure_wrap) => Wrap::Closure(closure_wrap),
                        };
                        (uri, wrap)
                    })
                    .collect(),
            }),
        }
    }
}
