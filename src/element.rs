//! Finite Element definitions

use crate::cell::*;
pub mod lagrange;
pub use lagrange::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[repr(u8)]
pub enum ElementFamily {
    Lagrange = 0,
}

/// A finite element
pub trait FiniteElement {
    const VALUE_SIZE: usize;
    fn cell_type(&self) -> ReferenceCellType;
    fn degree(&self) -> usize;
    fn highest_degree(&self) -> usize;
    fn family(&self) -> ElementFamily;
    fn discontinuous(&self) -> bool;

    fn value_size(&self) -> usize {
        Self::VALUE_SIZE
    }

    // fn tabulate(&self, points: &[f64], nderivs: usize) -> &[f64];
}

#[cfg(test)]
mod test {
    use crate::cell::*;
    use crate::element::*;

    #[test]
    fn test_lagrange_1() {
        let e = LagrangeElement {
            celltype: ReferenceCellType::Triangle,
            degree: 1,
        };
        assert_eq!(e.value_size(), 1);
    }
}
