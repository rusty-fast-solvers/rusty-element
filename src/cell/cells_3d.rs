//! Three-dimensional reference cells

use crate::cell::*;

/// The reference tetrahedron
pub struct Tetrahedron;

impl ReferenceCell for Tetrahedron {
    const DIM: usize = 3;
    fn vertices(&self) -> &[f64] {
        static VERTICES: [f64; 12] = [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];
        &VERTICES
    }

    fn edges(&self) -> &[usize] {
        static EDGES: [usize; 12] = [2, 3, 1, 3, 1, 2, 0, 3, 0, 2, 0, 1];
        &EDGES
    }

    fn faces(&self) -> &[usize] {
        static FACES: [usize; 12] = [1, 2, 3, 0, 2, 3, 0, 1, 3, 0, 1, 2];
        &FACES
    }
    fn faces_nvertices(&self) -> &[usize] {
        static FACES_NV: [usize; 4] = [3, 3, 3, 3];
        &FACES_NV
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
    fn connectivity<const ENTITY_DIM: usize, const CONNECTED_DIM: usize>(
        &self,
        entity_number: usize,
    ) -> Result<Vec<usize>, ()> {
        match ENTITY_DIM {
            0 => {
                assert!(entity_number < 4);
                match CONNECTED_DIM {
                    0 => Ok(vec![entity_number]),
                    1 => match entity_number {
                        0 => Ok(vec![3, 4, 5]),
                        1 => Ok(vec![1, 2, 5]),
                        2 => Ok(vec![0, 2, 4]),
                        3 => Ok(vec![0, 1, 3]),
                        _ => Err(()),
                    },
                    2 => match entity_number {
                        0 => Ok(vec![1, 2, 3]),
                        1 => Ok(vec![0, 2, 3]),
                        2 => Ok(vec![0, 1, 3]),
                        3 => Ok(vec![0, 1, 2]),
                        _ => Err(()),
                    },
                    3 => Ok(vec![0]),
                    _ => Err(()),
                }
            }
            1 => {
                assert!(entity_number < 6);
                match CONNECTED_DIM {
                    0 => match entity_number {
                        0 => Ok(vec![2, 3]),
                        1 => Ok(vec![1, 3]),
                        2 => Ok(vec![1, 2]),
                        3 => Ok(vec![0, 3]),
                        4 => Ok(vec![0, 2]),
                        5 => Ok(vec![0, 1]),
                        _ => Err(()),
                    },
                    1 => Ok(vec![entity_number]),
                    2 => match entity_number {
                        0 => Ok(vec![0, 1]),
                        1 => Ok(vec![0, 2]),
                        2 => Ok(vec![0, 3]),
                        3 => Ok(vec![1, 2]),
                        4 => Ok(vec![1, 3]),
                        5 => Ok(vec![2, 3]),
                        _ => Err(()),
                    },
                    3 => Ok(vec![0]),
                    _ => Err(()),
                }
            }
            2 => {
                assert!(entity_number < 4);
                match CONNECTED_DIM {
                    0 => match entity_number {
                        0 => Ok(vec![1, 2, 3]),
                        1 => Ok(vec![0, 2, 3]),
                        2 => Ok(vec![0, 1, 3]),
                        3 => Ok(vec![0, 1, 2]),
                        _ => Err(()),
                    },
                    1 => match entity_number {
                        0 => Ok(vec![0, 1, 2]),
                        1 => Ok(vec![0, 3, 4]),
                        2 => Ok(vec![1, 3, 5]),
                        3 => Ok(vec![2, 4, 5]),
                        _ => Err(()),
                    },
                    2 => Ok(vec![entity_number]),
                    3 => Ok(vec![0]),
                    _ => Err(()),
                }
            }
            3 => {
                assert!(entity_number == 0);
                match CONNECTED_DIM {
                    0 => Ok(vec![0, 1, 2, 3]),
                    1 => Ok(vec![0, 1, 2, 3, 4, 5]),
                    2 => Ok(vec![0, 1, 2, 3]),
                    3 => Ok(vec![0]),
                    _ => Err(()),
                }
            }
            _ => Err(()),
        }
    }
}
