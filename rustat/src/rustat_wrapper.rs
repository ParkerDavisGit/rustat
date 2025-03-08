use pyo3::prelude::*;

use std::collections::HashMap;

use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use crate::graphs::prelude::*;

#[pyclass]
pub struct TestClass {
    file_name: String,
    count: Arc<Mutex<u64>>,
    dict: HashMap<String, usize>,
    plot: graphs::box_plot::BoxPlot,
}

#[pymethods]
impl TestClass {
    #[new]
    pub fn new(file_name: String) -> Self {
        TestClass { 
            file_name: file_name, 
            count: Arc::new(Mutex::new(0u64)), 
            dict: HashMap::new(),
            plot: graphs::box_plot::BoxPlot::new(),
        }
    }

    fn run(&self, value: u64) {
        let the_ref = self.count.clone();
        thread::spawn(move || {
            counter(value, the_ref);
        });
        self.plot.get_render_data();
    }

    fn get_count(&self) -> u64{
        *self.count.lock().unwrap()
    }

    fn set_in_dict(&mut self, key: String, value: usize) {
        self.dict.insert(key, value);
    }

    fn get_from_dict(&self, key: String) -> usize {
        *self.dict.get(&key).unwrap()
    }
}

fn counter(value: u64, the_ref: Arc<Mutex<u64>>) {
    for _ in 0..value {
        *the_ref.lock().unwrap() += 1u64;
    }
}