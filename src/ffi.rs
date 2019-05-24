#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]


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

/// number of microseconds since 00:00:00 january 1, 1970 UTC
pub type apr_time_t = apr_int64_t;
/// intervals for I/O timeouts, in microseconds
pub type apr_interval_time_t = apr_int64_t;
/// short interval for I/O timeouts, in microseconds
pub type apr_short_interval_time_t = apr_int32_t;
/// @remark use apr_uint16_t just in case some system has a short that isn't 16 bits...
pub type apr_port_t = apr_uint16_t;

/// An opaque array type
#[cfg(feature = "apache22")]
pub use ffi22::apr_array_header_t;
#[cfg(not(feature = "apache22"))]
pub use ffi24::apr_array_header_t;

/// The type for each entry in a string-content table
#[cfg(feature = "apache22")]
pub use ffi22::apr_table_entry_t;
#[cfg(not(feature = "apache22"))]
pub use ffi24::apr_table_entry_t;

/// A list of buckets
#[cfg(feature = "apache22")]
pub use ffi22::apr_bucket_brigade;
#[cfg(not(feature = "apache22"))]
pub use ffi24::apr_bucket_brigade;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct apr_bucket_list {
   pub next: *mut apr_bucket,
   pub prev: *mut apr_bucket,
}

#[cfg(feature = "apache22")]
pub use ffi22::apr_bucket;
#[cfg(not(feature = "apache22"))]
pub use ffi24::apr_bucket;

#[cfg(feature = "apache22")]
pub use ffi22::apr_bucket_type_t;
#[cfg(not(feature = "apache22"))]
pub use ffi24::apr_bucket_type_t;

#[cfg(feature = "apache22")]
pub use ffi22::apr_uri_t;
#[cfg(not(feature = "apache22"))]
pub use ffi24::apr_uri_t;

#[cfg(feature = "apache22")]
pub use ffi22::apr_sockaddr_t;
#[cfg(not(feature = "apache22"))]
pub use ffi24::apr_sockaddr_t;

#[cfg(feature = "apache22")]
pub use ffi22::apr_sockaddr_sa_t;
#[cfg(not(feature = "apache22"))]
pub use ffi24::apr_sockaddr_sa_t;

#[cfg(feature = "apache22")]
pub use ffi22::apr_finfo_t;
#[cfg(not(feature = "apache22"))]
pub use ffi24::apr_finfo_t;

#[cfg(feature = "apache22")]
pub use ffi22::hostent;
#[cfg(not(feature = "apache22"))]
pub use ffi24::hostent;

#[cfg(feature = "apache22")]
pub use ffi22::apr_bucket_alloc_t;
#[cfg(not(feature = "apache22"))]
pub use ffi24::apr_bucket_alloc_t;

#[cfg(feature = "apache22")]
pub use ffi22::apr_pool_t;
#[cfg(not(feature = "apache22"))]
pub use ffi24::apr_pool_t;

#[cfg(feature = "apache22")]
pub use ffi22::apr_table_t;
#[cfg(not(feature = "apache22"))]
pub use ffi24::apr_table_t;

#[cfg(feature = "apache22")]
pub use ffi22::apr_thread_mutex_t;
#[cfg(not(feature = "apache22"))]
pub use ffi24::apr_thread_mutex_t;

#[cfg(feature = "apache22")]
pub use ffi22::apr_thread_t;
#[cfg(not(feature = "apache22"))]
pub use ffi24::apr_thread_t;


extern "C" {
    pub fn apr_version_string() -> *const c_char;
    pub fn apu_version_string() -> *const c_char;

    pub fn apr_table_get(t: *const apr_table_t, key: *const c_char) -> *const c_char;
    pub fn apr_table_set(t: *mut apr_table_t, key: *const c_char, val: *const c_char) -> ();
    pub fn apr_table_add(t: *mut apr_table_t, key: *const c_char, val: *const c_char) -> ();
    pub fn apr_table_do(f: *mut ap_table_do_callback_fb, r: *mut c_void, table: *const apr_table_t, ...) -> c_int;

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
    pub fn ap_die(status_type: c_int, r: *const request_rec);
}

