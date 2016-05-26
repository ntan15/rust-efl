#![allow(non_camel_case_types)]
extern crate libc;
extern crate eina_sys;
extern crate eo_sys;

use libc::*;
use eina_sys::*;
use eo_sys::*;

/* automatically generated by rust-bindgen */

#[repr(C)]
pub struct Ecore_Version {
    pub major: c_int,
    pub minor: c_int,
    pub micro: c_int,
    pub revision: c_int,
}

pub type EcoreTaskCb = Option<unsafe extern "C" fn(data: *mut c_void) -> EinaBool>;
pub type EcoreSelectFunction = Option<unsafe extern "C" fn(nfds: c_int, 
                                                             readfds: *mut fd_set,
                                                             writefds: *mut fd_set,
                                                             exceptfds: *mut fd_set,
                                                             timeout: *mut timeval)
                                                             -> c_int>;
pub type EcoreCb = Option<unsafe extern "C" fn(data: *mut c_void)>;
pub type EcoreDataCb = Option<unsafe extern "C" fn(data: *mut c_void) -> *mut c_void>;

pub enum Ecore_Win32_Handler { }
pub enum Ecore_Event_Handler { }
pub enum Ecore_Event_Filter { }
pub enum Ecore_Event { }
pub type EcoreFilterCb = Option<unsafe extern "C" fn(data: *mut c_void,
                                                       loop_data: *mut c_void,
                                                       _type: c_int, event: *mut c_void)
                                                       -> EinaBool>;
pub type EcoreEndCb = Option<unsafe extern "C" fn(user_data: *mut c_void,
                                                    func_data: *mut c_void)>;
pub type EcoreEventHandlerCb = Option<unsafe extern "C" fn(data: *mut c_void,
                                                            _type: c_int,
                                                            event: *mut c_void) -> EinaBool>;
#[repr(C)]
pub struct Ecore_Event_Signal_User {
    pub number: c_int,
    pub ext_data: *mut c_void,
    pub data: siginfo_t,
}

#[repr(C)]
pub struct Ecore_Event_Signal_Hup {
    pub ext_data: *mut c_void,
    pub data: siginfo_t,
}

#[repr(C)]
pub struct Ecore_Event_Signal_Exit {
    pub _bindgen_bitfield_1_: EinaBool,
    pub _bindgen_bitfield_2_: EinaBool,
    pub _bindgen_bitfield_3_: EinaBool,
    pub ext_data: *mut c_void,
    pub data: siginfo_t,
}

#[repr(C)]
pub struct Ecore_Event_Signal_Power {
    pub ext_data: *mut c_void,
    pub data: siginfo_t,
}

