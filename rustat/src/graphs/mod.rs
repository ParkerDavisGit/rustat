pub mod box_plot;
pub mod graph;

pub mod prelude {
    pub use crate::graphs;
    pub use crate::graphs::box_plot::BoxPlot;
    pub use crate::graphs::graph::Graph;
}
