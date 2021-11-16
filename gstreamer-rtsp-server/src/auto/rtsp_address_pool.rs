// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::RTSPAddress;
use crate::RTSPAddressFlags;
use glib::object::IsA;
use glib::translate::*;

glib::wrapper! {
    #[doc(alias = "GstRTSPAddressPool")]
    pub struct RTSPAddressPool(Object<ffi::GstRTSPAddressPool, ffi::GstRTSPAddressPoolClass>);

    match fn {
        type_ => || ffi::gst_rtsp_address_pool_get_type(),
    }
}

impl RTSPAddressPool {
    pub const NONE: Option<&'static RTSPAddressPool> = None;

    #[doc(alias = "gst_rtsp_address_pool_new")]
    pub fn new() -> RTSPAddressPool {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_rtsp_address_pool_new()) }
    }
}

impl Default for RTSPAddressPool {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for RTSPAddressPool {}
unsafe impl Sync for RTSPAddressPool {}

pub trait RTSPAddressPoolExt: 'static {
    #[doc(alias = "gst_rtsp_address_pool_acquire_address")]
    fn acquire_address(
        &self,
        flags: RTSPAddressFlags,
        n_ports: i32,
    ) -> Result<RTSPAddress, glib::BoolError>;

    #[doc(alias = "gst_rtsp_address_pool_add_range")]
    fn add_range(
        &self,
        min_address: &str,
        max_address: &str,
        min_port: u16,
        max_port: u16,
        ttl: u8,
    ) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "gst_rtsp_address_pool_clear")]
    fn clear(&self);

    #[doc(alias = "gst_rtsp_address_pool_dump")]
    fn dump(&self);

    #[doc(alias = "gst_rtsp_address_pool_has_unicast_addresses")]
    fn has_unicast_addresses(&self) -> bool;
}

impl<O: IsA<RTSPAddressPool>> RTSPAddressPoolExt for O {
    fn acquire_address(
        &self,
        flags: RTSPAddressFlags,
        n_ports: i32,
    ) -> Result<RTSPAddress, glib::BoolError> {
        unsafe {
            Option::<_>::from_glib_full(ffi::gst_rtsp_address_pool_acquire_address(
                self.as_ref().to_glib_none().0,
                flags.into_glib(),
                n_ports,
            ))
            .ok_or_else(|| glib::bool_error!("Failed to acquire address"))
        }
    }

    fn add_range(
        &self,
        min_address: &str,
        max_address: &str,
        min_port: u16,
        max_port: u16,
        ttl: u8,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_rtsp_address_pool_add_range(
                    self.as_ref().to_glib_none().0,
                    min_address.to_glib_none().0,
                    max_address.to_glib_none().0,
                    min_port,
                    max_port,
                    ttl
                ),
                "Failed to add address range"
            )
        }
    }

    fn clear(&self) {
        unsafe {
            ffi::gst_rtsp_address_pool_clear(self.as_ref().to_glib_none().0);
        }
    }

    fn dump(&self) {
        unsafe {
            ffi::gst_rtsp_address_pool_dump(self.as_ref().to_glib_none().0);
        }
    }

    fn has_unicast_addresses(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_rtsp_address_pool_has_unicast_addresses(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}