#[repr(C)]
pub struct Ecore_Event_Signal_Realtime {
    pub num: c_int,
    pub data: siginfo_t,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum EcoreMemoryState {
    EcoreMemoryStateNormal = 0,
    EcoreMemoryStateLow = 1,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum EcorePowerState {
    EcorePowerStateMains = 0,
    EcorePowerStateBattery = 1,
    EcorePowerStateLow = 2,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum EcoreExeFlags {
    EcoreExeNone = 0,
    EcoreExePipeRead = 1,
    EcoreExePipeWrite = 2,
    EcoreExePipeError = 4,
    EcoreExePipeReadLineBuffered = 8,
    EcoreExePipeErrorLineBuffered = 16,
    EcoreExePipeAuto = 32,
    EcoreExeRespawn = 64,
    EcoreExeUseSh = 128,
    EcoreExeNotLeader = 256,
    EcoreExeTermWithParent = 512,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum EcoreExeWin32Priority {
    EcoreExeWin32PriorityIdle = 0,
    EcoreExeWin32PriorityBelowNormal = 1,
    EcoreExeWin32PriorityNormal = 2,
    EcoreExeWin32PriorityAboveNormal = 3,
    EcoreExeWin32PriorityHigh = 4,
    EcoreExeWin32PriorityRealtime = 5,
}

pub type Ecore_Exe = Eo;
pub type EcoreExeCb = Option<unsafe extern "C" fn(data: *mut c_void, exe: *const Ecore_Exe)>;

#[repr(C)]
pub struct Ecore_Exe_Event_Add {
    pub exe: *mut Ecore_Exe,
    pub ext_data: *mut c_void,
}

#[repr(C)]
pub struct Ecore_Exe_Event_Del {
    pub pid: pid_t,
    pub exit_code: c_int,
    pub exe: *mut Ecore_Exe,
    pub exit_signal: c_int,
    pub _bindgen_bitfield_1_: EinaBool,
    pub _bindgen_bitfield_2_: EinaBool,
    pub ext_data: *mut c_void,
    pub data: siginfo_t,
}


#[repr(C)]
pub struct Ecore_Exe_Event_Data_Line {
    pub line: *mut c_char,
    pub size: c_int,
}

#[repr(C)]
pub struct Ecore_Exe_Event_Data {
    pub exe: *mut Ecore_Exe,
    pub data: *mut c_void,
    pub size: c_int,
    pub lines: *mut Ecore_Exe_Event_Data_Line,
}

pub enum Ecore_Fd_Handler { }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum EcoreFdHandlerFlags {
    EcoreFdRead = 1,
    EcoreFdWrite = 2,
    EcoreFdError = 4,
}

pub type EcoreFdCb = Option<unsafe extern "C" fn(data: *mut c_void,
                                                   fd_handler: *mut Ecore_Fd_Handler)
                                                   -> EinaBool>;
pub type EcoreFdPrepCb = Option<unsafe extern "C" fn(data: *mut c_void,
                                                        fd_handler: *mut Ecore_Fd_Handler)>;
pub type EcoreWin32HandleCb = Option<unsafe extern "C" fn(data: *mut c_void,
                                                             wh: *mut Ecore_Win32_Handler)
                                                             -> EinaBool>;
pub enum Ecore_Thread { }

pub type EcoreThreadCb = Option<unsafe extern "C" fn(data: *mut c_void,
                                                     thread: *mut Ecore_Thread)>;
pub type EcoreThreadNotifyCb = Option<unsafe extern "C" fn(data: *mut c_void,
                                                           thread: *mut Ecore_Thread,
                                                           msg_data: *mut c_void)>;
pub enum Ecore_Pipe { }

pub type EcorePipeCb = Option<unsafe extern "C" fn(data: *mut c_void,
                                                   buffer: *mut c_void,
                                                   nbyte: c_uint)>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum EcorePollerType { 
    EcorePollerCore = 0, 
    Dummy,
}

pub type Ecore_Poller = Eo;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum EcorePosMap {
    EcorePosMapLinear = 0,
    EcorePosMapAccelerate = 1,
    EcorePosMapDecelerate = 2,
    EcorePosMapSinusoidal = 3,
    EcorePosMapAccelerateFactor = 4,
    EcorePosMapDecelerateFactor = 5,
    EcorePosMapSinusoidalFactor = 6,
    EcorePosMapDivisorInterp = 7,
    EcorePosMapBounce = 8,
    EcorePosMapSpring = 9,
    EcorePosMapCubicBezier = 10,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum EcoreAnimatorSource {
    EcoreAnimatorSourceTimer = 0,
    EcoreAnimatorSourceCustom = 1,
}

pub type EcoreTimelineCb = Option<unsafe extern "C" fn(data: *mut c_void, pos: c_double)
                                                       -> EinaBool>;
pub type Ecore_Animator = Eo;
pub type Ecore_Timer = Eo;
pub type Ecore_Idler = Eo;
pub type Ecore_Idle_Enterer = Eo;
pub type Ecore_Idle_Exiter = Eo;
pub type Ecore_Job = Eo;
pub type Ecore_Mainloop = Eo;

#[link(name = "ecore")]
extern "C" {
    pub static mut ecore_version: *mut Ecore_Version;
    pub static mut ECORE_EXE_EVENT_ADD: c_int;
    pub static mut ECORE_EXE_EVENT_DEL: c_int;
    pub static mut ECORE_EXE_EVENT_DATA: c_int;
    pub static mut ECORE_EXE_EVENT_ERROR: c_int;
// Eo
    pub static _ECORE_EXE_EVENT_DATA_GET: Eo_Event_Description;
    pub static _ECORE_EXE_EVENT_DATA_ERROR: Eo_Event_Description;
    pub static _ECORE_MAINLOOP_EVENT_IDLE_ENTER: Eo_Event_Description;
    pub static _ECORE_MAINLOOP_EVENT_IDLE_EXIT: Eo_Event_Description;
    pub static _ECORE_MAINLOOP_EVENT_IDLE: Eo_Event_Description;
}
#[link(name = "ecore")]
extern "C" {
//------------------------------------------------------------------------------------------------
// Ecore Common
//------------------------------------------------------------------------------------------------
    pub fn ecore_init() -> c_int;
    pub fn ecore_shutdown() -> c_int;
    pub fn ecore_main_loop_iterate();
    pub fn ecore_main_loop_iterate_may_block(may_block: c_int)
     -> c_int;
    pub fn ecore_main_loop_select_func_set(func: EcoreSelectFunction);
    pub fn ecore_main_loop_select_func_get() -> EcoreSelectFunction;
    pub fn ecore_main_loop_glib_integrate() -> EinaBool;
    pub fn ecore_main_loop_glib_always_integrate_disable();
    pub fn ecore_main_loop_begin();
    pub fn ecore_main_loop_quit();
    pub fn ecore_main_loop_animator_ticked_get() -> EinaBool;
    pub fn ecore_main_loop_nested_get() -> c_int;
    pub fn ecore_fork_reset_callback_add(func: EcoreCb,
                                         data: *const c_void)
     -> EinaBool;
    pub fn ecore_fork_reset_callback_del(func: EcoreCb,
                                         data: *const c_void)
     -> EinaBool;
    pub fn ecore_fork_reset();
    pub fn ecore_main_loop_thread_safe_call_async(callback: EcoreCb,
                                                  data:
                                                      *mut c_void);
    pub fn ecore_main_loop_thread_safe_call_sync(callback: EcoreDataCb,
                                                 data:
                                                     *mut c_void)
     -> *mut c_void;
    pub fn ecore_main_loop_thread_safe_call_wait(wait:
                                                     c_double);
    pub fn ecore_thread_main_loop_begin() -> c_int;
    pub fn ecore_thread_main_loop_end() -> c_int;
    pub fn ecore_event_handler_add(_type: c_int,
                                   func: EcoreEventHandlerCb,
                                   data: *const c_void)
     -> *mut Ecore_Event_Handler;
    pub fn ecore_event_handler_del(event_handler: *mut Ecore_Event_Handler)
     -> *mut c_void;
    pub fn ecore_event_add(_type: c_int,
                           ev: *mut c_void,
                           func_free: EcoreEndCb,
                           data: *mut c_void)
     -> *mut Ecore_Event;
    pub fn ecore_event_del(event: *mut Ecore_Event)
     -> *mut c_void;
    pub fn ecore_event_handler_data_get(eh: *mut Ecore_Event_Handler)
     -> *mut c_void;
    pub fn ecore_event_handler_data_set(eh: *mut Ecore_Event_Handler,
                                        data: *const c_void)
     -> *mut c_void;
    pub fn ecore_event_type_new() -> c_int;
    pub fn ecore_event_filter_add(func_start: EcoreDataCb,
                                  func_filter: EcoreFilterCb,
                                  func_end: EcoreEndCb,
                                  data: *const c_void)
     -> *mut Ecore_Event_Filter;
    pub fn ecore_event_filter_del(ef: *mut Ecore_Event_Filter)
     -> *mut c_void;
    pub fn ecore_event_current_type_get() -> c_int;
    pub fn ecore_event_current_event_get() -> *mut c_void;
    pub fn ecore_memory_state_get() -> EcoreMemoryState;
    pub fn ecore_memory_state_set(state: EcoreMemoryState);
    pub fn ecore_power_state_get() -> EcorePowerState;
    pub fn ecore_power_state_set(state: EcorePowerState);
    pub fn ecore_exe_run_priority_set(pri: c_int);
    pub fn ecore_exe_run_priority_get() -> c_int;
    pub fn ecore_exe_run(exe_cmd: *const c_char,
                         data: *const c_void)
     -> *mut Ecore_Exe;
    pub fn ecore_exe_pipe_run(exe_cmd: *const c_char,
                              flags: EcoreExeFlags,
                              data: *const c_void)
     -> *mut Ecore_Exe;
    pub fn ecore_exe_callback_pre_free_set(exe: *mut Ecore_Exe,
                                           func: EcoreExeCb);
    pub fn ecore_exe_send(exe: *mut Ecore_Exe,
                          data: *const c_void,
                          size: c_int) -> EinaBool;
    pub fn ecore_exe_close_stdin(exe: *mut Ecore_Exe);
    pub fn ecore_exe_auto_limits_set(exe: *mut Ecore_Exe,
                                     start_bytes: c_int,
                                     end_bytes: c_int,
                                     start_lines: c_int,
                                     end_lines: c_int);
    pub fn ecore_exe_event_data_get(exe: *mut Ecore_Exe,
                                    flags: EcoreExeFlags)
     -> *mut Ecore_Exe_Event_Data;
    pub fn ecore_exe_event_data_free(data: *mut Ecore_Exe_Event_Data);
    pub fn ecore_exe_free(exe: *mut Ecore_Exe) -> *mut c_void;
    pub fn ecore_exe_pid_get(exe: *const Ecore_Exe) -> pid_t;
    pub fn ecore_exe_tag_set(exe: *mut Ecore_Exe,
                             tag: *const c_char);
    pub fn ecore_exe_tag_get(exe: *const Ecore_Exe)
     -> *const c_char;
    pub fn ecore_exe_cmd_get(exe: *const Ecore_Exe)
     -> *const c_char;
    pub fn ecore_exe_data_get(exe: *const Ecore_Exe)
     -> *mut c_void;
    pub fn ecore_exe_data_set(exe: *mut Ecore_Exe,
                              data: *mut c_void)
     -> *mut c_void;
    pub fn ecore_exe_flags_get(exe: *const Ecore_Exe) -> EcoreExeFlags;
    pub fn ecore_exe_pause(exe: *mut Ecore_Exe);
    pub fn ecore_exe_continue(exe: *mut Ecore_Exe);
    pub fn ecore_exe_interrupt(exe: *mut Ecore_Exe);
    pub fn ecore_exe_quit(exe: *mut Ecore_Exe);
    pub fn ecore_exe_terminate(exe: *mut Ecore_Exe);
    pub fn ecore_exe_kill(exe: *mut Ecore_Exe);
    pub fn ecore_exe_signal(exe: *mut Ecore_Exe, num: c_int);
    pub fn ecore_exe_hup(exe: *mut Ecore_Exe);
    pub fn ecore_main_fd_handler_add(fd: c_int,
                                     flags: EcoreFdHandlerFlags,
                                     func: EcoreFdCb,
                                     data: *const c_void,
                                     buf_func: EcoreFdCb,
                                     buf_data: *const c_void)
     -> *mut Ecore_Fd_Handler;
    pub fn ecore_main_fd_handler_file_add(fd: c_int,
                                          flags: EcoreFdHandlerFlags,
                                          func: EcoreFdCb,
                                          data: *const c_void,
                                          buf_func: EcoreFdCb,
                                          buf_data:
                                              *const c_void)
     -> *mut Ecore_Fd_Handler;
    pub fn ecore_main_fd_handler_prepare_callback_set(fd_handler:
                                                          *mut Ecore_Fd_Handler,
                                                      func: EcoreFdPrepCb,
                                                      data:
                                                          *const c_void);
    pub fn ecore_main_fd_handler_del(fd_handler: *mut Ecore_Fd_Handler)
     -> *mut c_void;
    pub fn ecore_main_fd_handler_fd_get(fd_handler: *mut Ecore_Fd_Handler)
     -> c_int;
    pub fn ecore_main_fd_handler_active_get(fd_handler: *mut Ecore_Fd_Handler,
                                            flags: EcoreFdHandlerFlags)
     -> EinaBool;
    pub fn ecore_main_fd_handler_active_set(fd_handler: *mut Ecore_Fd_Handler,
                                            flags: EcoreFdHandlerFlags);
    pub fn ecore_main_win32_handler_add(h: *mut c_void,
                                        func: EcoreWin32HandleCb,
                                        data: *const c_void)
     -> *mut Ecore_Win32_Handler;
    pub fn ecore_main_win32_handler_del(win32_handler:
                                            *mut Ecore_Win32_Handler)
     -> *mut c_void;
    pub fn ecore_time_get() -> c_double;
    pub fn ecore_time_unix_get() -> c_double;
    pub fn ecore_loop_time_get() -> c_double;
    pub fn ecore_loop_time_set(t: c_double);
    pub fn ecore_thread_run(func_blocking: EcoreThreadCb,
                            func_end: EcoreThreadCb,
                            func_cancel: EcoreThreadCb,
                            data: *const c_void)
     -> *mut Ecore_Thread;
    pub fn ecore_thread_feedback_run(func_heavy: EcoreThreadCb,
                                     func_notify: EcoreThreadNotifyCb,
                                     func_end: EcoreThreadCb,
                                     func_cancel: EcoreThreadCb,
                                     data: *const c_void,
                                     try_no_queue: EinaBool)
     -> *mut Ecore_Thread;
    pub fn ecore_thread_cancel(thread: *mut Ecore_Thread) -> EinaBool;
    pub fn ecore_thread_wait(thread: *mut Ecore_Thread,
                             wait: c_double) -> EinaBool;
    pub fn ecore_thread_check(thread: *mut Ecore_Thread) -> EinaBool;
    pub fn ecore_thread_feedback(thread: *mut Ecore_Thread,
                                 msg_data: *const c_void)
     -> EinaBool;
    pub fn ecore_thread_reschedule(thread: *mut Ecore_Thread) -> EinaBool;
    pub fn ecore_thread_active_get() -> c_int;
    pub fn ecore_thread_pending_get() -> c_int;
    pub fn ecore_thread_pending_feedback_get() -> c_int;
    pub fn ecore_thread_pending_total_get() -> c_int;
    pub fn ecore_thread_max_get() -> c_int;
    pub fn ecore_thread_max_set(num: c_int);
    pub fn ecore_thread_max_reset();
    pub fn ecore_thread_available_get() -> c_int;
    pub fn ecore_thread_local_data_add(thread: *mut Ecore_Thread,
                                       key: *const c_char,
                                       value: *mut c_void,
                                       cb: EinaFreeCb, direct: EinaBool)
     -> EinaBool;
    pub fn ecore_thread_local_data_set(thread: *mut Ecore_Thread,
                                       key: *const c_char,
                                       value: *mut c_void,
                                       cb: EinaFreeCb)
     -> *mut c_void;
    pub fn ecore_thread_local_data_find(thread: *mut Ecore_Thread,
                                        key: *const c_char)
     -> *mut c_void;
    pub fn ecore_thread_local_data_del(thread: *mut Ecore_Thread,
                                       key: *const c_char)
     -> EinaBool;
    pub fn ecore_thread_global_data_add(key: *const c_char,
                                        value: *mut c_void,
                                        cb: EinaFreeCb, direct: EinaBool)
     -> EinaBool;
    pub fn ecore_thread_global_data_set(key: *const c_char,
                                        value: *mut c_void,
                                        cb: EinaFreeCb)
     -> *mut c_void;
    pub fn ecore_thread_global_data_find(key: *const c_char)
     -> *mut c_void;
    pub fn ecore_thread_global_data_del(key: *const c_char)
     -> EinaBool;
    pub fn ecore_thread_global_data_wait(key: *const c_char,
                                         seconds: c_double)
     -> *mut c_void;
    pub fn ecore_pipe_add(handler: EcorePipeCb,
                          data: *const c_void)
     -> *mut Ecore_Pipe;
    pub fn ecore_pipe_full_add(handler: EcorePipeCb,
                               data: *const c_void,
                               fd_read: c_int,
                               fd_write: c_int,
                               read_survive_fork: EinaBool,
                               write_survive_fork: EinaBool)
     -> *mut Ecore_Pipe;
    pub fn ecore_pipe_del(p: *mut Ecore_Pipe) -> *mut c_void;
    pub fn ecore_pipe_write(p: *mut Ecore_Pipe,
                            buffer: *const c_void,
                            nbytes: c_uint) -> EinaBool;
    pub fn ecore_pipe_write_close(p: *mut Ecore_Pipe);
    pub fn ecore_pipe_read_close(p: *mut Ecore_Pipe);
    pub fn ecore_pipe_read_fd(p: *mut Ecore_Pipe) -> c_int;
    pub fn ecore_pipe_write_fd(p: *mut Ecore_Pipe) -> c_int;
    pub fn ecore_pipe_thaw(p: *mut Ecore_Pipe);
    pub fn ecore_pipe_freeze(p: *mut Ecore_Pipe);
    pub fn ecore_pipe_wait(p: *mut Ecore_Pipe,
                           message_count: c_int,
                           wait: c_double)
     -> c_int;
    pub fn ecore_app_args_set(argc: c_int,
                              argv: *mut *const c_char);
    pub fn ecore_app_args_get(argc: *mut c_int,
                              argv: *mut *mut *mut c_char);
    pub fn ecore_app_restart();
    pub fn ecore_app_no_system_modules();
    pub fn ecore_throttle_adjust(amount: c_double);
    pub fn ecore_throttle_get() -> c_double;
    pub fn ecore_poller_poll_interval_set(_type: EcorePollerType,
                                          poll_time: c_double);
    pub fn ecore_poller_poll_interval_get(_type: EcorePollerType)
     -> c_double;
    pub fn ecore_animator_frametime_set(frametime: c_double);
    pub fn ecore_animator_frametime_get() -> c_double;
    pub fn ecore_animator_pos_map(pos: c_double,
                                  map: EcorePosMap,
                                  v1: c_double,
                                  v2: c_double)
     -> c_double;
    pub fn ecore_animator_pos_map_n(pos: c_double,
                                    map: EcorePosMap,
                                    v_size: c_int,
                                    v: *mut c_double)
     -> c_double;
    pub fn ecore_animator_source_set(source: EcoreAnimatorSource);
    pub fn ecore_animator_source_get() -> EcoreAnimatorSource;
    pub fn ecore_animator_custom_source_tick_begin_callback_set(func: EcoreCb,
                                                                data: *const c_void);
    pub fn ecore_animator_custom_source_tick_end_callback_set(func: EcoreCb,
                                                              data: *const c_void);
    pub fn ecore_animator_custom_tick();
    pub fn ecore_timer_precision_get() -> c_double;
    pub fn ecore_timer_precision_set(precision: c_double);
    pub fn ecore_timer_dump() -> *mut c_char;

//------------------------------------------------------------------------------------------------
// Ecore Legacy
//------------------------------------------------------------------------------------------------
    pub fn ecore_poller_add(_type: EcorePollerType,
                            interval: c_int,
                            func: EcoreTaskCb,
                            data: *const c_void)
     -> *mut Ecore_Poller;
    pub fn ecore_poller_del(poller: *mut Ecore_Poller)
     -> *mut c_void;
    pub fn ecore_poller_poller_interval_set(obj: *mut Ecore_Poller,
                                            interval: c_int)
     -> EinaBool;
    pub fn ecore_poller_poller_interval_get(obj: *const Ecore_Poller)
     -> c_int;
    pub fn ecore_animator_add(func: EcoreTaskCb,
                              data: *const c_void)
     -> *mut Ecore_Animator;
    pub fn ecore_animator_timeline_add(runtime: c_double,
                                       func: EcoreTimelineCb,
                                       data: *const c_void)
     -> *mut Ecore_Animator;
    pub fn ecore_animator_del(animator: *mut Ecore_Animator)
     -> *mut c_void;
    pub fn ecore_animator_freeze(animator: *mut Ecore_Animator);
    pub fn ecore_animator_thaw(animator: *mut Ecore_Animator);
    pub fn ecore_timer_add(_in: c_double, func: EcoreTaskCb,
                           data: *const c_void)
     -> *mut Ecore_Timer;
    pub fn ecore_timer_loop_add(_in: c_double,
                                func: EcoreTaskCb,
                                data: *const c_void)
     -> *mut Ecore_Timer;
    pub fn ecore_timer_del(timer: *mut Ecore_Timer)
     -> *mut c_void;
    pub fn ecore_timer_freeze(timer: *mut Ecore_Timer);
    pub fn ecore_timer_freeze_get(timer: *mut Ecore_Timer) -> EinaBool;
    pub fn ecore_timer_thaw(timer: *mut Ecore_Timer);
    pub fn ecore_timer_interval_set(obj: *mut Ecore_Timer,
                                    _in: c_double);
    pub fn ecore_timer_interval_get(obj: *const Ecore_Timer)
     -> c_double;
    pub fn ecore_timer_pending_get(obj: *const Ecore_Timer)
     -> c_double;
    pub fn ecore_timer_reset(obj: *mut Ecore_Timer);
    pub fn ecore_timer_delay(obj: *mut Ecore_Timer,
                             add: c_double);
    pub fn ecore_idler_add(func: EcoreTaskCb,
                           data: *const c_void)
     -> *mut Ecore_Idler;
    pub fn ecore_idler_del(idler: *mut Ecore_Idler)
     -> *mut c_void;
    pub fn ecore_idle_enterer_add(func: EcoreTaskCb,
                                  data: *const c_void)
     -> *mut Ecore_Idle_Enterer;
    pub fn ecore_idle_enterer_before_add(func: EcoreTaskCb,
                                         data: *const c_void)
     -> *mut Ecore_Idle_Enterer;
    pub fn ecore_idle_enterer_del(idle_enterer: *mut Ecore_Idle_Enterer)
     -> *mut c_void;
    pub fn ecore_idle_exiter_add(func: EcoreTaskCb,
                                 data: *const c_void)
     -> *mut Ecore_Idle_Exiter;
    pub fn ecore_idle_exiter_del(idle_exiter: *mut Ecore_Idle_Exiter)
     -> *mut c_void;
    pub fn ecore_job_add(func: EcoreCb, data: *const c_void)
     -> *mut Ecore_Job;
    pub fn ecore_job_del(obj: *mut Ecore_Job) -> *mut c_void;

//------------------------------------------------------------------------------------------------
// Ecore Eo
//------------------------------------------------------------------------------------------------
    pub fn ecore_poller_class_get() -> *const Eo_Class;
    pub fn ecore_poller_interval_set(interval: c_int)
     -> EinaBool;
    pub fn ecore_poller_interval_get() -> c_int;
    pub fn ecore_poller_constructor(_type: EcorePollerType,
                                    interval: c_int,
                                    func: EcoreTaskCb,
                                    data: *const c_void);
    pub fn ecore_animator_class_get() -> *const Eo_Class;
    pub fn ecore_animator_timeline_constructor(runtime: c_double,
                                               func: EcoreTimelineCb,
                                               data: *const c_void);
    pub fn ecore_animator_constructor(func: EcoreTaskCb,
                                      data: *const c_void);
    pub fn ecore_timer_class_get() -> *const Eo_Class;
    pub fn ecore_obj_timer_interval_set(_in: c_double);
    pub fn ecore_obj_timer_interval_get() -> c_double;
    pub fn ecore_obj_timer_pending_get() -> c_double;
    pub fn ecore_obj_timer_loop_constructor(_in: c_double,
                                            func: EcoreTaskCb,
                                            data: *const c_void);
    pub fn ecore_obj_timer_constructor(_in: c_double,
                                       func: EcoreTaskCb,
                                       data: *const c_void);
    pub fn ecore_obj_timer_reset();
    pub fn ecore_obj_timer_delay(add: c_double);
    pub fn ecore_idler_class_get() -> *const Eo_Class;
    pub fn ecore_idler_constructor(func: EcoreTaskCb,
                                   data: *const c_void);
    pub fn ecore_idle_exiter_class_get() -> *const Eo_Class;
    pub fn ecore_idle_exiter_constructor(func: EcoreTaskCb,
                                         data: *const c_void);
    pub fn ecore_idle_enterer_class_get() -> *const Eo_Class;
    pub fn ecore_idle_enterer_before_constructor(func: EcoreTaskCb,
                                                 data: *const c_void);
    pub fn ecore_idle_enterer_after_constructor(func: EcoreTaskCb,
                                                data: *const c_void);
    pub fn ecore_exe_class_get() -> *const Eo_Class;
    pub fn ecore_obj_exe_command_set(exe_cmd: *const c_char,
                                     flags: EcoreExeFlags);
    pub fn ecore_obj_exe_command_get(exe_cmd: *mut *const c_char,
                                     flags: *mut EcoreExeFlags);
    pub fn ecore_job_class_get() -> *const Eo_Class;
    pub fn ecore_job_constructor(func: EcoreCb,
                                 data: *const c_void);
    pub fn ecore_mainloop_class_get() -> *const Eo_Class;
    pub fn ecore_mainloop_select_func_set(select_func: EcoreSelectFunction);
    pub fn ecore_mainloop_select_func_get() -> EcoreSelectFunction;
    pub fn ecore_mainloop_iterate();
    pub fn ecore_mainloop_iterate_may_block(may_block: c_int)
     -> c_int;
    pub fn ecore_mainloop_begin();
    pub fn ecore_mainloop_quit();
    pub fn ecore_mainloop_animator_ticked() -> EinaBool;
    pub fn ecore_main_loop_get() -> *mut Eo;
}
