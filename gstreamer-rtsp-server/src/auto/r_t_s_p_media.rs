// This file was generated by gir (https://github.com/gtk-rs/gir @ 6855214)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

use RTSPAddressPool;
use RTSPMediaStatus;
use RTSPPublishClockMode;
use RTSPStream;
use RTSPSuspendMode;
use RTSPTransportMode;
use ffi;
use glib;
use glib::StaticType;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use gst;
use gst_rtsp;
use libc;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct RTSPMedia(Object<ffi::GstRTSPMedia, ffi::GstRTSPMediaClass>);

    match fn {
        get_type => || ffi::gst_rtsp_media_get_type(),
    }
}

impl RTSPMedia {
    pub fn new<P: IsA<gst::Element>>(element: &P) -> RTSPMedia {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gst_rtsp_media_new(element.to_glib_full()))
        }
    }
}

unsafe impl Send for RTSPMedia {}
unsafe impl Sync for RTSPMedia {}

pub trait RTSPMediaExt {
    fn collect_streams(&self);

    //fn complete_pipeline(&self, transports: /*Unknown conversion*//*Unimplemented*/PtrArray TypeId { ns_id: 9, id: 31 }) -> bool;

    fn create_stream<P: IsA<gst::Element>, Q: IsA<gst::Pad>>(&self, payloader: &P, pad: &Q) -> Option<RTSPStream>;

    fn find_stream(&self, control: &str) -> Option<RTSPStream>;

    fn get_address_pool(&self) -> Option<RTSPAddressPool>;

    fn get_base_time(&self) -> gst::ClockTime;

    fn get_buffer_size(&self) -> u32;

    fn get_clock(&self) -> Option<gst::Clock>;

    fn get_element(&self) -> Option<gst::Element>;

    fn get_latency(&self) -> u32;

    fn get_multicast_iface(&self) -> Option<String>;

    //fn get_permissions(&self) -> /*Ignored*/Option<RTSPPermissions>;

    fn get_profiles(&self) -> gst_rtsp::RTSPProfile;

    fn get_protocols(&self) -> gst_rtsp::RTSPLowerTrans;

    fn get_publish_clock_mode(&self) -> RTSPPublishClockMode;

    fn get_range_string(&self, play: bool, unit: gst_rtsp::RTSPRangeUnit) -> Option<String>;

    fn get_retransmission_time(&self) -> gst::ClockTime;

    fn get_status(&self) -> RTSPMediaStatus;

    fn get_stream(&self, idx: u32) -> Option<RTSPStream>;

    fn get_suspend_mode(&self) -> RTSPSuspendMode;

    //fn get_time_provider<'a, P: Into<Option<&'a str>>>(&self, address: P, port: u16) -> /*Ignored*/Option<gst_net::NetTimeProvider>;

    fn get_transport_mode(&self) -> RTSPTransportMode;

    //fn handle_sdp(&self, sdp: /*Ignored*/&mut gst_sdp::SDPMessage) -> bool;

    fn is_eos_shutdown(&self) -> bool;

    fn is_reusable(&self) -> bool;

    fn is_shared(&self) -> bool;

    fn is_stop_on_disconnect(&self) -> bool;

    fn is_time_provider(&self) -> bool;

    fn n_streams(&self) -> u32;

    //fn prepare<'a, P: Into<Option<&'a /*Ignored*/RTSPThread>>>(&self, thread: P) -> bool;

    //fn seek(&self, range: /*Ignored*/&mut gst_rtsp::RTSPTimeRange) -> bool;

    //fn seek_full(&self, range: /*Ignored*/&mut gst_rtsp::RTSPTimeRange, flags: /*Ignored*/gst::SeekFlags) -> bool;

    //fn seekable(&self) -> /*Ignored*/gst::ClockTimeDiff;

    fn set_address_pool<'a, P: Into<Option<&'a RTSPAddressPool>>>(&self, pool: P);

    fn set_buffer_size(&self, size: u32);

    fn set_clock<'a, P: IsA<gst::Clock> + 'a, Q: Into<Option<&'a P>>>(&self, clock: Q);

