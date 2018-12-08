// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use WebRTCICEComponent;
use WebRTCICEConnectionState;
use WebRTCICEGatheringState;
use ffi;
use glib::StaticType;
use glib::Value;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct WebRTCICETransport(Object<ffi::GstWebRTCICETransport, ffi::GstWebRTCICETransportClass>);

    match fn {
        get_type => || ffi::gst_webrtc_ice_transport_get_type(),
    }
}

impl WebRTCICETransport {
    pub fn connection_state_change(&self, new_state: WebRTCICEConnectionState) {
        unsafe {
            ffi::gst_webrtc_ice_transport_connection_state_change(self.to_glib_none().0, new_state.to_glib());
        }
    }

    pub fn gathering_state_change(&self, new_state: WebRTCICEGatheringState) {
        unsafe {
            ffi::gst_webrtc_ice_transport_gathering_state_change(self.to_glib_none().0, new_state.to_glib());
        }
    }

    pub fn new_candidate(&self, stream_id: u32, component: WebRTCICEComponent, attr: &str) {
        unsafe {
            ffi::gst_webrtc_ice_transport_new_candidate(self.to_glib_none().0, stream_id, component.to_glib(), attr.to_glib_none().0);
        }
    }

    pub fn selected_pair_change(&self) {
        unsafe {
            ffi::gst_webrtc_ice_transport_selected_pair_change(self.to_glib_none().0);
        }
    }

    pub fn get_property_component(&self) -> WebRTCICEComponent {
        unsafe {
            let mut value = Value::from_type(<WebRTCICEComponent as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, b"component\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    pub fn get_property_gathering_state(&self) -> WebRTCICEGatheringState {
        unsafe {
            let mut value = Value::from_type(<WebRTCICEGatheringState as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, b"gathering-state\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    pub fn get_property_state(&self) -> WebRTCICEConnectionState {
        unsafe {
            let mut value = Value::from_type(<WebRTCICEConnectionState as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, b"state\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    pub fn connect_on_new_candidate<F: Fn(&WebRTCICETransport, &str) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&WebRTCICETransport, &str) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0, b"on-new-candidate\0".as_ptr() as *const _,
                transmute(on_new_candidate_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_on_selected_candidate_pair_change<F: Fn(&WebRTCICETransport) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&WebRTCICETransport) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0, b"on-selected-candidate-pair-change\0".as_ptr() as *const _,
                transmute(on_selected_candidate_pair_change_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_property_gathering_state_notify<F: Fn(&WebRTCICETransport) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&WebRTCICETransport) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0, b"notify::gathering-state\0".as_ptr() as *const _,
                transmute(notify_gathering_state_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_property_state_notify<F: Fn(&WebRTCICETransport) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&WebRTCICETransport) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0, b"notify::state\0".as_ptr() as *const _,
                transmute(notify_state_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe impl Send for WebRTCICETransport {}
unsafe impl Sync for WebRTCICETransport {}

unsafe extern "C" fn on_new_candidate_trampoline(this: *mut ffi::GstWebRTCICETransport, object: *mut libc::c_char, f: glib_ffi::gpointer) {
    let f: &&(Fn(&WebRTCICETransport, &str) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this), &String::from_glib_none(object))
}

unsafe extern "C" fn on_selected_candidate_pair_change_trampoline(this: *mut ffi::GstWebRTCICETransport, f: glib_ffi::gpointer) {
    let f: &&(Fn(&WebRTCICETransport) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_gathering_state_trampoline(this: *mut ffi::GstWebRTCICETransport, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    let f: &&(Fn(&WebRTCICETransport) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_state_trampoline(this: *mut ffi::GstWebRTCICETransport, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    let f: &&(Fn(&WebRTCICETransport) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}
