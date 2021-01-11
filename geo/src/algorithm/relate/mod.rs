mod edge_end_builder;
mod intersection_matrix;
pub mod relate_computer;
mod relate_node;

pub(crate) use edge_end_builder::EdgeEndBuilder;
pub(crate) use intersection_matrix::IntersectionMatrix;
pub(crate) use relate_node::RelateNodeFactory;

use crate::{GeoFloat, Geometry};

pub trait Relate<F, T> {
    fn relate(&self, other: &T) -> IntersectionMatrix;
}

impl<F: GeoFloat> Relate<F, Geometry<F>> for Geometry<F> {
    fn relate(&self, other: &Geometry<F>) -> IntersectionMatrix {
        let relate_computer = relate_computer::RelateComputer::new(self, other);
        relate_computer.run()
    }
}