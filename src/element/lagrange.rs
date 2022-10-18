//! Lagrange elements

use crate::element::*;
use crate::cell::*;

/// The reference interval
pub struct LagrangeElement {
    pub celltype: ReferenceCellType,
    pub degree: usize,
}

impl FiniteElement for LagrangeElement {
    const VALUE_SIZE: usize = 1;
    fn cell_type(&self) -> ReferenceCellType {self.celltype}
    fn degree(&self) -> usize { self.degree }
    fn highest_degree(&self) -> usize { self.degree }
    fn family(&self) -> ElementFamily { ElementFamily::Lagrange }
    fn discontinuous(&self) -> bool { false }
}