    fn set_eos_shutdown(&self, eos_shutdown: bool);

    fn set_latency(&self, latency: u32);

    fn set_multicast_iface<'a, P: Into<Option<&'a str>>>(&self, multicast_iface: P);

    //fn set_permissions<'a, P: Into<Option<&'a /*Ignored*/RTSPPermissions>>>(&self, permissions: P);

    fn set_pipeline_state(&self, state: gst::State);

    fn set_profiles(&self, profiles: gst_rtsp::RTSPProfile);

    fn set_protocols(&self, protocols: gst_rtsp::RTSPLowerTrans);

    fn set_publish_clock_mode(&self, mode: RTSPPublishClockMode);

    fn set_retransmission_time(&self, time: gst::ClockTime);

    fn set_reusable(&self, reusable: bool);

    fn set_shared(&self, shared: bool);

    //fn set_state(&self, state: gst::State, transports: /*Unknown conversion*//*Unimplemented*/PtrArray TypeId { ns_id: 1, id: 26 }) -> bool;

    fn set_stop_on_disconnect(&self, stop_on_disconnect: bool);

    fn set_suspend_mode(&self, mode: RTSPSuspendMode);

    fn set_transport_mode(&self, mode: RTSPTransportMode);

    //fn setup_sdp(&self, sdp: /*Ignored*/&mut gst_sdp::SDPMessage, info: /*Ignored*/&mut SDPInfo) -> bool;

    fn suspend(&self) -> Result<(), glib::error::BoolError>;

    fn take_pipeline(&self, pipeline: &gst::Pipeline);

    fn unprepare(&self) -> Result<(), glib::error::BoolError>;

    fn unsuspend(&self) -> Result<(), glib::error::BoolError>;

    fn use_time_provider(&self, time_provider: bool);

    fn get_property_eos_shutdown(&self) -> bool;

    fn get_property_reusable(&self) -> bool;

    fn get_property_shared(&self) -> bool;

    fn get_property_stop_on_disconnect(&self) -> bool;

    fn set_property_time_provider(&self, time_provider: bool);

