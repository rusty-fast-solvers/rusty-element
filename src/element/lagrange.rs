//! Lagrange elements

use crate::element::*;
use crate::cell::*;

/// The reference interval
pub struct LagrangeElement {
    pub celltype: ReferenceCellType,
    pub degree: usize,
}

impl FiniteElement for LagrangeElement {
}
