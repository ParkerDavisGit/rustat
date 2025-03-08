//use std::fmt::Display;

pub trait Graph {
    fn get_render_data(&self) -> Vec<Vec<String>>;
}