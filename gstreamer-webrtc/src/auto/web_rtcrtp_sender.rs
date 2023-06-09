// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
use crate::{WebRTCDTLSTransport, WebRTCPriorityType};
#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
use std::{boxed::Box as Box_, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GstWebRTCRTPSender")]
    pub struct WebRTCRTPSender(Object<ffi::GstWebRTCRTPSender, ffi::GstWebRTCRTPSenderClass>);

    match fn {
        type_ => || ffi::gst_webrtc_rtp_sender_get_type(),
    }
}

impl WebRTCRTPSender {
    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "gst_webrtc_rtp_sender_set_priority")]
    pub fn set_priority(&self, priority: WebRTCPriorityType) {
        unsafe {
            ffi::gst_webrtc_rtp_sender_set_priority(self.to_glib_none().0, priority.into_glib());
        }
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    pub fn priority(&self) -> WebRTCPriorityType {
        glib::ObjectExt::property(self, "priority")
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    pub fn transport(&self) -> Option<WebRTCDTLSTransport> {
        glib::ObjectExt::property(self, "transport")
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "priority")]
    pub fn connect_priority_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_priority_trampoline<
            F: Fn(&WebRTCRTPSender) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstWebRTCRTPSender,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::priority\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_priority_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "transport")]
    pub fn connect_transport_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_transport_trampoline<
            F: Fn(&WebRTCRTPSender) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstWebRTCRTPSender,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::transport\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_transport_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

unsafe impl Send for WebRTCRTPSender {}
unsafe impl Sync for WebRTCRTPSender {}
