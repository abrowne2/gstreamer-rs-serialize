// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::RTSPClient;
#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "GstRTSPOnvifClient")]
    pub struct RTSPOnvifClient(Object<ffi::GstRTSPOnvifClient, ffi::GstRTSPOnvifClientClass>) @extends RTSPClient;

    match fn {
        type_ => || ffi::gst_rtsp_onvif_client_get_type(),
    }
}

impl RTSPOnvifClient {
    pub const NONE: Option<&'static RTSPOnvifClient> = None;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "gst_rtsp_onvif_client_new")]
    pub fn new() -> RTSPOnvifClient {
        assert_initialized_main_thread!();
        unsafe { RTSPClient::from_glib_full(ffi::gst_rtsp_onvif_client_new()).unsafe_cast() }
    }
}

#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
impl Default for RTSPOnvifClient {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for RTSPOnvifClient {}
unsafe impl Sync for RTSPOnvifClient {}