pub fn strdup<T: Into<Vec<u8>>>(pool: *mut apr_pool_t, data: T) -> *mut c_char {
   let bytes = data.into();

   unsafe {
      apr_pstrmemdup(pool,        bytes.as_ptr() as *const c_char,bytes.len() as apr_size_t)
   }
}

// APACHE HTTPD
#[cfg(feature = "apache22")]
pub const MODULE_MAGIC_COOKIE: c_ulong = ::ffi22::MODULE_MAGIC_COOKIE; /* "AP22" */
#[cfg(feature = "apache22")]
pub const MODULE_MAGIC_NUMBER_MAJOR: c_int = ::ffi22::MODULE_MAGIC_NUMBER_MAJOR;
#[cfg(feature = "apache22")]
pub const MODULE_MAGIC_NUMBER_MINOR: c_int = ::ffi22::MODULE_MAGIC_NUMBER_MINOR;

#[cfg(not(feature = "apache22"))]
pub const MODULE_MAGIC_COOKIE: c_ulong  = ::ffi24::MODULE_MAGIC_COOKIE; /* "AP24" */
#[cfg(not(feature = "apache22"))]
pub const MODULE_MAGIC_NUMBER_MAJOR: c_int = ::ffi24::MODULE_MAGIC_NUMBER_MAJOR;
#[cfg(not(feature = "apache22"))]
pub const MODULE_MAGIC_NUMBER_MINOR: c_int = ::ffi24::MODULE_MAGIC_NUMBER_MINOR;

#[cfg(feature = "apache22")]
pub const APR_BUCKET_BUFF_SIZE: u32 = ::ffi22::APR_BUCKET_BUFF_SIZE;
#[cfg(not(feature = "apache22"))]
pub const APR_BUCKET_BUFF_SIZE: u32 = ::ffi24::APR_BUCKET_BUFF_SIZE;

pub const OK:        c_int = 0;
pub const DECLINED:  c_int = -1;
pub const DONE:      c_int = -2;
pub const SUSPENDED: c_int = -3;

#[cfg(feature = "apache22")]
pub const APR_SUCCESS: c_int = ::ffi22::APR_SUCCESS;
#[cfg(not(feature = "apache22"))]
pub const APR_SUCCESS: c_int = ::ffi24::APR_SUCCESS;

#[cfg(feature = "apache22")]
pub const AP_FILTER_ERROR: c_int = ::ffi22::AP_FILTER_ERROR;
#[cfg(not(feature = "apache22"))]
pub const AP_FILTER_ERROR: c_int = ::ffi24::AP_FILTER_ERROR;


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

#[cfg(feature = "apache22")]
pub use ffi22::request_rec;
#[cfg(not(feature = "apache22"))]
pub use ffi24::request_rec;

#[cfg(feature = "apache22")]
pub use ffi22::conn_rec;
#[cfg(not(feature = "apache22"))]
pub use ffi24::conn_rec;

#[cfg(feature = "apache22")]
pub use ffi22::server_rec;
#[cfg(not(feature = "apache22"))]
pub use ffi24::server_rec;

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

#[cfg(feature = "apache22")]
pub use ffi22::cmd_parms;
#[cfg(not(feature = "apache22"))]
pub use ffi24::cmd_parms;


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

#[cfg(feature = "apache22")]
pub use ffi22::ap_method_list_t;
#[cfg(not(feature = "apache22"))]
pub use ffi24::ap_method_list_t;

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
pub struct apr_file_t {
   pub pool: *mut apr_pool_t,
   pub filedes: c_int,
   pub fname: *mut c_char,
   pub flags: apr_int32_t,
   pub eof_hit: c_int,
   pub is_pipe: c_int,
   pub timeout: apr_interval_time_t,
   pub buffered: c_int,
   pub blocking: u32,
   pub ungetchar: c_int,
   pub buffer: *mut c_char,
   pub bufpos: apr_size_t,
   pub bufsize: apr_size_t,
   pub dataRead: c_ulong,
   pub direction: c_int,
   pub filePtr: apr_off_t,
   pub thlock: *mut apr_thread_mutex_t,
}


