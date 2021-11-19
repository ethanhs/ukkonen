use pyo3::prelude::*;
use pyo3::PyResult;
use pyo3::Python;

use crate::ukkonen::ukkonen as ukkonen_impl;

#[pymodule]
pub fn ukkonen_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m)]
    #[pyo3(name = "ukkonen")]
    fn ukkonen_py(s1: &str, s2: &str, k: isize) -> isize {
        ukkonen_impl(s1, s2, k)
    }
    Ok(())
}
