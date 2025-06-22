// src/lib.rs
mod math;
mod greetings;

pub use math::{add, multiply};
pub use greetings::say_hello;

