//! Cell definitions

pub mod cells_1d;
pub use cells_1d::*;
pub mod cells_2d;
pub use cells_2d::*;
pub mod cells_3d;
pub use cells_3d::*;

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

    /// The number of entities of dimension `dim`
    fn entity_count(&self, dim: usize) -> Result<usize, ()> {
        match dim {
            0 => Ok(self.vertex_count()),
            1 => Ok(self.edge_count()),
            2 => Ok(self.face_count()),
            3 => Ok(self.volume_count()),
            _ => Err(()),
        }
    }

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
    /// This function returns a list of entity numbers of entities of dimension `connected_dim` that are attached to the entity numbered `entity_dim` of number entity_number.
    /// For example connectivity(1, 0, 2) will return a list of faces (2D entities) that are connected to edge (1D entity) 0.
    fn connectivity(
        &self,
        entity_dim: usize,
        entity_number: usize,
        connected_dim: usize,
    ) -> Result<Vec<usize>, ()>;
}

#[cfg(test)]
mod test {
    use crate::cell::*;

    fn cell_tester(c: impl ReferenceCell) {
        assert_eq!(c.vertex_count(), c.vertices().len() / c.dim());
        assert_eq!(c.edge_count(), c.edges().len() / 2);
        assert_eq!(c.face_count(), c.faces_nvertices().len());

        for v_n in 0..c.vertex_count() {
            let v = c.connectivity(0, v_n, 0).unwrap();
            assert_eq!(v[0], v_n);
        }
        for e_n in 0..c.edge_count() {
            let v = c.connectivity(1, e_n, 0).unwrap();
            let edge = &c.edges()[2 * e_n..2 * (e_n + 1)];
            assert_eq!(v, edge);
        }
        let mut start = 0;
        for f_n in 0..c.face_count() {
            let v = c.connectivity(2, f_n, 0).unwrap();
            let face = &c.faces()[start..start + c.faces_nvertices()[f_n]];
            assert_eq!(v, face);
            start += c.faces_nvertices()[f_n];
        }

        for e_dim in 0..c.dim() + 1 {
            for e_n in 0..c.entity_count(e_dim).unwrap() {
                let e_vertices = c.connectivity(e_dim, e_n, 0).unwrap();
                for c_dim in 0..c.dim() + 1 {
                    let connectivity = c.connectivity(e_dim, e_n, c_dim).unwrap();
                    if e_dim == c_dim {
                        assert_eq!(connectivity.len(), 1);
                        assert_eq!(connectivity[0], e_n)
                    } else {
                        for c_n in &connectivity {
                            let c_vertices = c.connectivity(c_dim, *c_n, 0).unwrap();
                            println!("{} {} {} {}", e_dim, e_n, c_dim, c_n);
                            if e_dim < c_dim {
                                for i in &e_vertices {
                                    println!(" c {}", i);
                                    assert!(c_vertices.contains(&i));
                                }
                            } else {
                                for i in &c_vertices {
                                    println!(" e {}", i);
                                    assert!(e_vertices.contains(&i));
                                }
                            }
                            assert!(connectivity.contains(&c_n));
                        }
                    }
                }
            }
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
    #[test]
    fn test_hexahedron() {
        let h = Hexahedron {};
        cell_tester(h);
    }
    #[test]
    fn test_prism() {
        let p = Prism {};
        cell_tester(p);
    }
}
