//! Cell definitions

use more_asserts as ma;

/// A 0- to 3- dimensional reference cell
pub trait ReferenceCell {
    const DIM: usize;

    /// The dimension of the reference cell (eg a triangle's dimension is 2, tetrahedron's dimension is 3)
    fn dim(&self) -> usize {
        Self::DIM
    }

    /// The vertices of the cell
    ///
    /// The first dim components represent the first vertex, the next dim the second vertex, and so on.
    fn vertices(&self) -> Vec<f64>;

    /// The edges of the cell
    ///
    /// The first 2 components are the vertex numbers of the endpoints of the first edge, the next 2 the second edge, and so on.
    fn edges(&self) -> Vec<usize>;

    /// The faces of the cell
    ///
    /// The first `self.faces_nvertices()[0]` components are the vertex numbers of vertices of the first face, the next `self.faces_nvertices()[1]` the second edge, and so on.
    fn faces(&self) -> Vec<usize>;

    /// The number of vertices adjacent to each face
    fn faces_nvertices(&self) -> Vec<usize>;

    /// The volumes of the cell
    ///
    /// The components are the vertex numbers of the vertices of the volume.
    fn volumes(&self) -> Vec<usize>;

    /// The number of vertices
    fn vertex_count(&self) -> usize {
        self.vertices().len() / Self::DIM
    }

    /// The number of edges
    fn edge_count(&self) -> usize {
        self.edges().len() / 2
    }

    /// The number of faces
    fn face_count(&self) -> usize {
        self.faces_nvertices().len()
    }

    /// The number of volumes
    fn volume_count(&self) -> usize {
        if self.volumes().len() > 0 {
            return 1;
        }
        0
    }

    /// Get the `n`th vertex of the cell.
    ///
    /// * `n` - The vertex number
    fn vertex(&self, n: usize) -> Vec<f64> {
        ma::debug_assert_lt!(n, self.vertex_count());
        self.vertices()[n * Self::DIM..(n + 1) * Self::DIM].to_vec()
    }

    /// Get the `n`th edge of the cell.
    ///
    /// * `n` - The edge number
    fn edge(&self, n: usize) -> Vec<usize> {
        ma::debug_assert_lt!(n, self.edge_count());
        self.edges()[n * 2..(n + 1) * 2].to_vec()
    }

    /// Get the `n`th face of the cell.
    ///
    /// * `n` - The face number
    fn face(&self, n: usize) -> Vec<usize> {
        ma::debug_assert_lt!(n, self.face_count());
        let mut start = 0;
        let nv = self.faces_nvertices();
        for i in 1..n {
            start += nv[i];
        }
        self.faces()[start..start + nv[n]].to_vec()
    }

    /// Get the `n`th volume of the cell.
    ///
    /// * `n` - The volume number
    fn volume(&self, n: usize) -> Vec<usize> {
        ma::debug_assert_lt!(n, self.volume_count());
        self.volumes()
    }
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
    const DIM: usize = 1;
    fn vertices(&self) -> Vec<f64> {
        vec![0.0, 1.0]
    }
    fn edges(&self) -> Vec<usize> {
        vec![0, 1]
    }
    fn faces(&self) -> Vec<usize> {
        vec![]
    }
    fn faces_nvertices(&self) -> Vec<usize> {
        vec![]
    }
    fn volumes(&self) -> Vec<usize> {
        vec![]
    }
}

impl ReferenceCell for Triangle {
    const DIM: usize = 2;
    fn vertices(&self) -> Vec<f64> {
        vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0]
    }
    fn edges(&self) -> Vec<usize> {
        vec![1, 2, 0, 2, 0, 1]
    }
    fn faces(&self) -> Vec<usize> {
        vec![0, 1, 2]
    }
    fn faces_nvertices(&self) -> Vec<usize> {
        vec![3]
    }
    fn volumes(&self) -> Vec<usize> {
        vec![]
    }
}

impl ReferenceCell for Quadrilateral {
    const DIM: usize = 2;
    fn vertices(&self) -> Vec<f64> {
        vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0]
    }
    fn edges(&self) -> Vec<usize> {
        vec![0, 1, 0, 2, 1, 3, 2, 3]
    }
    fn faces(&self) -> Vec<usize> {
        vec![0, 1, 2, 3]
    }
    fn faces_nvertices(&self) -> Vec<usize> {
        vec![4]
    }
    fn volumes(&self) -> Vec<usize> {
        vec![]
    }
}

impl ReferenceCell for Tetrahedron {
    const DIM: usize = 3;
    fn vertices(&self) -> Vec<f64> {
        vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]
    }
    fn edges(&self) -> Vec<usize> {
        vec![2, 3, 1, 3, 1, 2, 0, 3, 0, 2, 0, 1]
    }
    fn faces(&self) -> Vec<usize> {
        vec![1, 2, 3, 0, 2, 3, 0, 1, 3, 0, 1, 2]
    }
    fn faces_nvertices(&self) -> Vec<usize> {
        vec![3, 3, 3, 3]
    }
    fn volumes(&self) -> Vec<usize> {
        vec![0, 1, 2, 3]
    }
}

