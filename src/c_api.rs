//! C Interface

use crate::cell::ReferenceCell;

fn get_element<C: ReferenceCell>(
    elem: *const std::ffi::c_void,
    elem_type: usize,
) -> Result<&C, ()> {
    match elem_type {
        0 => unsafe { Ok(&(*elem as C)) },
        _ => Err(()),
    }
}

#[no_mangle]
pub extern "C" fn vertices(elem: *const std::ffi::c_void, elem_type: usize) {}
