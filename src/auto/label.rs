// This file was generated by gir (adc662d) from gir-files (11e0e6d)
// DO NOT EDIT

use Justification;
use Menu;
use Misc;
use MovementStep;
use Widget;
use ffi;
use ffi::GtkLabel;
use ffi::GtkMenu;
use ffi::GtkMovementStep;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi::gboolean;
use glib_ffi::gpointer;
use libc::c_char;
use libc::c_int;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;

glib_wrapper! {
    pub struct Label(Object<ffi::GtkLabel>): Misc, Widget;

    match fn {
        get_type => || ffi::gtk_label_get_type(),
    }
}

impl Label {
    pub fn new(str: Option<&str>) -> Label {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_label_new(str.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_with_mnemonic(str: Option<&str>) -> Label {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_label_new_with_mnemonic(str.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn get_angle(&self) -> f64 {
        unsafe {
            ffi::gtk_label_get_angle(self.to_glib_none().0)
        }
    }

    //pub fn get_attributes(&self) -> /*Ignored*/Option<pango::AttrList> {
    //    unsafe { TODO: call ffi::gtk_label_get_attributes() }
    //}

    pub fn get_current_uri(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_label_get_current_uri(self.to_glib_none().0))
        }
    }

    //pub fn get_ellipsize(&self) -> /*Ignored*/pango::EllipsizeMode {
    //    unsafe { TODO: call ffi::gtk_label_get_ellipsize() }
    //}

    pub fn get_justify(&self) -> Justification {
        unsafe {
            from_glib(ffi::gtk_label_get_justify(self.to_glib_none().0))
        }
    }

    pub fn get_label(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_label_get_label(self.to_glib_none().0))
        }
    }

    //pub fn get_layout(&self) -> /*Ignored*/Option<pango::Layout> {
    //    unsafe { TODO: call ffi::gtk_label_get_layout() }
    //}

    pub fn get_layout_offsets(&self) -> (i32, i32) {
        unsafe {
            let mut x = mem::uninitialized();
            let mut y = mem::uninitialized();
            ffi::gtk_label_get_layout_offsets(self.to_glib_none().0, &mut x, &mut y);
            (x, y)
        }
    }

