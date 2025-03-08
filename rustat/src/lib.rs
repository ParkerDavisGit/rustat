use pyo3::prelude::*;

mod rustat_wrapper;
pub mod graphs;

#[pyfunction]
fn load_file(file_name: String) -> rustat_wrapper::TestClass {
    rustat_wrapper::TestClass::new(file_name)
}

/// A Python module implemented in Rust.
#[pymodule]
fn rustat(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(load_file, m)?)?;
    Ok(())
}
