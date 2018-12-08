// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Error;
use GLContext;
use GLSLProfile;
use GLSLVersion;
use ffi;
use glib::translate::*;
use gst;
use gst_ffi;
use std::ptr;

glib_wrapper! {
    pub struct GLSLStage(Object<ffi::GstGLSLStage, ffi::GstGLSLStageClass>): [
        gst::Object => gst_ffi::GstObject,
    ];

    match fn {
        get_type => || ffi::gst_glsl_stage_get_type(),
    }
}

impl GLSLStage {
    pub fn new(context: &GLContext, type_: u32) -> GLSLStage {
        skip_assert_initialized!();
        unsafe {
            from_glib_none(ffi::gst_glsl_stage_new(context.to_glib_none().0, type_))
        }
    }

    pub fn new_default_fragment(context: &GLContext) -> GLSLStage {
        skip_assert_initialized!();
        unsafe {
            from_glib_none(ffi::gst_glsl_stage_new_default_fragment(context.to_glib_none().0))
        }
    }

    pub fn new_default_vertex(context: &GLContext) -> GLSLStage {
        skip_assert_initialized!();
        unsafe {
            from_glib_none(ffi::gst_glsl_stage_new_default_vertex(context.to_glib_none().0))
        }
    }

    pub fn new_with_string(context: &GLContext, type_: u32, version: GLSLVersion, profile: GLSLProfile, str: &str) -> GLSLStage {
        skip_assert_initialized!();
        unsafe {
            from_glib_none(ffi::gst_glsl_stage_new_with_string(context.to_glib_none().0, type_, version.to_glib(), profile.to_glib(), str.to_glib_none().0))
        }
    }

    pub fn new_with_strings(context: &GLContext, type_: u32, version: GLSLVersion, profile: GLSLProfile, str: &[&str]) -> GLSLStage {
        skip_assert_initialized!();
        let n_strings = str.len() as i32;
        unsafe {
            from_glib_none(ffi::gst_glsl_stage_new_with_strings(context.to_glib_none().0, type_, version.to_glib(), profile.to_glib(), n_strings, str.to_glib_none().0))
        }
    }

    pub fn compile(&self) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gst_glsl_stage_compile(self.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn get_handle(&self) -> u32 {
        unsafe {
            ffi::gst_glsl_stage_get_handle(self.to_glib_none().0)
        }
    }

    pub fn get_profile(&self) -> GLSLProfile {
        unsafe {
            from_glib(ffi::gst_glsl_stage_get_profile(self.to_glib_none().0))
        }
    }

    pub fn get_shader_type(&self) -> u32 {
        unsafe {
            ffi::gst_glsl_stage_get_shader_type(self.to_glib_none().0)
        }
    }

    pub fn get_version(&self) -> GLSLVersion {
        unsafe {
            from_glib(ffi::gst_glsl_stage_get_version(self.to_glib_none().0))
        }
    }

    pub fn set_strings(&self, version: GLSLVersion, profile: GLSLProfile, str: &[&str]) -> bool {
        let n_strings = str.len() as i32;
        unsafe {
            from_glib(ffi::gst_glsl_stage_set_strings(self.to_glib_none().0, version.to_glib(), profile.to_glib(), n_strings, str.to_glib_none().0))
        }
    }
}

unsafe impl Send for GLSLStage {}
unsafe impl Sync for GLSLStage {}
