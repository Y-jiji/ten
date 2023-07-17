#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

#[macro_use]
mod helper; // macro_use attribute make macros available throughout the crate

// declaration of operators
mod source;
mod action;
mod binops;
mod uniops;
mod redops;

// modules exposing public functions
mod tgraph;
pub use tgraph::*;
mod tensor;
pub use tensor::*;