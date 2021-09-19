// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::Asset;
#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
use crate::Clip;
use crate::Extractable;
#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
use crate::FrameNumber;
use crate::Group;
use crate::Layer;
use crate::TimelineElement;
use crate::Track;
use crate::TrackElement;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::mem::transmute;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "GESTimeline")]
    pub struct Timeline(Object<ffi::GESTimeline, ffi::GESTimelineClass>) @extends gst::Bin, gst::Element, gst::Object, @implements Extractable;

    match fn {
        type_ => || ffi::ges_timeline_get_type(),
    }
}

impl Timeline {
    #[doc(alias = "ges_timeline_new")]
    pub fn new() -> Timeline {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::ges_timeline_new()) }
    }

    #[doc(alias = "ges_timeline_new_audio_video")]
    pub fn new_audio_video() -> Timeline {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::ges_timeline_new_audio_video()) }
    }

    #[doc(alias = "ges_timeline_new_from_uri")]
    #[doc(alias = "new_from_uri")]
    pub fn from_uri(uri: &str) -> Result<Option<Timeline>, glib::Error> {
        assert_initialized_main_thread!();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::ges_timeline_new_from_uri(uri.to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_none(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

impl Default for Timeline {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_TIMELINE: Option<&Timeline> = None;

pub trait TimelineExt: 'static {
    #[cfg_attr(feature = "v1_18", deprecated = "Since 1.18")]
    #[doc(alias = "ges_timeline_add_layer")]
    fn add_layer<P: IsA<Layer>>(&self, layer: &P) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "ges_timeline_add_track")]
    fn add_track<P: IsA<Track>>(&self, track: &P) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "ges_timeline_append_layer")]
    fn append_layer(&self) -> Layer;

    #[doc(alias = "ges_timeline_commit")]
    fn commit(&self) -> bool;

    #[doc(alias = "ges_timeline_commit_sync")]
    fn commit_sync(&self) -> bool;

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "ges_timeline_freeze_commit")]
    fn freeze_commit(&self);

    #[doc(alias = "ges_timeline_get_auto_transition")]
    #[doc(alias = "get_auto_transition")]
    fn is_auto_transition(&self) -> bool;

    #[doc(alias = "ges_timeline_get_duration")]
    #[doc(alias = "get_duration")]
    fn duration(&self) -> gst::ClockTime;

    #[doc(alias = "ges_timeline_get_element")]
    #[doc(alias = "get_element")]
    fn element(&self, name: &str) -> Option<TimelineElement>;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_timeline_get_frame_at")]
    #[doc(alias = "get_frame_at")]
    fn frame_at(&self, timestamp: gst::ClockTime) -> FrameNumber;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_timeline_get_frame_time")]
    #[doc(alias = "get_frame_time")]
    fn frame_time(&self, frame_number: FrameNumber) -> Option<gst::ClockTime>;

    #[doc(alias = "ges_timeline_get_groups")]
    #[doc(alias = "get_groups")]
    fn groups(&self) -> Vec<Group>;

    #[doc(alias = "ges_timeline_get_layer")]
    #[doc(alias = "get_layer")]
    fn layer(&self, priority: u32) -> Option<Layer>;

    #[doc(alias = "ges_timeline_get_layers")]
    #[doc(alias = "get_layers")]
    fn layers(&self) -> Vec<Layer>;

    #[doc(alias = "ges_timeline_get_pad_for_track")]
    #[doc(alias = "get_pad_for_track")]
    fn pad_for_track<P: IsA<Track>>(&self, track: &P) -> Option<gst::Pad>;

    #[doc(alias = "ges_timeline_get_snapping_distance")]
    #[doc(alias = "get_snapping_distance")]
    fn snapping_distance(&self) -> Option<gst::ClockTime>;

    #[doc(alias = "ges_timeline_get_track_for_pad")]
    #[doc(alias = "get_track_for_pad")]
    fn track_for_pad<P: IsA<gst::Pad>>(&self, pad: &P) -> Option<Track>;

    #[doc(alias = "ges_timeline_get_tracks")]
    #[doc(alias = "get_tracks")]
    fn tracks(&self) -> Vec<Track>;

    #[doc(alias = "ges_timeline_is_empty")]
    fn is_empty(&self) -> bool;

    #[doc(alias = "ges_timeline_load_from_uri")]
    fn load_from_uri(&self, uri: &str) -> Result<(), glib::Error>;

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    #[doc(alias = "ges_timeline_move_layer")]
    fn move_layer<P: IsA<Layer>>(
        &self,
        layer: &P,
        new_layer_priority: u32,
    ) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "ges_timeline_paste_element")]
    fn paste_element<P: IsA<TimelineElement>>(
        &self,
        element: &P,
        position: gst::ClockTime,
        layer_priority: i32,
    ) -> Option<TimelineElement>;

    #[doc(alias = "ges_timeline_remove_layer")]
    fn remove_layer<P: IsA<Layer>>(&self, layer: &P) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "ges_timeline_remove_track")]
    fn remove_track<P: IsA<Track>>(&self, track: &P) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "ges_timeline_save_to_uri")]
    fn save_to_uri<P: IsA<Asset>>(
        &self,
        uri: &str,
        formatter_asset: Option<&P>,
        overwrite: bool,
    ) -> Result<(), glib::Error>;

    #[doc(alias = "ges_timeline_set_auto_transition")]
    fn set_auto_transition(&self, auto_transition: bool);

    #[doc(alias = "ges_timeline_set_snapping_distance")]
    fn set_snapping_distance(&self, snapping_distance: gst::ClockTime);

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "ges_timeline_thaw_commit")]
    fn thaw_commit(&self);

    #[doc(alias = "commited")]
    fn connect_commited<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "group-added")]
    fn connect_group_added<F: Fn(&Self, &Group) + 'static>(&self, f: F) -> SignalHandlerId;

    //#[doc(alias = "group-removed")]
    //fn connect_group_removed<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "layer-added")]
    fn connect_layer_added<F: Fn(&Self, &Layer) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "layer-removed")]
    fn connect_layer_removed<F: Fn(&Self, &Layer) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "select-element-track")]
    fn connect_select_element_track<F: Fn(&Self, &Clip, &TrackElement) -> Track + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    //#[doc(alias = "select-tracks-for-object")]
    //fn connect_select_tracks_for_object<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "snapping-ended")]
    fn connect_snapping_ended<F: Fn(&Self, &TrackElement, &TrackElement, u64) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "snapping-started")]
    fn connect_snapping_started<F: Fn(&Self, &TrackElement, &TrackElement, u64) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "track-added")]
    fn connect_track_added<F: Fn(&Self, &Track) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "track-removed")]
    fn connect_track_removed<F: Fn(&Self, &Track) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "auto-transition")]
    fn connect_auto_transition_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "duration")]
    fn connect_duration_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "snapping-distance")]
    fn connect_snapping_distance_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Timeline>> TimelineExt for O {
    fn add_layer<P: IsA<Layer>>(&self, layer: &P) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::ges_timeline_add_layer(
                    self.as_ref().to_glib_none().0,
                    layer.as_ref().to_glib_none().0
                ),
                "Failed to add layer"
            )
        }
    }

    fn add_track<P: IsA<Track>>(&self, track: &P) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::ges_timeline_add_track(
                    self.as_ref().to_glib_none().0,
                    track.as_ref().to_glib_full()
                ),
                "Failed to add track"
            )
        }
    }

    fn append_layer(&self) -> Layer {
        unsafe {
            from_glib_none(ffi::ges_timeline_append_layer(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn commit(&self) -> bool {
        unsafe { from_glib(ffi::ges_timeline_commit(self.as_ref().to_glib_none().0)) }
    }

    fn commit_sync(&self) -> bool {
        unsafe {
            from_glib(ffi::ges_timeline_commit_sync(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    fn freeze_commit(&self) {
        unsafe {
            ffi::ges_timeline_freeze_commit(self.as_ref().to_glib_none().0);
        }
    }

    fn is_auto_transition(&self) -> bool {
        unsafe {
            from_glib(ffi::ges_timeline_get_auto_transition(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn duration(&self) -> gst::ClockTime {
        unsafe {
            try_from_glib(ffi::ges_timeline_get_duration(
                self.as_ref().to_glib_none().0,
            ))
            .expect("mandatory glib value is None")
        }
    }

    fn element(&self, name: &str) -> Option<TimelineElement> {
        unsafe {
            from_glib_full(ffi::ges_timeline_get_element(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn frame_at(&self, timestamp: gst::ClockTime) -> FrameNumber {
        unsafe {
            ffi::ges_timeline_get_frame_at(self.as_ref().to_glib_none().0, timestamp.into_glib())
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn frame_time(&self, frame_number: FrameNumber) -> Option<gst::ClockTime> {
        unsafe {
            from_glib(ffi::ges_timeline_get_frame_time(
                self.as_ref().to_glib_none().0,
                frame_number,
            ))
        }
    }

    fn groups(&self) -> Vec<Group> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::ges_timeline_get_groups(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn layer(&self, priority: u32) -> Option<Layer> {
        unsafe {
            from_glib_full(ffi::ges_timeline_get_layer(
                self.as_ref().to_glib_none().0,
                priority,
            ))
        }
    }

    fn layers(&self) -> Vec<Layer> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::ges_timeline_get_layers(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn pad_for_track<P: IsA<Track>>(&self, track: &P) -> Option<gst::Pad> {
        unsafe {
            from_glib_none(ffi::ges_timeline_get_pad_for_track(
                self.as_ref().to_glib_none().0,
                track.as_ref().to_glib_none().0,
            ))
        }
    }

    fn snapping_distance(&self) -> Option<gst::ClockTime> {
        unsafe {
            from_glib(ffi::ges_timeline_get_snapping_distance(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn track_for_pad<P: IsA<gst::Pad>>(&self, pad: &P) -> Option<Track> {
        unsafe {
            from_glib_none(ffi::ges_timeline_get_track_for_pad(
                self.as_ref().to_glib_none().0,
                pad.as_ref().to_glib_none().0,
            ))
        }
    }

    fn tracks(&self) -> Vec<Track> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::ges_timeline_get_tracks(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_empty(&self) -> bool {
        unsafe { from_glib(ffi::ges_timeline_is_empty(self.as_ref().to_glib_none().0)) }
    }

    fn load_from_uri(&self, uri: &str) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::ges_timeline_load_from_uri(
                self.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    fn move_layer<P: IsA<Layer>>(
        &self,
        layer: &P,
        new_layer_priority: u32,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::ges_timeline_move_layer(
                    self.as_ref().to_glib_none().0,
                    layer.as_ref().to_glib_none().0,
                    new_layer_priority
                ),
                "Failed to move layer"
            )
        }
    }

    fn paste_element<P: IsA<TimelineElement>>(
        &self,
        element: &P,
        position: gst::ClockTime,
        layer_priority: i32,
    ) -> Option<TimelineElement> {
        unsafe {
            from_glib_full(ffi::ges_timeline_paste_element(
                self.as_ref().to_glib_none().0,
                element.as_ref().to_glib_none().0,
                position.into_glib(),
                layer_priority,
            ))
        }
    }

    fn remove_layer<P: IsA<Layer>>(&self, layer: &P) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::ges_timeline_remove_layer(
                    self.as_ref().to_glib_none().0,
                    layer.as_ref().to_glib_none().0
                ),
                "Failed to remove layer"
            )
        }
    }

    fn remove_track<P: IsA<Track>>(&self, track: &P) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::ges_timeline_remove_track(
                    self.as_ref().to_glib_none().0,
                    track.as_ref().to_glib_none().0
                ),
                "Failed to remove track"
            )
        }
    }

    fn save_to_uri<P: IsA<Asset>>(
        &self,
        uri: &str,
        formatter_asset: Option<&P>,
        overwrite: bool,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::ges_timeline_save_to_uri(
                self.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
                formatter_asset.map(|p| p.as_ref()).to_glib_none().0,
                overwrite.into_glib(),
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_auto_transition(&self, auto_transition: bool) {
        unsafe {
            ffi::ges_timeline_set_auto_transition(
                self.as_ref().to_glib_none().0,
                auto_transition.into_glib(),
            );
        }
    }

    fn set_snapping_distance(&self, snapping_distance: gst::ClockTime) {
        unsafe {
            ffi::ges_timeline_set_snapping_distance(
                self.as_ref().to_glib_none().0,
                snapping_distance.into_glib(),
            );
        }
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    fn thaw_commit(&self) {
        unsafe {
            ffi::ges_timeline_thaw_commit(self.as_ref().to_glib_none().0);
        }
    }

    fn connect_commited<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn commited_trampoline<P: IsA<Timeline>, F: Fn(&P) + 'static>(
            this: *mut ffi::GESTimeline,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Timeline::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"commited\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    commited_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_group_added<F: Fn(&Self, &Group) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn group_added_trampoline<
            P: IsA<Timeline>,
            F: Fn(&P, &Group) + 'static,
        >(
            this: *mut ffi::GESTimeline,
            group: *mut ffi::GESGroup,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Timeline::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(group),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"group-added\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    group_added_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    //fn connect_group_removed<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Empty ctype children: *.PtrArray TypeId { ns_id: 1, id: 54 }
    //}

    fn connect_layer_added<F: Fn(&Self, &Layer) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn layer_added_trampoline<
            P: IsA<Timeline>,
            F: Fn(&P, &Layer) + 'static,
        >(
            this: *mut ffi::GESTimeline,
            layer: *mut ffi::GESLayer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Timeline::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(layer),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"layer-added\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    layer_added_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_layer_removed<F: Fn(&Self, &Layer) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn layer_removed_trampoline<
            P: IsA<Timeline>,
            F: Fn(&P, &Layer) + 'static,
        >(
            this: *mut ffi::GESTimeline,
            layer: *mut ffi::GESLayer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Timeline::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(layer),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"layer-removed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    layer_removed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn connect_select_element_track<F: Fn(&Self, &Clip, &TrackElement) -> Track + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn select_element_track_trampoline<
            P: IsA<Timeline>,
            F: Fn(&P, &Clip, &TrackElement) -> Track + 'static,
        >(
            this: *mut ffi::GESTimeline,
            clip: *mut ffi::GESClip,
            track_element: *mut ffi::GESTrackElement,
            f: glib::ffi::gpointer,
        ) -> *mut ffi::GESTrack {
            let f: &F = &*(f as *const F);
            f(
                Timeline::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(clip),
                &from_glib_borrow(track_element),
            )
            .to_glib_full()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"select-element-track\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    select_element_track_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    //fn connect_select_tracks_for_object<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Empty ctype return value *.PtrArray TypeId { ns_id: 1, id: 17 }
    //}

    fn connect_snapping_ended<F: Fn(&Self, &TrackElement, &TrackElement, u64) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn snapping_ended_trampoline<
            P: IsA<Timeline>,
            F: Fn(&P, &TrackElement, &TrackElement, u64) + 'static,
        >(
            this: *mut ffi::GESTimeline,
            obj1: *mut ffi::GESTrackElement,
            obj2: *mut ffi::GESTrackElement,
            position: u64,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Timeline::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(obj1),
                &from_glib_borrow(obj2),
                position,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"snapping-ended\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    snapping_ended_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_snapping_started<F: Fn(&Self, &TrackElement, &TrackElement, u64) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn snapping_started_trampoline<
            P: IsA<Timeline>,
            F: Fn(&P, &TrackElement, &TrackElement, u64) + 'static,
        >(
            this: *mut ffi::GESTimeline,
            obj1: *mut ffi::GESTrackElement,
            obj2: *mut ffi::GESTrackElement,
            position: u64,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Timeline::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(obj1),
                &from_glib_borrow(obj2),
                position,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"snapping-started\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    snapping_started_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_track_added<F: Fn(&Self, &Track) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn track_added_trampoline<
            P: IsA<Timeline>,
            F: Fn(&P, &Track) + 'static,
        >(
            this: *mut ffi::GESTimeline,
            track: *mut ffi::GESTrack,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Timeline::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(track),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"track-added\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    track_added_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_track_removed<F: Fn(&Self, &Track) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn track_removed_trampoline<
            P: IsA<Timeline>,
            F: Fn(&P, &Track) + 'static,
        >(
            this: *mut ffi::GESTimeline,
            track: *mut ffi::GESTrack,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Timeline::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(track),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"track-removed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    track_removed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_auto_transition_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_auto_transition_trampoline<
            P: IsA<Timeline>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GESTimeline,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Timeline::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::auto-transition\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_auto_transition_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_duration_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_duration_trampoline<P: IsA<Timeline>, F: Fn(&P) + 'static>(
            this: *mut ffi::GESTimeline,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Timeline::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::duration\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_duration_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_snapping_distance_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_snapping_distance_trampoline<
            P: IsA<Timeline>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GESTimeline,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Timeline::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::snapping-distance\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_snapping_distance_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
