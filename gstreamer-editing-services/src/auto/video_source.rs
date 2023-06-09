// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::{Extractable, MetaContainer, Source, TimelineElement, TrackElement};
use glib::prelude::*;
#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
use glib::translate::*;
#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
use std::mem;

glib::wrapper! {
    #[doc(alias = "GESVideoSource")]
    pub struct VideoSource(Object<ffi::GESVideoSource, ffi::GESVideoSourceClass>) @extends Source, TrackElement, TimelineElement, @implements Extractable, MetaContainer;

    match fn {
        type_ => || ffi::ges_video_source_get_type(),
    }
}

impl VideoSource {
    pub const NONE: Option<&'static VideoSource> = None;
}

pub trait VideoSourceExt: 'static {
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_video_source_get_natural_size")]
    #[doc(alias = "get_natural_size")]
    fn natural_size(&self) -> Option<(i32, i32)>;
}

impl<O: IsA<VideoSource>> VideoSourceExt for O {
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn natural_size(&self) -> Option<(i32, i32)> {
        unsafe {
            let mut width = mem::MaybeUninit::uninit();
            let mut height = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::ges_video_source_get_natural_size(
                self.as_ref().to_glib_none().0,
                width.as_mut_ptr(),
                height.as_mut_ptr(),
            ));
            if ret {
                Some((width.assume_init(), height.assume_init()))
            } else {
                None
            }
        }
    }
}
