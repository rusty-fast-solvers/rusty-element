use ndarray::{arr2, Array2};

pub trait Geometry {}

pub trait Topology {
    fn vertex_count(&self) -> u8;
    fn edge_count(&self) -> u8;
    fn face_count(&self) -> u8 { 1 }
}

pub struct Cell {
    pub vertices: Array2<f64>,
    pub edges: Array2<u8>,
}

impl Geometry for Cell {}

impl Topology for Cell {
    fn vertex_count(&self) -> u8 {
        self.vertices.shape()[0] as u8
    }
    fn edge_count(&self) -> u8 {
        self.edges.shape()[0] as u8
    }
}

pub fn make_triangle() -> Cell {
    Cell {
        vertices: arr2(&[[0.0, 0.0], [1.0, 0.0], [0.0, 1.0]]),
        edges: arr2(&[[1, 2], [0, 2], [0, 1]]),
    }
}

pub fn make_quadrilateral() -> Cell {
    Cell {
        vertices: arr2(&[[0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [1.0, 1.0]]),
        edges: arr2(&[[0, 1], [0, 2], [1, 3], [2, 3]]),
    }
}

pub fn add(a: i64, b: i64) -> i64 {
    a + b
}

#[cfg(test)]
mod test {
    use crate::cell::*;

    #[test]
    fn test_add() {
        assert_eq!(4, add(2, 2));
    }

    fn test_cell(c: Cell) {
        assert_eq!(usize::from(c.vertex_count()), c.vertices.shape()[0]);
        assert_eq!(usize::from(c.edge_count()), c.edges.shape()[0]);
        assert_eq!(usize::from(c.face_count()), 1);
    }

    #[test]
    fn test_triangle() {
        let triangle = make_triangle();
        assert_eq!(triangle.vertex_count(), 3);
        test_cell(triangle);
    }

    #[test]
    fn test_quadrilateral() {
        let quadrilateral = make_quadrilateral();
        assert_eq!(quadrilateral.vertex_count(), 4);
        test_cell(quadrilateral);
    }
}
