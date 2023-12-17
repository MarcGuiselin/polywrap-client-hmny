use tokio::sync::Mutex;

mod closure;
pub use closure::*;
mod instance;
pub use instance::*;
mod loaded;
pub use loaded::*;

pub enum Wrap {
    Loaded(Mutex<LoadedWrap>),
    Closure(ClosureWrap),
}
