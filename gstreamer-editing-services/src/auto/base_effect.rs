// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::{Extractable, MetaContainer, Operation, TimelineElement, TrackElement};
use glib::prelude::*;
#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
use glib::translate::*;

glib::wrapper! {
    #[doc(alias = "GESBaseEffect")]
    pub struct BaseEffect(Object<ffi::GESBaseEffect, ffi::GESBaseEffectClass>) @extends Operation, TrackElement, TimelineElement, @implements Extractable, MetaContainer;

    match fn {
        type_ => || ffi::ges_base_effect_get_type(),
    }
}

impl BaseEffect {
    pub const NONE: Option<&'static BaseEffect> = None;
}

pub trait BaseEffectExt: 'static {
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_base_effect_is_time_effect")]
    fn is_time_effect(&self) -> bool;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_base_effect_register_time_property")]
    fn register_time_property(&self, child_property_name: &str) -> bool;
}

impl<O: IsA<BaseEffect>> BaseEffectExt for O {
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn is_time_effect(&self) -> bool {
        unsafe {
            from_glib(ffi::ges_base_effect_is_time_effect(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn register_time_property(&self, child_property_name: &str) -> bool {
        unsafe {
            from_glib(ffi::ges_base_effect_register_time_property(
                self.as_ref().to_glib_none().0,
                child_property_name.to_glib_none().0,
            ))
        }
    }
}