impl ReferenceCell for Prism {
    const DIM: usize = 3;
    fn vertices(&self) -> Vec<f64> {
        vec![
            0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 1.0,
            1.0,
        ]
    }
    fn edges(&self) -> Vec<usize> {
        vec![0, 1, 0, 2, 0, 4, 1, 2, 1, 5, 2, 6, 3, 4, 3, 5, 4, 5]
    }
    fn faces(&self) -> Vec<usize> {
        vec![0, 1, 2, 0, 1, 3, 4, 0, 2, 3, 5, 1, 2, 4, 5, 3, 4, 5]
    }
    fn faces_nvertices(&self) -> Vec<usize> {
        vec![3, 4, 4, 4, 3]
    }
    fn volumes(&self) -> Vec<usize> {
        vec![0, 1, 2, 3, 4, 5]
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

    #[test]
    fn test_interval() {
        let interval = Interval {};
        assert_eq!(interval.vertex_count(), 2);
        assert_eq!(interval.edge_count(), 1);
        assert_eq!(interval.face_count(), 0);
        assert_eq!(interval.volume_count(), 0);

        assert_eq!(interval.vertex(0).len(), 1);
        assert_eq!(interval.vertex(1).len(), 1);
        assert_eq!(interval.edge(0).len(), 2);
    }

    #[test]
    fn test_triangle() {
        let triangle = Triangle {};
        assert_eq!(triangle.vertex_count(), 3);
        assert_eq!(triangle.edge_count(), 3);
        assert_eq!(triangle.face_count(), 1);
        assert_eq!(triangle.volume_count(), 0);

        assert_eq!(triangle.vertex(0).len(), 2);
        assert_eq!(triangle.vertex(1).len(), 2);
        assert_eq!(triangle.vertex(2).len(), 2);
        assert_eq!(triangle.edge(0).len(), 2);
        assert_eq!(triangle.edge(1).len(), 2);
        assert_eq!(triangle.edge(2).len(), 2);
        assert_eq!(triangle.face(0).len(), 3);
    }

    #[test]
    fn test_quadrilateral() {
        let quadrilateral = Quadrilateral {};
        assert_eq!(quadrilateral.vertex_count(), 4);
        assert_eq!(quadrilateral.edge_count(), 4);
        assert_eq!(quadrilateral.face_count(), 1);
        assert_eq!(quadrilateral.volume_count(), 0);

        assert_eq!(quadrilateral.vertex(0).len(), 2);
        assert_eq!(quadrilateral.vertex(1).len(), 2);
        assert_eq!(quadrilateral.vertex(2).len(), 2);
        assert_eq!(quadrilateral.vertex(3).len(), 2);
        assert_eq!(quadrilateral.edge(0).len(), 2);
        assert_eq!(quadrilateral.edge(1).len(), 2);
        assert_eq!(quadrilateral.edge(2).len(), 2);
        assert_eq!(quadrilateral.edge(3).len(), 2);
        assert_eq!(quadrilateral.face(0).len(), 4);
    }

    #[test]
    fn test_tetrahedron() {
        let tetrahedron = Tetrahedron {};
        assert_eq!(tetrahedron.vertex_count(), 4);
        assert_eq!(tetrahedron.edge_count(), 6);
        assert_eq!(tetrahedron.face_count(), 4);
        assert_eq!(tetrahedron.volume_count(), 1);

        assert_eq!(tetrahedron.vertex(0).len(), 3);
        assert_eq!(tetrahedron.vertex(1).len(), 3);
        assert_eq!(tetrahedron.vertex(2).len(), 3);
        assert_eq!(tetrahedron.vertex(3).len(), 3);
        assert_eq!(tetrahedron.edge(0).len(), 2);
        assert_eq!(tetrahedron.edge(1).len(), 2);
        assert_eq!(tetrahedron.edge(2).len(), 2);
        assert_eq!(tetrahedron.edge(3).len(), 2);
        assert_eq!(tetrahedron.edge(4).len(), 2);
        assert_eq!(tetrahedron.edge(5).len(), 2);
        assert_eq!(tetrahedron.face(0).len(), 3);
        assert_eq!(tetrahedron.face(1).len(), 3);
        assert_eq!(tetrahedron.face(2).len(), 3);
        assert_eq!(tetrahedron.face(3).len(), 3);
        assert_eq!(tetrahedron.volume(0).len(), 4);
    }

    #[test]
    fn test_prism() {
        let prism = Prism {};
        assert_eq!(prism.vertex_count(), 6);
        assert_eq!(prism.edge_count(), 9);
        assert_eq!(prism.face_count(), 5);
        assert_eq!(prism.volume_count(), 1);

        assert_eq!(prism.vertex(0).len(), 3);
        assert_eq!(prism.vertex(1).len(), 3);
        assert_eq!(prism.vertex(2).len(), 3);
        assert_eq!(prism.vertex(3).len(), 3);
        assert_eq!(prism.edge(0).len(), 2);
        assert_eq!(prism.edge(1).len(), 2);
        assert_eq!(prism.edge(2).len(), 2);
        assert_eq!(prism.edge(3).len(), 2);
        assert_eq!(prism.edge(4).len(), 2);
        assert_eq!(prism.edge(5).len(), 2);
        assert_eq!(prism.edge(6).len(), 2);
        assert_eq!(prism.edge(7).len(), 2);
        assert_eq!(prism.edge(8).len(), 2);
        assert_eq!(prism.face(0).len(), 3);
        assert_eq!(prism.face(1).len(), 4);
        assert_eq!(prism.face(2).len(), 4);
        assert_eq!(prism.face(3).len(), 4);
        assert_eq!(prism.face(4).len(), 3);
        assert_eq!(prism.volume(0).len(), 6);
    }
}
