extern crate libc;

pub use ::rtld::{RtldValue, RtldMain, RtldOr};
pub use ::libc::{dlopen as unsafe_open, dlsym as unsafe_sym};
pub use ::dynlib::Library;

pub mod rtld;
pub mod dynlib;
