use crate::{GLContext, GLDisplay};

use glib::prelude::*;
use glib::translate::*;

impl GLDisplay {
    #[doc(alias = "gst_gl_display_get_gl_context_for_thread")]
    pub fn get_gl_context_for_current_thread(
        display: &gst::ObjectLockGuard<GLDisplay>,
    ) -> Option<GLContext> {
        skip_assert_initialized!();
        unsafe {
            let ctx = ffi::gst_gl_display_get_gl_context_for_thread(
                display.as_ref().to_glib_none().0,
                std::ptr::null_mut(),
            );
            from_glib_full(ctx)
        }
    }

    #[doc(alias = "gst_gl_display_create_context")]
    pub fn create_context(
        display: &gst::ObjectLockGuard<GLDisplay>,
        other_context: Option<&impl IsA<GLContext>>,
    ) -> Result<GLContext, glib::Error> {
        skip_assert_initialized!();
        unsafe {
            let mut p_context = std::ptr::null_mut();
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::gst_gl_display_create_context(
                display.as_ref().to_glib_none().0,
                other_context.map(|p| p.as_ref()).to_glib_none().0,
                &mut p_context,
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(from_glib_full(p_context))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "gst_gl_display_add_context")]
    pub fn add_context(
        display: &gst::ObjectLockGuard<GLDisplay>,
        context: &impl IsA<GLContext>,
    ) -> Result<(), glib::error::BoolError> {
        skip_assert_initialized!();
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_gl_display_add_context(
                    display.as_ref().to_glib_none().0,
                    context.as_ref().to_glib_none().0
                ),
                "Failed to add OpenGL context"
            )
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "gst_gl_display_remove_context")]
    pub fn remove_context(
        display: &gst::ObjectLockGuard<GLDisplay>,
        context: &impl IsA<GLContext>,
    ) {
        skip_assert_initialized!();
        unsafe {
            ffi::gst_gl_display_remove_context(
                display.as_ref().to_glib_none().0,
                context.as_ref().to_glib_none().0,
            );
        }
    }
}
