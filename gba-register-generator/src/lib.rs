#[macro_use]
extern crate quote;

#[macro_use]
extern crate derive_builder;

#[macro_use]
extern crate error_chain;

extern crate syn;

mod errors {
    error_chain!{}
}
use errors::*;

mod register;
mod generate;

pub use register::*;
pub use generate::generate_register;
