//#![feature(prelude_import)]

#[cfg(not(feature = "regex"))]
pub mod grammer;
#[cfg(feature = "regex")]
pub mod regex;

pub mod prelude;


//#[prelude_import]
#[feature(prelude_import)]
#[allow(unused)]
pub use prelude::*;