    fn connect_new_state<F: Fn(&Self, i32) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_new_stream<F: Fn(&Self, &RTSPStream) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_prepared<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_removed_stream<F: Fn(&Self, &RTSPStream) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_target_state<F: Fn(&Self, i32) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_unprepared<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_buffer_size_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_clock_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_element_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_eos_shutdown_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_latency_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_profiles_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_protocols_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_reusable_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_shared_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_stop_on_disconnect_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_suspend_mode_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_time_provider_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_transport_mode_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<RTSPMedia> + IsA<glib::object::Object>> RTSPMediaExt for O {
    fn collect_streams(&self) {
        unsafe {
            ffi::gst_rtsp_media_collect_streams(self.to_glib_none().0);
        }
    }

    //fn complete_pipeline(&self, transports: /*Unknown conversion*//*Unimplemented*/PtrArray TypeId { ns_id: 9, id: 31 }) -> bool {
    //    unsafe { TODO: call ffi::gst_rtsp_media_complete_pipeline() }
    //}

    fn create_stream<P: IsA<gst::Element>, Q: IsA<gst::Pad>>(&self, payloader: &P, pad: &Q) -> Option<RTSPStream> {
        unsafe {
            from_glib_none(ffi::gst_rtsp_media_create_stream(self.to_glib_none().0, payloader.to_glib_none().0, pad.to_glib_none().0))
        }
    }

    fn find_stream(&self, control: &str) -> Option<RTSPStream> {
        unsafe {
            from_glib_none(ffi::gst_rtsp_media_find_stream(self.to_glib_none().0, control.to_glib_none().0))
        }
    }

    fn get_address_pool(&self) -> Option<RTSPAddressPool> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_media_get_address_pool(self.to_glib_none().0))
        }
    }

    fn get_base_time(&self) -> gst::ClockTime {
        unsafe {
            from_glib(ffi::gst_rtsp_media_get_base_time(self.to_glib_none().0))
        }
    }

    fn get_buffer_size(&self) -> u32 {
        unsafe {
            ffi::gst_rtsp_media_get_buffer_size(self.to_glib_none().0)
        }
    }

    fn get_clock(&self) -> Option<gst::Clock> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_media_get_clock(self.to_glib_none().0))
        }
    }

    fn get_element(&self) -> Option<gst::Element> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_media_get_element(self.to_glib_none().0))
        }
    }

    fn get_latency(&self) -> u32 {
        unsafe {
            ffi::gst_rtsp_media_get_latency(self.to_glib_none().0)
        }
    }

    fn get_multicast_iface(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_media_get_multicast_iface(self.to_glib_none().0))
        }
    }

    //fn get_permissions(&self) -> /*Ignored*/Option<RTSPPermissions> {
    //    unsafe { TODO: call ffi::gst_rtsp_media_get_permissions() }
    //}

    fn get_profiles(&self) -> gst_rtsp::RTSPProfile {
        unsafe {
            from_glib(ffi::gst_rtsp_media_get_profiles(self.to_glib_none().0))
        }
    }

    fn get_protocols(&self) -> gst_rtsp::RTSPLowerTrans {
        unsafe {
            from_glib(ffi::gst_rtsp_media_get_protocols(self.to_glib_none().0))
        }
    }

    fn get_publish_clock_mode(&self) -> RTSPPublishClockMode {
        unsafe {
            from_glib(ffi::gst_rtsp_media_get_publish_clock_mode(self.to_glib_none().0))
        }
    }

    fn get_range_string(&self, play: bool, unit: gst_rtsp::RTSPRangeUnit) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_media_get_range_string(self.to_glib_none().0, play.to_glib(), unit.to_glib()))
        }
    }

    fn get_retransmission_time(&self) -> gst::ClockTime {
        unsafe {
            from_glib(ffi::gst_rtsp_media_get_retransmission_time(self.to_glib_none().0))
        }
    }

    fn get_status(&self) -> RTSPMediaStatus {
        unsafe {
            from_glib(ffi::gst_rtsp_media_get_status(self.to_glib_none().0))
        }
    }

    fn get_stream(&self, idx: u32) -> Option<RTSPStream> {
        unsafe {
            from_glib_none(ffi::gst_rtsp_media_get_stream(self.to_glib_none().0, idx))
        }
    }

    fn get_suspend_mode(&self) -> RTSPSuspendMode {
        unsafe {
            from_glib(ffi::gst_rtsp_media_get_suspend_mode(self.to_glib_none().0))
        }
    }

    //fn get_time_provider<'a, P: Into<Option<&'a str>>>(&self, address: P, port: u16) -> /*Ignored*/Option<gst_net::NetTimeProvider> {
    //    unsafe { TODO: call ffi::gst_rtsp_media_get_time_provider() }
    //}

    fn get_transport_mode(&self) -> RTSPTransportMode {
        unsafe {
            from_glib(ffi::gst_rtsp_media_get_transport_mode(self.to_glib_none().0))
        }
    }

    //fn handle_sdp(&self, sdp: /*Ignored*/&mut gst_sdp::SDPMessage) -> bool {
    //    unsafe { TODO: call ffi::gst_rtsp_media_handle_sdp() }
    //}

    fn is_eos_shutdown(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_rtsp_media_is_eos_shutdown(self.to_glib_none().0))
        }
    }

    fn is_reusable(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_rtsp_media_is_reusable(self.to_glib_none().0))
        }
    }

    fn is_shared(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_rtsp_media_is_shared(self.to_glib_none().0))
        }
    }

    fn is_stop_on_disconnect(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_rtsp_media_is_stop_on_disconnect(self.to_glib_none().0))
        }
    }

    fn is_time_provider(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_rtsp_media_is_time_provider(self.to_glib_none().0))
        }
    }

    fn n_streams(&self) -> u32 {
        unsafe {
            ffi::gst_rtsp_media_n_streams(self.to_glib_none().0)
        }
    }

    //fn prepare<'a, P: Into<Option<&'a /*Ignored*/RTSPThread>>>(&self, thread: P) -> bool {
    //    unsafe { TODO: call ffi::gst_rtsp_media_prepare() }
    //}

    //fn seek(&self, range: /*Ignored*/&mut gst_rtsp::RTSPTimeRange) -> bool {
    //    unsafe { TODO: call ffi::gst_rtsp_media_seek() }
    //}

    //fn seek_full(&self, range: /*Ignored*/&mut gst_rtsp::RTSPTimeRange, flags: /*Ignored*/gst::SeekFlags) -> bool {
    //    unsafe { TODO: call ffi::gst_rtsp_media_seek_full() }
    //}

    //fn seekable(&self) -> /*Ignored*/gst::ClockTimeDiff {
    //    unsafe { TODO: call ffi::gst_rtsp_media_seekable() }
    //}

    fn set_address_pool<'a, P: Into<Option<&'a RTSPAddressPool>>>(&self, pool: P) {
        let pool = pool.into();
        let pool = pool.to_glib_none();
        unsafe {
            ffi::gst_rtsp_media_set_address_pool(self.to_glib_none().0, pool.0);
        }
    }

    fn set_buffer_size(&self, size: u32) {
        unsafe {
            ffi::gst_rtsp_media_set_buffer_size(self.to_glib_none().0, size);
        }
    }

    fn set_clock<'a, P: IsA<gst::Clock> + 'a, Q: Into<Option<&'a P>>>(&self, clock: Q) {
        let clock = clock.into();
        let clock = clock.to_glib_none();
        unsafe {
            ffi::gst_rtsp_media_set_clock(self.to_glib_none().0, clock.0);
        }
    }

    fn set_eos_shutdown(&self, eos_shutdown: bool) {
        unsafe {
            ffi::gst_rtsp_media_set_eos_shutdown(self.to_glib_none().0, eos_shutdown.to_glib());
        }
    }

    fn set_latency(&self, latency: u32) {
        unsafe {
            ffi::gst_rtsp_media_set_latency(self.to_glib_none().0, latency);
        }
    }

    fn set_multicast_iface<'a, P: Into<Option<&'a str>>>(&self, multicast_iface: P) {
        let multicast_iface = multicast_iface.into();
        let multicast_iface = multicast_iface.to_glib_none();
        unsafe {
            ffi::gst_rtsp_media_set_multicast_iface(self.to_glib_none().0, multicast_iface.0);
        }
    }

    //fn set_permissions<'a, P: Into<Option<&'a /*Ignored*/RTSPPermissions>>>(&self, permissions: P) {
    //    unsafe { TODO: call ffi::gst_rtsp_media_set_permissions() }
    //}

    fn set_pipeline_state(&self, state: gst::State) {
        unsafe {
            ffi::gst_rtsp_media_set_pipeline_state(self.to_glib_none().0, state.to_glib());
        }
    }

    fn set_profiles(&self, profiles: gst_rtsp::RTSPProfile) {
        unsafe {
            ffi::gst_rtsp_media_set_profiles(self.to_glib_none().0, profiles.to_glib());
        }
    }

    fn set_protocols(&self, protocols: gst_rtsp::RTSPLowerTrans) {
        unsafe {
            ffi::gst_rtsp_media_set_protocols(self.to_glib_none().0, protocols.to_glib());
        }
    }

    fn set_publish_clock_mode(&self, mode: RTSPPublishClockMode) {
        unsafe {
            ffi::gst_rtsp_media_set_publish_clock_mode(self.to_glib_none().0, mode.to_glib());
        }
    }

    fn set_retransmission_time(&self, time: gst::ClockTime) {
        unsafe {
            ffi::gst_rtsp_media_set_retransmission_time(self.to_glib_none().0, time.to_glib());
        }
    }

    fn set_reusable(&self, reusable: bool) {
        unsafe {
            ffi::gst_rtsp_media_set_reusable(self.to_glib_none().0, reusable.to_glib());
        }
    }

    fn set_shared(&self, shared: bool) {
        unsafe {
            ffi::gst_rtsp_media_set_shared(self.to_glib_none().0, shared.to_glib());
        }
    }

    //fn set_state(&self, state: gst::State, transports: /*Unknown conversion*//*Unimplemented*/PtrArray TypeId { ns_id: 1, id: 26 }) -> bool {
    //    unsafe { TODO: call ffi::gst_rtsp_media_set_state() }
    //}

    fn set_stop_on_disconnect(&self, stop_on_disconnect: bool) {
        unsafe {
            ffi::gst_rtsp_media_set_stop_on_disconnect(self.to_glib_none().0, stop_on_disconnect.to_glib());
        }
    }

    fn set_suspend_mode(&self, mode: RTSPSuspendMode) {
        unsafe {
            ffi::gst_rtsp_media_set_suspend_mode(self.to_glib_none().0, mode.to_glib());
        }
    }

    fn set_transport_mode(&self, mode: RTSPTransportMode) {
        unsafe {
            ffi::gst_rtsp_media_set_transport_mode(self.to_glib_none().0, mode.to_glib());
        }
    }

    //fn setup_sdp(&self, sdp: /*Ignored*/&mut gst_sdp::SDPMessage, info: /*Ignored*/&mut SDPInfo) -> bool {
    //    unsafe { TODO: call ffi::gst_rtsp_media_setup_sdp() }
    //}

    fn suspend(&self) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::error::BoolError::from_glib(ffi::gst_rtsp_media_suspend(self.to_glib_none().0), "Failed to suspend media")
        }
    }

    fn take_pipeline(&self, pipeline: &gst::Pipeline) {
        unsafe {
            ffi::gst_rtsp_media_take_pipeline(self.to_glib_none().0, pipeline.to_glib_full());
        }
    }

    fn unprepare(&self) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::error::BoolError::from_glib(ffi::gst_rtsp_media_unprepare(self.to_glib_none().0), "Failed to unprepare media")
        }
    }

    fn unsuspend(&self) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::error::BoolError::from_glib(ffi::gst_rtsp_media_unsuspend(self.to_glib_none().0), "Failed to unsuspend media")
        }
    }

    fn use_time_provider(&self, time_provider: bool) {
        unsafe {
            ffi::gst_rtsp_media_use_time_provider(self.to_glib_none().0, time_provider.to_glib());
        }
    }

    fn get_property_eos_shutdown(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "eos-shutdown".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_reusable(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "reusable".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_shared(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "shared".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_stop_on_disconnect(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "stop-on-disconnect".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_time_provider(&self, time_provider: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "time-provider".to_glib_none().0, Value::from(&time_provider).to_glib_none().0);
        }
    }

    fn connect_new_state<F: Fn(&Self, i32) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, i32) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "new-state",
                transmute(new_state_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_new_stream<F: Fn(&Self, &RTSPStream) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &RTSPStream) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "new-stream",
                transmute(new_stream_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_prepared<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "prepared",
                transmute(prepared_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_removed_stream<F: Fn(&Self, &RTSPStream) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &RTSPStream) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "removed-stream",
                transmute(removed_stream_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_target_state<F: Fn(&Self, i32) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, i32) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "target-state",
                transmute(target_state_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_unprepared<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "unprepared",
                transmute(unprepared_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_buffer_size_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::buffer-size",
                transmute(notify_buffer_size_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_clock_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::clock",
                transmute(notify_clock_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_element_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::element",
                transmute(notify_element_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_eos_shutdown_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::eos-shutdown",
                transmute(notify_eos_shutdown_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_latency_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::latency",
                transmute(notify_latency_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_profiles_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::profiles",
                transmute(notify_profiles_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_protocols_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::protocols",
                transmute(notify_protocols_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_reusable_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::reusable",
                transmute(notify_reusable_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_shared_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::shared",
                transmute(notify_shared_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_stop_on_disconnect_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::stop-on-disconnect",
                transmute(notify_stop_on_disconnect_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_suspend_mode_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::suspend-mode",
                transmute(notify_suspend_mode_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_time_provider_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::time-provider",
                transmute(notify_time_provider_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_transport_mode_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::transport-mode",
                transmute(notify_transport_mode_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn new_state_trampoline<P>(this: *mut ffi::GstRTSPMedia, object: libc::c_int, f: glib_ffi::gpointer)
where P: IsA<RTSPMedia> {
    callback_guard!();
    let f: &&(Fn(&P, i32) + Send + Sync + 'static) = transmute(f);
    f(&RTSPMedia::from_glib_borrow(this).downcast_unchecked(), object)
}

unsafe extern "C" fn new_stream_trampoline<P>(this: *mut ffi::GstRTSPMedia, object: *mut ffi::GstRTSPStream, f: glib_ffi::gpointer)
where P: IsA<RTSPMedia> {
    callback_guard!();
    let f: &&(Fn(&P, &RTSPStream) + Send + Sync + 'static) = transmute(f);
    f(&RTSPMedia::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(object))
}

unsafe extern "C" fn prepared_trampoline<P>(this: *mut ffi::GstRTSPMedia, f: glib_ffi::gpointer)
where P: IsA<RTSPMedia> {
    callback_guard!();
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&RTSPMedia::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn removed_stream_trampoline<P>(this: *mut ffi::GstRTSPMedia, object: *mut ffi::GstRTSPStream, f: glib_ffi::gpointer)
where P: IsA<RTSPMedia> {
    callback_guard!();
    let f: &&(Fn(&P, &RTSPStream) + Send + Sync + 'static) = transmute(f);
    f(&RTSPMedia::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(object))
}

unsafe extern "C" fn target_state_trampoline<P>(this: *mut ffi::GstRTSPMedia, object: libc::c_int, f: glib_ffi::gpointer)
where P: IsA<RTSPMedia> {
    callback_guard!();
    let f: &&(Fn(&P, i32) + Send + Sync + 'static) = transmute(f);
    f(&RTSPMedia::from_glib_borrow(this).downcast_unchecked(), object)
}

unsafe extern "C" fn unprepared_trampoline<P>(this: *mut ffi::GstRTSPMedia, f: glib_ffi::gpointer)
where P: IsA<RTSPMedia> {
    callback_guard!();
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&RTSPMedia::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_buffer_size_trampoline<P>(this: *mut ffi::GstRTSPMedia, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<RTSPMedia> {
    callback_guard!();
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&RTSPMedia::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_clock_trampoline<P>(this: *mut ffi::GstRTSPMedia, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<RTSPMedia> {
    callback_guard!();
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&RTSPMedia::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_element_trampoline<P>(this: *mut ffi::GstRTSPMedia, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<RTSPMedia> {
    callback_guard!();
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&RTSPMedia::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_eos_shutdown_trampoline<P>(this: *mut ffi::GstRTSPMedia, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<RTSPMedia> {
    callback_guard!();
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&RTSPMedia::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_latency_trampoline<P>(this: *mut ffi::GstRTSPMedia, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<RTSPMedia> {
    callback_guard!();
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&RTSPMedia::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_profiles_trampoline<P>(this: *mut ffi::GstRTSPMedia, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<RTSPMedia> {
    callback_guard!();
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&RTSPMedia::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_protocols_trampoline<P>(this: *mut ffi::GstRTSPMedia, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<RTSPMedia> {
    callback_guard!();
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&RTSPMedia::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_reusable_trampoline<P>(this: *mut ffi::GstRTSPMedia, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<RTSPMedia> {
    callback_guard!();
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&RTSPMedia::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_shared_trampoline<P>(this: *mut ffi::GstRTSPMedia, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<RTSPMedia> {
    callback_guard!();
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&RTSPMedia::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_stop_on_disconnect_trampoline<P>(this: *mut ffi::GstRTSPMedia, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<RTSPMedia> {
    callback_guard!();
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&RTSPMedia::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_suspend_mode_trampoline<P>(this: *mut ffi::GstRTSPMedia, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<RTSPMedia> {
    callback_guard!();
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&RTSPMedia::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_time_provider_trampoline<P>(this: *mut ffi::GstRTSPMedia, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<RTSPMedia> {
    callback_guard!();
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&RTSPMedia::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_transport_mode_trampoline<P>(this: *mut ffi::GstRTSPMedia, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<RTSPMedia> {
    callback_guard!();
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&RTSPMedia::from_glib_borrow(this).downcast_unchecked())
}