    pub fn get_line_wrap(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_label_get_line_wrap(self.to_glib_none().0))
        }
    }

    //pub fn get_line_wrap_mode(&self) -> /*Ignored*/pango::WrapMode {
    //    unsafe { TODO: call ffi::gtk_label_get_line_wrap_mode() }
    //}

    #[cfg(feature = "v3_10")]
    pub fn get_lines(&self) -> i32 {
        unsafe {
            ffi::gtk_label_get_lines(self.to_glib_none().0)
        }
    }

    pub fn get_max_width_chars(&self) -> i32 {
        unsafe {
            ffi::gtk_label_get_max_width_chars(self.to_glib_none().0)
        }
    }

    pub fn get_mnemonic_keyval(&self) -> u32 {
        unsafe {
            ffi::gtk_label_get_mnemonic_keyval(self.to_glib_none().0)
        }
    }

    pub fn get_mnemonic_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_label_get_mnemonic_widget(self.to_glib_none().0))
        }
    }

    pub fn get_selectable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_label_get_selectable(self.to_glib_none().0))
        }
    }

    pub fn get_selection_bounds(&self) -> Option<(i32, i32)> {
        unsafe {
            let mut start = mem::uninitialized();
            let mut end = mem::uninitialized();
            let ret = from_glib(ffi::gtk_label_get_selection_bounds(self.to_glib_none().0, &mut start, &mut end));
            if ret { Some((start, end)) } else { None }
        }
    }

    pub fn get_single_line_mode(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_label_get_single_line_mode(self.to_glib_none().0))
        }
    }

    pub fn get_text(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_label_get_text(self.to_glib_none().0))
        }
    }

    pub fn get_track_visited_links(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_label_get_track_visited_links(self.to_glib_none().0))
        }
    }

    pub fn get_use_markup(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_label_get_use_markup(self.to_glib_none().0))
        }
    }

    pub fn get_use_underline(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_label_get_use_underline(self.to_glib_none().0))
        }
    }

    pub fn get_width_chars(&self) -> i32 {
        unsafe {
            ffi::gtk_label_get_width_chars(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn get_xalign(&self) -> f32 {
        unsafe {
            ffi::gtk_label_get_xalign(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn get_yalign(&self) -> f32 {
        unsafe {
            ffi::gtk_label_get_yalign(self.to_glib_none().0)
        }
    }

    pub fn select_region(&self, start_offset: i32, end_offset: i32) {
        unsafe {
            ffi::gtk_label_select_region(self.to_glib_none().0, start_offset, end_offset);
        }
    }

    pub fn set_angle(&self, angle: f64) {
        unsafe {
            ffi::gtk_label_set_angle(self.to_glib_none().0, angle);
        }
    }

    //pub fn set_attributes(&self, attrs: /*Ignored*/Option<&pango::AttrList>) {
    //    unsafe { TODO: call ffi::gtk_label_set_attributes() }
    //}

    //pub fn set_ellipsize(&self, mode: /*Ignored*/pango::EllipsizeMode) {
    //    unsafe { TODO: call ffi::gtk_label_set_ellipsize() }
    //}

    pub fn set_justify(&self, jtype: Justification) {
        unsafe {
            ffi::gtk_label_set_justify(self.to_glib_none().0, jtype.to_glib());
        }
    }

    pub fn set_label(&self, str: &str) {
        unsafe {
            ffi::gtk_label_set_label(self.to_glib_none().0, str.to_glib_none().0);
        }
    }

    pub fn set_line_wrap(&self, wrap: bool) {
        unsafe {
            ffi::gtk_label_set_line_wrap(self.to_glib_none().0, wrap.to_glib());
        }
    }

    //pub fn set_line_wrap_mode(&self, wrap_mode: /*Ignored*/pango::WrapMode) {
    //    unsafe { TODO: call ffi::gtk_label_set_line_wrap_mode() }
    //}

    #[cfg(feature = "v3_10")]
    pub fn set_lines(&self, lines: i32) {
        unsafe {
            ffi::gtk_label_set_lines(self.to_glib_none().0, lines);
        }
    }

    pub fn set_markup(&self, str: &str) {
        unsafe {
            ffi::gtk_label_set_markup(self.to_glib_none().0, str.to_glib_none().0);
        }
    }

    pub fn set_markup_with_mnemonic(&self, str: &str) {
        unsafe {
            ffi::gtk_label_set_markup_with_mnemonic(self.to_glib_none().0, str.to_glib_none().0);
        }
    }

    pub fn set_max_width_chars(&self, n_chars: i32) {
        unsafe {
            ffi::gtk_label_set_max_width_chars(self.to_glib_none().0, n_chars);
        }
    }

    pub fn set_mnemonic_widget<T: IsA<Widget>>(&self, widget: Option<&T>) {
        unsafe {
            ffi::gtk_label_set_mnemonic_widget(self.to_glib_none().0, widget.to_glib_none().0);
        }
    }

    pub fn set_pattern(&self, pattern: &str) {
        unsafe {
            ffi::gtk_label_set_pattern(self.to_glib_none().0, pattern.to_glib_none().0);
        }
    }

    pub fn set_selectable(&self, setting: bool) {
        unsafe {
            ffi::gtk_label_set_selectable(self.to_glib_none().0, setting.to_glib());
        }
    }

    pub fn set_single_line_mode(&self, single_line_mode: bool) {
        unsafe {
            ffi::gtk_label_set_single_line_mode(self.to_glib_none().0, single_line_mode.to_glib());
        }
    }

    pub fn set_text(&self, str: &str) {
        unsafe {
            ffi::gtk_label_set_text(self.to_glib_none().0, str.to_glib_none().0);
        }
    }

    pub fn set_text_with_mnemonic(&self, str: &str) {
        unsafe {
            ffi::gtk_label_set_text_with_mnemonic(self.to_glib_none().0, str.to_glib_none().0);
        }
    }

    pub fn set_track_visited_links(&self, track_links: bool) {
        unsafe {
            ffi::gtk_label_set_track_visited_links(self.to_glib_none().0, track_links.to_glib());
        }
    }

    pub fn set_use_markup(&self, setting: bool) {
        unsafe {
            ffi::gtk_label_set_use_markup(self.to_glib_none().0, setting.to_glib());
        }
    }

    pub fn set_use_underline(&self, setting: bool) {
        unsafe {
            ffi::gtk_label_set_use_underline(self.to_glib_none().0, setting.to_glib());
        }
    }

    pub fn set_width_chars(&self, n_chars: i32) {
        unsafe {
            ffi::gtk_label_set_width_chars(self.to_glib_none().0, n_chars);
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn set_xalign(&self, xalign: f32) {
        unsafe {
            ffi::gtk_label_set_xalign(self.to_glib_none().0, xalign);
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn set_yalign(&self, yalign: f32) {
        unsafe {
            ffi::gtk_label_set_yalign(self.to_glib_none().0, yalign);
        }
    }

    pub fn connect_activate_current_link<F: Fn(&Label) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Label) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "activate-current-link",
                transmute(activate_current_link_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_activate_link<F: Fn(&Label, &str) -> bool + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Label, &str) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "activate-link",
                transmute(activate_link_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_copy_clipboard<F: Fn(&Label) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Label) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "copy-clipboard",
                transmute(copy_clipboard_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_move_cursor<F: Fn(&Label, MovementStep, i32, bool) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Label, MovementStep, i32, bool) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "move-cursor",
                transmute(move_cursor_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_populate_popup<F: Fn(&Label, &Menu) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Label, &Menu) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "populate-popup",
                transmute(populate_popup_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn activate_current_link_trampoline(this: *mut GtkLabel, f: gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&Label) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn activate_link_trampoline(this: *mut GtkLabel, uri: *mut c_char, f: gpointer) -> gboolean {
    callback_guard!();
    let f: &Box_<Fn(&Label, &str) -> bool + 'static> = transmute(f);
    f(&from_glib_none(this), &String::from_glib_none(uri)).to_glib()
}

unsafe extern "C" fn copy_clipboard_trampoline(this: *mut GtkLabel, f: gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&Label) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn move_cursor_trampoline(this: *mut GtkLabel, step: GtkMovementStep, count: c_int, extend_selection: gboolean, f: gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&Label, MovementStep, i32, bool) + 'static> = transmute(f);
    f(&from_glib_none(this), from_glib(step), count, from_glib(extend_selection))
}

unsafe extern "C" fn populate_popup_trampoline(this: *mut GtkLabel, menu: *mut GtkMenu, f: gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&Label, &Menu) + 'static> = transmute(f);
    f(&from_glib_none(this), &from_glib_none(menu))
}
