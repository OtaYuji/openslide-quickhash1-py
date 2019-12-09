use openslide::OpenSlide;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::path::Path;

#[pyfunction]
/// Formats the sum of two numbers as string
fn quickhash1(path: String) -> PyResult<String> {
    if let Ok(slide) = OpenSlide::new(Path::new(&path)) {
        let q_hash = slide.properties.quickhash_1().unwrap_or_default();
        Ok(q_hash)
    } else {
        Ok("".to_string())
    }
}

/// This module is a python module implemented in Rust.
#[pymodule]
fn openslide_quickhash1(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(quickhash1))?;

    Ok(())
}
