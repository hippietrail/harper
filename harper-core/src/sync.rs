#[cfg(not(feature = "concurrent"))]
pub use std::rc::Rc as Lrc;
#[cfg(feature = "concurrent")]
pub use std::sync::Arc as Lrc;

#[cfg(not(feature = "concurrent"))]
pub trait LSend {}

#[cfg(not(feature = "concurrent"))]
impl<T> LSend for T {}

#[cfg(feature = "concurrent")]
pub trait LSend: Send + Sync {}

#[cfg(feature = "concurrent")]
impl<T: Send + Sync> LSend for T {}
