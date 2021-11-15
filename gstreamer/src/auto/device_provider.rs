// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::Bus;
use crate::Device;
use crate::DeviceProviderFactory;
use crate::Object;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GstDeviceProvider")]
    pub struct DeviceProvider(Object<ffi::GstDeviceProvider, ffi::GstDeviceProviderClass>) @extends Object;

    match fn {
        type_ => || ffi::gst_device_provider_get_type(),
    }
}

impl DeviceProvider {
    pub const NONE: Option<&'static DeviceProvider> = None;
}

unsafe impl Send for DeviceProvider {}
unsafe impl Sync for DeviceProvider {}

pub trait DeviceProviderExt: 'static {
    #[doc(alias = "gst_device_provider_can_monitor")]
    fn can_monitor(&self) -> bool;

    #[doc(alias = "gst_device_provider_device_add")]
    fn device_add(&self, device: &impl IsA<Device>);

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    #[doc(alias = "gst_device_provider_device_changed")]
    fn device_changed(&self, device: &impl IsA<Device>, changed_device: &impl IsA<Device>);

    #[doc(alias = "gst_device_provider_device_remove")]
    fn device_remove(&self, device: &impl IsA<Device>);

    #[doc(alias = "gst_device_provider_get_bus")]
    #[doc(alias = "get_bus")]
    fn bus(&self) -> Bus;

    #[doc(alias = "gst_device_provider_get_factory")]
    #[doc(alias = "get_factory")]
    fn factory(&self) -> Option<DeviceProviderFactory>;

    #[doc(alias = "gst_device_provider_get_hidden_providers")]
    #[doc(alias = "get_hidden_providers")]
    fn hidden_providers(&self) -> Vec<glib::GString>;

    #[doc(alias = "gst_device_provider_hide_provider")]
    fn hide_provider(&self, name: &str);

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "gst_device_provider_is_started")]
    fn is_started(&self) -> bool;

    #[doc(alias = "gst_device_provider_start")]
    fn start(&self) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "gst_device_provider_stop")]
    fn stop(&self);

    #[doc(alias = "gst_device_provider_unhide_provider")]
    fn unhide_provider(&self, name: &str);

    #[doc(alias = "provider-hidden")]
    fn connect_provider_hidden<F: Fn(&Self, &str) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "provider-unhidden")]
    fn connect_provider_unhidden<F: Fn(&Self, &str) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<DeviceProvider>> DeviceProviderExt for O {
    fn can_monitor(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_device_provider_can_monitor(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn device_add(&self, device: &impl IsA<Device>) {
        unsafe {
            ffi::gst_device_provider_device_add(
                self.as_ref().to_glib_none().0,
                device.as_ref().to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    fn device_changed(&self, device: &impl IsA<Device>, changed_device: &impl IsA<Device>) {
        unsafe {
            ffi::gst_device_provider_device_changed(
                self.as_ref().to_glib_none().0,
                device.as_ref().to_glib_none().0,
                changed_device.as_ref().to_glib_none().0,
            );
        }
    }

    fn device_remove(&self, device: &impl IsA<Device>) {
        unsafe {
            ffi::gst_device_provider_device_remove(
                self.as_ref().to_glib_none().0,
                device.as_ref().to_glib_none().0,
            );
        }
    }

    fn bus(&self) -> Bus {
        unsafe {
            from_glib_full(ffi::gst_device_provider_get_bus(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn factory(&self) -> Option<DeviceProviderFactory> {
        unsafe {
            from_glib_none(ffi::gst_device_provider_get_factory(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn hidden_providers(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_device_provider_get_hidden_providers(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn hide_provider(&self, name: &str) {
        unsafe {
            ffi::gst_device_provider_hide_provider(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    fn is_started(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_device_provider_is_started(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn start(&self) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_device_provider_start(self.as_ref().to_glib_none().0),
                "Failed to start"
            )
        }
    }

    fn stop(&self) {
        unsafe {
            ffi::gst_device_provider_stop(self.as_ref().to_glib_none().0);
        }
    }

    fn unhide_provider(&self, name: &str) {
        unsafe {
            ffi::gst_device_provider_unhide_provider(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            );
        }
    }

    fn connect_provider_hidden<F: Fn(&Self, &str) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn provider_hidden_trampoline<
            P: IsA<DeviceProvider>,
            F: Fn(&P, &str) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstDeviceProvider,
            object: *mut libc::c_char,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                DeviceProvider::from_glib_borrow(this).unsafe_cast_ref(),
                &glib::GString::from_glib_borrow(object),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"provider-hidden\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    provider_hidden_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_provider_unhidden<F: Fn(&Self, &str) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn provider_unhidden_trampoline<
            P: IsA<DeviceProvider>,
            F: Fn(&P, &str) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstDeviceProvider,
            object: *mut libc::c_char,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                DeviceProvider::from_glib_borrow(this).unsafe_cast_ref(),
                &glib::GString::from_glib_borrow(object),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"provider-unhidden\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    provider_unhidden_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
