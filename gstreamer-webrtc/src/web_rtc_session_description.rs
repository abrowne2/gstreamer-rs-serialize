// Take a look at the license at the top of the repository in the LICENSE file.

use std::mem;

use glib::translate::*;

use crate::{WebRTCSDPType, WebRTCSessionDescription};

impl WebRTCSessionDescription {
    #[doc(alias = "gst_webrtc_session_description_new")]
    pub fn new(type_: WebRTCSDPType, sdp: gst_sdp::SDPMessage) -> WebRTCSessionDescription {
        skip_assert_initialized!();
        unsafe {
            let mut sdp = mem::ManuallyDrop::new(sdp);
            from_glib_full(ffi::gst_webrtc_session_description_new(
                type_.into_glib(),
                sdp.to_glib_none_mut().0,
            ))
        }
    }

    #[doc(alias = "get_type")]
    pub fn type_(&self) -> crate::WebRTCSDPType {
        unsafe { from_glib((*self.as_ptr()).type_) }
    }

    #[doc(alias = "get_sdp")]
    pub fn sdp(&self) -> gst_sdp::SDPMessage {
        unsafe { from_glib_none((*self.as_ptr()).sdp) }
    }
}
