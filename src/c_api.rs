//! C Interface

use crate::cell::*;
use libc::size_t;
pub use rusty_cffi::RustyDataContainer;

pub struct ReferenceCellContainer(Box<dyn ReferenceCell>);

impl ReferenceCellContainer {
    pub fn to_box(self) -> Box<ReferenceCellContainer> {
        Box::new(self)
    }
}

// impl Drop for ReferenceCellContainer {
//     fn drop(&mut self) {
//         match self.cell_type {
//             ReferenceCellType::Interval => {
//                 unsafe { Box::from_raw(self.element as *mut Interval) };
//             }
//             ReferenceCellType::Triangle => {
//                 unsafe { Box::from_raw(self.element as *mut Triangle) };
//             }
//             ReferenceCellType::Quadrilateral => {
//                 unsafe { Box::from_raw(self.element as *mut Quadrilateral) };
//             }
//             ReferenceCellType::Tetrahedron => {
//                 unsafe { Box::from_raw(self.element as *mut Tetrahedron) };
//             }
//             ReferenceCellType::Hexahedron => {
//                 unsafe { Box::from_raw(self.element as *mut Hexahedron) };
//             }
//             ReferenceCellType::Prism => {
//                 unsafe { Box::from_raw(self.element as *mut Prism) };
//             }
//             ReferenceCellType::Pyramid => {
//                 unsafe { Box::from_raw(self.element as *mut Pyramid) };
//             }
//         };
//     }
// }

// impl ReferenceCellContainer {
//     pub fn new<T: ReferenceCell>(cell: T) -> ReferenceCellContainer {
//         ReferenceCellContainer {
//             element: Box::into_raw(Box::new(cell)) as *mut c_void,
//             cell_type: T::CELLTYPE,
//         }
//     }
// }

// fn get_reference_cell<C: ReferenceCell>(
//     cell_container: Option<Box<ReferenceCellContainer>>,
// ) -> &'static C {
//     let cell_container = Box::leak(cell_container.unwrap());
//     assert_eq!(cell_container.cell_type, C::CELLTYPE);
//     unsafe { (cell_container.element as *const C).as_ref().unwrap() }
// }

// fn get_dim_inner<C: ReferenceCell>(cell: &C) -> size_t {
//     cell.dim() as size_t
// }

// macro_rules! get_cell_property_impl {

//     ($property:ident, $output:ty) =>

//     {
//         paste! {

//             fn [<get_ $property _impl>](cell_container: Option<Box<ReferenceCellContainer>>) -> $output {
//                 match cell_container.as_ref().unwrap().cell_type {
//                     ReferenceCellType::Interval => {
//                         let cell = get_reference_cell::<Interval>(cell_container);
//                         [<get_ $property _inner>](cell)
//                     },
//                     ReferenceCellType::Triangle => {
//                         let cell = get_reference_cell::<Triangle>(cell_container);
//                         [<get_ $property _inner>](cell)

//                     },
//                     ReferenceCellType::Quadrilateral => {
//                         let cell = get_reference_cell::<Quadrilateral>(cell_container);
//                         [<get_ $property _inner>](cell)

//                     },
//                     ReferenceCellType::Tetrahedron => {
//                         let cell = get_reference_cell::<Tetrahedron>(cell_container);
//                         [<get_ $property _inner>](cell)

//                     },
//                     ReferenceCellType::Hexahedron => {
//                         let cell = get_reference_cell::<Hexahedron>(cell_container);
//                         [<get_ $property _inner>](cell)

//                     }
//                     ReferenceCellType::Prism => {
//                         let cell = get_reference_cell::<Prism>(cell_container);
//                         [<get_ $property _inner>](cell)
//                     },
//                     ReferenceCellType::Pyramid => {
//                         let cell = get_reference_cell::<Pyramid>(cell_container);
//                         [<get_ $property _inner>](cell)
//                     }
//                 }
//             }

//         }
//     };

// }

// get_cell_property_impl!(dim, size_t);

fn get_reference(
    cell_container: Option<Box<ReferenceCellContainer>>,
) -> &'static dyn ReferenceCell {
    Box::leak(cell_container.unwrap()).0.as_ref()    
}

