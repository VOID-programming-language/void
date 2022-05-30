//! void lang logic

extern crate c_rs;
pub use c_rs::c;
pub(crate) use c_rs::ctypes::*;
pub mod lexer;

c!{
    #include "crate/logic.h"
}