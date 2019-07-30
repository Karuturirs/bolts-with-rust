
#![allow(unused_variables)]
fn main() {
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
    enum Option<T> {
        Some(T),
        None,
    }
}
