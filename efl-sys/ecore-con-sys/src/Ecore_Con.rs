extern crate libc;
extern crate eina_sys;
extern crate eo_sys;

use libc::*;
use eina_sys::*;
use eo_sys::*;
/* automatically generated by rust-bindgen */

pub type EcoreCon = Eo;
pub enum EcoreConSocks { }
pub type EcoreConDnsCb = Option<unsafe extern "C" fn(canonname: *const c_char,
                                                     ip: *const c_char,
                                                     addr: *mut Struct_sockaddr,
                                                     addrlen: c_int,
                                                     data: *mut c_void)>;
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum EcoreConType {
    ECORE_CON_LOCAL_USER = 0,
    ECORE_CON_LOCAL_SYSTEM = 1,
    ECORE_CON_LOCAL_ABSTRACT = 2,
    ECORE_CON_REMOTE_TCP = 3,
    ECORE_CON_REMOTE_MCAST = 4,
    ECORE_CON_REMOTE_UDP = 5,
    ECORE_CON_REMOTE_BROADCAST = 6,
    ECORE_CON_REMOTE_NODELAY = 7,
    ECORE_CON_REMOTE_CORK = 8,
    ECORE_CON_USE_SSL2 = 16,
    ECORE_CON_USE_SSL3 = 32,
    ECORE_CON_USE_TLS = 64,
    ECORE_CON_USE_MIXED = 96,
    ECORE_CON_LOAD_CERT = 128,
    ECORE_CON_NO_PROXY = 256,
    ECORE_CON_SOCKET_ACTIVATE = 512,
}
pub type EcoreConBase = Eo;
#[repr(C)]
pub struct EcoreConEventDataReceived {
    pub data: *mut c_void,
    pub size: c_int,
}
pub type EcoreConServer = Eo;
pub type EcoreConConnector = Eo;
pub type EcoreConClient = Eo;
pub type EcoreConUrl = Eo;
pub type EflNetworkUrl = Eo;
#[repr(C)]
pub struct EcoreConEventClientAdd {
    pub client: *mut EcoreConClient,
}
#[repr(C)]
pub struct EcoreConEventClientUpgrade {
    pub client: *mut EcoreConClient,
}
#[repr(C)]
pub struct EcoreConEventClientDel {
    pub client: *mut EcoreConClient,
}
#[repr(C)]
pub struct EcoreConEventClientError {
    pub client: *mut EcoreConClient,
    pub error: *mut c_char,
}
#[repr(C)]
pub struct EcoreConEventServerAdd {
    pub server: *mut EcoreConServer,
}
#[repr(C)]
pub struct EcoreConEventServerUpgrade {
    pub server: *mut EcoreConServer,
}
#[repr(C)]
pub struct EcoreConEventServerDel {
    pub server: *mut EcoreConServer,
}
#[repr(C)]
pub struct EcoreConEventServerError {
    pub server: *mut EcoreConServer,
    pub error: *mut c_char,
}
#[repr(C)]
pub struct EcoreConEventClientData {
    pub client: *mut EcoreConClient,
    pub data: *mut c_void,
    pub size: c_int,
}
#[repr(C)]
pub struct EcoreConEventServerData {
    pub server: *mut EcoreConServer,
    pub data: *mut c_void,
    pub size: c_int,
}
#[repr(C)]
pub struct EcoreConEventClientWrite {
    pub client: *mut EcoreConClient,
    pub size: c_int,
}
#[repr(C)]
pub struct EcoreConEventServerWrite {
    pub server: *mut EcoreConServer,
    pub size: c_int,
}
#[repr(C)]
pub struct EcoreConEventProxyBind {
    pub server: *mut EcoreConServer,
    pub ip: *const c_char,
    pub port: c_int,
}
#[repr(C)]
pub struct EcoreConEventUrlData {
    pub url_con: *mut EcoreConUrl,
    pub size: c_int,
    pub data: [c_uchar; 1usize],
}
#[repr(C)]
pub struct EcoreConEventUrlComplete {
    pub url_con: *mut EcoreConUrl,
    pub status: c_int,
}
#[repr(C)]
pub struct EcoreConEventUrlProgress {
    pub url_con: *mut EcoreConUrl,
    pub down: EcoreConStructUnnamed1,
    pub up: EcoreConStructUnnamed2,
}
#[repr(C)]
pub struct EcoreConStructUnnamed1 {
    pub total: c_double,
    pub now: c_double,
}
#[repr(C)]
pub struct EcoreConStructUnnamed2 {
    pub total: c_double,
    pub now: c_double,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum EcoreConUrlTime {
    ECORE_CON_URL_TIME_NONE = 0,
    ECORE_CON_URL_TIME_IFMODSINCE = 1,
    ECORE_CON_URL_TIME_IFUNMODSINCE = 2,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum EcoreConUrlHttpVersion {
    ECORE_CON_URL_HTTP_VERSION_1_0 = 0,
    ECORE_CON_URL_HTTP_VERSION_1_1 = 1,
}

#[link(name = "ecore_con")]
extern "C" {
    pub static _ECORE_CON_BASE_EVENT_DATA_RECEIVED: EoEventDescription;
    pub static _ECORE_CON_BASE_EVENT_CONNECTION_UPGRADED:
               EoEventDescription;
    pub static _ECORE_CON_BASE_EVENT_CONNECTION_ERROR: EoEventDescription;
    pub static _EFL_NETWORK_URL_EVENT_DATA: EoEventDescription;
    pub static _EFL_NETWORK_URL_EVENT_PROGRESS: EoEventDescription;
    pub static _EFL_NETWORK_URL_EVENT_COMPLETE: EoEventDescription;
    pub static mut ECORE_CON_EVENT_CLIENT_ADD: c_int;
    pub static mut ECORE_CON_EVENT_CLIENT_DEL: c_int;
    pub static mut ECORE_CON_EVENT_CLIENT_ERROR: c_int;
    pub static mut ECORE_CON_EVENT_CLIENT_UPGRADE: c_int;
    pub static mut ECORE_CON_EVENT_SERVER_ADD: c_int;
    pub static mut ECORE_CON_EVENT_SERVER_DEL: c_int;
    pub static mut ECORE_CON_EVENT_SERVER_ERROR: c_int;
    pub static mut ECORE_CON_EVENT_SERVER_UPGRADE: c_int;
    pub static mut ECORE_CON_EVENT_CLIENT_WRITE: c_int;
    pub static mut ECORE_CON_EVENT_SERVER_WRITE: c_int;
    pub static mut ECORE_CON_EVENT_CLIENT_DATA: c_int;
    pub static mut ECORE_CON_EVENT_SERVER_DATA: c_int;
    pub static mut ECORE_CON_EVENT_PROXY_BIND: c_int;
    pub static mut ECORE_CON_EVENT_URL_DATA: c_int;
    pub static mut ECORE_CON_EVENT_URL_COMPLETE: c_int;
    pub static mut ECORE_CON_EVENT_URL_PROGRESS: c_int;
}
#[link(name = "ecore_con")]
extern "C" {
    pub fn ecore_con_lookup(name: *const c_char,
                            done_cb: EcoreConDnsCb,
                            data: *const c_void) -> EinaBool;
    pub fn ecore_con_server_name_get(obj: *const EcoreConServer)
     -> *const c_char;
    pub fn ecore_con_server_client_limit_set(svr: *mut EcoreConServer,
                                             client_limit:
                                                 c_int,
                                             reject_excess_clients:
                                                 c_char);
    pub fn ecore_con_server_clients_get(obj: *const EcoreConServer)
     -> *const EinaList;
    pub fn ecore_con_client_server_get(obj: *const EcoreConClient)
     -> *mut EcoreConServer;
    pub fn ecore_con_url_url_set(obj: *mut EcoreConUrl,
                                 url: *const c_char)
     -> EinaBool;
    pub fn ecore_con_url_url_get(obj: *const EcoreConUrl)
     -> *const c_char;
    pub fn ecore_con_base_class_get() -> *const EoClass;
    pub fn ecore_con_obj_ip_get() -> *const c_char;
    pub fn ecore_con_obj_uptime_get() -> c_double;
    pub fn ecore_con_obj_port_set(port: c_int);
    pub fn ecore_con_obj_port_get() -> c_int;
    pub fn ecore_con_obj_fd_get() -> c_int;
    pub fn ecore_con_obj_connected_get() -> EinaBool;
    pub fn ecore_con_obj_timeout_set(timeout: c_double);
    pub fn ecore_con_obj_timeout_get() -> c_double;
    pub fn ecore_con_obj_flush();
    pub fn ecore_con_obj_send(data: *const c_void,
                              size: c_int)
     -> c_int;
    pub fn ecore_con_obj_lookup(name: *const c_char,
                                done_cb: EcoreConDnsCb,
                                data: *const c_void)
     -> EinaBool;
    pub fn ecore_con_server_class_get() -> *const EoClass;
    pub fn ecore_con_server_obj_name_set(name: *const c_char);
    pub fn ecore_con_server_obj_name_get() -> *const c_char;
    pub fn ecore_con_server_obj_client_limit_set(client_limit:
                                                     c_int,
                                                 reject_excess_clients:
                                                     c_char);
    pub fn ecore_con_server_obj_client_limit_get(client_limit:
                                                     *mut c_int,
                                                 reject_excess_clients:
                                                     *mut c_char);
    pub fn ecore_con_server_obj_clients_get() -> *const EinaList;
    pub fn ecore_con_server_obj_connection_type_set(conn_type:
                                                        EcoreConType);
    pub fn ecore_con_server_obj_connection_type_get() -> EcoreConType;
    pub fn ecore_con_connector_class_get() -> *const EoClass;
    pub fn ecore_con_client_class_get() -> *const EoClass;
    pub fn ecore_con_client_obj_server_get() -> *mut EcoreConServer;
    pub fn efl_network_url_class_get() -> *const EoClass;
    pub fn efl_network_url_set(url: *const c_char)
     -> EinaBool;
    pub fn efl_network_url_get() -> *const c_char;
    pub fn ecore_con_init() -> c_int;
    pub fn ecore_con_shutdown() -> c_int;
    pub fn ecore_con_ssl_available_get() -> c_int;
    pub fn ecore_con_ssl_server_cert_add(svr: *mut EcoreConServer,
                                         cert: *const c_char)
     -> EinaBool;
    pub fn ecore_con_ssl_server_privkey_add(svr: *mut EcoreConServer,
                                            key_file:
                                                *const c_char)
     -> EinaBool;
    pub fn ecore_con_ssl_server_crl_add(svr: *mut EcoreConServer,
                                        crl_file:
                                            *const c_char)
     -> EinaBool;
    pub fn ecore_con_ssl_server_cafile_add(svr: *mut EcoreConServer,
                                           ca_file:
                                               *const c_char)
     -> EinaBool;
    pub fn ecore_con_ssl_server_verify(svr: *mut EcoreConServer);
    pub fn ecore_con_ssl_server_verify_basic(svr: *mut EcoreConServer);
    pub fn ecore_con_ssl_server_verify_name_set(svr: *mut EcoreConServer,
                                                name:
                                                    *const c_char);
    pub fn ecore_con_ssl_server_verify_name_get(svr: *mut EcoreConServer)
     -> *const c_char;
    pub fn ecore_con_ssl_server_upgrade(svr: *mut EcoreConServer,
                                        compl_type: EcoreConType)
     -> EinaBool;
    pub fn ecore_con_ssl_client_upgrade(cl: *mut EcoreConClient,
                                        compl_type: EcoreConType)
     -> EinaBool;
    pub fn ecore_con_socks4_remote_add(ip: *const c_char,
                                       port: c_int,
                                       username:
                                           *const c_char)
     -> *mut EcoreConSocks;
    pub fn ecore_con_socks4_remote_exists(ip: *const c_char,
                                          port: c_int,
                                          username:
                                              *const c_char)
     -> EinaBool;
    pub fn ecore_con_socks4_remote_del(ip: *const c_char,
                                       port: c_int,
                                       username:
                                           *const c_char);
    pub fn ecore_con_socks5_remote_add(ip: *const c_char,
                                       port: c_int,
                                       username:
                                           *const c_char,
                                       password:
                                           *const c_char)
     -> *mut EcoreConSocks;
    pub fn ecore_con_socks5_remote_exists(ip: *const c_char,
                                          port: c_int,
                                          username:
                                              *const c_char,
                                          password:
                                              *const c_char)
     -> EinaBool;
    pub fn ecore_con_socks5_remote_del(ip: *const c_char,
                                       port: c_int,
                                       username:
                                           *const c_char,
                                       password:
                                           *const c_char);
    pub fn ecore_con_socks_lookup_set(ecs: *mut EcoreConSocks,
                                      enable: EinaBool);
    pub fn ecore_con_socks_lookup_get(ecs: *mut EcoreConSocks) -> EinaBool;
    pub fn ecore_con_socks_bind_set(ecs: *mut EcoreConSocks,
                                    is_bind: EinaBool);
    pub fn ecore_con_socks_bind_get(ecs: *mut EcoreConSocks) -> EinaBool;
    pub fn ecore_con_socks_version_get(ecs: *mut EcoreConSocks)
     -> c_uint;
    pub fn ecore_con_socks_remote_del(ecs: *mut EcoreConSocks);
    pub fn ecore_con_socks_apply_once(ecs: *mut EcoreConSocks);
    pub fn ecore_con_socks_apply_always(ecs: *mut EcoreConSocks);
    pub fn ecore_con_server_add(_type: EcoreConType,
                                name: *const c_char,
                                port: c_int,
                                data: *const c_void)
     -> *mut EcoreConServer;
    pub fn ecore_con_server_connect(_type: EcoreConType,
                                    name: *const c_char,
                                    port: c_int,
                                    data: *const c_void)
     -> *mut EcoreConServer;
    pub fn ecore_con_server_del(svr: *mut EcoreConServer)
     -> *mut c_void;
    pub fn ecore_con_server_data_get(svr: *mut EcoreConServer)
     -> *mut c_void;
    pub fn ecore_con_server_data_set(svr: *mut EcoreConServer,
                                     data: *mut c_void)
     -> *mut c_void;
    pub fn ecore_con_server_connected_get(svr: *const EcoreConServer)
     -> EinaBool;
    pub fn ecore_con_server_port_get(svr: *const EcoreConServer)
     -> c_int;
    pub fn ecore_con_server_uptime_get(svr: *const EcoreConServer)
     -> c_double;
    pub fn ecore_con_server_send(svr: *mut EcoreConServer,
                                 data: *const c_void,
                                 size: c_int)
     -> c_int;
    pub fn ecore_con_server_ip_get(svr: *const EcoreConServer)
     -> *const c_char;
    pub fn ecore_con_server_flush(svr: *mut EcoreConServer);
    pub fn ecore_con_server_timeout_set(svr: *mut EcoreConServer,
                                        timeout: c_double);
    pub fn ecore_con_server_timeout_get(svr: *const EcoreConServer)
     -> c_double;
    pub fn ecore_con_server_fd_get(svr: *const EcoreConServer)
     -> c_int;
    pub fn ecore_con_client_fd_get(cl: *const EcoreConClient)
     -> c_int;
    pub fn ecore_con_client_send(cl: *mut EcoreConClient,
                                 data: *const c_void,
                                 size: c_int)
     -> c_int;
    pub fn ecore_con_client_del(cl: *mut EcoreConClient)
     -> *mut c_void;
    pub fn ecore_con_client_data_set(cl: *mut EcoreConClient,
                                     data: *const c_void);
    pub fn ecore_con_client_data_get(cl: *mut EcoreConClient)
     -> *mut c_void;
    pub fn ecore_con_client_ip_get(cl: *const EcoreConClient)
     -> *const c_char;
    pub fn ecore_con_client_flush(cl: *mut EcoreConClient);
    pub fn ecore_con_client_uptime_get(cl: *const EcoreConClient)
     -> c_double;
    pub fn ecore_con_client_timeout_get(cl: *const EcoreConClient)
     -> c_double;
    pub fn ecore_con_client_timeout_set(cl: *mut EcoreConClient,
                                        timeout: c_double);
    pub fn ecore_con_client_connected_get(cl: *const EcoreConClient)
     -> EinaBool;
    pub fn ecore_con_client_port_get(cl: *const EcoreConClient)
     -> c_int;
    pub fn ecore_con_url_http_version_set(url_con: *mut EcoreConUrl,
                                          version: EcoreConUrlHttpVersion)
     -> EinaBool;
    pub fn ecore_con_url_init() -> c_int;
    pub fn ecore_con_url_shutdown() -> c_int;
    pub fn ecore_con_url_pipeline_set(enable: EinaBool);
    pub fn ecore_con_url_pipeline_get() -> EinaBool;
    pub fn ecore_con_url_new(url: *const c_char)
     -> *mut EcoreConUrl;
    pub fn ecore_con_url_custom_new(url: *const c_char,
                                    custom_request:
                                        *const c_char)
     -> *mut EcoreConUrl;
    pub fn ecore_con_url_free(url_con: *mut EcoreConUrl);
    pub fn ecore_con_url_data_set(url_con: *mut EcoreConUrl,
                                  data: *mut c_void);
    pub fn ecore_con_url_data_get(url_con: *mut EcoreConUrl)
     -> *mut c_void;
    pub fn ecore_con_url_additional_header_add(url_con: *mut EcoreConUrl,
                                               key:
                                                   *const c_char,
                                               value:
                                                   *const c_char);
    pub fn ecore_con_url_additional_headers_clear(url_con:
                                                      *mut EcoreConUrl);
    pub fn ecore_con_url_response_headers_get(url_con: *mut EcoreConUrl)
     -> *const EinaList;
    pub fn ecore_con_url_fd_set(url_con: *mut EcoreConUrl,
                                fd: c_int);
    pub fn ecore_con_url_received_bytes_get(url_con: *mut EcoreConUrl)
     -> c_int;
    pub fn ecore_con_url_httpauth_set(url_con: *mut EcoreConUrl,
                                      username: *const c_char,
                                      password: *const c_char,
                                      safe: EinaBool) -> EinaBool;
    pub fn ecore_con_url_get(url_con: *mut EcoreConUrl) -> EinaBool;
    pub fn ecore_con_url_head(url_con: *mut EcoreConUrl) -> EinaBool;
    pub fn ecore_con_url_post(url_con: *mut EcoreConUrl,
                              data: *const c_void,
                              length: c_long,
                              content_type: *const c_char)
     -> EinaBool;
    pub fn ecore_con_url_time(url_con: *mut EcoreConUrl,
                              time_condition: EcoreConUrlTime,
                              timestamp: c_double);
    pub fn ecore_con_url_ftp_upload(url_con: *mut EcoreConUrl,
                                    filename: *const c_char,
                                    user: *const c_char,
                                    pass: *const c_char,
                                    upload_dir: *const c_char)
     -> EinaBool;
    pub fn ecore_con_url_verbose_set(url_con: *mut EcoreConUrl,
                                     verbose: EinaBool);
    pub fn ecore_con_url_ftp_use_epsv_set(url_con: *mut EcoreConUrl,
                                          use_epsv: EinaBool);
    pub fn ecore_con_url_cookies_init(url_con: *mut EcoreConUrl);
    pub fn ecore_con_url_cookies_ignore_old_session_set(url_con:
                                                            *mut EcoreConUrl,
                                                        ignore: EinaBool);
    pub fn ecore_con_url_cookies_clear(url_con: *mut EcoreConUrl);
    pub fn ecore_con_url_cookies_session_clear(url_con: *mut EcoreConUrl);
    pub fn ecore_con_url_cookies_file_add(url_con: *mut EcoreConUrl,
                                          file_name:
                                              *const c_char);
    pub fn ecore_con_url_cookies_jar_file_set(url_con: *mut EcoreConUrl,
                                              cookiejar_file:
                                                  *const c_char)
     -> EinaBool;
    pub fn ecore_con_url_cookies_jar_write(url_con: *mut EcoreConUrl);
    pub fn ecore_con_url_ssl_verify_peer_set(url_con: *mut EcoreConUrl,
                                             verify: EinaBool);
    pub fn ecore_con_url_ssl_ca_set(url_con: *mut EcoreConUrl,
                                    ca_path: *const c_char)
     -> c_int;
    pub fn ecore_con_url_proxy_set(url_con: *mut EcoreConUrl,
                                   proxy: *const c_char)
     -> EinaBool;
    pub fn ecore_con_url_proxy_username_set(url_con: *mut EcoreConUrl,
                                            username:
                                                *const c_char)
     -> EinaBool;
    pub fn ecore_con_url_proxy_password_set(url_con: *mut EcoreConUrl,
                                            password:
                                                *const c_char)
     -> EinaBool;
    pub fn ecore_con_url_timeout_set(url_con: *mut EcoreConUrl,
                                     timeout: c_double);
    pub fn ecore_con_url_status_code_get(url_con: *mut EcoreConUrl)
     -> c_int;
}