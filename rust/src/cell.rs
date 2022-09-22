//! Cell definitions

/// A 0- to 3- dimensional reference cell
pub trait ReferenceCell {
    const DIM: u8;

    /// The dimension of the reference cell (eg a triangle's dimension is 2, tetrahedron's dimension is 3)
    fn dim(&self) -> u8 { Self::DIM }

    /// The vertices of the cell
    ///
    /// The first dim components represent the first vertex, the next dim the second vertex, and so on.
    fn vertices(&self) -> Vec<f64>;

    /// The edges of the cell
    ///
    /// The first 2 components are the vertex numbers of the endpoints of the first edge, the next 2 the second edge, and so on.
    fn edges(&self) -> Vec<u8>;

    /// The faces of the cell
    ///
    /// The first `self.faces_nvertices()[0]` components are the vertex numbers of vertices of the first face, the next `self.faces_nvertices()[1]` the second edge, and so on.
    fn faces(&self) -> Vec<u8>;

    /// The number of vertices adjacent to each face
    fn faces_nvertices(&self) -> Vec<u8>;

    /// The volumes of the cell
    ///
    /// The components are the vertex numbers of the vertices of the volume.
    fn volumes(&self) -> Vec<u8>;

    /// The number of vertices
    fn vertex_count(&self) -> u8 {
        (self.vertices().len() / Self::DIM as usize) as u8
    }

    /// The number of edges
    fn edge_count(&self) -> u8 {
        (self.edges().len() / 2) as u8
    }

    /// The number of faces
    fn face_count(&self) -> u8 {
        self.faces_nvertices().len() as u8
    }

    /// The number of volumes
    fn volume_count(&self) -> u8 {
        if self.volumes().len() > 0 {
            return 1;
        }
        0
    }
    // fn vertex(&self) -> Vec<f64>;
}

/// The reference triangle
struct Interval;

/// The reference triangle
struct Triangle;

/// The reference quadrilateral
struct Quadrilateral;

/// The reference tetrahedron
struct Tetrahedron;

/// The reference prism
struct Prism;

impl ReferenceCell for Interval {
    const DIM: u8 = 1;
    fn vertices(&self) -> Vec<f64> { vec![0.0, 1.0] }
    fn edges(&self) -> Vec<u8> { vec![0, 1] }
    fn faces(&self) -> Vec<u8> { vec![] }
    fn faces_nvertices(&self) -> Vec<u8> { vec![] }
    fn volumes(&self) -> Vec<u8> { vec![] }
}

impl ReferenceCell for Triangle {
    const DIM: u8 = 2;
    fn vertices(&self) -> Vec<f64> { vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0] }
    fn edges(&self) -> Vec<u8> { vec![1, 2, 0, 2, 0, 1] }
    fn faces(&self) -> Vec<u8> { vec![0, 1, 2] }
    fn faces_nvertices(&self) -> Vec<u8> { vec![3] }
    fn volumes(&self) -> Vec<u8> { vec![] }
}

impl ReferenceCell for Quadrilateral {
    const DIM: u8 = 2;
    fn vertices(&self) -> Vec<f64> { vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0] }
    fn edges(&self) -> Vec<u8> { vec![0, 1, 0, 2, 1, 3, 2, 3] }
    fn faces(&self) -> Vec<u8> { vec![0, 1, 2, 3] }
    fn faces_nvertices(&self) -> Vec<u8> { vec![4] }
    fn volumes(&self) -> Vec<u8> { vec![] }
}

impl ReferenceCell for Tetrahedron {
    const DIM: u8 = 3;
    fn vertices(&self) -> Vec<f64> { vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0] }
    fn edges(&self) -> Vec<u8> { vec![2, 3, 1, 3, 1, 2, 0, 3, 0, 2, 0, 1] }
    fn faces(&self) -> Vec<u8> { vec![1, 2, 3, 0, 2, 3, 0, 1, 3, 0, 1, 2] }
    fn faces_nvertices(&self) -> Vec<u8> { vec![3, 3, 3, 3] }
    fn volumes(&self) -> Vec<u8> { vec![0, 1, 2, 3] }
}

impl ReferenceCell for Prism {
    const DIM: u8 = 3;
    fn vertices(&self) -> Vec<f64> { vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 1.0, 1.0] }
    fn edges(&self) -> Vec<u8> { vec![0, 1, 0, 2, 0, 4, 1, 2, 1, 5, 2, 6, 3, 4, 3, 5, 4, 5] }
    fn faces(&self) -> Vec<u8> { vec![0, 1, 2, 0, 1, 3, 4, 0, 2, 3, 5, 1, 2, 4, 5, 3, 4, 5] }
    fn faces_nvertices(&self) -> Vec<u8> { vec![3, 4, 4, 4, 3] }
    fn volumes(&self) -> Vec<u8> { vec![0, 1, 2, 3] }
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

    #[test]
    fn test_interval() {
        let interval = Interval{};
        assert_eq!(interval.vertex_count(), 2);
        assert_eq!(interval.edge_count(), 1);
        assert_eq!(interval.face_count(), 0);
        assert_eq!(interval.volume_count(), 0);
    }

    #[test]
    fn test_triangle() {
        let triangle = Triangle{};
        assert_eq!(triangle.vertex_count(), 3);
        assert_eq!(triangle.edge_count(), 3);
        assert_eq!(triangle.face_count(), 1);
        assert_eq!(triangle.volume_count(), 0);
    }

    #[test]
    fn test_quadrilateral() {
        let quadrilateral = Quadrilateral{};
        assert_eq!(quadrilateral.vertex_count(), 4);
        assert_eq!(quadrilateral.edge_count(), 4);
        assert_eq!(quadrilateral.face_count(), 1);
        assert_eq!(quadrilateral.volume_count(), 0);
    }

    #[test]
    fn test_tetrahedron() {
        let tetrahedron = Tetrahedron{};
        assert_eq!(tetrahedron.vertex_count(), 4);
        assert_eq!(tetrahedron.edge_count(), 6);
        assert_eq!(tetrahedron.face_count(), 4);
        assert_eq!(tetrahedron.volume_count(), 1);
    }

    #[test]
    fn test_prism() {
        let prism = Prism{};
        assert_eq!(prism.vertex_count(), 6);
        assert_eq!(prism.edge_count(), 9);
        assert_eq!(prism.face_count(), 5);
        assert_eq!(prism.volume_count(), 1);
    }

}
