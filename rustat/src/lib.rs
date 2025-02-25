use pyo3::prelude::*;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

#[pyclass]
struct test_class {
    count: Arc<Mutex<u64>>,
}

#[pymethods]
impl test_class {
    #[new]
    fn new(value: u64) -> Self {
        println!("Made with {}", value);
        test_class { count: Arc::new(Mutex::new(0u64)), }
    }

    fn run(&self, value: u64) {
        println!("Hi!");
        
        let the_ref = self.count.clone();
        thread::spawn(move || {
            for _ in 0..value {
                let mut data = the_ref.lock().unwrap();
                *data += 1u64;
            }
            println!("done!");
        });
    }

    fn get_count(&self) -> u64{
        *self.count.lock().unwrap()
    }
}


// /// Formats the sum of two numbers as string.
// #[pyfunction]
// fn test_func(py: Python<'_>, a: u64) -> PyResult<u64> {
//     let count = py.allow_threads(|| count_to(a));
//     println!("AHHHHHHHHHHH");
    
//     Ok(count)
// }

// fn count_to (a: u64) -> u64 {
//     let mut count = 0u64;
//     for _ in (0..a) {
//         count += 1u64;
//     }
    
//     count
// }

/// A Python module implemented in Rust.
#[pymodule]
fn rustat(m: &Bound<'_, PyModule>) -> PyResult<()> {
    //m.add_function(wrap_pyfunction!(test_func, m)?)?;
    m.add_class::<test_class>()?;
    Ok(())
}
