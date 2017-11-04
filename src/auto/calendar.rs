// This file was generated by gir (94e079d) from gir-files (469db10)
// DO NOT EDIT

use CalendarDisplayOptions;
use Widget;
use ffi;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Calendar(Object<ffi::GtkCalendar>): Widget;

    match fn {
        get_type => || ffi::gtk_calendar_get_type(),
    }
}

impl Calendar {
    pub fn new() -> Calendar {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_calendar_new()).downcast_unchecked()
        }
    }
}

impl Default for Calendar {
    fn default() -> Self {
        Self::new()
    }
}

pub trait CalendarExt {
    fn clear_marks(&self);

    fn get_date(&self) -> (u32, u32, u32);

    fn get_day_is_marked(&self, day: u32) -> bool;

    fn get_detail_height_rows(&self) -> i32;

    fn get_detail_width_chars(&self) -> i32;

    fn get_display_options(&self) -> CalendarDisplayOptions;

    fn mark_day(&self, day: u32);

    fn select_day(&self, day: u32);

    fn select_month(&self, month: u32, year: u32);

    //fn set_detail_func<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, func: /*Unknown conversion*//*Unimplemented*/CalendarDetailFunc, data: P, destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify);

    fn set_detail_height_rows(&self, rows: i32);

    fn set_detail_width_chars(&self, chars: i32);

    fn set_display_options(&self, flags: CalendarDisplayOptions);

    fn unmark_day(&self, day: u32);

    fn get_property_day(&self) -> i32;

    fn set_property_day(&self, day: i32);

    fn get_property_month(&self) -> i32;

    fn set_property_month(&self, month: i32);

    fn get_property_no_month_change(&self) -> bool;

    fn set_property_no_month_change(&self, no_month_change: bool);

    fn get_property_show_day_names(&self) -> bool;

    fn set_property_show_day_names(&self, show_day_names: bool);

    fn get_property_show_details(&self) -> bool;

    fn set_property_show_details(&self, show_details: bool);

    fn get_property_show_heading(&self) -> bool;

    fn set_property_show_heading(&self, show_heading: bool);

    fn get_property_show_week_numbers(&self) -> bool;

    fn set_property_show_week_numbers(&self, show_week_numbers: bool);

    fn get_property_year(&self) -> i32;

    fn set_property_year(&self, year: i32);

