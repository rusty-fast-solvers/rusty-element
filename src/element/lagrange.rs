//! Lagrange elements

use crate::cell::*;
use crate::element::*;

/// Lagrange element
pub struct LagrangeElement {
    pub celltype: ReferenceCellType,
    pub degree: usize,
}

impl FiniteElement for LagrangeElement {
    const VALUE_SIZE: usize = 1;
    fn cell_type(&self) -> ReferenceCellType {
        self.celltype
    }
    fn degree(&self) -> usize {
        self.degree
    }
    fn highest_degree(&self) -> usize {
        self.degree
    }
    fn family(&self) -> ElementFamily {
        ElementFamily::Lagrange
    }
    fn discontinuous(&self) -> bool {
        false
    }
    fn dim(&self) -> usize {
        unimplemented!("dim not yet implemented for this element");
    }
    fn tabulate(&self, points: &[f64], nderivs: usize, mut data: &mut TabulatedData<Self>) {
        unimplemented!("tabulate not yet implemented for this element");
    }
}

/// Degree 1 Lagrange element on an interval
pub struct LagrangeElementIntervalDegree1 {}

impl FiniteElement for LagrangeElementIntervalDegree1 {
    const VALUE_SIZE: usize = 1;
    fn cell_type(&self) -> ReferenceCellType {
        ReferenceCellType::Interval
    }
    fn degree(&self) -> usize {
        1
    }
    fn highest_degree(&self) -> usize {
        1
    }
    fn family(&self) -> ElementFamily {
        ElementFamily::Lagrange
    }
    fn discontinuous(&self) -> bool {
        false
    }
    fn dim(&self) -> usize {
        2
    }
    fn tabulate(&self, points: &[f64], nderivs: usize, mut data: &mut TabulatedData<Self>) {
        // Basis functions are 1-x-y, x, y
        for deriv in 0..data.deriv_count() {
            for pt in 0..data.point_count() {
                if deriv == 0 {
                    *data.get_mut(deriv, pt, 0, 0) = 1.0 - points[pt];
                    *data.get_mut(deriv, pt, 1, 0) = points[pt];
                } else if deriv == 1 {
                    *data.get_mut(deriv, pt, 0, 0) = -1.0;
                    *data.get_mut(deriv, pt, 1, 0) = 1.0;
                } else {
                    for fun in 0..2 {
                        *data.get_mut(deriv, pt, fun, 0) = 0.;
                    }
                }
            }
        }
    }
}

/// Degree 1 Lagrange element on a triangle
pub struct LagrangeElementTriangleDegree1 {}

impl FiniteElement for LagrangeElementTriangleDegree1 {
    const VALUE_SIZE: usize = 1;
    fn cell_type(&self) -> ReferenceCellType {
        ReferenceCellType::Triangle
    }
    fn degree(&self) -> usize {
        1
    }
    fn highest_degree(&self) -> usize {
        1
    }
    fn family(&self) -> ElementFamily {
        ElementFamily::Lagrange
    }
    fn discontinuous(&self) -> bool {
        false
    }
    fn dim(&self) -> usize {
        3
    }
    fn tabulate(&self, points: &[f64], nderivs: usize, mut data: &mut TabulatedData<Self>) {
        // Basis functions are 1-x-y, x, y
        for deriv in 0..data.deriv_count() {
            for pt in 0..data.point_count() {
                if deriv == 0 {
                    *data.get_mut(deriv, pt, 0, 0) = 1.0 - points[2 * pt] - points[2 * pt + 1];
                    *data.get_mut(deriv, pt, 1, 0) = points[2 * pt];
                    *data.get_mut(deriv, pt, 2, 0) = points[2 * pt + 1];
                } else if deriv == 1 {
                    *data.get_mut(deriv, pt, 0, 0) = -1.0;
                    *data.get_mut(deriv, pt, 1, 0) = 1.0;
                    *data.get_mut(deriv, pt, 2, 0) = 0.0;
                } else if deriv == 2 {
                    *data.get_mut(deriv, pt, 0, 0) = -1.0;
                    *data.get_mut(deriv, pt, 1, 0) = 0.0;
                    *data.get_mut(deriv, pt, 2, 0) = 1.0;
                } else {
                    for fun in 0..3 {
                        *data.get_mut(deriv, pt, fun, 0) = 0.;
                    }
                }
            }
        }
    }
}

