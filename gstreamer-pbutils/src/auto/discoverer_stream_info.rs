// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use glib::object::IsA;
use glib::translate::*;

glib::wrapper! {
    #[doc(alias = "GstDiscovererStreamInfo")]
    pub struct DiscovererStreamInfo(Object<ffi::GstDiscovererStreamInfo>);

    match fn {
        type_ => || ffi::gst_discoverer_stream_info_get_type(),
    }
}

impl DiscovererStreamInfo {
    pub const NONE: Option<&'static DiscovererStreamInfo> = None;
}

unsafe impl Send for DiscovererStreamInfo {}
unsafe impl Sync for DiscovererStreamInfo {}

pub trait DiscovererStreamInfoExt: 'static {
    #[doc(alias = "gst_discoverer_stream_info_get_caps")]
    #[doc(alias = "get_caps")]
    fn caps(&self) -> Option<gst::Caps>;

    #[doc(alias = "gst_discoverer_stream_info_get_misc")]
    #[doc(alias = "get_misc")]
    fn misc(&self) -> Option<gst::Structure>;

    #[doc(alias = "gst_discoverer_stream_info_get_next")]
    #[doc(alias = "get_next")]
    fn next(&self) -> Option<DiscovererStreamInfo>;

    #[doc(alias = "gst_discoverer_stream_info_get_previous")]
    #[doc(alias = "get_previous")]
    fn previous(&self) -> Option<DiscovererStreamInfo>;

    #[doc(alias = "gst_discoverer_stream_info_get_stream_id")]
    #[doc(alias = "get_stream_id")]
    fn stream_id(&self) -> Option<glib::GString>;

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "gst_discoverer_stream_info_get_stream_number")]
    #[doc(alias = "get_stream_number")]
    fn stream_number(&self) -> i32;

    #[doc(alias = "gst_discoverer_stream_info_get_stream_type_nick")]
    #[doc(alias = "get_stream_type_nick")]
    fn stream_type_nick(&self) -> glib::GString;

    #[doc(alias = "gst_discoverer_stream_info_get_tags")]
    #[doc(alias = "get_tags")]
    fn tags(&self) -> Option<gst::TagList>;

    #[doc(alias = "gst_discoverer_stream_info_get_toc")]
    #[doc(alias = "get_toc")]
    fn toc(&self) -> Option<gst::Toc>;
}

impl<O: IsA<DiscovererStreamInfo>> DiscovererStreamInfoExt for O {
    fn caps(&self) -> Option<gst::Caps> {
        unsafe {
            from_glib_full(ffi::gst_discoverer_stream_info_get_caps(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn misc(&self) -> Option<gst::Structure> {
        unsafe {
            from_glib_none(ffi::gst_discoverer_stream_info_get_misc(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn next(&self) -> Option<DiscovererStreamInfo> {
        unsafe {
            from_glib_full(ffi::gst_discoverer_stream_info_get_next(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn previous(&self) -> Option<DiscovererStreamInfo> {
        unsafe {
            from_glib_full(ffi::gst_discoverer_stream_info_get_previous(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn stream_id(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gst_discoverer_stream_info_get_stream_id(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    fn stream_number(&self) -> i32 {
        unsafe { ffi::gst_discoverer_stream_info_get_stream_number(self.as_ref().to_glib_none().0) }
    }

    fn stream_type_nick(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::gst_discoverer_stream_info_get_stream_type_nick(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn tags(&self) -> Option<gst::TagList> {
        unsafe {
            from_glib_none(ffi::gst_discoverer_stream_info_get_tags(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn toc(&self) -> Option<gst::Toc> {
        unsafe {
            from_glib_none(ffi::gst_discoverer_stream_info_get_toc(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}
