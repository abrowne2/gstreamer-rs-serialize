// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::Clip;
use crate::Container;
use crate::Extractable;
use crate::MetaContainer;
use crate::OperationClip;
use crate::TimelineElement;

glib::wrapper! {
    #[doc(alias = "GESBaseTransitionClip")]
    pub struct BaseTransitionClip(Object<ffi::GESBaseTransitionClip, ffi::GESBaseTransitionClipClass>) @extends OperationClip, Clip, Container, TimelineElement, @implements Extractable, MetaContainer;

    match fn {
        type_ => || ffi::ges_base_transition_clip_get_type(),
    }
}

impl BaseTransitionClip {
    pub const NONE: Option<&'static BaseTransitionClip> = None;
}
