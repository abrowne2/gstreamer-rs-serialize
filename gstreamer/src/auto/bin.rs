// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::ChildProxy;
use crate::Element;
#[cfg(any(feature = "v1_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
use crate::ElementFlags;
use crate::Object;
use crate::Pad;
use crate::PadDirection;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GstBin")]
    pub struct Bin(Object<ffi::GstBin, ffi::GstBinClass>) @extends Element, Object, @implements ChildProxy;

    match fn {
        type_ => || ffi::gst_bin_get_type(),
    }
}

impl Bin {
    pub const NONE: Option<&'static Bin> = None;

    #[doc(alias = "gst_bin_new")]
    pub fn new(name: Option<&str>) -> Bin {
        assert_initialized_main_thread!();
        unsafe { Element::from_glib_none(ffi::gst_bin_new(name.to_glib_none().0)).unsafe_cast() }
    }
}

unsafe impl Send for Bin {}
unsafe impl Sync for Bin {}

pub trait GstBinExt: 'static {
    #[doc(alias = "gst_bin_add")]
    fn add(&self, element: &impl IsA<Element>) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "gst_bin_find_unlinked_pad")]
    fn find_unlinked_pad(&self, direction: PadDirection) -> Option<Pad>;

    #[doc(alias = "gst_bin_get_by_interface")]
    #[doc(alias = "get_by_interface")]
    fn by_interface(&self, iface: glib::types::Type) -> Option<Element>;

    #[doc(alias = "gst_bin_get_by_name")]
    #[doc(alias = "get_by_name")]
    fn by_name(&self, name: &str) -> Option<Element>;

    #[doc(alias = "gst_bin_get_by_name_recurse_up")]
    #[doc(alias = "get_by_name_recurse_up")]
    fn by_name_recurse_up(&self, name: &str) -> Option<Element>;

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    #[doc(alias = "gst_bin_get_suppressed_flags")]
    #[doc(alias = "get_suppressed_flags")]
    fn suppressed_flags(&self) -> ElementFlags;

    #[doc(alias = "gst_bin_recalculate_latency")]
    fn recalculate_latency(&self) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "gst_bin_remove")]
    fn remove(&self, element: &impl IsA<Element>) -> Result<(), glib::error::BoolError>;

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    #[doc(alias = "gst_bin_set_suppressed_flags")]
    fn set_suppressed_flags(&self, flags: ElementFlags);

    #[doc(alias = "gst_bin_sync_children_states")]
    fn sync_children_states(&self) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "async-handling")]
    fn is_async_handling(&self) -> bool;

    #[doc(alias = "async-handling")]
    fn set_async_handling(&self, async_handling: bool);

    #[doc(alias = "message-forward")]
    fn is_message_forward(&self) -> bool;

    #[doc(alias = "message-forward")]
    fn set_message_forward(&self, message_forward: bool);

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    #[doc(alias = "deep-element-added")]
    fn connect_deep_element_added<F: Fn(&Self, &Bin, &Element) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    #[doc(alias = "deep-element-removed")]
    fn connect_deep_element_removed<F: Fn(&Self, &Bin, &Element) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "element-added")]
    fn connect_element_added<F: Fn(&Self, &Element) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "element-removed")]
    fn connect_element_removed<F: Fn(&Self, &Element) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "async-handling")]
    fn connect_async_handling_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "message-forward")]
    fn connect_message_forward_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<Bin>> GstBinExt for O {
    fn add(&self, element: &impl IsA<Element>) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_bin_add(
                    self.as_ref().to_glib_none().0,
                    element.as_ref().to_glib_none().0
                ),
                "Failed to add element"
            )
        }
    }

    fn find_unlinked_pad(&self, direction: PadDirection) -> Option<Pad> {
        unsafe {
            from_glib_full(ffi::gst_bin_find_unlinked_pad(
                self.as_ref().to_glib_none().0,
                direction.into_glib(),
            ))
        }
    }

    fn by_interface(&self, iface: glib::types::Type) -> Option<Element> {
        unsafe {
            from_glib_full(ffi::gst_bin_get_by_interface(
                self.as_ref().to_glib_none().0,
                iface.into_glib(),
            ))
        }
    }

    fn by_name(&self, name: &str) -> Option<Element> {
        unsafe {
            from_glib_full(ffi::gst_bin_get_by_name(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    fn by_name_recurse_up(&self, name: &str) -> Option<Element> {
        unsafe {
            from_glib_full(ffi::gst_bin_get_by_name_recurse_up(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    fn suppressed_flags(&self) -> ElementFlags {
        unsafe {
            from_glib(ffi::gst_bin_get_suppressed_flags(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn recalculate_latency(&self) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_bin_recalculate_latency(self.as_ref().to_glib_none().0),
                "Failed to recalculate latency"
            )
        }
    }

    fn remove(&self, element: &impl IsA<Element>) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_bin_remove(
                    self.as_ref().to_glib_none().0,
                    element.as_ref().to_glib_none().0
                ),
                "Failed to remove element"
            )
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    fn set_suppressed_flags(&self, flags: ElementFlags) {
        unsafe {
            ffi::gst_bin_set_suppressed_flags(self.as_ref().to_glib_none().0, flags.into_glib());
        }
    }

    fn sync_children_states(&self) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_bin_sync_children_states(self.as_ref().to_glib_none().0),
                "Failed to sync children states"
            )
        }
    }

    fn is_async_handling(&self) -> bool {
        glib::ObjectExt::property(self.as_ref(), "async-handling")
    }

    fn set_async_handling(&self, async_handling: bool) {
        glib::ObjectExt::set_property(self.as_ref(), "async-handling", &async_handling)
    }

    fn is_message_forward(&self) -> bool {
        glib::ObjectExt::property(self.as_ref(), "message-forward")
    }

    fn set_message_forward(&self, message_forward: bool) {
        glib::ObjectExt::set_property(self.as_ref(), "message-forward", &message_forward)
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    fn connect_deep_element_added<F: Fn(&Self, &Bin, &Element) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn deep_element_added_trampoline<
            P: IsA<Bin>,
            F: Fn(&P, &Bin, &Element) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstBin,
            sub_bin: *mut ffi::GstBin,
            element: *mut ffi::GstElement,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Bin::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(sub_bin),
                &from_glib_borrow(element),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"deep-element-added\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    deep_element_added_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    fn connect_deep_element_removed<F: Fn(&Self, &Bin, &Element) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn deep_element_removed_trampoline<
            P: IsA<Bin>,
            F: Fn(&P, &Bin, &Element) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstBin,
            sub_bin: *mut ffi::GstBin,
            element: *mut ffi::GstElement,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Bin::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(sub_bin),
                &from_glib_borrow(element),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"deep-element-removed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    deep_element_removed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_element_added<F: Fn(&Self, &Element) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn element_added_trampoline<
            P: IsA<Bin>,
            F: Fn(&P, &Element) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstBin,
            element: *mut ffi::GstElement,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Bin::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(element),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"element-added\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    element_added_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_element_removed<F: Fn(&Self, &Element) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn element_removed_trampoline<
            P: IsA<Bin>,
            F: Fn(&P, &Element) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstBin,
            element: *mut ffi::GstElement,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Bin::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(element),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"element-removed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    element_removed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_async_handling_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_async_handling_trampoline<
            P: IsA<Bin>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstBin,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Bin::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::async-handling\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_async_handling_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_message_forward_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_message_forward_trampoline<
            P: IsA<Bin>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstBin,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Bin::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::message-forward\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_message_forward_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
