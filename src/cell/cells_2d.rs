/// Two-dimensional reference cells
use crate::cell::*;

/// The reference triangle
pub struct Triangle;

/// The reference triangle
pub struct Quadrilateral;

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

impl ReferenceCell for Quadrilateral {
    const DIM: usize = 2;
    fn vertices(&self) -> &[f64] {
        static VERTICES: [f64; 8] = [0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0];
        &VERTICES
    }

    fn edges(&self) -> &[usize] {
        static EDGES: [usize; 8] = [0, 1, 0, 2, 1, 3, 2, 3];
        &EDGES
    }

    fn faces(&self) -> &[usize] {
        static FACES: [usize; 4] = [0, 1, 2, 3];
        &FACES
    }
    fn faces_nvertices(&self) -> &[usize] {
        static FACES_NV: [usize; 1] = [4];
        &FACES_NV
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
                        0 => Ok(vec![0, 1]),
                        1 => Ok(vec![0, 2]),
                        2 => Ok(vec![1, 3]),
                        3 => Ok(vec![2, 3]),
                        _ => Err(()),
                    },
                    2 => Ok(vec![0]),
                    _ => Err(()),
                }
            }
            1 => {
                assert!(entity_number < 4);
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
                    0 => Ok(vec![0, 1, 2, 3]),
                    1 => Ok(vec![0, 1, 2, 3]),
                    2 => Ok(vec![0]),
                    _ => Err(()),
                }
            }
            _ => Err(()),
        }
    }
}
