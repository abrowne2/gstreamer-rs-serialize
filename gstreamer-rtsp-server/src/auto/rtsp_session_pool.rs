// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use RTSPSession;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct RTSPSessionPool(Object<ffi::GstRTSPSessionPool, ffi::GstRTSPSessionPoolClass>);

    match fn {
        get_type => || ffi::gst_rtsp_session_pool_get_type(),
    }
}

impl RTSPSessionPool {
    pub fn new() -> RTSPSessionPool {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gst_rtsp_session_pool_new())
        }
    }
}

impl Default for RTSPSessionPool {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for RTSPSessionPool {}
unsafe impl Sync for RTSPSessionPool {}

pub trait RTSPSessionPoolExt: 'static {
    fn cleanup(&self) -> u32;

    fn create(&self) -> Option<RTSPSession>;

    //fn filter<'a, P: Into<Option<&'a /*Unimplemented*/RTSPSessionPoolFilterFunc>>, Q: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, func: P, user_data: Q) -> Vec<RTSPSession>;

    fn find(&self, sessionid: &str) -> Option<RTSPSession>;

    fn get_max_sessions(&self) -> u32;

    fn get_n_sessions(&self) -> u32;

    fn remove(&self, sess: &RTSPSession) -> Result<(), glib::error::BoolError>;

    fn set_max_sessions(&self, max: u32);

    fn connect_session_removed<F: Fn(&Self, &RTSPSession) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_max_sessions_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<RTSPSessionPool>> RTSPSessionPoolExt for O {
    fn cleanup(&self) -> u32 {
        unsafe {
            ffi::gst_rtsp_session_pool_cleanup(self.to_glib_none().0)
        }
    }

    fn create(&self) -> Option<RTSPSession> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_session_pool_create(self.to_glib_none().0))
        }
    }

    //fn filter<'a, P: Into<Option<&'a /*Unimplemented*/RTSPSessionPoolFilterFunc>>, Q: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, func: P, user_data: Q) -> Vec<RTSPSession> {
    //    unsafe { TODO: call ffi::gst_rtsp_session_pool_filter() }
    //}

    fn find(&self, sessionid: &str) -> Option<RTSPSession> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_session_pool_find(self.to_glib_none().0, sessionid.to_glib_none().0))
        }
    }

    fn get_max_sessions(&self) -> u32 {
        unsafe {
            ffi::gst_rtsp_session_pool_get_max_sessions(self.to_glib_none().0)
        }
    }

    fn get_n_sessions(&self) -> u32 {
        unsafe {
            ffi::gst_rtsp_session_pool_get_n_sessions(self.to_glib_none().0)
        }
    }

    fn remove(&self, sess: &RTSPSession) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::error::BoolError::from_glib(ffi::gst_rtsp_session_pool_remove(self.to_glib_none().0, sess.to_glib_none().0), "Failed to remove session from pool")
        }
    }

    fn set_max_sessions(&self, max: u32) {
        unsafe {
            ffi::gst_rtsp_session_pool_set_max_sessions(self.to_glib_none().0, max);
        }
    }

    fn connect_session_removed<F: Fn(&Self, &RTSPSession) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &RTSPSession) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0 as *mut _, b"session-removed\0".as_ptr() as *const _,
                transmute(session_removed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_max_sessions_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0 as *mut _, b"notify::max-sessions\0".as_ptr() as *const _,
                transmute(notify_max_sessions_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn session_removed_trampoline<P>(this: *mut ffi::GstRTSPSessionPool, object: *mut ffi::GstRTSPSession, f: glib_ffi::gpointer)
where P: IsA<RTSPSessionPool> {
    let f: &&(Fn(&P, &RTSPSession) + Send + Sync + 'static) = transmute(f);
    f(&RTSPSessionPool::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(object))
}

unsafe extern "C" fn notify_max_sessions_trampoline<P>(this: *mut ffi::GstRTSPSessionPool, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<RTSPSessionPool> {
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&RTSPSessionPool::from_glib_borrow(this).downcast_unchecked())
}
