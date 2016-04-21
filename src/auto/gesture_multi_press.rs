// This file was generated by gir (adc662d) from gir-files (11e0e6d)
// DO NOT EDIT

use EventController;
use Gesture;
use GestureSingle;
#[cfg(feature = "v3_14")]
use Rectangle;
#[cfg(feature = "v3_14")]
use Widget;
use ffi;
#[cfg(feature = "v3_14")]
use ffi::GtkGestureMultiPress;
use glib::object::Downcast;
use glib::object::IsA;
#[cfg(feature = "v3_14")]
use glib::signal::connect;
use glib::translate::*;
#[cfg(feature = "v3_14")]
use glib_ffi::gpointer;
#[cfg(feature = "v3_14")]
use libc::c_double;
#[cfg(feature = "v3_14")]
use libc::c_int;
#[cfg(feature = "v3_14")]
use std::boxed::Box as Box_;
#[cfg(feature = "v3_14")]
use std::mem::transmute;

glib_wrapper! {
    pub struct GestureMultiPress(Object<ffi::GtkGestureMultiPress>): GestureSingle, Gesture, EventController;

    match fn {
        get_type => || ffi::gtk_gesture_multi_press_get_type(),
    }
}

impl GestureMultiPress {
    #[cfg(feature = "v3_14")]
    pub fn new<T: IsA<Widget>>(widget: &T) -> GestureMultiPress {
        skip_assert_initialized!();
        unsafe {
            Gesture::from_glib_full(ffi::gtk_gesture_multi_press_new(widget.to_glib_none().0)).downcast_unchecked()
        }
    }

    #[cfg(feature = "v3_14")]
    pub fn get_area(&self) -> Option<Rectangle> {
        unsafe {
            let mut rect = Rectangle::uninitialized();
            let ret = from_glib(ffi::gtk_gesture_multi_press_get_area(self.to_glib_none().0, rect.to_glib_none_mut().0));
            if ret { Some(rect) } else { None }
        }
    }

    #[cfg(feature = "v3_14")]
    pub fn set_area(&self, rect: Option<&Rectangle>) {
        unsafe {
            ffi::gtk_gesture_multi_press_set_area(self.to_glib_none().0, rect.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_14")]
    pub fn connect_pressed<F: Fn(&GestureMultiPress, i32, f64, f64) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&GestureMultiPress, i32, f64, f64) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "pressed",
                transmute(pressed_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(feature = "v3_14")]
    pub fn connect_released<F: Fn(&GestureMultiPress, i32, f64, f64) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&GestureMultiPress, i32, f64, f64) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "released",
                transmute(released_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(feature = "v3_14")]
    pub fn connect_stopped<F: Fn(&GestureMultiPress) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&GestureMultiPress) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "stopped",
                transmute(stopped_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(feature = "v3_14")]
unsafe extern "C" fn pressed_trampoline(this: *mut GtkGestureMultiPress, n_press: c_int, x: c_double, y: c_double, f: gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&GestureMultiPress, i32, f64, f64) + 'static> = transmute(f);
    f(&from_glib_none(this), n_press, x, y)
}

#[cfg(feature = "v3_14")]
unsafe extern "C" fn released_trampoline(this: *mut GtkGestureMultiPress, n_press: c_int, x: c_double, y: c_double, f: gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&GestureMultiPress, i32, f64, f64) + 'static> = transmute(f);
    f(&from_glib_none(this), n_press, x, y)
}

#[cfg(feature = "v3_14")]
unsafe extern "C" fn stopped_trampoline(this: *mut GtkGestureMultiPress, f: gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&GestureMultiPress) + 'static> = transmute(f);
    f(&from_glib_none(this))
}
