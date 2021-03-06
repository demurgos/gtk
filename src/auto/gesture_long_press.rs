// This file was generated by gir (d50d839) from gir-files (469db10)
// DO NOT EDIT

use EventController;
use Gesture;
use GestureSingle;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use Widget;
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
#[cfg(any(feature = "v3_14", feature = "dox"))]
use libc;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct GestureLongPress(Object<ffi::GtkGestureLongPress, ffi::GtkGestureLongPressClass>): GestureSingle, Gesture, EventController;

    match fn {
        get_type => || ffi::gtk_gesture_long_press_get_type(),
    }
}

impl GestureLongPress {
    #[cfg(any(feature = "v3_14", feature = "dox"))]
    pub fn new<P: IsA<Widget>>(widget: &P) -> GestureLongPress {
        skip_assert_initialized!();
        unsafe {
            Gesture::from_glib_full(ffi::gtk_gesture_long_press_new(widget.to_glib_none().0)).downcast_unchecked()
        }
    }
}

pub trait GestureLongPressExt {
    fn get_property_delay_factor(&self) -> f64;

    fn set_property_delay_factor(&self, delay_factor: f64);

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_cancelled<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_pressed<F: Fn(&Self, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_delay_factor_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<GestureLongPress> + IsA<glib::object::Object>> GestureLongPressExt for O {
    fn get_property_delay_factor(&self) -> f64 {
        unsafe {
            let mut value = Value::uninitialized();
            gobject_ffi::g_value_init(value.to_glib_none_mut().0, <f64 as StaticType>::static_type().to_glib());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "delay-factor".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_delay_factor(&self, delay_factor: f64) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "delay-factor".to_glib_none().0, Value::from(&delay_factor).to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_cancelled<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "cancelled",
                transmute(cancelled_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_pressed<F: Fn(&Self, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, f64, f64) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "pressed",
                transmute(pressed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_delay_factor_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::delay-factor",
                transmute(notify_delay_factor_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
unsafe extern "C" fn cancelled_trampoline<P>(this: *mut ffi::GtkGestureLongPress, f: glib_ffi::gpointer)
where P: IsA<GestureLongPress> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&GestureLongPress::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
unsafe extern "C" fn pressed_trampoline<P>(this: *mut ffi::GtkGestureLongPress, x: libc::c_double, y: libc::c_double, f: glib_ffi::gpointer)
where P: IsA<GestureLongPress> {
    callback_guard!();
    let f: &&(Fn(&P, f64, f64) + 'static) = transmute(f);
    f(&GestureLongPress::from_glib_borrow(this).downcast_unchecked(), x, y)
}

unsafe extern "C" fn notify_delay_factor_trampoline<P>(this: *mut ffi::GtkGestureLongPress, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<GestureLongPress> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&GestureLongPress::from_glib_borrow(this).downcast_unchecked())
}
