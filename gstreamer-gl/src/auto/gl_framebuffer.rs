// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use GLContext;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use gst;
use gst_ffi;
use std::mem;

glib_wrapper! {
    pub struct GLFramebuffer(Object<ffi::GstGLFramebuffer, ffi::GstGLFramebufferClass>): [
        gst::Object => gst_ffi::GstObject,
    ];

    match fn {
        get_type => || ffi::gst_gl_framebuffer_get_type(),
    }
}

impl GLFramebuffer {
    pub fn new(context: &GLContext) -> GLFramebuffer {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gst_gl_framebuffer_new(context.to_glib_none().0))
        }
    }

    pub fn new_with_default_depth(context: &GLContext, width: u32, height: u32) -> GLFramebuffer {
        skip_assert_initialized!();
        unsafe {
            from_glib_none(ffi::gst_gl_framebuffer_new_with_default_depth(context.to_glib_none().0, width, height))
        }
    }
}

unsafe impl Send for GLFramebuffer {}
unsafe impl Sync for GLFramebuffer {}

pub trait GLFramebufferExt: 'static {
    //fn attach(&self, attachment_point: u32, mem: /*Ignored*/&mut GLBaseMemory);

    fn bind(&self);

    //fn draw_to_texture<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, mem: /*Ignored*/&mut GLMemory, func: /*Unknown conversion*//*Unimplemented*/GLFramebufferFunc, user_data: P) -> bool;

    fn get_effective_dimensions(&self) -> (u32, u32);

    fn get_id(&self) -> u32;
}

impl<O: IsA<GLFramebuffer>> GLFramebufferExt for O {
    //fn attach(&self, attachment_point: u32, mem: /*Ignored*/&mut GLBaseMemory) {
    //    unsafe { TODO: call ffi::gst_gl_framebuffer_attach() }
    //}

    fn bind(&self) {
        unsafe {
            ffi::gst_gl_framebuffer_bind(self.to_glib_none().0);
        }
    }

    //fn draw_to_texture<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, mem: /*Ignored*/&mut GLMemory, func: /*Unknown conversion*//*Unimplemented*/GLFramebufferFunc, user_data: P) -> bool {
    //    unsafe { TODO: call ffi::gst_gl_framebuffer_draw_to_texture() }
    //}

    fn get_effective_dimensions(&self) -> (u32, u32) {
        unsafe {
            let mut width = mem::uninitialized();
            let mut height = mem::uninitialized();
            ffi::gst_gl_framebuffer_get_effective_dimensions(self.to_glib_none().0, &mut width, &mut height);
            (width, height)
        }
    }

    fn get_id(&self) -> u32 {
        unsafe {
            ffi::gst_gl_framebuffer_get_id(self.to_glib_none().0)
        }
    }
}