/// New cell container from interval.
#[no_mangle]
pub extern "C" fn reference_cell_container_new_from_interval() -> Box<ReferenceCellContainer> {
    ReferenceCellContainer(Box::new(Interval {})).to_box()
}

/// New cell container from triangle.
#[no_mangle]
pub extern "C" fn reference_cell_container_new_from_triangle() -> Box<ReferenceCellContainer> {
    ReferenceCellContainer(Box::new(Triangle {})).to_box()
}
/// New cell container from quadrilateral.
#[no_mangle]
pub extern "C" fn reference_cell_container_new_from_quadrilateral() -> Box<ReferenceCellContainer> {
    ReferenceCellContainer(Box::new(Quadrilateral {})).to_box()
}

/// New cell container from tetrahedron.
#[no_mangle]
pub extern "C" fn reference_cell_container_new_from_tetrahedron() -> Box<ReferenceCellContainer> {
    ReferenceCellContainer(Box::new(Tetrahedron {})).to_box()
}
/// New cell container from Hexahedron.
#[no_mangle]
pub extern "C" fn reference_cell_container_new_from_hexahedron() -> Box<ReferenceCellContainer> {
    ReferenceCellContainer(Box::new(Hexahedron {})).to_box()
}
/// New cell container from Prism.
#[no_mangle]
pub extern "C" fn reference_cell_container_new_from_prism() -> Box<ReferenceCellContainer> {
    ReferenceCellContainer(Box::new(Prism {})).to_box()
}
/// New cell container from Pyramid.
#[no_mangle]
pub extern "C" fn reference_cell_container_new_from_pyramid() -> Box<ReferenceCellContainer> {
    ReferenceCellContainer(Box::new(Pyramid {})).to_box()
}

/// Destroy a cell container.
#[no_mangle]
pub extern "C" fn reference_cell_container_destroy(_: Option<Box<ReferenceCellContainer>>) {
}

/// Get dimension.
#[no_mangle]
pub extern "C" fn reference_cell_container_get_dim(
    cell_container: Option<Box<ReferenceCellContainer>>,
) -> size_t {
    get_reference(cell_container).dim()
}

#[no_mangle]
pub extern "C" fn reference_cell_container_get_vertices(
    cell_container: Option<Box<ReferenceCellContainer>>,
) -> Box<RustyDataContainer> {
    let cell = get_reference(cell_container);
    RustyDataContainer::from_slice(cell.vertices()).to_box()
}

// /// New cell container from triangle.
// #[no_mangle]
// pub extern "C" fn reference_cell_container_new_from_triangle() -> Box<ReferenceCellContainer> {
//     Box::new(ReferenceCellContainer::new(Triangle {}))
// }

// /// New cell container from quadrilateral.
// #[no_mangle]
// pub extern "C" fn reference_cell_container_new_from_quadrilateral() -> Box<ReferenceCellContainer> {
//     Box::new(ReferenceCellContainer::new(Quadrilateral {}))
// }

// /// New cell container from tetrahedron.
// #[no_mangle]
// pub extern "C" fn reference_cell_container_new_from_tetrahedron() -> Box<ReferenceCellContainer> {
//     Box::new(ReferenceCellContainer::new(Tetrahedron {}))
// }

// /// New cell container from hexahedron.
// #[no_mangle]
// pub extern "C" fn reference_cell_container_new_from_hexahedron() -> Box<ReferenceCellContainer> {
//     Box::new(ReferenceCellContainer::new(Hexahedron {}))
// }

// /// New cell container from pyramid.
// #[no_mangle]
// pub extern "C" fn reference_cell_container_new_from_pyramid() -> Box<ReferenceCellContainer> {
//     Box::new(ReferenceCellContainer::new(Pyramid {}))
// }

// /// New cell container from prism.
// #[no_mangle]
// pub extern "C" fn reference_cell_container_new_from_prism() -> Box<ReferenceCellContainer> {
//     Box::new(ReferenceCellContainer::new(Prism {}))
// }
