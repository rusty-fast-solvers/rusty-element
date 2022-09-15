use pyo3::prelude::*;
use ndarray::{Array1};

pub trait Cell {
    fn geometry() -> Array1<f64>;
    fn topology() -> Array1<u64>;
}

#[pyfunction]
#[no_mangle]
pub fn add(a: i64, b: i64) -> i64 {
    a + b
}

#[cfg(test)]
mod test {
    use crate::cell::add;

    #[test]
    fn test_add() {
        assert_eq!(4, add(2, 2));
    }
}
