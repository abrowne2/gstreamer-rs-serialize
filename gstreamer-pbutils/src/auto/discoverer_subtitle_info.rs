// This file was generated by gir (https://github.com/gtk-rs/gir @ 6855214)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

use DiscovererStreamInfo;
use ffi;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct DiscovererSubtitleInfo(Object<ffi::GstDiscovererSubtitleInfo>): DiscovererStreamInfo;

    match fn {
        get_type => || ffi::gst_discoverer_subtitle_info_get_type(),
    }
}

impl DiscovererSubtitleInfo {
    pub fn get_language(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gst_discoverer_subtitle_info_get_language(self.to_glib_none().0))
        }
    }
}

unsafe impl Send for DiscovererSubtitleInfo {}
unsafe impl Sync for DiscovererSubtitleInfo {}
