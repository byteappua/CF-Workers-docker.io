//! # Cargo Demo Crate
//!
//! `cargo_demo` is a collection of utilities to demonstrate
//! how to use cargo documentation comments.

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = cargo_demo::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

/// A custom enum for demonstration.
#[derive(Debug)]
pub enum Color {
    Red,
    Green,
    Blue,
}
