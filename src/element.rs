//! Finite Element definitions

use crate::cell;
pub mod lagrange;
pub use lagrange::*;

/// A 0- to 3- dimensional reference cell
pub trait FiniteElement {
}

#[cfg(test)]
mod test {
    use crate::cell::*;
    use crate::element::*;

    #[test]
    fn test_lagrange_1() {
        let e = LagrangeElement{
            celltype: ReferenceCellType::Triangle,
            degree: 1
        };
    }
}
