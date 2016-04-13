extern crate libc;
extern crate eina_sys;

use libc::*;
use eina_sys::*;

/* automatically generated by rust-bindgen */

pub enum EcoreFileMonitor { }
pub enum EcoreFileDownloadJob { }
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum EcoreFileEvent {
    ECORE_FILE_EVENT_NONE = 0,
    ECORE_FILE_EVENT_CREATED_FILE = 1,
    ECORE_FILE_EVENT_CREATED_DIRECTORY = 2,
    ECORE_FILE_EVENT_DELETED_FILE = 3,
    ECORE_FILE_EVENT_DELETED_DIRECTORY = 4,
    ECORE_FILE_EVENT_DELETED_SELF = 5,
    ECORE_FILE_EVENT_MODIFIED = 6,
    ECORE_FILE_EVENT_CLOSED = 7,
}
pub type EcoreFileMonitorCb = Option<unsafe extern "C" fn(data: *mut c_void,
                                                           em: *mut EcoreFileMonitor,
                                                           event: EcoreFileEvent,
                                                           path: *const c_char)>;
pub type EcoreFileDownloadCompletionCb = Option<unsafe extern "C" fn(data: *mut c_void,
                                                                     file: *const c_char,
                                                                     status: c_int)>;
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum EcoreFileProgressReturn {
    ECORE_FILE_PROGRESS_CONTINUE = 0,
    ECORE_FILE_PROGRESS_ABORT = 1,
}
pub type EcoreFileDownloadProgressCb = Option<unsafe extern "C" fn(data: *mut c_void,
                                                                   file: *const c_char,
                                                                   dltotal: c_long,
                                                                   dlnow: c_long,
                                                                   ultotal: c_long,
                                                                   ulnow: c_long)
                              -> c_int>;
#[link(name = "ecore_file")]
extern "C" {
    pub fn ecore_file_init() -> c_int;
    pub fn ecore_file_shutdown() -> c_int;
    pub fn ecore_file_mod_time(file: *const c_char)
     -> c_longlong;
    pub fn ecore_file_size(file: *const c_char)
     -> c_longlong;
    pub fn ecore_file_exists(file: *const c_char)
     -> EinaBool;
    pub fn ecore_file_is_dir(file: *const c_char)
     -> EinaBool;
    pub fn ecore_file_mkdir(dir: *const c_char) -> EinaBool;
    pub fn ecore_file_mkdirs(dirs: *mut *const c_char)
     -> c_int;
    pub fn ecore_file_mksubdirs(base: *const c_char,
                                subdirs: *mut *const c_char)
     -> c_int;
    pub fn ecore_file_rmdir(dir: *const c_char) -> EinaBool;
    pub fn ecore_file_recursive_rm(dir: *const c_char)
     -> EinaBool;
    pub fn ecore_file_mkpath(path: *const c_char)
     -> EinaBool;
    pub fn ecore_file_mkpaths(paths: *mut *const c_char)
     -> c_int;
    pub fn ecore_file_cp(src: *const c_char,
                         dst: *const c_char) -> EinaBool;
    pub fn ecore_file_mv(src: *const c_char,
                         dst: *const c_char) -> EinaBool;
    pub fn ecore_file_symlink(src: *const c_char,
                              dest: *const c_char)
     -> EinaBool;
    pub fn ecore_file_realpath(file: *const c_char)
     -> *mut c_char;
    pub fn ecore_file_unlink(file: *const c_char)
     -> EinaBool;
    pub fn ecore_file_remove(file: *const c_char)
     -> EinaBool;
    pub fn ecore_file_file_get(path: *const c_char)
     -> *const c_char;
    pub fn ecore_file_dir_get(path: *const c_char)
     -> *mut c_char;
    pub fn ecore_file_can_read(file: *const c_char)
     -> EinaBool;
    pub fn ecore_file_can_write(file: *const c_char)
     -> EinaBool;
    pub fn ecore_file_can_exec(file: *const c_char)
     -> EinaBool;
    pub fn ecore_file_readlink(link: *const c_char)
     -> *mut c_char;
    pub fn ecore_file_ls(dir: *const c_char)
     -> *mut EinaList;
    pub fn ecore_file_app_exe_get(app: *const c_char)
     -> *mut c_char;
    pub fn ecore_file_escape_name(filename: *const c_char)
     -> *mut c_char;
    pub fn ecore_file_strip_ext(file: *const c_char)
     -> *mut c_char;
    pub fn ecore_file_dir_is_empty(dir: *const c_char)
     -> c_int;
    pub fn ecore_file_monitor_add(path: *const c_char,
                                  func: EcoreFileMonitorCb,
                                  data: *mut c_void)
     -> *mut EcoreFileMonitor;
    pub fn ecore_file_monitor_del(ecore_file_monitor:
                                      *mut EcoreFileMonitor);
    pub fn ecore_file_monitor_path_get(ecore_file_monitor:
                                           *mut EcoreFileMonitor)
     -> *const c_char;
    pub fn ecore_file_path_dir_exists(in_dir: *const c_char)
     -> EinaBool;
    pub fn ecore_file_app_installed(exe: *const c_char)
     -> EinaBool;
    pub fn ecore_file_app_list() -> *mut EinaList;
    pub fn ecore_file_download(url: *const c_char,
                               dst: *const c_char,
                               completion_cb:
                                   EcoreFileDownloadCompletionCb,
                               progress_cb: EcoreFileDownloadProgressCb,
                               data: *mut c_void,
                               job_ret: *mut *mut EcoreFileDownloadJob)
     -> EinaBool;
    pub fn ecore_file_download_full(url: *const c_char,
                                    dst: *const c_char,
                                    completion_cb:
                                        EcoreFileDownloadCompletionCb,
                                    progress_cb:
                                        EcoreFileDownloadProgressCb,
                                    data: *mut c_void,
                                    job_ret:
                                        *mut *mut EcoreFileDownloadJob,
                                    headers: *mut EinaHash) -> EinaBool;
    pub fn ecore_file_download_abort_all();
    pub fn ecore_file_download_abort(job: *mut EcoreFileDownloadJob);
    pub fn ecore_file_download_protocol_available(protocol:
                                                      *const c_char)
     -> EinaBool;
}