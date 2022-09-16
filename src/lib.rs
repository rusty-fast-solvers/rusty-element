use pyo3::prelude::*;

// pub(crate) mod cell;

#[pyfunction]
fn myadd(a: usize, b: usize) -> PyResult<usize> {
    Ok(a + b)
}

#[pymodule]
#[pyo3(name = "rusty_element")]
fn my_extension(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(myadd))?;

    Ok(())
}
