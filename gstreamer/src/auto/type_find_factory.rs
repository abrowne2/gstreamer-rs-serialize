// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::Caps;
use crate::Object;
use crate::PluginFeature;
use glib::translate::*;

glib::wrapper! {
    #[doc(alias = "GstTypeFindFactory")]
    pub struct TypeFindFactory(Object<ffi::GstTypeFindFactory, ffi::GstTypeFindFactoryClass>) @extends PluginFeature, Object;

    match fn {
        type_ => || ffi::gst_type_find_factory_get_type(),
    }
}

impl TypeFindFactory {
    #[doc(alias = "gst_type_find_factory_get_caps")]
    #[doc(alias = "get_caps")]
    pub fn caps(&self) -> Option<Caps> {
        unsafe { from_glib_none(ffi::gst_type_find_factory_get_caps(self.to_glib_none().0)) }
    }

    #[doc(alias = "gst_type_find_factory_get_extensions")]
    #[doc(alias = "get_extensions")]
    pub fn extensions(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gst_type_find_factory_get_extensions(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_type_find_factory_has_function")]
    pub fn has_function(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_type_find_factory_has_function(
                self.to_glib_none().0,
            ))
        }
    }
}

unsafe impl Send for TypeFindFactory {}
unsafe impl Sync for TypeFindFactory {}
