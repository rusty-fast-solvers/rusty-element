//! Push forward and pull back maps

use crate::element::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[repr(u8)]
pub enum MapType {
    Identity = 0,
    CovariantPiola = 1,
    ContravariantPiola = 2,
    L2Piola = 3,
}

pub fn identity_push_forward<'a, F: FiniteElement>(data: &mut TabulatedData<'a, F>) {
    assert_eq!(data.deriv_count(), 1);
}

pub fn identity_pull_back<'a, F: FiniteElement>(data: &mut TabulatedData<'a, F>) {
    assert_eq!(data.deriv_count(), 1);
}

#[cfg(test)]
mod test {
    use crate::element::*;
    use crate::map::*;
    use approx::*;

    #[test]
    fn test_identity() {
        let e = LagrangeElementTriangleDegree1 {};
        let mut data = TabulatedData::new(&e, 0, 1);

        *data.get_mut(0, 0, 0, 0) = 0.5;
        *data.get_mut(0, 0, 1, 0) = 0.4;
        *data.get_mut(0, 0, 2, 0) = 0.3;

        identity_push_forward(&mut data);

        assert_relative_eq!(*data.get(0, 0, 0, 0), 0.5);
        assert_relative_eq!(*data.get(0, 0, 1, 0), 0.4);
        assert_relative_eq!(*data.get(0, 0, 2, 0), 0.3);
    }
}
