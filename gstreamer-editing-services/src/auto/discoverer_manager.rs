// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GESDiscovererManager")]
    pub struct DiscovererManager(Object<ffi::GESDiscovererManager, ffi::GESDiscovererManagerClass>);

    match fn {
        type_ => || ffi::ges_discoverer_manager_get_type(),
    }
}

impl DiscovererManager {
    #[cfg(any(feature = "v1_24", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_24")))]
    #[doc(alias = "ges_discoverer_manager_get_timeout")]
    #[doc(alias = "get_timeout")]
    pub fn timeout(&self) -> Option<gst::ClockTime> {
        unsafe {
            from_glib(ffi::ges_discoverer_manager_get_timeout(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_24", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_24")))]
    #[doc(alias = "ges_discoverer_manager_get_use_cache")]
    #[doc(alias = "get_use_cache")]
    pub fn uses_cache(&self) -> bool {
        unsafe {
            from_glib(ffi::ges_discoverer_manager_get_use_cache(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_24", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_24")))]
    #[doc(alias = "ges_discoverer_manager_set_timeout")]
    pub fn set_timeout(&self, timeout: impl Into<Option<gst::ClockTime>>) {
        unsafe {
            ffi::ges_discoverer_manager_set_timeout(
                self.to_glib_none().0,
                timeout.into().into_glib(),
            );
        }
    }

    #[cfg(any(feature = "v1_24", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_24")))]
    #[doc(alias = "ges_discoverer_manager_set_use_cache")]
    pub fn set_use_cache(&self, use_cache: bool) {
        unsafe {
            ffi::ges_discoverer_manager_set_use_cache(self.to_glib_none().0, use_cache.into_glib());
        }
    }

    pub fn get_property_timeout(&self) -> u64 {
        glib::ObjectExt::property(self, "timeout")
    }

    pub fn set_property_timeout(&self, timeout: u64) {
        glib::ObjectExt::set_property(self, "timeout", &timeout)
    }

    #[doc(alias = "use-cache")]
    pub fn get_property_use_cache(&self) -> bool {
        glib::ObjectExt::property(self, "use-cache")
    }

    #[doc(alias = "use-cache")]
    pub fn set_property_use_cache(&self, use_cache: bool) {
        glib::ObjectExt::set_property(self, "use-cache", &use_cache)
    }

    #[doc(alias = "ges_discoverer_manager_get_default")]
    #[doc(alias = "get_default")]
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> DiscovererManager {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::ges_discoverer_manager_get_default()) }
    }

    #[cfg(any(feature = "v1_24", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_24")))]
    #[doc(alias = "load-serialized-info")]
    pub fn connect_load_serialized_info<
        F: Fn(&Self, &str) -> Option<gst_pbutils::DiscovererInfo> + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn load_serialized_info_trampoline<
            F: Fn(&DiscovererManager, &str) -> Option<gst_pbutils::DiscovererInfo> + 'static,
        >(
            this: *mut ffi::GESDiscovererManager,
            uri: *mut libc::c_char,
            f: glib::ffi::gpointer,
        ) -> *mut gst_pbutils::ffi::GstDiscovererInfo {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                &glib::GString::from_glib_borrow(uri),
            )
            .to_glib_full()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"load-serialized-info\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    load_serialized_info_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_24", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_24")))]
    pub fn emit_load_serialized_info(&self, uri: &str) -> Option<gst_pbutils::DiscovererInfo> {
        self.emit_by_name("load-serialized-info", &[&uri])
    }

    #[doc(alias = "timeout")]
    pub fn connect_timeout_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_timeout_trampoline<F: Fn(&DiscovererManager) + 'static>(
            this: *mut ffi::GESDiscovererManager,
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
                b"notify::timeout\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_timeout_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "use-cache")]
    pub fn connect_use_cache_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_use_cache_trampoline<F: Fn(&DiscovererManager) + 'static>(
            this: *mut ffi::GESDiscovererManager,
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
                b"notify::use-cache\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_use_cache_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
