//! C Interface

use crate::cell::*;

fn get_element<C: ReferenceCell>(
    elem: *const std::ffi::c_void,
    elem_type: usize,
) -> Result<&'static C, ()> {
    match elem_type {
        0 => unsafe { Ok(&(*(elem as *const C))) },
        _ => Err(()),
    }
}

#[no_mangle]
pub extern "C" fn vertices(
    elem: *const std::ffi::c_void,
    elem_type: usize,
    nvertices: *mut usize,
    output: *mut *const f64,
) {
    let elem = match elem_type {
        0 => get_element::<Triangle>(elem, elem_type),
        _ => panic!("Error"),
    };
    unsafe {*nvertices = elem.unwrap().vertex_count() };
    unsafe {*output = elem.unwrap().vertices().as_ptr() };
}
