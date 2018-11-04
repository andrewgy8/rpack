//! A Bin Packing algorithm implemented in Rust. 
//! 
//! This library is meant to assist in solving packing issues
//! using the common heuristic "best fit, decreasing."
//! 

mod packing;

pub use packing::{Packing, Bin, Item};