#[cfg(feature = "apache22")]
pub use ffi22::process_rec;
#[cfg(not(feature = "apache22"))]
pub use ffi24::process_rec;

#[cfg(feature = "apache22")]
pub use ffi22::server_addr_rec;
#[cfg(not(feature = "apache22"))]
pub use ffi24::server_addr_rec;

#[cfg(feature = "apache22")]
pub use ffi22::ap_filter_t;
#[cfg(not(feature = "apache22"))]
pub use ffi24::ap_filter_t;

#[cfg(feature = "apache22")]
pub use ffi22::ap_filter_rec_t;
#[cfg(not(feature = "apache22"))]
pub use ffi24::ap_filter_rec_t;

#[cfg(feature = "apache22")]
pub use ffi22::ap_filter_func;
#[cfg(not(feature = "apache22"))]
pub use ffi24::ap_filter_func;

#[cfg(feature = "apache22")]
pub use ffi22::ap_out_filter_func;
#[cfg(not(feature = "apache22"))]
pub use ffi24::ap_out_filter_func;

#[cfg(feature = "apache22")]
pub use ffi22::ap_in_filter_func;
#[cfg(not(feature = "apache22"))]
pub use ffi24::ap_in_filter_func;

#[cfg(feature = "apache22")]
pub use ffi22::ap_init_filter_func;
#[cfg(not(feature = "apache22"))]
pub use ffi24::ap_init_filter_func;



//#[repr(C)]
//#[derive(Copy, Clone)]
//pub struct ap_filter_func {
//   pub _bindgen_data_: [u64; 1usize],
//}
//impl ap_filter_func {
//   pub unsafe fn out_func(&mut self) -> *mut Option<ap_out_filter_func> {
//      let raw: *mut u8 = mem::transmute(&self._bindgen_data_);
//      mem::transmute(raw.offset(0))
//   }
//   pub unsafe fn in_func(&mut self) -> *mut Option<ap_in_filter_func> {
//      let raw: *mut u8 = mem::transmute(&self._bindgen_data_);
//      mem::transmute(raw.offset(0))
//   }
//}

/// the configuration directives
#[cfg(feature = "apache22")]
pub use ffi22::ap_conf_vector_t;
#[cfg(not(feature = "apache22"))]
pub use ffi24::ap_conf_vector_t;

#[derive(Copy, Clone)]
pub enum ap_filter_provider_t { }

pub type cmd_how = c_uint;

pub type ap_conn_keepalive_e = c_uint;

pub type ap_filter_type = c_uint;

pub type ap_input_mode_t = c_uint;

