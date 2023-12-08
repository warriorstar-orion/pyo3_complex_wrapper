use pyo3::{
    exceptions::PyRuntimeError, pyclass, pymethods, IntoPy, Py, PyAny, PyObject,
    PyResult, Python, types::PyList
};

use crate::foreign;

#[pyclass(module="pyo3_complex_wrapper")]
#[pyo3(name = "Foo")]
pub struct FooWrapper(foreign::Foo);

#[pyclass(module="pyo3_complex_wrapper")]
#[pyo3(name = "Bar")]
pub struct BarWrapper(foreign::Bar);

#[pymethods]
impl BarWrapper {
    pub fn __str__(&self) -> String {
        format!("<Bar {}, '{}'>", self.0.num, self.0.word)
    }

    pub fn __repr__(&self) -> String {
        self.__str__()
    }

    #[getter(word)]
    fn get_word(&self) -> PyResult<String> {
        Ok(self.0.word.to_string())
    }

    #[setter(word)]
    fn set_word(&mut self, value: String) -> PyResult<()> {
        self.0.word = value;
        Ok(())
    }    
}

#[pymethods]
impl FooWrapper {
    #[new]
    pub fn new() -> Self {
        FooWrapper(foreign::Foo::new())
    }

    pub fn __getitem__(&mut self, key: String) {
        /* ... */
    }

    pub fn pop(&mut self, key: String, py: Python<'_>) -> PyResult<PyObject> {
        if self.0.mapping.contains_key(&key) {
            let bars = self.0.mapping.remove(&key).unwrap();
            let mut vec = Vec::new();
            for bar in bars {
                vec.push(BarWrapper(bar).into_py(py));
            }

            return Ok(PyList::new(py, vec).into_py(py));
        }
        Err(PyRuntimeError::new_err(format!("can't find key {}", key)))
    }
   
    pub fn compute(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let result = self.0.compute().to_string();
        Ok(result.into_py(py))
    }
}