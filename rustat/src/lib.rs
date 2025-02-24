use pyo3::prelude::*;


#[pyclass]
struct test_class {
    inner: i32,
}

#[pymethods]
impl test_class {
    #[new]
    fn new(value: i32) -> Self {
        println!("Made with {}", value);
        test_class { inner: value, }
    }
}


/// Formats the sum of two numbers as string.
#[pyfunction]
fn test_func(a: usize, b: usize) -> PyResult<String> {
    println!("Gello Gorld!");
    
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn rustat(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(test_func, m)?)?;
    m.add_class::<test_class>()?;
    Ok(())
}