//pub type ap_init_filter_func = extern "C" fn(f: *mut ap_filter_t) -> apr_status_t;
//
//pub type ap_out_filter_func = extern "C" fn(
//   f: *mut ap_filter_t,
//   b: *mut apr_bucket_brigade
//) -> apr_status_t;
//
//pub type ap_in_filter_func = extern "C" fn(
//   f: *mut ap_filter_t,
//   b: *mut apr_bucket_brigade,
//   mode: ap_input_mode_t,
//   block: apr_read_type_e,
//   readbytes: apr_off_t
//) -> apr_status_t;

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
pub type hook_open_logs_fn = extern "C" fn(pconf: *mut apr_pool_t, plog: *mut apr_pool_t, ptemp: *mut apr_pool_t, s: *mut server_rec) -> c_int;
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

   pub fn ap_hook_post_read_request(f: Option<hook_handler_fn>, pre: *const *const c_char, succ: *const *const c_char, order: c_int);
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
   pub fn ap_hook_log_transaction(    f: Option<hook_handler_fn>, pre: *const *const c_char, succ: *const *const c_char, order: c_int);
   pub fn ap_hook_child_init(         f: Option<hook_child_init_fn>, pre: *const *const c_char, succ: *const *const c_char, order: c_int);
   pub fn ap_hook_open_logs(          f: Option<hook_open_logs_fn>, pre: *const *const c_char, succ: *const *const c_char, order: c_int);



   //todo:  this might have gotten messed up from the bindgen conversion of function args.  Look at the old param types
   pub fn ap_register_output_filter(name: *const c_char, filter_fn: ap_out_filter_func, init_fn: ap_init_filter_func, filter_type: ap_filter_type) -> *const ap_filter_rec_t;
   pub fn ap_register_input_filter(name: *const c_char, filter_fn: ap_in_filter_func, init_fn: ap_init_filter_func, filter_type: ap_filter_type) -> *const ap_filter_rec_t;

   pub fn ap_hook_insert_filter      (f: Option<hook_insert_output_filter_fn>, pre: *const *const c_char, succ: *const *const c_char, order: c_int);
   pub fn ap_hook_insert_error_filter(f: Option<hook_insert_output_filter_fn>, pre: *const *const c_char, succ: *const *const c_char, order: c_int);

   pub fn ap_add_output_filter(name: *const c_char, ctx: *const c_void, r: *mut request_rec, c: *mut conn_rec) -> *mut ap_filter_t;
   pub fn ap_remove_output_filter(f: *mut ap_filter_t);
   pub fn ap_add_input_filter(name: *const c_char, ctx: *const c_void, r: *mut request_rec, c: *mut conn_rec) -> *mut ap_filter_t;
   pub fn ap_pass_brigade(next: *mut ap_filter_t, bb: *mut apr_bucket_brigade) -> apr_status_t;
//(ap_filter_t *) ap_add_output_filter(const char *name, void *ctx,
//request_rec *r, conn_rec *c)
}

pub fn apr_table_elts_local(t: *const apr_table_t) -> *const apr_array_header_t {
   unsafe {
      ::std::mem::transmute(t)
   }
}

#[cfg(test)]
mod test {
   use super::*;

   #[test]
   fn bindgen_test_layout_apr_array_header_t() {
      assert_eq!(::std::mem::size_of::<apr_array_header_t>(), 32usize, concat!( "Size of: " , stringify ! ( apr_array_header_t ) ));
      assert_eq!(::std::mem::align_of::<apr_array_header_t>(), 8usize, concat!( "Alignment of " , stringify ! ( apr_array_header_t ) ));
      assert_eq!(unsafe { &(*(::std::ptr::null::<apr_array_header_t>())).pool as *const _ as usize }, 0usize, concat!( "Offset of field: " , stringify ! ( apr_array_header_t ) , "::" , stringify ! ( pool ) ));
      assert_eq!(unsafe { &(*(::std::ptr::null::<apr_array_header_t>())).elt_size as *const _ as usize }, 8usize, concat!( "Offset of field: " , stringify ! ( apr_array_header_t ) , "::" , stringify ! ( elt_size ) ));
      assert_eq!(unsafe { &(*(::std::ptr::null::<apr_array_header_t>())).nelts as *const _ as usize }, 12usize, concat!( "Offset of field: " , stringify ! ( apr_array_header_t ) , "::" , stringify ! ( nelts ) ));
      assert_eq!(unsafe { &(*(::std::ptr::null::<apr_array_header_t>())).nalloc as *const _ as usize }, 16usize, concat!( "Offset of field: " , stringify ! ( apr_array_header_t ) , "::" , stringify ! ( nalloc ) ));
      assert_eq!(unsafe { &(*(::std::ptr::null::<apr_array_header_t>())).elts as *const _ as usize }, 24usize, concat!( "Offset of field: " , stringify ! ( apr_array_header_t ) , "::" , stringify ! ( elts ) ));
   }

   #[test]
   fn test_magic() {
      println!("MAGIC {}", MODULE_MAGIC_COOKIE);
   }
}