/// Degree 1 Lagrange element on a quadrilateral
pub struct LagrangeElementQuadrilateralDegree1 {}

impl FiniteElement for LagrangeElementQuadrilateralDegree1 {
    const VALUE_SIZE: usize = 1;
    fn cell_type(&self) -> ReferenceCellType {
        ReferenceCellType::Quadrilateral
    }
    fn degree(&self) -> usize {
        1
    }
    fn highest_degree(&self) -> usize {
        1
    }
    fn family(&self) -> ElementFamily {
        ElementFamily::Lagrange
    }
    fn discontinuous(&self) -> bool {
        false
    }
    fn dim(&self) -> usize {
        4
    }
    fn tabulate(&self, points: &[f64], nderivs: usize, mut data: &mut TabulatedData<Self>) {
        // Basis functions are (1-x)(1-y), x(1-y), (1-x)y, xy
        for deriv in 0..data.deriv_count() {
            for pt in 0..data.point_count() {
                if deriv == 0 {
                    *data.get_mut(deriv, pt, 0, 0) =
                        (1.0 - points[2 * pt]) * (1.0 - points[2 * pt + 1]);
                    *data.get_mut(deriv, pt, 1, 0) = points[2 * pt] * (1.0 - points[2 * pt + 1]);
                    *data.get_mut(deriv, pt, 2, 0) = (1.0 - points[2 * pt]) * points[2 * pt + 1];
                    *data.get_mut(deriv, pt, 3, 0) = points[2 * pt] * points[2 * pt + 1];
                } else if deriv == 1 {
                    // d/dx
                    *data.get_mut(deriv, pt, 0, 0) = points[2 * pt + 1] - 1.0;
                    *data.get_mut(deriv, pt, 1, 0) = 1.0 - points[2 * pt + 1];
                    *data.get_mut(deriv, pt, 2, 0) = -points[2 * pt + 1];
                    *data.get_mut(deriv, pt, 3, 0) = points[2 * pt + 1];
                } else if deriv == 2 {
                    // d/dy
                    *data.get_mut(deriv, pt, 0, 0) = points[2 * pt] - 1.0;
                    *data.get_mut(deriv, pt, 1, 0) = -points[2 * pt];
                    *data.get_mut(deriv, pt, 2, 0) = 1.0 - points[2 * pt];
                    *data.get_mut(deriv, pt, 3, 0) = points[2 * pt];
                } else if deriv == 4 {
                    // d2/dxdy
                    *data.get_mut(deriv, pt, 0, 0) = 1.0;
                    *data.get_mut(deriv, pt, 1, 0) = -1.0;
                    *data.get_mut(deriv, pt, 2, 0) = -1.0;
                    *data.get_mut(deriv, pt, 3, 0) = 1.0;
                } else {
                    for fun in 0..3 {
                        *data.get_mut(deriv, pt, fun, 0) = 0.;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::element::*;
    use approx::*;

    #[test]
    fn test_lagrange_1_interval() {
        let e = LagrangeElementIntervalDegree1 {};
        assert_eq!(e.value_size(), 1);
        let mut data = TabulatedData::new(&e, 0, 4);
        let points = vec![0.0, 0.2, 0.4, 1.0];
        e.tabulate(&points, 0, &mut data);

        assert_relative_eq!(*data.get(0, 0, 0, 0), 1.0);
        assert_relative_eq!(*data.get(0, 0, 1, 0), 0.0);
        assert_relative_eq!(*data.get(0, 1, 0, 0), 0.8);
        assert_relative_eq!(*data.get(0, 1, 1, 0), 0.2);
        assert_relative_eq!(*data.get(0, 2, 0, 0), 0.6);
        assert_relative_eq!(*data.get(0, 2, 1, 0), 0.4);
        assert_relative_eq!(*data.get(0, 3, 0, 0), 0.0);
        assert_relative_eq!(*data.get(0, 3, 1, 0), 1.0);
    }

    #[test]
    fn test_lagrange_1_triangle() {
        let e = LagrangeElementTriangleDegree1 {};
        assert_eq!(e.value_size(), 1);
        let mut data = TabulatedData::new(&e, 0, 6);
        let points = vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.5, 0.0, 0.0, 0.5, 0.5, 0.5];
        e.tabulate(&points, 0, &mut data);

        assert_relative_eq!(*data.get(0, 0, 0, 0), 1.0);
        assert_relative_eq!(*data.get(0, 0, 1, 0), 0.0);
        assert_relative_eq!(*data.get(0, 0, 2, 0), 0.0);
        assert_relative_eq!(*data.get(0, 1, 0, 0), 0.0);
        assert_relative_eq!(*data.get(0, 1, 1, 0), 1.0);
        assert_relative_eq!(*data.get(0, 1, 2, 0), 0.0);
        assert_relative_eq!(*data.get(0, 2, 0, 0), 0.0);
        assert_relative_eq!(*data.get(0, 2, 1, 0), 0.0);
        assert_relative_eq!(*data.get(0, 2, 2, 0), 1.0);
        assert_relative_eq!(*data.get(0, 3, 0, 0), 0.5);
        assert_relative_eq!(*data.get(0, 3, 1, 0), 0.5);
        assert_relative_eq!(*data.get(0, 3, 2, 0), 0.0);
        assert_relative_eq!(*data.get(0, 4, 0, 0), 0.5);
        assert_relative_eq!(*data.get(0, 4, 1, 0), 0.0);
        assert_relative_eq!(*data.get(0, 4, 2, 0), 0.5);
        assert_relative_eq!(*data.get(0, 5, 0, 0), 0.0);
        assert_relative_eq!(*data.get(0, 5, 1, 0), 0.5);
        assert_relative_eq!(*data.get(0, 5, 2, 0), 0.5);
    }

    #[test]
    fn test_lagrange_1_quadrilateral() {
        let e = LagrangeElementQuadrilateralDegree1 {};
        assert_eq!(e.value_size(), 1);
        let mut data = TabulatedData::new(&e, 0, 6);
        let points = vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0.25, 0.5, 0.3, 0.2];
        e.tabulate(&points, 0, &mut data);

        assert_relative_eq!(*data.get(0, 0, 0, 0), 1.0);
        assert_relative_eq!(*data.get(0, 0, 1, 0), 0.0);
        assert_relative_eq!(*data.get(0, 0, 2, 0), 0.0);
        assert_relative_eq!(*data.get(0, 0, 3, 0), 0.0);
        assert_relative_eq!(*data.get(0, 1, 0, 0), 0.0);
        assert_relative_eq!(*data.get(0, 1, 1, 0), 1.0);
        assert_relative_eq!(*data.get(0, 1, 2, 0), 0.0);
        assert_relative_eq!(*data.get(0, 1, 3, 0), 0.0);
        assert_relative_eq!(*data.get(0, 2, 0, 0), 0.0);
        assert_relative_eq!(*data.get(0, 2, 1, 0), 0.0);
        assert_relative_eq!(*data.get(0, 2, 2, 0), 1.0);
        assert_relative_eq!(*data.get(0, 2, 3, 0), 0.0);
        assert_relative_eq!(*data.get(0, 3, 0, 0), 0.0);
        assert_relative_eq!(*data.get(0, 3, 1, 0), 0.0);
        assert_relative_eq!(*data.get(0, 3, 2, 0), 0.0);
        assert_relative_eq!(*data.get(0, 3, 3, 0), 1.0);
        assert_relative_eq!(*data.get(0, 4, 0, 0), 0.375);
        assert_relative_eq!(*data.get(0, 4, 1, 0), 0.125);
        assert_relative_eq!(*data.get(0, 4, 2, 0), 0.375);
        assert_relative_eq!(*data.get(0, 4, 3, 0), 0.125);
        assert_relative_eq!(*data.get(0, 5, 0, 0), 0.56);
        assert_relative_eq!(*data.get(0, 5, 1, 0), 0.24);
        assert_relative_eq!(*data.get(0, 5, 2, 0), 0.14);
        assert_relative_eq!(*data.get(0, 5, 3, 0), 0.06);
    }
}
