// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::Extractable;
use crate::MetaContainer;
use crate::TimelineElement;
use crate::TrackElement;

glib::wrapper! {
    #[doc(alias = "GESOperation")]
    pub struct Operation(Object<ffi::GESOperation, ffi::GESOperationClass>) @extends TrackElement, TimelineElement, @implements Extractable, MetaContainer;

    match fn {
        type_ => || ffi::ges_operation_get_type(),
    }
}

impl Operation {
    pub const NONE: Option<&'static Operation> = None;
}
