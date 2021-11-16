// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::Clip;
use crate::Container;
use crate::Extractable;
use crate::MetaContainer;
use crate::TimelineElement;

glib::wrapper! {
    #[doc(alias = "GESOperationClip")]
    pub struct OperationClip(Object<ffi::GESOperationClip, ffi::GESOperationClipClass>) @extends Clip, Container, TimelineElement, @implements Extractable, MetaContainer;

    match fn {
        type_ => || ffi::ges_operation_clip_get_type(),
    }
}

impl OperationClip {
    pub const NONE: Option<&'static OperationClip> = None;
}
