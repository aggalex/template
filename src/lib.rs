#![feature(unboxed_closures)]
#![feature(fn_traits)]

pub extern crate prelude;

mod template;
#[cfg(test)]
pub mod tests;

pub use template::*;