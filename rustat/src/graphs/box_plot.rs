use std::vec::Vec;
use pyo3::prelude::*;

use crate::graphs::graph::Graph;

//sort_unstable_by(|a, b| a.partial_cmp(b).unwrap())

#[pyclass]
pub struct BoxPlot {
    value: u64,
}

impl BoxPlot {
    pub fn new() -> Self {
        BoxPlot {
            value: 7u64,
        }
    }
}


impl Graph for BoxPlot {
    fn get_render_data(&self) -> Vec<Vec<String>> {
        let mut vec = Vec::new();
        let mut twoec = Vec::new();
        twoec.push("test".to_string());
        vec.push(twoec);
        println!("o");
        vec
    }
}