    fn connect_day_selected<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_day_selected_double_click<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_month_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_next_month<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_next_year<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_prev_month<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_prev_year<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_day_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_detail_height_rows_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_detail_width_chars_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_month_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_no_month_change_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_show_day_names_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_show_details_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_show_heading_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_show_week_numbers_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_year_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Calendar> + IsA<glib::object::Object>> CalendarExt for O {
    fn clear_marks(&self) {
        unsafe {
            ffi::gtk_calendar_clear_marks(self.to_glib_none().0);
        }
    }

    fn get_date(&self) -> (u32, u32, u32) {
        unsafe {
            let mut year = mem::uninitialized();
            let mut month = mem::uninitialized();
            let mut day = mem::uninitialized();
            ffi::gtk_calendar_get_date(self.to_glib_none().0, &mut year, &mut month, &mut day);
            (year, month, day)
        }
    }

    fn get_day_is_marked(&self, day: u32) -> bool {
        unsafe {
            from_glib(ffi::gtk_calendar_get_day_is_marked(self.to_glib_none().0, day))
        }
    }

    fn get_detail_height_rows(&self) -> i32 {
        unsafe {
            ffi::gtk_calendar_get_detail_height_rows(self.to_glib_none().0)
        }
    }

    fn get_detail_width_chars(&self) -> i32 {
        unsafe {
            ffi::gtk_calendar_get_detail_width_chars(self.to_glib_none().0)
        }
    }

    fn get_display_options(&self) -> CalendarDisplayOptions {
        unsafe {
            from_glib(ffi::gtk_calendar_get_display_options(self.to_glib_none().0))
        }
    }

    fn mark_day(&self, day: u32) {
        unsafe {
            ffi::gtk_calendar_mark_day(self.to_glib_none().0, day);
        }
    }

    fn select_day(&self, day: u32) {
        unsafe {
            ffi::gtk_calendar_select_day(self.to_glib_none().0, day);
        }
    }

    fn select_month(&self, month: u32, year: u32) {
        unsafe {
            ffi::gtk_calendar_select_month(self.to_glib_none().0, month, year);
        }
    }

    //fn set_detail_func<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, func: /*Unknown conversion*//*Unimplemented*/CalendarDetailFunc, data: P, destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gtk_calendar_set_detail_func() }
    //}

    fn set_detail_height_rows(&self, rows: i32) {
        unsafe {
            ffi::gtk_calendar_set_detail_height_rows(self.to_glib_none().0, rows);
        }
    }

    fn set_detail_width_chars(&self, chars: i32) {
        unsafe {
            ffi::gtk_calendar_set_detail_width_chars(self.to_glib_none().0, chars);
        }
    }

    fn set_display_options(&self, flags: CalendarDisplayOptions) {
        unsafe {
            ffi::gtk_calendar_set_display_options(self.to_glib_none().0, flags.to_glib());
        }
    }

    fn unmark_day(&self, day: u32) {
        unsafe {
            ffi::gtk_calendar_unmark_day(self.to_glib_none().0, day);
        }
    }

    fn get_property_day(&self) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "day".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_day(&self, day: i32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "day".to_glib_none().0, Value::from(&day).to_glib_none().0);
        }
    }

    fn get_property_month(&self) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "month".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_month(&self, month: i32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "month".to_glib_none().0, Value::from(&month).to_glib_none().0);
        }
    }

    fn get_property_no_month_change(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "no-month-change".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_no_month_change(&self, no_month_change: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "no-month-change".to_glib_none().0, Value::from(&no_month_change).to_glib_none().0);
        }
    }

    fn get_property_show_day_names(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "show-day-names".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_show_day_names(&self, show_day_names: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "show-day-names".to_glib_none().0, Value::from(&show_day_names).to_glib_none().0);
        }
    }

    fn get_property_show_details(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "show-details".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_show_details(&self, show_details: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "show-details".to_glib_none().0, Value::from(&show_details).to_glib_none().0);
        }
    }

    fn get_property_show_heading(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "show-heading".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_show_heading(&self, show_heading: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "show-heading".to_glib_none().0, Value::from(&show_heading).to_glib_none().0);
        }
    }

    fn get_property_show_week_numbers(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "show-week-numbers".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_show_week_numbers(&self, show_week_numbers: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "show-week-numbers".to_glib_none().0, Value::from(&show_week_numbers).to_glib_none().0);
        }
    }

    fn get_property_year(&self) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "year".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_year(&self, year: i32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "year".to_glib_none().0, Value::from(&year).to_glib_none().0);
        }
    }

    fn connect_day_selected<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "day-selected",
                transmute(day_selected_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_day_selected_double_click<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "day-selected-double-click",
                transmute(day_selected_double_click_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_month_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "month-changed",
                transmute(month_changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_next_month<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "next-month",
                transmute(next_month_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_next_year<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "next-year",
                transmute(next_year_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_prev_month<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "prev-month",
                transmute(prev_month_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_prev_year<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "prev-year",
                transmute(prev_year_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_day_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::day",
                transmute(notify_day_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_detail_height_rows_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::detail-height-rows",
                transmute(notify_detail_height_rows_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_detail_width_chars_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::detail-width-chars",
                transmute(notify_detail_width_chars_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_month_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::month",
                transmute(notify_month_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_no_month_change_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::no-month-change",
                transmute(notify_no_month_change_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_show_day_names_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::show-day-names",
                transmute(notify_show_day_names_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_show_details_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::show-details",
                transmute(notify_show_details_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_show_heading_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::show-heading",
                transmute(notify_show_heading_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_show_week_numbers_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::show-week-numbers",
                transmute(notify_show_week_numbers_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_year_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::year",
                transmute(notify_year_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn day_selected_trampoline<P>(this: *mut ffi::GtkCalendar, f: glib_ffi::gpointer)
where P: IsA<Calendar> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Calendar::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn day_selected_double_click_trampoline<P>(this: *mut ffi::GtkCalendar, f: glib_ffi::gpointer)
where P: IsA<Calendar> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Calendar::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn month_changed_trampoline<P>(this: *mut ffi::GtkCalendar, f: glib_ffi::gpointer)
where P: IsA<Calendar> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Calendar::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn next_month_trampoline<P>(this: *mut ffi::GtkCalendar, f: glib_ffi::gpointer)
where P: IsA<Calendar> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Calendar::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn next_year_trampoline<P>(this: *mut ffi::GtkCalendar, f: glib_ffi::gpointer)
where P: IsA<Calendar> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Calendar::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn prev_month_trampoline<P>(this: *mut ffi::GtkCalendar, f: glib_ffi::gpointer)
where P: IsA<Calendar> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Calendar::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn prev_year_trampoline<P>(this: *mut ffi::GtkCalendar, f: glib_ffi::gpointer)
where P: IsA<Calendar> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Calendar::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_day_trampoline<P>(this: *mut ffi::GtkCalendar, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Calendar> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Calendar::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_detail_height_rows_trampoline<P>(this: *mut ffi::GtkCalendar, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Calendar> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Calendar::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_detail_width_chars_trampoline<P>(this: *mut ffi::GtkCalendar, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Calendar> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Calendar::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_month_trampoline<P>(this: *mut ffi::GtkCalendar, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Calendar> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Calendar::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_no_month_change_trampoline<P>(this: *mut ffi::GtkCalendar, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Calendar> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Calendar::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_show_day_names_trampoline<P>(this: *mut ffi::GtkCalendar, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Calendar> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Calendar::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_show_details_trampoline<P>(this: *mut ffi::GtkCalendar, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Calendar> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Calendar::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_show_heading_trampoline<P>(this: *mut ffi::GtkCalendar, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Calendar> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Calendar::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_show_week_numbers_trampoline<P>(this: *mut ffi::GtkCalendar, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Calendar> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Calendar::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_year_trampoline<P>(this: *mut ffi::GtkCalendar, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Calendar> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Calendar::from_glib_borrow(this).downcast_unchecked())
}
