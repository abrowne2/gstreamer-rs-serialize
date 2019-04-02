// Copyright (C) 2019 Víctor Jáquez <vjaquez@igalia.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi;
use glib::translate::*;
use glib_ffi::gpointer;
use libc::uintptr_t;
use GLDisplayX11;

impl GLDisplayX11 {
    pub unsafe fn new_with_display(display: uintptr_t) -> Option<GLDisplayX11> {
        from_glib_full(ffi::gst_gl_display_x11_new_with_display(
            display as gpointer,
        ))
    }
}