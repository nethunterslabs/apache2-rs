
pub mod raw {
   use libc::{c_int, c_char, c_uint};

   use apr::raw::{apr_pool_t, apr_time_t, apr_array_header_t, apr_off_t, apr_thread_mutex_t,
      apr_int64_t, apr_table_t, apr_bucket_brigade, apr_uri_t, apr_finfo_t, apr_sockaddr_t};

   use util_filter::raw::{ap_filter_t};

   pub const OK:        c_int = 0;
   pub const DECLINED:  c_int = -1;
   pub const DONE:      c_int = -2;
   pub const SUSPENDED: c_int = -3;

   #[repr(C)]
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
   pub struct ap_logconf {
       pub module_levels: *mut c_char,
       pub level: c_int,
   }

   #[repr(C)]
   pub struct ap_conf_vector_t;

   #[repr(C)]
   pub struct ap_method_list_t;

   #[repr(C)]
   pub struct conn_rec;

   #[repr(C)]
   pub struct htaccess_result;

   #[repr(C)]
   pub struct process_rec;

   #[repr(C)]
   pub struct server_rec;

}

struct Wrapper<'a, T: 'a> {
   raw: &'a mut T
}

impl<'a, T> Wrapper<'a, T> {
   pub fn from_raw_ptr(ptr: *mut T) -> Option<Self> {
      if ptr.is_null() {
         None
      } else {
         let raw: &mut T = unsafe { &mut *ptr };
         Some(
            Wrapper::<T> {
               raw: raw
            }
         )
      }
   }
}

pub type Request<'a> = Wrapper<'a, raw::request_rec>;

use std::str;
use std::ffi::CStr;

macro_rules! str_getter {
   ( $field:ident ) => {
      pub fn $field(&self) -> Option<&'a str> {
         let ptr = self.raw.$field;
         if ptr.is_null() {
            return None
         }

         let data = unsafe { CStr::from_ptr(ptr) }.to_bytes();
         match str::from_utf8(data) {
            Ok(s) => Some(s),
            Err(_) => None
         }
      }
   }
}

impl<'a> Request<'a> {
   str_getter!(the_request);
   str_getter!(protocol);
   str_getter!(hostname);
   str_getter!(status_line);
   str_getter!(method);
   str_getter!(range);
   str_getter!(content_type);
   str_getter!(handler);
   str_getter!(content_encoding);
   str_getter!(vlist_validator);
   str_getter!(user);
   str_getter!(ap_auth_type);
   str_getter!(unparsed_uri);
   str_getter!(uri);
   str_getter!(filename);
   str_getter!(canonical_filename);
   str_getter!(path_info);
   str_getter!(args);
   str_getter!(log_id);
   str_getter!(useragent_ip);
}
