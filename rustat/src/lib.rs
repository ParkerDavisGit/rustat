use pyo3::prelude::*;
mod rustat_wrapper;

/// A Python module implemented in Rust.
#[pymodule]
fn rustat(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<rustat_wrapper::TestClass>()?;
    Ok(())
}
