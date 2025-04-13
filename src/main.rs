//! src/main.rs
use std::net::TcpListener;

use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind port");
    run(listener)?.await
}

// Check if even
/// ```rust
/// use my_mod::is_even;
///
/// assert!(is_even(2));
/// assert!(!is_even(3));
/// ```
pub fn is_even(x: u64) -> bool {
    x % 2 == 0
}