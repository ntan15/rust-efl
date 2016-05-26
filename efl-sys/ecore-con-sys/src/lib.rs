#![allow(non_camel_case_types)]
extern crate libc;
extern crate eina_sys;
extern crate eo_sys;

use libc::*;
use eina_sys::*;
use eo_sys::*;
/* automatically generated by rust-bindgen */

pub type Ecore_Con = Eo;
pub enum Ecore_Con_Socks { }
pub type EcoreConDnsCb = Option<unsafe extern "C" fn(canonname: *const c_char,
                                                     ip: *const c_char,
                                                     addr: *mut sockaddr,
                                                     addrlen: c_int,
                                                     data: *mut c_void)>;
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum EcoreConType {
    EcoreConLocalUser = 0,
    EcoreConLocalSystem = 1,
    EcoreConLocalAbstract = 2,
    EcoreConRemoteTcp = 3,
    EcoreConRemoteMcast = 4,
    EcoreConRemoteUdp = 5,
    EcoreConRemoteBroadcast = 6,
    EcoreConRemoteNodelay = 7,
    EcoreConRemoteCork = 8,
    EcoreConUseSsl2 = 16,
    EcoreConUseSsl3 = 32,
    EcoreConUseTls = 64,
    EcoreConUseMixed = 96,
    EcoreConLoadCert = 128,
    EcoreConNoProxy = 256,
    EcoreConSocketActivate = 512,
}
pub type Ecore_Con_Base = Eo;
#[repr(C)]
pub struct Ecore_Con_Data_Received {
    pub data: *mut c_void,
    pub size: c_int,
}
pub type Ecore_Con_Server = Eo;
pub type Ecore_Con_Connector = Eo;
pub type Ecore_Con_Client = Eo;
pub type Ecore_Con_Url = Eo;
pub type Efl_Network_Url = Eo;
#[repr(C)]
pub struct Ecore_Con_Event_Client_Add {
    pub client: *mut Ecore_Con_Client,
}
#[repr(C)]
pub struct Ecore_Con_Event_Client_Upgrade {
    pub client: *mut Ecore_Con_Client,
}
#[repr(C)]
pub struct Ecore_Con_Event_Client_Del {
    pub client: *mut Ecore_Con_Client,
}
#[repr(C)]
pub struct Ecore_Con_Event_Client_Error {
    pub client: *mut Ecore_Con_Client,
    pub error: *mut c_char,
}
#[repr(C)]
pub struct Ecore_Con_Event_Server_Add {
    pub server: *mut Ecore_Con_Server,
}
#[repr(C)]
pub struct Ecore_Con_Event_Server_Upgrade {
    pub server: *mut Ecore_Con_Server,
}
#[repr(C)]
pub struct Ecore_Con_Event_Server_Del {
    pub server: *mut Ecore_Con_Server,
}
#[repr(C)]
pub struct Ecore_Con_Event_Server_Error {
    pub server: *mut Ecore_Con_Server,
    pub error: *mut c_char,
}
#[repr(C)]
pub struct Ecore_Con_Event_Client_Data {
    pub client: *mut Ecore_Con_Client,
    pub data: *mut c_void,
    pub size: c_int,
}
#[repr(C)]
pub struct Ecore_Con_Event_Server_Data {
    pub server: *mut Ecore_Con_Server,
    pub data: *mut c_void,
    pub size: c_int,
}
#[repr(C)]
pub struct Ecore_Con_Event_Client_Write {
    pub client: *mut Ecore_Con_Client,
    pub size: c_int,
}
#[repr(C)]
pub struct Ecore_Con_Event_Server_Write {
    pub server: *mut Ecore_Con_Server,
    pub size: c_int,
}
#[repr(C)]
pub struct Ecore_Con_Event_Proxy_Bind {
    pub server: *mut Ecore_Con_Server,
    pub ip: *const c_char,
    pub port: c_int,
}
#[repr(C)]
pub struct Ecore_Con_Event_Url_Data {
    pub url_con: *mut Ecore_Con_Url,
    pub size: c_int,
    pub data: [c_uchar; 1usize],
}
#[repr(C)]
pub struct Ecore_Con_Event_Url_Complete {
    pub url_con: *mut Ecore_Con_Url,
    pub status: c_int,
}
#[repr(C)]
pub struct Ecore_Con_Event_Url_Progress {
    pub url_con: *mut Ecore_Con_Url,
    pub down: Ecore_ConStructUnnamed1,
    pub up: Ecore_ConStructUnnamed2,
}
#[repr(C)]
pub struct Ecore_ConStructUnnamed1 {
    pub total: c_double,
    pub now: c_double,
}
#[repr(C)]
pub struct Ecore_ConStructUnnamed2 {
    pub total: c_double,
    pub now: c_double,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum EcoreConUrlTime {
    EcoreConUrlTimeNone = 0,
    EcoreConUrlTimeIfmodsince = 1,
    EcoreConUrlTimeIfunmodsince = 2,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum EcoreConUrlHttpVersion {
    EcoreConUrlHttpVersion1_0 = 0,
    EcoreConUrlHttpVersion1_1 = 1,
}

#[link(name = "ecore_con")]
extern "C" {
    pub static _ECORE_CON_BASE_EVENT_DATA_RECEIVED: Eo_Event_Description;
    pub static _ECORE_CON_BASE_EVENT_CONNECTION_UPGRADED:
               Eo_Event_Description;
    pub static _ECORE_CON_BASE_EVENT_CONNECTION_ERROR: Eo_Event_Description;
    pub static _EFL_NETWORK_URL_EVENT_DATA: Eo_Event_Description;
    pub static _EFL_NETWORK_URL_EVENT_PROGRESS: Eo_Event_Description;
    pub static _EFL_NETWORK_URL_EVENT_COMPLETE: Eo_Event_Description;
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
    pub fn ecore_con_server_name_get(obj: *const Ecore_Con_Server)
     -> *const c_char;
    pub fn ecore_con_server_client_limit_set(svr: *mut Ecore_Con_Server,
                                             client_limit:
                                                 c_int,
                                             reject_excess_clients:
                                                 c_char);
    pub fn ecore_con_server_clients_get(obj: *const Ecore_Con_Server)
     -> *const Eina_List;
    pub fn ecore_con_client_server_get(obj: *const Ecore_Con_Client)
     -> *mut Ecore_Con_Server;
    pub fn ecore_con_url_url_set(obj: *mut Ecore_Con_Url,
                                 url: *const c_char)
     -> EinaBool;
    pub fn ecore_con_url_url_get(obj: *const Ecore_Con_Url)
     -> *const c_char;
    pub fn ecore_con_base_class_get() -> *const Eo_Class;
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
    pub fn ecore_con_server_class_get() -> *const Eo_Class;
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
    pub fn ecore_con_server_obj_clients_get() -> *const Eina_List;
    pub fn ecore_con_server_obj_connection_type_set(conn_type:
                                                        EcoreConType);
    pub fn ecore_con_server_obj_connection_type_get() -> EcoreConType;
    pub fn ecore_con_connector_class_get() -> *const Eo_Class;
    pub fn ecore_con_client_class_get() -> *const Eo_Class;
    pub fn ecore_con_client_obj_server_get() -> *mut Ecore_Con_Server;
    pub fn efl_network_url_class_get() -> *const Eo_Class;
    pub fn efl_network_url_set(url: *const c_char)
     -> EinaBool;
    pub fn efl_network_url_get() -> *const c_char;
    pub fn ecore_con_init() -> c_int;
    pub fn ecore_con_shutdown() -> c_int;
    pub fn ecore_con_ssl_available_get() -> c_int;
    pub fn ecore_con_ssl_server_cert_add(svr: *mut Ecore_Con_Server,
                                         cert: *const c_char)
     -> EinaBool;
    pub fn ecore_con_ssl_server_privkey_add(svr: *mut Ecore_Con_Server,
                                            key_file:
                                                *const c_char)
     -> EinaBool;
    pub fn ecore_con_ssl_server_crl_add(svr: *mut Ecore_Con_Server,
                                        crl_file:
                                            *const c_char)
     -> EinaBool;
    pub fn ecore_con_ssl_server_cafile_add(svr: *mut Ecore_Con_Server,
                                           ca_file:
                                               *const c_char)
     -> EinaBool;
    pub fn ecore_con_ssl_server_verify(svr: *mut Ecore_Con_Server);
    pub fn ecore_con_ssl_server_verify_basic(svr: *mut Ecore_Con_Server);
    pub fn ecore_con_ssl_server_verify_name_set(svr: *mut Ecore_Con_Server,
                                                name:
                                                    *const c_char);
    pub fn ecore_con_ssl_server_verify_name_get(svr: *mut Ecore_Con_Server)
     -> *const c_char;
    pub fn ecore_con_ssl_server_upgrade(svr: *mut Ecore_Con_Server,
                                        compl_type: EcoreConType)
     -> EinaBool;
    pub fn ecore_con_ssl_client_upgrade(cl: *mut Ecore_Con_Client,
                                        compl_type: EcoreConType)
     -> EinaBool;
    pub fn ecore_con_socks4_remote_add(ip: *const c_char,
                                       port: c_int,
                                       username:
                                           *const c_char)
     -> *mut Ecore_Con_Socks;
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
     -> *mut Ecore_Con_Socks;
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
    pub fn ecore_con_socks_lookup_set(ecs: *mut Ecore_Con_Socks,
                                      enable: EinaBool);
    pub fn ecore_con_socks_lookup_get(ecs: *mut Ecore_Con_Socks) -> EinaBool;
    pub fn ecore_con_socks_bind_set(ecs: *mut Ecore_Con_Socks,
                                    is_bind: EinaBool);
    pub fn ecore_con_socks_bind_get(ecs: *mut Ecore_Con_Socks) -> EinaBool;
    pub fn ecore_con_socks_version_get(ecs: *mut Ecore_Con_Socks)
     -> c_uint;
    pub fn ecore_con_socks_remote_del(ecs: *mut Ecore_Con_Socks);
    pub fn ecore_con_socks_apply_once(ecs: *mut Ecore_Con_Socks);
    pub fn ecore_con_socks_apply_always(ecs: *mut Ecore_Con_Socks);
    pub fn ecore_con_server_add(_type: EcoreConType,
                                name: *const c_char,
                                port: c_int,
                                data: *const c_void)
     -> *mut Ecore_Con_Server;
    pub fn ecore_con_server_connect(_type: EcoreConType,
                                    name: *const c_char,
                                    port: c_int,
                                    data: *const c_void)
     -> *mut Ecore_Con_Server;
    pub fn ecore_con_server_del(svr: *mut Ecore_Con_Server)
     -> *mut c_void;
    pub fn ecore_con_server_data_get(svr: *mut Ecore_Con_Server)
     -> *mut c_void;
    pub fn ecore_con_server_data_set(svr: *mut Ecore_Con_Server,
                                     data: *mut c_void)
     -> *mut c_void;
    pub fn ecore_con_server_connected_get(svr: *const Ecore_Con_Server)
     -> EinaBool;
    pub fn ecore_con_server_port_get(svr: *const Ecore_Con_Server)
     -> c_int;
    pub fn ecore_con_server_uptime_get(svr: *const Ecore_Con_Server)
     -> c_double;
    pub fn ecore_con_server_send(svr: *mut Ecore_Con_Server,
                                 data: *const c_void,
                                 size: c_int)
     -> c_int;
    pub fn ecore_con_server_ip_get(svr: *const Ecore_Con_Server)
     -> *const c_char;
    pub fn ecore_con_server_flush(svr: *mut Ecore_Con_Server);
    pub fn ecore_con_server_timeout_set(svr: *mut Ecore_Con_Server,
                                        timeout: c_double);
    pub fn ecore_con_server_timeout_get(svr: *const Ecore_Con_Server)
     -> c_double;
    pub fn ecore_con_server_fd_get(svr: *const Ecore_Con_Server)
     -> c_int;
    pub fn ecore_con_client_fd_get(cl: *const Ecore_Con_Client)
     -> c_int;
    pub fn ecore_con_client_send(cl: *mut Ecore_Con_Client,
                                 data: *const c_void,
                                 size: c_int)
     -> c_int;
    pub fn ecore_con_client_del(cl: *mut Ecore_Con_Client)
     -> *mut c_void;
    pub fn ecore_con_client_data_set(cl: *mut Ecore_Con_Client,
                                     data: *const c_void);
    pub fn ecore_con_client_data_get(cl: *mut Ecore_Con_Client)
     -> *mut c_void;
    pub fn ecore_con_client_ip_get(cl: *const Ecore_Con_Client)
     -> *const c_char;
    pub fn ecore_con_client_flush(cl: *mut Ecore_Con_Client);
    pub fn ecore_con_client_uptime_get(cl: *const Ecore_Con_Client)
     -> c_double;
    pub fn ecore_con_client_timeout_get(cl: *const Ecore_Con_Client)
     -> c_double;
    pub fn ecore_con_client_timeout_set(cl: *mut Ecore_Con_Client,
                                        timeout: c_double);
    pub fn ecore_con_client_connected_get(cl: *const Ecore_Con_Client)
     -> EinaBool;
    pub fn ecore_con_client_port_get(cl: *const Ecore_Con_Client)
     -> c_int;
    pub fn ecore_con_url_http_version_set(url_con: *mut Ecore_Con_Url,
                                          version: EcoreConUrlHttpVersion)
     -> EinaBool;
    pub fn ecore_con_url_init() -> c_int;
    pub fn ecore_con_url_shutdown() -> c_int;
    pub fn ecore_con_url_pipeline_set(enable: EinaBool);
    pub fn ecore_con_url_pipeline_get() -> EinaBool;
    pub fn ecore_con_url_new(url: *const c_char)
     -> *mut Ecore_Con_Url;
    pub fn ecore_con_url_custom_new(url: *const c_char,
                                    custom_request:
                                        *const c_char)
     -> *mut Ecore_Con_Url;
    pub fn ecore_con_url_free(url_con: *mut Ecore_Con_Url);
    pub fn ecore_con_url_data_set(url_con: *mut Ecore_Con_Url,
                                  data: *mut c_void);
    pub fn ecore_con_url_data_get(url_con: *mut Ecore_Con_Url)
     -> *mut c_void;
    pub fn ecore_con_url_additional_header_add(url_con: *mut Ecore_Con_Url,
                                               key:
                                                   *const c_char,
                                               value:
                                                   *const c_char);
    pub fn ecore_con_url_additional_headers_clear(url_con:
                                                      *mut Ecore_Con_Url);
    pub fn ecore_con_url_response_headers_get(url_con: *mut Ecore_Con_Url)
     -> *const Eina_List;
    pub fn ecore_con_url_fd_set(url_con: *mut Ecore_Con_Url,
                                fd: c_int);
    pub fn ecore_con_url_received_bytes_get(url_con: *mut Ecore_Con_Url)
     -> c_int;
    pub fn ecore_con_url_httpauth_set(url_con: *mut Ecore_Con_Url,
                                      username: *const c_char,
                                      password: *const c_char,
                                      safe: EinaBool) -> EinaBool;
    pub fn ecore_con_url_get(url_con: *mut Ecore_Con_Url) -> EinaBool;
    pub fn ecore_con_url_head(url_con: *mut Ecore_Con_Url) -> EinaBool;
    pub fn ecore_con_url_post(url_con: *mut Ecore_Con_Url,
                              data: *const c_void,
                              length: c_long,
                              content_type: *const c_char)
     -> EinaBool;
    pub fn ecore_con_url_time(url_con: *mut Ecore_Con_Url,
                              time_condition: EcoreConUrlTime,
                              timestamp: c_double);
    pub fn ecore_con_url_ftp_upload(url_con: *mut Ecore_Con_Url,
                                    filename: *const c_char,
                                    user: *const c_char,
                                    pass: *const c_char,
                                    upload_dir: *const c_char)
     -> EinaBool;
    pub fn ecore_con_url_verbose_set(url_con: *mut Ecore_Con_Url,
                                     verbose: EinaBool);
    pub fn ecore_con_url_ftp_use_epsv_set(url_con: *mut Ecore_Con_Url,
                                          use_epsv: EinaBool);
    pub fn ecore_con_url_cookies_init(url_con: *mut Ecore_Con_Url);
    pub fn ecore_con_url_cookies_ignore_old_session_set(url_con:
                                                            *mut Ecore_Con_Url,
                                                        ignore: EinaBool);
    pub fn ecore_con_url_cookies_clear(url_con: *mut Ecore_Con_Url);
    pub fn ecore_con_url_cookies_session_clear(url_con: *mut Ecore_Con_Url);
    pub fn ecore_con_url_cookies_file_add(url_con: *mut Ecore_Con_Url,
                                          file_name:
                                              *const c_char);
    pub fn ecore_con_url_cookies_jar_file_set(url_con: *mut Ecore_Con_Url,
                                              cookiejar_file:
                                                  *const c_char)
     -> EinaBool;
    pub fn ecore_con_url_cookies_jar_write(url_con: *mut Ecore_Con_Url);
    pub fn ecore_con_url_ssl_verify_peer_set(url_con: *mut Ecore_Con_Url,
                                             verify: EinaBool);
    pub fn ecore_con_url_ssl_ca_set(url_con: *mut Ecore_Con_Url,
                                    ca_path: *const c_char)
     -> c_int;
    pub fn ecore_con_url_proxy_set(url_con: *mut Ecore_Con_Url,
                                   proxy: *const c_char)
     -> EinaBool;
    pub fn ecore_con_url_proxy_username_set(url_con: *mut Ecore_Con_Url,
                                            username:
                                                *const c_char)
     -> EinaBool;
    pub fn ecore_con_url_proxy_password_set(url_con: *mut Ecore_Con_Url,
                                            password:
                                                *const c_char)
     -> EinaBool;
    pub fn ecore_con_url_timeout_set(url_con: *mut Ecore_Con_Url,
                                     timeout: c_double);
    pub fn ecore_con_url_status_code_get(url_con: *mut Ecore_Con_Url)
     -> c_int;
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
