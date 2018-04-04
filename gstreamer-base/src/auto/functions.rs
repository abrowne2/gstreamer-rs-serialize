// This file was generated by gir (https://github.com/gtk-rs/gir @ 6855214)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

use ffi;
use glib::object::IsA;
use glib::translate::*;
use gst;
use std::mem;


pub fn type_find_helper<P: IsA<gst::Pad>>(src: &P, size: u64) -> Option<gst::Caps> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(ffi::gst_type_find_helper(src.to_glib_none().0, size))
    }
}

pub fn type_find_helper_for_buffer<'a, P: IsA<gst::Object> + 'a, Q: Into<Option<&'a P>>>(obj: Q, buf: &gst::Buffer) -> (Option<gst::Caps>, gst::TypeFindProbability) {
    assert_initialized_main_thread!();
    let obj = obj.into();
    let obj = obj.to_glib_none();
    unsafe {
        let mut prob = mem::uninitialized();
        let ret = from_glib_full(ffi::gst_type_find_helper_for_buffer(obj.0, buf.to_glib_none().0, &mut prob));
        (ret, from_glib(prob))
    }
}

pub fn type_find_helper_for_extension<'a, P: IsA<gst::Object> + 'a, Q: Into<Option<&'a P>>>(obj: Q, extension: &str) -> Option<gst::Caps> {
    assert_initialized_main_thread!();
    let obj = obj.into();
    let obj = obj.to_glib_none();
    unsafe {
        from_glib_full(ffi::gst_type_find_helper_for_extension(obj.0, extension.to_glib_none().0))
    }
}

//pub fn type_find_helper_get_range<'a, P: IsA<gst::Object>, Q: IsA<gst::Object> + 'a, R: Into<Option<&'a Q>>>(obj: &P, parent: R, func: /*Unknown conversion*//*Unimplemented*/TypeFindHelperGetRangeFunction, size: u64, extension: &str) -> (Option<gst::Caps>, gst::TypeFindProbability) {
//    unsafe { TODO: call ffi::gst_type_find_helper_get_range() }
//}
