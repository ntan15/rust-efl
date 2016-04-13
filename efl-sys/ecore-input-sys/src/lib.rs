extern crate libc;
extern crate eina_sys;

use libc::*;
use eina_sys::*;

/* automatically generated by rust-bindgen */

pub type EcoreWindow = uintptr_t;
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum EcoreEventModifier {
    ECORE_NONE = 0,
    ECORE_SHIFT = 1,
    ECORE_CTRL = 2,
    ECORE_ALT = 3,
    ECORE_WIN = 4,
    ECORE_SCROLL = 5,
    ECORE_CAPS = 6,
    ECORE_MODE = 7,
    ECORE_LAST = 8,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum EcoreEventPress {
    ECORE_DOWN = 0,
    ECORE_UP = 1,
    ECORE_CANCEL = 2,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum EcoreEventIo { 
    ECORE_IN = 0, 
    ECORE_OUT = 1, 
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum EcoreComposeState {
    ECORE_COMPOSE_NONE = 0,
    ECORE_COMPOSE_MIDDLE = 1,
    ECORE_COMPOSE_DONE = 2,
}
#[repr(C)]
pub struct EcoreEventKey {
    pub keyname: *const c_char,
    pub key: *const c_char,
    pub string: *const c_char,
    pub compose: *const c_char,
    pub window: EcoreWindow,
    pub root_window: EcoreWindow,
    pub event_window: EcoreWindow,
    pub timestamp: c_uint,
    pub modifiers: c_uint,
    pub same_screen: c_int,
    pub keycode: c_uint,
    pub data: *mut c_void,
}
#[repr(C)]
pub struct EcoreEventMouseButton {
    pub window: EcoreWindow,
    pub root_window: EcoreWindow,
    pub event_window: EcoreWindow,
    pub timestamp: c_uint,
    pub modifiers: c_uint,
    pub buttons: c_uint,
    pub double_click: c_uint,
    pub triple_click: c_uint,
    pub same_screen: c_int,
    pub x: c_int,
    pub y: c_int,
    pub root: EcoreInputStructUnnamed1,
    pub multi: EcoreInputStructUnnamed2,
}
#[repr(C)]
pub struct EcoreInputStructUnnamed1 {
    pub x: c_int,
    pub y: c_int,
}

#[repr(C)]
pub struct EcoreInputStructUnnamed2 {
    pub device: c_int,
    pub radius: c_double,
    pub radius_x: c_double,
    pub radius_y: c_double,
    pub pressure: c_double,
    pub angle: c_double,
    pub x: c_double,
    pub y: c_double,
    pub root: EcoreInputStructUnnamed3,
}
#[repr(C)]
pub struct EcoreInputStructUnnamed3 {
    pub x: c_double,
    pub y: c_double,
}
#[repr(C)]
pub struct EcoreEventMouseWheel {
    pub window: EcoreWindow,
    pub root_window: EcoreWindow,
    pub event_window: EcoreWindow,
    pub timestamp: c_uint,
    pub modifiers: c_uint,
    pub same_screen: c_int,
    pub direction: c_int,
    pub z: c_int,
    pub x: c_int,
    pub y: c_int,
    pub root: EcoreInputStructUnnamed4,
}
#[repr(C)]
pub struct EcoreInputStructUnnamed4 {
    pub x: c_int,
    pub y: c_int,
}
#[repr(C)]
pub struct EcoreEventMouseMove {
    pub window: EcoreWindow,
    pub root_window: EcoreWindow,
    pub event_window: EcoreWindow,
    pub timestamp: c_uint,
    pub modifiers: c_uint,
    pub same_screen: c_int,
    pub x: c_int,
    pub y: c_int,
    pub root: EcoreInputStructUnnamed5,
    pub multi: EcoreInputStructUnnamed6,
}
#[repr(C)]
pub struct EcoreInputStructUnnamed5 {
    pub x: c_int,
    pub y: c_int,
}
#[repr(C)]
pub struct EcoreInputStructUnnamed6 {
    pub device: c_int,
    pub radius: c_double,
    pub radius_x: c_double,
    pub radius_y: c_double,
    pub pressure: c_double,
    pub angle: c_double,
    pub x: c_double,
    pub y: c_double,
    pub root: EcoreInputStructUnnamed7,
}
#[repr(C)]
pub struct EcoreInputStructUnnamed7 {
    pub x: c_double,
    pub y: c_double,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum EcoreAxisLabel {
    ECORE_AXIS_LABEL_UNKNOWN = 0,
    ECORE_AXIS_LABEL_X = 1,
    ECORE_AXIS_LABEL_Y = 2,
    ECORE_AXIS_LABEL_PRESSURE = 3,
    ECORE_AXIS_LABEL_DISTANCE = 4,
    ECORE_AXIS_LABEL_AZIMUTH = 5,
    ECORE_AXIS_LABEL_TILT = 6,
    ECORE_AXIS_LABEL_TWIST = 7,
    ECORE_AXIS_LABEL_TOUCH_WIDTH_MAJOR = 8,
    ECORE_AXIS_LABEL_TOUCH_WIDTH_MINOR = 9,
    ECORE_AXIS_LABEL_TOOL_WIDTH_MAJOR = 10,
    ECORE_AXIS_LABEL_TOOL_WIDTH_MINOR = 11,
}
#[repr(C)]
pub struct EcoreAxis {
    pub label: EcoreAxisLabel,
    pub value: c_double,
}
#[repr(C)]
pub struct EcoreEventAxisUpdate {
    pub window: EcoreWindow,
    pub root_window: EcoreWindow,
    pub event_window: EcoreWindow,
    pub timestamp: c_uint,
    pub device: c_int,
    pub toolid: c_int,
    pub naxis: c_int,
    pub axis: *mut EcoreAxis,
}
#[repr(C)]
pub struct EcoreEventMouseIo {
    pub window: EcoreWindow,
    pub event_window: EcoreWindow,
    pub timestamp: c_uint,
    pub modifiers: c_uint,
    pub x: c_int,
    pub y: c_int,
}
#[repr(C)]
pub struct EcoreEventModifiers {
    pub size: c_uint,
    pub array: [c_uint; 8usize],
}

#[link(name = "ecore_input")]
extern "C" {
    pub static mut ECORE_EVENT_KEY_DOWN: c_int;
    pub static mut ECORE_EVENT_KEY_UP: c_int;
    pub static mut ECORE_EVENT_MOUSE_BUTTON_DOWN: c_int;
    pub static mut ECORE_EVENT_MOUSE_BUTTON_UP: c_int;
    pub static mut ECORE_EVENT_MOUSE_MOVE: c_int;
    pub static mut ECORE_EVENT_MOUSE_WHEEL: c_int;
    pub static mut ECORE_EVENT_MOUSE_IN: c_int;
    pub static mut ECORE_EVENT_MOUSE_OUT: c_int;
    pub static mut ECORE_EVENT_AXIS_UPDATE: c_int;
    pub static mut ECORE_EVENT_MOUSE_BUTTON_CANCEL: c_int;
}
#[link(name = "ecore_input")]
extern "C" {
    pub fn ecore_event_init() -> c_int;
    pub fn ecore_event_shutdown() -> c_int;
    pub fn ecore_event_modifier_mask(modifier: EcoreEventModifier)
     -> c_uint;
    pub fn ecore_event_update_modifier(key: *const c_char,
                                       modifiers: *mut EcoreEventModifiers,
                                       inc: c_int)
     -> EcoreEventModifier;
    pub fn ecore_compose_get(seq: *const EinaList, seqstr_ret: *mut *mut c_char)
     -> EcoreComposeState;
}
