use pyo3::prelude::*;
mod cell;

#[pyfunction]
#[pyo3(name = "add")]
fn add_py(a: i64, b: i64) -> PyResult<i64> {
    Ok(cell::add(a, b))
}

#[pymodule]
#[pyo3(name = "_rusty_element")]
fn my_extension(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(add_py))?;

    Ok(())
}
