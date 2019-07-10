// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib;
use glib::translate::*;
use glib::ObjectExt;
use gst;
use gst_base_sys;
use std::mem;

glib_wrapper! {
    pub struct Adapter(Object<gst_base_sys::GstAdapter, gst_base_sys::GstAdapterClass, AdapterClass>);

    match fn {
        get_type => || gst_base_sys::gst_adapter_get_type(),
    }
}

impl Adapter {
    pub fn new() -> Adapter {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(gst_base_sys::gst_adapter_new()) }
    }

    pub fn available(&self) -> usize {
        unsafe { gst_base_sys::gst_adapter_available(self.to_glib_none().0) }
    }

    pub fn available_fast(&self) -> usize {
        unsafe { gst_base_sys::gst_adapter_available_fast(self.to_glib_none().0) }
    }

    pub fn clear(&self) {
        unsafe {
            gst_base_sys::gst_adapter_clear(self.to_glib_none().0);
        }
    }

    pub fn copy_bytes(&self, offset: usize, size: usize) -> Option<glib::Bytes> {
        unsafe {
            from_glib_full(gst_base_sys::gst_adapter_copy_bytes(
                self.to_glib_none().0,
                offset,
                size,
            ))
        }
    }

    pub fn distance_from_discont(&self) -> u64 {
        unsafe { gst_base_sys::gst_adapter_distance_from_discont(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn dts_at_discont(&self) -> gst::ClockTime {
        unsafe {
            from_glib(gst_base_sys::gst_adapter_dts_at_discont(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn flush(&self, flush: usize) {
        unsafe {
            gst_base_sys::gst_adapter_flush(self.to_glib_none().0, flush);
        }
    }

    pub fn get_buffer(&self, nbytes: usize) -> Option<gst::Buffer> {
        unsafe {
            from_glib_full(gst_base_sys::gst_adapter_get_buffer(
                self.to_glib_none().0,
                nbytes,
            ))
        }
    }

    pub fn get_buffer_fast(&self, nbytes: usize) -> Option<gst::Buffer> {
        unsafe {
            from_glib_full(gst_base_sys::gst_adapter_get_buffer_fast(
                self.to_glib_none().0,
                nbytes,
            ))
        }
    }

    pub fn get_buffer_list(&self, nbytes: usize) -> Option<gst::BufferList> {
        unsafe {
            from_glib_full(gst_base_sys::gst_adapter_get_buffer_list(
                self.to_glib_none().0,
                nbytes,
            ))
        }
    }

    pub fn get_list(&self, nbytes: usize) -> Vec<gst::Buffer> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(gst_base_sys::gst_adapter_get_list(
                self.to_glib_none().0,
                nbytes,
            ))
        }
    }

    pub fn masked_scan_uint32(&self, mask: u32, pattern: u32, offset: usize, size: usize) -> isize {
        unsafe {
            gst_base_sys::gst_adapter_masked_scan_uint32(
                self.to_glib_none().0,
                mask,
                pattern,
                offset,
                size,
            )
        }
    }

    pub fn masked_scan_uint32_peek(
        &self,
        mask: u32,
        pattern: u32,
        offset: usize,
        size: usize,
    ) -> (isize, u32) {
        unsafe {
            let mut value = mem::MaybeUninit::uninit();
            let ret = gst_base_sys::gst_adapter_masked_scan_uint32_peek(
                self.to_glib_none().0,
                mask,
                pattern,
                offset,
                size,
                value.as_mut_ptr(),
            );
            let value = value.assume_init();
            (ret, value)
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn offset_at_discont(&self) -> u64 {
        unsafe { gst_base_sys::gst_adapter_offset_at_discont(self.to_glib_none().0) }
    }

    pub fn prev_dts(&self) -> (gst::ClockTime, u64) {
        unsafe {
            let mut distance = mem::MaybeUninit::uninit();
            let ret = from_glib(gst_base_sys::gst_adapter_prev_dts(
                self.to_glib_none().0,
                distance.as_mut_ptr(),
            ));
            let distance = distance.assume_init();
            (ret, distance)
        }
    }

    pub fn prev_dts_at_offset(&self, offset: usize) -> (gst::ClockTime, u64) {
        unsafe {
            let mut distance = mem::MaybeUninit::uninit();
            let ret = from_glib(gst_base_sys::gst_adapter_prev_dts_at_offset(
                self.to_glib_none().0,
                offset,
                distance.as_mut_ptr(),
            ));
            let distance = distance.assume_init();
            (ret, distance)
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn prev_offset(&self) -> (u64, u64) {
        unsafe {
            let mut distance = mem::MaybeUninit::uninit();
            let ret =
                gst_base_sys::gst_adapter_prev_offset(self.to_glib_none().0, distance.as_mut_ptr());
            let distance = distance.assume_init();
            (ret, distance)
        }
    }

    pub fn prev_pts(&self) -> (gst::ClockTime, u64) {
        unsafe {
            let mut distance = mem::MaybeUninit::uninit();
            let ret = from_glib(gst_base_sys::gst_adapter_prev_pts(
                self.to_glib_none().0,
                distance.as_mut_ptr(),
            ));
            let distance = distance.assume_init();
            (ret, distance)
        }
    }

    pub fn prev_pts_at_offset(&self, offset: usize) -> (gst::ClockTime, u64) {
        unsafe {
            let mut distance = mem::MaybeUninit::uninit();
            let ret = from_glib(gst_base_sys::gst_adapter_prev_pts_at_offset(
                self.to_glib_none().0,
                offset,
                distance.as_mut_ptr(),
            ));
            let distance = distance.assume_init();
            (ret, distance)
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn pts_at_discont(&self) -> gst::ClockTime {
        unsafe {
            from_glib(gst_base_sys::gst_adapter_pts_at_discont(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn take_buffer(&self, nbytes: usize) -> Option<gst::Buffer> {
        unsafe {
            from_glib_full(gst_base_sys::gst_adapter_take_buffer(
                self.to_glib_none().0,
                nbytes,
            ))
        }
    }

    pub fn take_buffer_fast(&self, nbytes: usize) -> Option<gst::Buffer> {
        unsafe {
            from_glib_full(gst_base_sys::gst_adapter_take_buffer_fast(
                self.to_glib_none().0,
                nbytes,
            ))
        }
    }

    pub fn take_buffer_list(&self, nbytes: usize) -> Option<gst::BufferList> {
        unsafe {
            from_glib_full(gst_base_sys::gst_adapter_take_buffer_list(
                self.to_glib_none().0,
                nbytes,
            ))
        }
    }

    pub fn take_list(&self, nbytes: usize) -> Vec<gst::Buffer> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(gst_base_sys::gst_adapter_take_list(
                self.to_glib_none().0,
                nbytes,
            ))
        }
    }
}

impl Default for Adapter {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl glib::SendUnique for Adapter {
    fn is_unique(&self) -> bool {
        self.ref_count() == 1
    }
}
