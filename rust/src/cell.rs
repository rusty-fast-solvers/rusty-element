//! Cell definitions

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

/// The reference interval
struct Interval;

/// The reference triangle
pub struct Triangle;

impl ReferenceCell for Triangle {
    const DIM: usize = 2;
    fn vertices(&self) -> &[f64] {
        static VERTICES: [f64; 6] = [0.0, 0.0, 1.0, 0.0, 0.0, 1.0];
        &VERTICES
    }

    fn edges(&self) -> &[usize] {
        static EDGES: [usize; 6] = [1, 2, 0, 2, 0, 1];
        &EDGES
    }

    fn faces(&self) -> &[usize] {
        static FACES: [usize; 3] = [0, 1, 2];
        &FACES
    }
    fn faces_nvertices(&self) -> &[usize] {
        static FACES_NV: [usize; 1] = [3];
        &FACES_NV
    }

    fn vertex_count(&self) -> usize {
        3
    }
    fn edge_count(&self) -> usize {
        3
    }
    fn face_count(&self) -> usize {
        1
    }
    fn volume_count(&self) -> usize {
        0
    }
    fn connectivity<const ENTITY_DIM: usize, const CONNECTED_DIM: usize>(
        &self,
        entity_number: usize,
    ) -> Result<Vec<usize>, ()> {
        match ENTITY_DIM {
            0 => {
                assert!(entity_number < 3);
                match CONNECTED_DIM {
                    0 => Ok(vec![entity_number]),
                    1 => match entity_number {
                        0 => Ok(vec![1, 2]),
                        1 => Ok(vec![0, 2]),
                        2 => Ok(vec![0, 1]),
                        _ => Err(()),
                    },
                    2 => Ok(vec![0]),
                    _ => Err(()),
                }
            }
            1 => {
                assert!(entity_number < 3);
                match CONNECTED_DIM {
                    0 => Ok(self.edges()[entity_number * 2..(entity_number + 1) * 2].to_vec()),
                    1 => Ok(vec![entity_number]),
                    2 => Ok(vec![0]),
                    _ => Err(()),
                }
            }
            2 => {
                assert!(entity_number == 0);
                match CONNECTED_DIM {
                    0 => Ok(vec![0, 1, 2]),
                    1 => Ok(vec![0, 1, 2]),
                    2 => Ok(vec![0]),
                    _ => Err(()),
                }
            }
            _ => Err(()),
        }
    }
}

/// The reference quadrilateral
struct Quadrilateral;

/// The reference tetrahedron
struct Tetrahedron;

/// The reference prism
struct Prism;
/*
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
    fn vertex_count(&self) -> usize {
        2
    }
    fn edge_count(&self) -> usize {
        1
    }
    fn face_count(&self) -> usize {
        0
    }
    fn volume_count(&self) -> usize {
        0
    }
}
*/

/*
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
    fn vertex_count(&self) -> usize {
        4
    }
    fn edge_count(&self) -> usize {
        4
    }
    fn face_count(&self) -> usize {
        1
    }
    fn volume_count(&self) -> usize {
        0
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

    fn cell_tester(c: impl ReferenceCell) {
        assert_eq!(c.vertex_count(), c.vertices().len() / c.dim());
        assert_eq!(c.edge_count(), c.edges().len() / 2);
        assert_eq!(c.face_count(), c.faces_nvertices().len());
    }

    /*#[test]
        fn test_interval() {
            cell_tester::<Interval>();
        }
    */
    #[test]
    fn test_triangle() {
        let t = Triangle {};
        cell_tester(t);
    }
    /*
    #[test]
    fn test_quadrilateral() {
        cell_tester::<Quadrilateral>();
    }

    #[test]
    fn test_tetrahedron() {
        cell_tester::<Tetrahedron>();
    }

    #[test]
    fn test_prism() {
        cell_tester::<Prism>();
    }
    */
}
