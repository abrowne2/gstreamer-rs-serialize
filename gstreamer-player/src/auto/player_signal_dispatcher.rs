// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ffi;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct PlayerSignalDispatcher(Object<ffi::GstPlayerSignalDispatcher, ffi::GstPlayerSignalDispatcherInterface>);

    match fn {
        get_type => || ffi::gst_player_signal_dispatcher_get_type(),
    }
}

unsafe impl Send for PlayerSignalDispatcher {}
unsafe impl Sync for PlayerSignalDispatcher {}

pub trait PlayerSignalDispatcherExt: 'static {}

impl<O: IsA<PlayerSignalDispatcher>> PlayerSignalDispatcherExt for O {}
