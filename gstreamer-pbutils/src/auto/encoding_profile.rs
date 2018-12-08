// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use DiscovererInfo;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use gst;

glib_wrapper! {
    pub struct EncodingProfile(Object<ffi::GstEncodingProfile, ffi::GstEncodingProfileClass>);

    match fn {
        get_type => || ffi::gst_encoding_profile_get_type(),
    }
}

impl EncodingProfile {
    pub fn find<'a, 'b, P: Into<Option<&'a str>>, Q: Into<Option<&'b str>>>(targetname: &str, profilename: P, category: Q) -> Option<EncodingProfile> {
        assert_initialized_main_thread!();
        let profilename = profilename.into();
        let profilename = profilename.to_glib_none();
        let category = category.into();
        let category = category.to_glib_none();
        unsafe {
            from_glib_full(ffi::gst_encoding_profile_find(targetname.to_glib_none().0, profilename.0, category.0))
        }
    }

    pub fn from_discoverer(info: &DiscovererInfo) -> Option<EncodingProfile> {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gst_encoding_profile_from_discoverer(info.to_glib_none().0))
        }
    }
}

unsafe impl Send for EncodingProfile {}
unsafe impl Sync for EncodingProfile {}

pub trait EncodingProfileExt: 'static {
    fn copy(&self) -> EncodingProfile;

    fn get_allow_dynamic_output(&self) -> bool;

    fn get_description(&self) -> Option<String>;

    fn get_file_extension(&self) -> Option<String>;

    fn get_format(&self) -> gst::Caps;

    fn get_input_caps(&self) -> gst::Caps;

    fn get_name(&self) -> Option<String>;

    fn get_presence(&self) -> u32;

    fn get_preset(&self) -> Option<String>;

    fn get_preset_name(&self) -> Option<String>;

    fn get_restriction(&self) -> Option<gst::Caps>;

    fn get_type_nick(&self) -> Option<String>;

    fn is_enabled(&self) -> bool;

    fn is_equal<P: IsA<EncodingProfile>>(&self, b: &P) -> bool;
}

impl<O: IsA<EncodingProfile>> EncodingProfileExt for O {
    fn copy(&self) -> EncodingProfile {
        unsafe {
            from_glib_full(ffi::gst_encoding_profile_copy(self.to_glib_none().0))
        }
    }

    fn get_allow_dynamic_output(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_encoding_profile_get_allow_dynamic_output(self.to_glib_none().0))
        }
    }

    fn get_description(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gst_encoding_profile_get_description(self.to_glib_none().0))
        }
    }

    fn get_file_extension(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gst_encoding_profile_get_file_extension(self.to_glib_none().0))
        }
    }

    fn get_format(&self) -> gst::Caps {
        unsafe {
            from_glib_full(ffi::gst_encoding_profile_get_format(self.to_glib_none().0))
        }
    }

    fn get_input_caps(&self) -> gst::Caps {
        unsafe {
            from_glib_full(ffi::gst_encoding_profile_get_input_caps(self.to_glib_none().0))
        }
    }

    fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gst_encoding_profile_get_name(self.to_glib_none().0))
        }
    }

    fn get_presence(&self) -> u32 {
        unsafe {
            ffi::gst_encoding_profile_get_presence(self.to_glib_none().0)
        }
    }

    fn get_preset(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gst_encoding_profile_get_preset(self.to_glib_none().0))
        }
    }

    fn get_preset_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gst_encoding_profile_get_preset_name(self.to_glib_none().0))
        }
    }

    fn get_restriction(&self) -> Option<gst::Caps> {
        unsafe {
            from_glib_full(ffi::gst_encoding_profile_get_restriction(self.to_glib_none().0))
        }
    }

    fn get_type_nick(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gst_encoding_profile_get_type_nick(self.to_glib_none().0))
        }
    }

    fn is_enabled(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_encoding_profile_is_enabled(self.to_glib_none().0))
        }
    }

    fn is_equal<P: IsA<EncodingProfile>>(&self, b: &P) -> bool {
        unsafe {
            from_glib(ffi::gst_encoding_profile_is_equal(self.to_glib_none().0, b.to_glib_none().0))
        }
    }
}
