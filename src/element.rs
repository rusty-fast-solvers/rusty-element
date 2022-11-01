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
pub trait FiniteElement: Sized {
    const VALUE_SIZE: usize;
    fn cell_type(&self) -> ReferenceCellType;
    fn degree(&self) -> usize;
    fn highest_degree(&self) -> usize;
    fn family(&self) -> ElementFamily;
    fn dim(&self) -> usize;
    fn discontinuous(&self) -> bool;

    fn value_size(&self) -> usize {
        Self::VALUE_SIZE
    }

    fn tabulate(&self, points: &[f64], nderivs: usize, data: &mut TabulatedData<Self>);
}

pub struct TabulatedData<'a, F: FiniteElement> {
    data: Vec<f64>,
    element: &'a F,
    deriv_count: usize,
    point_count: usize,
    basis_count: usize,
    value_size: usize,
}

impl<'a, F: FiniteElement> TabulatedData<'a, F> {
    pub fn new(element: &'a F, nderivs: usize, npoints: usize) -> Self {
        let deriv_count = (nderivs + 1) * (nderivs + 2) / 2; // 2D is hardcoded here
        let point_count = npoints;
        let basis_count = element.dim();
        let value_size = element.value_size();
        let data = vec![0.0; deriv_count * point_count * basis_count * value_size];
        Self {
            data,
            element,
            deriv_count,
            point_count,
            basis_count,
            value_size,
        }
    }

    pub fn get_mut(
        &mut self,
        deriv: usize,
        point: usize,
        basis: usize,
        component: usize,
    ) -> &mut f64 {
        /// Debug here
        let index = ((deriv * self.point_count + point) * self.basis_count + basis)
            * self.value_size
            + component;
        self.data.get_mut(index).unwrap()
    }

    pub fn get(&mut self, deriv: usize, point: usize, basis: usize, component: usize) -> &f64 {
        /// Debug here
        let index = ((deriv * self.point_count + point) * self.basis_count + basis)
            * self.value_size
            + component;
        self.data.get(index).unwrap()
    }

    pub fn deriv_count(&self) -> usize {
        self.deriv_count
    }
    pub fn point_count(&self) -> usize {
        self.point_count
    }
    pub fn basis_count(&self) -> usize {
        self.basis_count
    }
    pub fn value_size(&self) -> usize {
        self.value_size
    }
}

#[cfg(test)]
mod test {
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
