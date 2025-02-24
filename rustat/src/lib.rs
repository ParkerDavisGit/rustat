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
fn test_func(py: Python<'_>, a: u64) -> PyResult<u64> {
    let count = py.allow_threads(|| count_to(a));
    println!("AHHHHHHHHHHH");
    
    Ok(count)
}

fn count_to (a: u64) -> u64 {
    let mut count = 0u64;
    for _ in (0..a) {
        count += 1u64;
    }
    
    count
}

/// A Python module implemented in Rust.
#[pymodule]
fn rustat(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(test_func, m)?)?;
    m.add_class::<test_class>()?;
    Ok(())
}
