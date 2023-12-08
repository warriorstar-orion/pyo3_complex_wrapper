use pyo3::prelude::*;

pub mod foreign;
pub mod wrapper;

/// A Python module implemented in Rust.
#[pymodule]
fn pyo3_complex_wrapper(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<wrapper::FooWrapper>()?;
    Ok(())
}
