//! Cell definitions

pub mod cells_1d;
pub use cells_1d::*;
pub mod cells_2d;
pub use cells_2d::*;

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
    fn vertices(&self) -> &[f64];

    /// The edges of the cell
    ///
    /// The first 2 components are the vertex numbers of the endpoints of the first edge, the next 2 the second edge, and so on.
    fn edges(&self) -> &[usize];

    /// The faces of the cell
    ///
    /// The first `self.faces_nvertices()[0]` components are the vertex numbers of vertices of the first face, the next `self.faces_nvertices()[1]` the second edge, and so on.
    fn faces(&self) -> &[usize];

    /// The number of vertices adjacent to each face
    fn faces_nvertices(&self) -> &[usize];

    /// The number of vertices
    fn vertex_count(&self) -> usize;

    /// The number of edges
    fn edge_count(&self) -> usize;

    /// The number of faces
    fn face_count(&self) -> usize;

    /// The number of volumes
    fn volume_count(&self) -> usize;

    /// Get the entities connected to an entity
    ///
    /// This function returns a list of entity numbers of entities of dimension CONNECTED_DIM that are attached to the entity numbered ENTITY_DIM of number entity_number.
    /// For example connectivity<1, 2>(0) will return a list of faces (2D entities) that are connected to edge (1D entity) 0.
    fn connectivity<const ENTITY_DIM: usize, const CONNECTED_DIM: usize>(
        &self,
        entity_number: usize,
    ) -> Result<Vec<usize>, ()>;
}

/*
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
    fn vertex_count(&self) -> usize {
        4
    }
    fn edge_count(&self) -> usize {
        6
    }
    fn face_count(&self) -> usize {
        4
    }
    fn volume_count(&self) -> usize {
        1
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
    fn vertex_count(&self) -> usize {
        6
    }
    fn edge_count(&self) -> usize {
        9
    }
    fn face_count(&self) -> usize {
        5
    }
    fn volume_count(&self) -> usize {
        1
    }
}
*/

#[cfg(test)]
mod test {
    use crate::cell::*;

    fn cell_tester(c: impl ReferenceCell) {
        assert_eq!(c.vertex_count(), c.vertices().len() / c.dim());
        assert_eq!(c.edge_count(), c.edges().len() / 2);
        assert_eq!(c.face_count(), c.faces_nvertices().len());

        for i in 0..c.vertex_count() {
            let v = c.connectivity::<0, 0>(i).unwrap();
            assert_eq!(v.len(), 1);
            assert_eq!(v[0], i);
            if c.dim() >= 1 {
                let e = c.connectivity::<0, 1>(i).unwrap();
                for j in e {
                    let ev = c.connectivity::<1, 0>(j).unwrap();
                    assert!(ev.contains(&i));
                }
            }
            if c.dim() >= 2 {
                let f = c.connectivity::<0, 2>(i).unwrap();
                for j in f {
                    let fv = c.connectivity::<2, 0>(j).unwrap();
                    assert!(fv.contains(&i));
                }
            }
            if c.dim() >= 3 {
                let w = c.connectivity::<0, 3>(i).unwrap();
                for j in w {
                    let wv = c.connectivity::<3, 0>(j).unwrap();
                    assert!(wv.contains(&i));
                }
            }
        }
        if c.dim() >= 1 {
            for i in 0..c.edge_count() {
                let e = c.connectivity::<1, 1>(i).unwrap();
                assert_eq!(e.len(), 1);
                assert_eq!(e[0], i);
                let ev = c.connectivity::<1, 0>(i).unwrap();
                if c.dim() >= 2 {
                    let f = c.connectivity::<1, 2>(i).unwrap();
                    for j in f {
                        let fv = c.connectivity::<2, 0>(j).unwrap();
                        assert!(fv.contains(&ev[0]));
                        assert!(fv.contains(&ev[1]));
                    }
                }
                if c.dim() >= 3 {
                    let w = c.connectivity::<1, 3>(i).unwrap();
                    for j in w {
                        let wv = c.connectivity::<3, 0>(j).unwrap();
                        assert!(wv.contains(&ev[0]));
                        assert!(wv.contains(&ev[1]));
                    }
                }
            }
        }
        if c.dim() >= 2 {
            for i in 0..c.face_count() {
                let f = c.connectivity::<2, 2>(i).unwrap();
                assert_eq!(f.len(), 1);
                assert_eq!(f[0], i);
                let fv = c.connectivity::<2, 0>(i).unwrap();
                if c.dim() >= 3 {
                    let w = c.connectivity::<2, 3>(i).unwrap();
                    for j in w {
                        let wv = c.connectivity::<3, 0>(j).unwrap();
                        for v in &fv {
                            assert!(wv.contains(&v));
                        }
                    }
                }
            }
        }
        if c.dim() >= 3 {
            let w = c.connectivity::<3, 3>(0).unwrap();
            assert_eq!(w.len(), 1);
            assert_eq!(w[0], 0);
        }
    }

    #[test]
    fn test_interval() {
        let i = Interval {};
        cell_tester(i);
    }
    #[test]
    fn test_triangle() {
        let t = Triangle {};
        cell_tester(t);
    }
    #[test]
    fn test_quadrilateral() {
        let q = Quadrilateral {};
        cell_tester(q);
    }

    #[test]
    fn test_tetrahedron() {
        let t = Tetrahedron {};
        cell_tester(t);
    }
    /*
    #[test]
    fn test_prism() {
        cell_tester::<Prism>();
    }
    */
}
