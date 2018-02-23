#![allow(raw_pointer_derive)]
#![allow(non_camel_case_types)]

use std::mem;

use libc::{c_void, c_char, c_uchar, c_short, c_ushort, c_int, c_uint, c_long, c_ulong};


// APACHE PORTABLE RUNTIME

pub const APR_RFC822_DATE_LEN:    apr_size_t = 30;

// run this hook first, before ANYTHING
pub const APR_HOOK_REALLY_FIRST:  c_int = -10;
// run this hook first
pub const APR_HOOK_FIRST:         c_int = 0;
// run this hook somewhere
pub const APR_HOOK_MIDDLE:        c_int = 10;
// run this hook after every other hook which is defined
pub const APR_HOOK_LAST:          c_int = 20;
// run this hook last, after EVERYTHING
pub const APR_HOOK_REALLY_LAST:   c_int = 30;

pub type sockaddr_in = c_void;
pub type sockaddr_in6 = c_void;
pub type sockaddr_storage = c_void;
pub type conn_state_t = c_void;

pub type apr_byte_t = c_uchar;
pub type apr_int16_t = c_short;
pub type apr_uint16_t = c_ushort;
pub type apr_int32_t = c_int;
pub type apr_uint32_t = c_uint;
pub type apr_int64_t = c_long;
pub type apr_uint64_t = c_ulong;
pub type apr_size_t = c_ulong;
pub type apr_ssize_t = c_long;
pub type apr_off_t = c_long;
pub type apr_socklen_t = c_uint;
pub type apr_ino_t = c_ulong;
pub type apr_uintptr_t = apr_uint64_t;
pub type apr_status_t = c_int;
pub type apr_signum_t = c_int;
pub type apr_read_type_e = c_uint;
pub type apr_bucket_is_metadata_t = c_uint;
pub type apr_filetype_e = c_uint;
pub type apr_uid_t = c_uint;
pub type apr_gid_t = c_uint;
pub type apr_dev_t = c_ulong;
pub type apr_fileperms_t = apr_int32_t;
pub type apr_time_t = apr_int64_t;
pub type apr_interval_time_t = apr_int64_t;
pub type apr_port_t = apr_uint16_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct apr_array_header_t {
   pub pool: *mut apr_pool_t,
   pub elt_size: c_int,
   pub nelts: c_int,
   pub nalloc: c_int,
   pub elts: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct apr_table_entry_t {
   pub key: *mut c_char,
   pub val: *mut c_char,
   pub key_checksum: apr_uint32_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct apr_bucket_brigade {
   pub p: *mut apr_pool_t,
   pub list: apr_bucket_list,
   pub bucket_alloc: *mut apr_bucket_alloc_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct apr_bucket_list {
   pub next: *mut apr_bucket,
   pub prev: *mut apr_bucket,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct apr_bucket {
   pub link: apr_bucket_list,
   pub _type: *const apr_bucket_type_t,
   pub length: apr_size_t,
   pub start: apr_off_t,
   pub data: *mut c_void,
   pub free: Option<extern "C" fn(e: *mut c_void) -> ()>,
   pub list: *mut apr_bucket_alloc_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct apr_bucket_type_t {
   pub name: *const c_char,
   pub num_func: c_int,
   pub is_metadata: apr_bucket_is_metadata_t,

   pub destroy: Option<extern "C" fn(
      data: *mut c_void
   ) -> ()>,

   pub read: Option<extern "C" fn(
      b: *mut apr_bucket,
      str: *mut *const c_char,
      len: *mut apr_size_t,
      block: apr_read_type_e
   ) -> apr_status_t>,

   pub setaside: Option<extern "C" fn(
      e: *mut apr_bucket,
      pool: *mut apr_pool_t
   ) -> apr_status_t>,

   pub split: Option<extern "C" fn(
      e: *mut apr_bucket,
      point: apr_size_t
   ) -> apr_status_t>,

   pub copy: Option<extern "C" fn(
      e: *mut apr_bucket,
      c: *mut *mut apr_bucket
   ) -> apr_status_t>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct apr_uri_t {
   pub scheme: *mut c_char,
   pub hostinfo: *mut c_char,
   pub user: *mut c_char,
   pub password: *mut c_char,
   pub hostname: *mut c_char,
   pub port_str: *mut c_char,
   pub path: *mut c_char,
   pub query: *mut c_char,
   pub fragment: *mut c_char,
   pub hostent: *mut hostent,
   pub port: apr_port_t,
   pub _bindgen_bitfield_1_: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct apr_sockaddr_t {
   pub pool: *mut apr_pool_t,
   pub hostname: *mut c_char,
   pub servname: *mut c_char,
   pub port: apr_port_t,
   pub family: apr_int32_t,
   pub salen: apr_socklen_t,
   pub ipaddr_len: c_int,
   pub addr_str_len: c_int,
   pub ipaddr_ptr: *mut c_void,
   pub next: *mut apr_sockaddr_t,
   pub sa: apr_sockaddr_sa_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct apr_sockaddr_sa_t {
   pub _bindgen_data_: [u64; 16usize],
}
impl apr_sockaddr_sa_t {
   pub unsafe fn sin(&mut self) -> *mut sockaddr_in {
      let raw: *mut u8 = mem::transmute(&self._bindgen_data_);
      mem::transmute(raw.offset(0))
   }
   pub unsafe fn sin6(&mut self) -> *mut sockaddr_in6 {
      let raw: *mut u8 = mem::transmute(&self._bindgen_data_);
      mem::transmute(raw.offset(0))
   }
   pub unsafe fn sas(&mut self) -> *mut sockaddr_storage {
      let raw: *mut u8 = mem::transmute(&self._bindgen_data_);
      mem::transmute(raw.offset(0))
   }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct apr_finfo_t {
   pub pool: *mut apr_pool_t,
   pub valid: apr_int32_t,
   pub protection: apr_fileperms_t,
   pub filetype: apr_filetype_e,
   pub user: apr_uid_t,
   pub group: apr_gid_t,
   pub inode: apr_ino_t,
   pub device: apr_dev_t,
   pub nlink: apr_int32_t,
   pub size: apr_off_t,
   pub csize: apr_off_t,
   pub atime: apr_time_t,
   pub mtime: apr_time_t,
   pub ctime: apr_time_t,
   pub fname: *const c_char,
   pub name: *const c_char,
   pub filehand: *mut apr_file_t,
}

#[derive(Copy, Clone)]
pub enum hostent { }

#[derive(Copy, Clone)]
pub enum apr_bucket_alloc_t { }

#[derive(Copy, Clone)]
pub enum apr_pool_t { }

#[derive(Copy, Clone)]
pub enum apr_table_t { }

#[derive(Copy, Clone)]
pub enum apr_thread_mutex_t { }

#[derive(Copy, Clone)]
pub enum apr_thread_t { }

#[derive(Copy, Clone)]
pub enum apr_file_t { }

extern "C" {
   pub fn apr_version_string() -> *const c_char;
   pub fn apu_version_string() -> *const c_char;

   pub fn apr_table_get(t: *const apr_table_t, key: *const c_char) -> *const c_char;
   pub fn apr_table_set(t: *mut apr_table_t, key: *const c_char, val: *const c_char) -> ();
   pub fn apr_table_add(t: *mut apr_table_t, key: *const c_char, val: *const c_char) -> ();
   pub fn apr_table_do(f: *mut ap_table_do_callback_fb, r: *mut c_void, table: * const apr_table_t, ...) -> c_int;

   pub fn apr_table_elts(t: *const apr_table_t) -> *const apr_array_header_t;

   pub fn apr_pstrmemdup(p: *mut apr_pool_t, s: *const c_char, n: apr_size_t) -> *mut c_char;
   pub fn apr_palloc(p: *mut apr_pool_t, size: apr_size_t) -> *mut c_void;
   pub fn apr_pcalloc(p: *mut apr_pool_t, size: apr_size_t) -> *mut c_void;

   pub fn apr_base64_encode_len(len: c_int) -> c_int;
   pub fn apr_base64_encode(coded_dst: *mut c_char, plain_src: *const c_char, len_plain_src: c_int) -> c_int;
   pub fn apr_base64_decode_len(coded_src: *const c_char) -> c_int;
   pub fn apr_base64_decode(plain_dst: *mut c_char, coded_src: *const c_char) -> c_int;

   pub fn apr_time_now() -> apr_time_t;
   pub fn apr_rfc822_date(date_str: *mut c_char, t: apr_time_t) -> apr_status_t;
}

pub fn strdup<T: Into<Vec<u8>>>(pool: *mut apr_pool_t, data: T) -> *mut c_char {
   let bytes = data.into();

   unsafe {
      apr_pstrmemdup(pool,        bytes.as_ptr() as *const c_char,bytes.len() as apr_size_t)
   }
}

// APACHE HTTPD

pub const MODULE_MAGIC_COOKIE: c_ulong = 0x41503234u64; /* "AP24" */

pub const MODULE_MAGIC_NUMBER_MAJOR: c_int = 20120211;
pub const MODULE_MAGIC_NUMBER_MINOR: c_int = 36;

pub const OK:        c_int = 0;
pub const DECLINED:  c_int = -1;
pub const DONE:      c_int = -2;
pub const SUSPENDED: c_int = -3;

pub const HTTP_CONTINUE:                        c_int = 100;
pub const HTTP_SWITCHING_PROTOCOLS:             c_int = 101;
pub const HTTP_PROCESSING:                      c_int = 102;
pub const HTTP_OK:                              c_int = 200;
pub const HTTP_CREATED:                         c_int = 201;
pub const HTTP_ACCEPTED:                        c_int = 202;
pub const HTTP_NON_AUTHORITATIVE:               c_int = 203;
pub const HTTP_NO_CONTENT:                      c_int = 204;
pub const HTTP_RESET_CONTENT:                   c_int = 205;
pub const HTTP_PARTIAL_CONTENT:                 c_int = 206;
pub const HTTP_MULTI_STATUS:                    c_int = 207;
pub const HTTP_ALREADY_REPORTED:                c_int = 208;
pub const HTTP_IM_USED:                         c_int = 226;
pub const HTTP_MULTIPLE_CHOICES:                c_int = 300;
pub const HTTP_MOVED_PERMANENTLY:               c_int = 301;
pub const HTTP_MOVED_TEMPORARILY:               c_int = 302;
pub const HTTP_SEE_OTHER:                       c_int = 303;
pub const HTTP_NOT_MODIFIED:                    c_int = 304;
pub const HTTP_USE_PROXY:                       c_int = 305;
pub const HTTP_TEMPORARY_REDIRECT:              c_int = 307;
pub const HTTP_PERMANENT_REDIRECT:              c_int = 308;
pub const HTTP_BAD_REQUEST:                     c_int = 400;
pub const HTTP_UNAUTHORIZED:                    c_int = 401;
pub const HTTP_PAYMENT_REQUIRED:                c_int = 402;
pub const HTTP_FORBIDDEN:                       c_int = 403;
pub const HTTP_NOT_FOUND:                       c_int = 404;
pub const HTTP_METHOD_NOT_ALLOWED:              c_int = 405;
pub const HTTP_NOT_ACCEPTABLE:                  c_int = 406;
pub const HTTP_PROXY_AUTHENTICATION_REQUIRED:   c_int = 407;
pub const HTTP_REQUEST_TIME_OUT:                c_int = 408;
pub const HTTP_CONFLICT:                        c_int = 409;
pub const HTTP_GONE:                            c_int = 410;
pub const HTTP_LENGTH_REQUIRED:                 c_int = 411;
pub const HTTP_PRECONDITION_FAILED:             c_int = 412;
pub const HTTP_REQUEST_ENTITY_TOO_LARGE:        c_int = 413;
pub const HTTP_REQUEST_URI_TOO_LARGE:           c_int = 414;
pub const HTTP_UNSUPPORTED_MEDIA_TYPE:          c_int = 415;
pub const HTTP_RANGE_NOT_SATISFIABLE:           c_int = 416;
pub const HTTP_EXPECTATION_FAILED:              c_int = 417;
pub const HTTP_IM_A_TEAPOT:                     c_int = 418;
pub const HTTP_UNPROCESSABLE_ENTITY:            c_int = 422;
pub const HTTP_LOCKED:                          c_int = 423;
pub const HTTP_FAILED_DEPENDENCY:               c_int = 424;
pub const HTTP_UPGRADE_REQUIRED:                c_int = 426;
pub const HTTP_PRECONDITION_REQUIRED:           c_int = 428;
pub const HTTP_TOO_MANY_REQUESTS:               c_int = 429;
pub const HTTP_REQUEST_HEADER_FIELDS_TOO_LARGE: c_int = 431;
pub const HTTP_INTERNAL_SERVER_ERROR:           c_int = 500;
pub const HTTP_NOT_IMPLEMENTED:                 c_int = 501;
pub const HTTP_BAD_GATEWAY:                     c_int = 502;
pub const HTTP_SERVICE_UNAVAILABLE:             c_int = 503;
pub const HTTP_GATEWAY_TIME_OUT:                c_int = 504;
pub const HTTP_VERSION_NOT_SUPPORTED:           c_int = 505;
pub const HTTP_VARIANT_ALSO_VARIES:             c_int = 506;
pub const HTTP_INSUFFICIENT_STORAGE:            c_int = 507;
pub const HTTP_LOOP_DETECTED:                   c_int = 508;
pub const HTTP_NOT_EXTENDED:                    c_int = 510;
pub const HTTP_NETWORK_AUTHENTICATION_REQUIRED: c_int = 511;

pub const  PROXYREQ_NONE:     c_int = 0;
pub const  PROXYREQ_PROXY:    c_int = 1;
pub const  PROXYREQ_REVERSE:  c_int = 2;
pub const  PROXYREQ_RESPONSE: c_int = 3;

pub const RAW_ARGS:   c_uint = 0;
pub const TAKE1:      c_uint = 1;
pub const TAKE2:      c_uint = 2;
pub const ITERATE:    c_uint = 3;
pub const ITERATE2:   c_uint = 4;
pub const FLAG:       c_uint = 5;
pub const NO_ARGS:    c_uint = 6;
pub const TAKE12:     c_uint = 7;
pub const TAKE3:      c_uint = 8;
pub const TAKE23:     c_uint = 9;
pub const TAKE123:    c_uint = 10;
pub const TAKE13:     c_uint = 11;
pub const TAKE_ARGV:  c_uint = 12;

pub const OR_NONE:            c_int = 0;
pub const OR_LIMIT:           c_int = 1;
pub const OR_OPTIONS:         c_int = 2;
pub const OR_FILEINFO:        c_int = 4;
pub const OR_AUTHCFG:         c_int = 8;
pub const OR_INDEXES:         c_int = 16;
pub const OR_UNSET:           c_int = 32;
pub const ACCESS_CONF:        c_int = 64;
pub const RSRC_CONF:          c_int = 128;
pub const EXEC_ON_READ:       c_int = 256;
pub const NONFATAL_OVERRIDE:  c_int = 512;
pub const NONFATAL_UNKNOWN:   c_int = 1024;
pub const NONFATAL_ALL:       c_int = NONFATAL_OVERRIDE | NONFATAL_UNKNOWN;
pub const OR_ALL:             c_int = OR_LIMIT | OR_OPTIONS | OR_FILEINFO | OR_AUTHCFG | OR_INDEXES;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct request_rec {
   pub pool: *mut apr_pool_t,
   pub connection: *mut conn_rec,
   pub server: *mut server_rec,
   pub next: *mut request_rec,
   pub prev: *mut request_rec,
   pub main: *mut request_rec,
   pub the_request: *mut c_char,
   pub assbackwards: c_int,
   pub proxyreq: c_int,
   pub header_only: c_int,
   pub proto_num: c_int,
   pub protocol: *mut c_char,
   pub hostname: *const c_char,
   pub request_time: apr_time_t,
   pub status_line: *const c_char,
   pub status: c_int,
   pub method_number: c_int,
   pub method: *const c_char,
   pub allowed: apr_int64_t,
   pub allowed_xmethods: *mut apr_array_header_t,
   pub allowed_methods: *mut ap_method_list_t,
   pub sent_bodyct: apr_off_t,
   pub bytes_sent: apr_off_t,
   pub mtime: apr_time_t,
   pub range: *const c_char,
   pub clength: apr_off_t,
   pub chunked: c_int,
   pub read_body: c_int,
   pub read_chunked: c_int,
   pub expecting_100: c_uint,
   pub kept_body: *mut apr_bucket_brigade,
   pub body_table: *mut apr_table_t,
   pub remaining: apr_off_t,
   pub read_length: apr_off_t,
   pub headers_in: *mut apr_table_t,
   pub headers_out: *mut apr_table_t,
   pub err_headers_out: *mut apr_table_t,
   pub subprocess_env: *mut apr_table_t,
   pub notes: *mut apr_table_t,
   pub content_type: *const c_char,
   pub handler: *const c_char,
   pub content_encoding: *const c_char,
   pub content_languages: *mut apr_array_header_t,
   pub vlist_validator: *mut c_char,
   pub user: *mut c_char,
   pub ap_auth_type: *mut c_char,
   pub unparsed_uri: *mut c_char,
   pub uri: *mut c_char,
   pub filename: *mut c_char,
   pub canonical_filename: *mut c_char,
   pub path_info: *mut c_char,
   pub args: *mut c_char,
   pub used_path_info: c_int,
   pub eos_sent: c_int,
   pub per_dir_config: *mut ap_conf_vector_t,
   pub request_config: *mut ap_conf_vector_t,
   pub log: *const ap_logconf,
   pub log_id: *const c_char,
   pub htaccess: *const htaccess_result,
   pub output_filters: *mut ap_filter_t,
   pub input_filters: *mut ap_filter_t,
   pub proto_output_filters: *mut ap_filter_t,
   pub proto_input_filters: *mut ap_filter_t,
   pub no_cache: c_int,
   pub no_local_copy: c_int,
   pub invoke_mtx: *mut apr_thread_mutex_t,
   pub parsed_uri: apr_uri_t,
   pub finfo: apr_finfo_t,
   pub useragent_addr: *mut apr_sockaddr_t,
   pub useragent_ip: *mut c_char,
   pub trailers_in: *mut apr_table_t,
   pub trailers_out: *mut apr_table_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct conn_rec {
   pub pool: *mut apr_pool_t,
   pub base_server: *mut server_rec,
   pub vhost_lookup_data: *mut c_void,
   pub local_addr: *mut apr_sockaddr_t,
   pub client_addr: *mut apr_sockaddr_t,
   pub client_ip: *mut c_char,
   pub remote_host: *mut c_char,
   pub remote_logname: *mut c_char,
   pub local_ip: *mut c_char,
   pub local_host: *mut c_char,
   pub id: c_long,
   pub conn_config: *mut ap_conf_vector_t,
   pub notes: *mut apr_table_t,
   pub input_filters: *mut ap_filter_t,
   pub output_filters: *mut ap_filter_t,
   pub sbh: *mut c_void,
   pub bucket_alloc: *mut apr_bucket_alloc_t,
   pub cs: *mut conn_state_t,
   pub data_in_input_filters: c_int,
   pub data_in_output_filters: c_int,
   pub _bindgen_bitfield_1_: c_uint,
   pub _bindgen_bitfield_2_: c_int,
   pub aborted: c_uint,
   pub keepalive: ap_conn_keepalive_e,
   pub keepalives: c_int,
   pub log: *const ap_logconf,
   pub log_id: *const c_char,
   pub current_thread: *mut apr_thread_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct server_rec {
   pub process: *mut process_rec,
   pub next: *mut server_rec,
   pub error_fname: *mut c_char,
   pub error_log: *mut apr_file_t,
   pub log: ap_logconf,
   pub module_config: *mut ap_conf_vector_t,
   pub lookup_defaults: *mut ap_conf_vector_t,
   pub defn_name: *const c_char,
   pub defn_line_number: c_uint,
   pub is_virtual: c_char,
   pub port: apr_port_t,
   pub server_scheme: *const c_char,
   pub server_admin: *mut c_char,
   pub server_hostname: *mut c_char,
   pub addrs: *mut server_addr_rec,
   pub timeout: apr_interval_time_t,
   pub keep_alive_timeout: apr_interval_time_t,
   pub keep_alive_max: c_int,
   pub keep_alive: c_int,
   pub names: *mut apr_array_header_t,
   pub wild_names: *mut apr_array_header_t,
   pub path: *const c_char,
   pub pathlen: c_int,
   pub limit_req_line: c_int,
   pub limit_req_fieldsize: c_int,
   pub limit_req_fields: c_int,
   pub context: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ap_logconf {
   pub module_levels: *mut c_char,
   pub level: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct module {
   pub version: c_int,
   pub minor_version: c_int,
   pub module_index: c_int,
   pub name: *const c_char,
   pub dynamic_load_handle: *mut c_void,
   pub next: *mut module,
   pub magic: c_ulong,
   pub rewrite_args: Option<rewrite_args_fn>,
   pub create_dir_config: Option<create_dir_config_fn>,
   pub merge_dir_config: Option<merge_config_fn>,
   pub create_server_config: Option<create_server_config_fn>,
   pub merge_server_config: Option<merge_config_fn>,
   pub cmds: *const command_rec,
   pub register_hooks: Option<register_hooks_fn>
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_func {
   pub _bindgen_data_: [u64; 1usize],
}
impl cmd_func {
   pub unsafe fn no_args(&mut self) -> *mut Option<no_args_fn> {
      mem::transmute(&self._bindgen_data_)
   }

   pub unsafe fn raw_args(&mut self) -> *mut Option<raw_args_fn> {
      mem::transmute(&self._bindgen_data_)
   }

   pub unsafe fn take_argv(&mut self) -> *mut Option<take_argv_fn> {
      mem::transmute(&self._bindgen_data_)
   }

   pub unsafe fn take1(&mut self) -> *mut Option<take1_fn> {
      mem::transmute(&self._bindgen_data_)
   }

   pub unsafe fn take2(&mut self) -> *mut Option<take2_fn> {
      mem::transmute(&self._bindgen_data_)
   }

   pub unsafe fn take3(&mut self) -> *mut Option<take3_fn> {
      mem::transmute(&self._bindgen_data_)
   }

   pub unsafe fn flag(&mut self) -> *mut Option<flag_fn> {
      mem::transmute(&self._bindgen_data_)
   }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct command_rec {
   pub name: *const c_char,
   pub func: cmd_func2,
   pub cmd_data: *mut c_void,
   pub req_override: c_int,
   pub args_how: cmd_how,
   pub errmsg: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cmd_func2 {
   /// function to call for a no-args
   pub no_args: Option<unsafe extern "C" fn(parms: *mut cmd_parms, mconfig: *mut c_void) -> *const c_char>,
   /// function to call for a raw-args
   pub raw_args: Option<unsafe extern "C" fn(parms: *mut cmd_parms, mconfig: *mut c_void, args: *const c_char) -> *const c_char>,
   /// function to call for a argv/argc
   pub take_argv: Option<unsafe extern "C" fn(parms: *mut cmd_parms, mconfig: *mut c_void, argc: c_int, argv: *const *const c_char) -> *const c_char>,
   /// function to call for a take1
   pub take1: Option<unsafe extern "C" fn(parms: *mut cmd_parms, mconfig: *mut c_void, w: *const c_char) -> *const c_char>,
   /// function to call for a take2
   pub take2: Option<unsafe extern "C" fn(parms: *mut cmd_parms, mconfig: *mut c_void, w: *const c_char, w2: *const c_char) -> *const c_char>,
   /// function to call for a take3
   pub take3: Option<unsafe extern "C" fn(parms: *mut cmd_parms, mconfig: *mut c_void, w: *const c_char, w2: *const c_char, w3: *const c_char) -> *const c_char>,
   /// function to call for a flag
   pub flag: Option<unsafe extern "C" fn(parms: *mut cmd_parms, mconfig: *mut c_void, on: c_int) -> *const c_char>,
   _bindgen_union_align: u64,
}


#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_parms {
   pub info: *mut c_void,
   pub _override: c_int,
   pub override_opts: c_int,
   pub override_list: *mut apr_table_t,
   pub limited: apr_int64_t,
   pub limited_xmethods: *mut apr_array_header_t,
   pub xlimited: *mut ap_method_list_t,
   pub config_file: *mut ap_configfile_t,
   pub directive: *mut ap_directive_t,
   pub pool: *mut apr_pool_t,
   pub temp_pool: *mut apr_pool_t,
   pub server: *mut server_rec,
   pub path: *mut c_char,
   pub cmd: *const command_rec,
   pub context: *mut ap_conf_vector_t,
   pub err_directive: *const ap_directive_t,
}

//#[repr(C)]
//#[derive(Copy, Clone)]
//pub struct ap_list_provider_names_t {
//   pub provider_name: *const c_char,
//}

//#[repr(C)]
//#[derive(Copy, Clone)]
//pub struct ap_list_provider_groups_t {
//   pub provider_group: *const c_char,
//   pub provider_version: *const c_char,
//}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ap_method_list_t {
   pub method_mask: apr_int64_t,
   pub method_list: *mut apr_array_header_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ap_configfile_t {
   pub getch: Option<extern "C" fn(
      ch: *mut c_char,
      param: *mut c_void
   ) -> apr_status_t>,

   pub getstr: Option<extern "C" fn(
      buf: *mut c_void,
      bufsiz: apr_size_t,
      param: *mut c_void
   ) -> apr_status_t>,

   pub close: Option<extern "C" fn(param: *mut c_void) -> apr_status_t>,

   pub param: *mut c_void,
   pub name: *const c_char,
   pub line_number: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ap_directive_t {
   pub directive: *const c_char,
   pub args: *const c_char,
   pub next: *mut ap_directive_t,
   pub first_child: *mut ap_directive_t,
   pub parent: *mut ap_directive_t,
   pub data: *mut c_void,
   pub filename: *const c_char,
   pub line_num: c_int,
   pub last: *mut ap_directive_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htaccess_result {
   pub dir: *const c_char,
   pub _override: c_int,
   pub override_opts: c_int,
   pub override_list: *mut apr_table_t,
   pub htaccess: *mut ap_conf_vector_t,
   pub next: *const htaccess_result,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct process_rec {
   pub pool: *mut apr_pool_t,
   pub pconf: *mut apr_pool_t,
   pub short_name: *const c_char,
   pub argv: *const *const c_char,
   pub argc: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct server_addr_rec {
   pub next: *mut server_addr_rec,
   pub virthost: *mut c_char,
   pub host_addr: *mut apr_sockaddr_t,
   pub host_port: apr_port_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ap_filter_t {
   pub frec: *mut ap_filter_rec_t,
   pub ctx: *mut c_void,
   pub next: *mut ap_filter_t,
   pub r: *mut request_rec,
   pub c: *mut conn_rec,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ap_filter_rec_t {
   pub name: *const c_char,
   pub filter_func: ap_filter_func,
   pub filter_init_func: Option<ap_init_filter_func>,
   pub next: *mut ap_filter_rec_t,
   pub providers: *mut ap_filter_provider_t,
   pub ftype: ap_filter_type,
   pub debug: c_int,
   pub proto_flags: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ap_filter_func {
   pub _bindgen_data_: [u64; 1usize],
}
impl ap_filter_func {
   pub unsafe fn out_func(&mut self) -> *mut Option<ap_out_filter_func> {
      let raw: *mut u8 = mem::transmute(&self._bindgen_data_);
      mem::transmute(raw.offset(0))
   }
   pub unsafe fn in_func(&mut self) -> *mut Option<ap_in_filter_func> {
      let raw: *mut u8 = mem::transmute(&self._bindgen_data_);
      mem::transmute(raw.offset(0))
   }
}

#[derive(Copy, Clone)]
pub enum ap_conf_vector_t { }

#[derive(Copy, Clone)]
pub enum ap_filter_provider_t { }

pub type cmd_how = c_uint;

pub type ap_conn_keepalive_e = c_uint;

pub type ap_filter_type = c_uint;

pub type ap_input_mode_t = c_uint;

pub type ap_init_filter_func = extern "C" fn(f: *mut ap_filter_t) -> apr_status_t;

pub type ap_out_filter_func = extern "C" fn(
   f: *mut ap_filter_t,
   b: *mut apr_bucket_brigade
) -> apr_status_t;

pub type ap_in_filter_func = extern "C" fn(
   f: *mut ap_filter_t,
   b: *mut apr_bucket_brigade,
   mode: ap_input_mode_t,
   block: apr_read_type_e,
   readbytes: apr_off_t
) -> apr_status_t;

pub type rewrite_args_fn = extern "C" fn(
   process: *mut process_rec
);

pub type create_dir_config_fn = extern "C" fn(p: *mut apr_pool_t, dir: *mut c_char) -> *mut c_void;
pub type merge_config_fn = extern "C" fn(p: *mut apr_pool_t, base_conf: *mut c_void, new_conf: *mut c_void) -> *mut c_void;
pub type create_server_config_fn = extern "C" fn(p: *mut apr_pool_t, s: *mut server_rec) -> *mut c_void;

pub type register_hooks_fn = extern "C" fn(p: *mut apr_pool_t);

pub type no_args_fn = extern "C" fn(parms: *mut cmd_parms, mconfig: *mut c_void) -> *const c_char;
pub type raw_args_fn = extern "C" fn(parms: *mut cmd_parms, mconfig: *mut c_void, args: *const c_char) -> *const c_char;
pub type take_argv_fn = extern "C" fn(parms: *mut cmd_parms, mconfig: *mut c_void, argc: c_int, argv: *const *mut c_char) -> *const c_char;
pub type take1_fn = extern "C" fn(parms: *mut cmd_parms, mconfig: *mut c_void, w: *const c_char) -> *const c_char;
pub type take2_fn = extern "C" fn(parms: *mut cmd_parms, mconfig: *mut c_void, w: *const c_char, w2: *const c_char) -> *const c_char;
pub type take3_fn = extern "C" fn(parms: *mut cmd_parms, mconfig: *mut c_void, w: *const c_char, w2: *const c_char, w3: *const c_char) -> *const c_char;
pub type flag_fn = extern "C" fn(parms: *mut cmd_parms, mconfig: *mut c_void, on: c_int) -> *const c_char;

pub type hook_handler_fn = extern "C" fn(r: *mut request_rec) -> c_int;
pub type hook_pre_config_fn = extern "C" fn(conf: *mut apr_pool_t, log: *mut apr_pool_t, temp: *mut apr_pool_t) -> c_int;
pub type hook_check_config_fn = extern "C" fn(conf: *mut apr_pool_t, log: *mut apr_pool_t, temp: *mut apr_pool_t, s: *mut server_rec) -> c_int;
pub type hook_test_config_fn = extern "C" fn(conf: *mut apr_pool_t, s: *mut server_rec) -> c_int;
pub type hook_post_config_fn = extern "C" fn(conf: *mut apr_pool_t, log: *mut apr_pool_t, temp: *mut apr_pool_t, s: *mut server_rec) -> c_int;
pub type hook_child_init_fn = extern "C" fn(p: *mut apr_pool_t, s: *mut server_rec);
pub type hook_insert_output_filter_fn = extern "C" fn(r: *mut request_rec);

pub type ap_table_do_callback_fb = extern "C" fn(r: *mut request_rec, key: *const c_char, value: *const c_char) -> c_int;

extern "C" {
   #[no_mangle]
   #[allow(non_upper_case_globals)]
   pub static ap_server_root: *const c_char;
}

extern "C" {
   pub fn ap_get_server_banner() -> *const c_char;
   pub fn ap_get_server_description() -> *const c_char;
   pub fn ap_get_server_built() -> *const c_char;

   pub fn ap_show_mpm() -> *const c_char;

   pub fn ap_escape_html2(p: *mut apr_pool_t, s: *const c_char, toasc: c_int) -> *mut c_char;

   pub fn ap_rwrite(buf: *const c_void, nbyte: c_int, r: *const request_rec) -> c_int;
   pub fn ap_set_content_type(r: *const request_rec, ct: *const c_char) -> ();
   pub fn ap_get_basic_auth_pw(r: *const request_rec, pw: *mut *const c_char) -> c_int;

   //pub fn ap_context_document_root(r: *const request_rec) -> *const c_char;
   //pub fn ap_context_prefix(r: *const request_rec) -> *const c_char;

   pub fn ap_run_http_scheme(r: *const request_rec) -> *const c_char;
   pub fn ap_run_default_port(r: *const request_rec) -> apr_port_t;

   pub fn ap_is_initial_req(r: *const request_rec) -> c_int;

   pub fn ap_some_auth_required(r: *const request_rec) -> c_int;

   //pub fn ap_cookie_read(r: *const request_rec, name: *const c_char, val: *mut *const c_char, remove: c_int) -> apr_status_t;
   //pub fn ap_cookie_write(r: *const request_rec, name: *const c_char, val: *const c_char, attrs: *const c_char, maxage: c_int, ...) -> apr_status_t;

   //pub fn ap_escape_urlencoded(p: *mut apr_pool_t, s: *const c_char) -> *mut c_char;
   //pub fn ap_unescape_urlencoded(query: *mut c_char) -> c_int;

   pub fn ap_document_root(r: *const request_rec) -> *const c_char;
   pub fn ap_get_server_name(r: *const request_rec) -> *const c_char;
   pub fn ap_get_server_port(r: *const request_rec) -> apr_port_t;
   pub fn ap_auth_name(r: *const request_rec) -> *const c_char;

   pub fn ap_set_last_modified(r: *mut request_rec) -> ();
   pub fn ap_update_mtime(r: *mut request_rec, dependency_mtime: apr_time_t) -> ();

   pub fn ap_get_module_config(cv: *const ap_conf_vector_t, m: *const module) -> *mut c_void;
   pub fn ap_set_module_config(cv: *mut ap_conf_vector_t, m: *const module, val: *mut c_void) -> ();

   pub fn ap_register_provider(pool: *mut apr_pool_t, provider_group: *const c_char, provider_name: *const c_char, provider_version: *const c_char, provider: *const c_void) -> apr_status_t;
   pub fn ap_lookup_provider(provider_group: *const c_char, provider_name: *const c_char, provider_version: *const c_char) -> *mut c_void;
//   pub fn ap_list_provider_names(pool: *mut apr_pool_t, provider_group: *const c_char, provider_version: *const c_char) -> *mut apr_array_header_t;
//   pub fn ap_list_provider_groups(pool: *mut apr_pool_t) -> *mut apr_array_header_t;

   pub fn ap_hook_handler(f: Option<hook_handler_fn>, pre: *const *const c_char, succ: *const *const c_char, order: c_int);
   pub fn ap_hook_pre_config(f: Option<hook_pre_config_fn>, pre: *const *const c_char, succ: *const *const c_char, order: c_int);
//   pub fn ap_hook_check_config(f: Option<hook_check_config_fn>, pre: *const *const c_char, succ: *const *const c_char, order: c_int);
   pub fn ap_hook_test_config(f: Option<hook_test_config_fn>, pre: *const *const c_char, succ: *const *const c_char, order: c_int);
   pub fn ap_hook_post_config(        f: Option<hook_post_config_fn>, pre: *const *const c_char, succ: *const *const c_char, order: c_int);
   pub fn ap_hook_create_request(     f: Option<hook_handler_fn>, pre: *const *const c_char, succ: *const *const c_char, order: c_int);
   pub fn ap_hook_translate_name(     f: Option<hook_handler_fn>, pre: *const *const c_char, succ: *const *const c_char, order: c_int);
   pub fn ap_hook_map_to_storage(     f: Option<hook_handler_fn>, pre: *const *const c_char, succ: *const *const c_char, order: c_int);
   pub fn ap_hook_check_user_id(      f: Option<hook_handler_fn>, pre: *const *const c_char, succ: *const *const c_char, order: c_int);
   pub fn ap_hook_fixups(             f: Option<hook_handler_fn>, pre: *const *const c_char, succ: *const *const c_char, order: c_int);
   pub fn ap_hook_type_checker(       f: Option<hook_handler_fn>, pre: *const *const c_char, succ: *const *const c_char, order: c_int);
   pub fn ap_hook_access_checker(     f: Option<hook_handler_fn>, pre: *const *const c_char, succ: *const *const c_char, order: c_int);
   pub fn ap_hook_access_checker_ex(  f: Option<hook_handler_fn>, pre: *const *const c_char, succ: *const *const c_char, order: c_int);
   pub fn ap_hook_auth_checker(       f: Option<hook_handler_fn>, pre: *const *const c_char, succ: *const *const c_char, order: c_int);
   pub fn ap_hook_log_transaction(f: Option<hook_handler_fn>, pre: *const *const c_char, succ: *const *const c_char, order: c_int);
   pub fn ap_hook_child_init(         f: Option<hook_child_init_fn>, pre: *const *const c_char, succ: *const *const c_char, order: c_int);

   pub fn ap_register_output_filter(name: *const c_char, filter_fn: Option<ap_out_filter_func>, init_fn: Option<ap_init_filter_func>, filter_type: ap_filter_type) -> *const ap_filter_rec_t;
   pub fn ap_hook_insert_filter      (f: Option<hook_insert_output_filter_fn>, pre: *const *const c_char, succ: *const *const c_char, order: c_int);
   pub fn ap_hook_insert_error_filter(f: Option<hook_insert_output_filter_fn>, pre: *const *const c_char, succ: *const *const c_char, order: c_int);

   pub fn ap_add_output_filter(name: *const c_char, ctx: *const c_void, r: *mut request_rec, c: *mut conn_rec) -> *mut ap_filter_t;
   pub fn ap_pass_brigade(next: *mut ap_filter_t, bb: *mut apr_bucket_brigade) -> apr_status_t;
//(ap_filter_t *) ap_add_output_filter(const char *name, void *ctx,
//request_rec *r, conn_rec *c)
}

pub fn apr_table_elts_local(t: *const apr_table_t) -> *const apr_array_header_t {
   unsafe {
      ::std::mem::transmute(t)
   }
}
