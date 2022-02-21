#![allow(non_camel_case_types)]
#![allow(unused_unsafe)]

use libc::{c_char, c_int, c_void};

use std::ffi::CString;
use std::{fmt, ptr};

use ffi;

use wrapper::{from_char_ptr, FromRaw, Wrapper};

use apr::{Pool, Table};

macro_rules! field {
    ($that:ident, $field:ident) => {
        unsafe { (*$that.ptr).$field }
    };
}

macro_rules! nested_field {
    ($that:ident, $field1:ident, $field2:ident) => {
        unsafe { (*(*$that.ptr).$field1).$field2 }
    };
}

macro_rules! set_field {
    ($that:ident, $field:ident, $value:expr) => {
        unsafe { (*$that.ptr).$field = $value }
    };
}

macro_rules! option_getter {
    ($name:ident, $wrapped:ident) => {
        pub fn $name(&self) -> Option<$wrapped> {
            $wrapped::from_raw(field!(self, $name))
        }
    };
}

macro_rules! type_getter {
    ($name:ident, $restype:ident) => {
        pub fn $name(&self) -> $restype {
            field!(self, $name) as $restype
        }
    };
}

macro_rules! type_field_getter {
    ($name:ident, $field1:ident, $field2:ident, $restype:ident) => {
        pub fn $name(&self) -> $restype {
            nested_field!(self, $field1, $field2) as $restype
        }
    };
}

macro_rules! into_getter {
    ($name:ident, $restype:ident) => {
        pub fn $name(&self) -> $restype {
            field!(self, $name).into()
        }
    };
}

macro_rules! str_getter {
    ($name:ident) => {
        pub fn $name<'a>(&self) -> Option<&'a str> {
            from_char_ptr(field!(self, $name))
        }
    };
}

macro_rules! bool_getter {
    ($name:ident) => {
        pub fn $name(&self) -> bool {
            field!(self, $name) != 0
        }
    };
}

#[derive(Debug, PartialEq)]
pub enum Status {
    // non-HTTP status codes returned by hooks
    OK,       // Module has handled this stage.
    DECLINED, // Module declines to handle
    DONE,     // Module has served the response completely
    // - it's safe to die() with no more output
    SUSPENDED, // Module will handle the remainder of the request.
    // The core will never invoke the request again,
    HTTP_CONTINUE,
    HTTP_SWITCHING_PROTOCOLS,
    HTTP_PROCESSING,
    HTTP_OK,
    HTTP_CREATED,
    HTTP_ACCEPTED,
    HTTP_NON_AUTHORITATIVE,
    HTTP_NO_CONTENT,
    HTTP_RESET_CONTENT,
    HTTP_PARTIAL_CONTENT,
    HTTP_MULTI_STATUS,
    HTTP_ALREADY_REPORTED,
    HTTP_IM_USED,
    HTTP_MULTIPLE_CHOICES,
    HTTP_MOVED_PERMANENTLY,
    HTTP_MOVED_TEMPORARILY,
    HTTP_SEE_OTHER,
    HTTP_NOT_MODIFIED,
    HTTP_USE_PROXY,
    HTTP_TEMPORARY_REDIRECT,
    HTTP_PERMANENT_REDIRECT,
    HTTP_BAD_REQUEST,
    HTTP_UNAUTHORIZED,
    HTTP_PAYMENT_REQUIRED,
    HTTP_FORBIDDEN,
    HTTP_NOT_FOUND,
    HTTP_METHOD_NOT_ALLOWED,
    HTTP_NOT_ACCEPTABLE,
    HTTP_PROXY_AUTHENTICATION_REQUIRED,
    HTTP_REQUEST_TIME_OUT,
    HTTP_CONFLICT,
    HTTP_GONE,
    HTTP_LENGTH_REQUIRED,
    HTTP_PRECONDITION_FAILED,
    HTTP_REQUEST_ENTITY_TOO_LARGE,
    HTTP_REQUEST_URI_TOO_LARGE,
    HTTP_UNSUPPORTED_MEDIA_TYPE,
    HTTP_RANGE_NOT_SATISFIABLE,
    HTTP_EXPECTATION_FAILED,
    HTTP_IM_A_TEAPOT,
    HTTP_UNPROCESSABLE_ENTITY,
    HTTP_LOCKED,
    HTTP_FAILED_DEPENDENCY,
    HTTP_UPGRADE_REQUIRED,
    HTTP_PRECONDITION_REQUIRED,
    HTTP_TOO_MANY_REQUESTS,
    HTTP_REQUEST_HEADER_FIELDS_TOO_LARGE,
    HTTP_INTERNAL_SERVER_ERROR,
    HTTP_NOT_IMPLEMENTED,
    HTTP_BAD_GATEWAY,
    HTTP_SERVICE_UNAVAILABLE,
    HTTP_GATEWAY_TIME_OUT,
    HTTP_VERSION_NOT_SUPPORTED,
    HTTP_VARIANT_ALSO_VARIES,
    HTTP_INSUFFICIENT_STORAGE,
    HTTP_LOOP_DETECTED,
    HTTP_NOT_EXTENDED,
    HTTP_NETWORK_AUTHENTICATION_REQUIRED,
    HTTP_UNKNOWN(c_int),
}

impl Into<c_int> for Status {
    fn into(self) -> c_int {
        match self {
            Status::OK => ffi::OK,
            Status::DECLINED => ffi::DECLINED,
            Status::DONE => ffi::DONE,
            Status::SUSPENDED => ffi::SUSPENDED,

            Status::HTTP_CONTINUE => ffi::HTTP_CONTINUE,
            Status::HTTP_SWITCHING_PROTOCOLS => ffi::HTTP_SWITCHING_PROTOCOLS,
            Status::HTTP_PROCESSING => ffi::HTTP_PROCESSING,
            Status::HTTP_OK => ffi::HTTP_OK,
            Status::HTTP_CREATED => ffi::HTTP_CREATED,
            Status::HTTP_ACCEPTED => ffi::HTTP_ACCEPTED,
            Status::HTTP_NON_AUTHORITATIVE => ffi::HTTP_NON_AUTHORITATIVE,
            Status::HTTP_NO_CONTENT => ffi::HTTP_NO_CONTENT,
            Status::HTTP_RESET_CONTENT => ffi::HTTP_RESET_CONTENT,
            Status::HTTP_PARTIAL_CONTENT => ffi::HTTP_PARTIAL_CONTENT,
            Status::HTTP_MULTI_STATUS => ffi::HTTP_MULTI_STATUS,
            Status::HTTP_ALREADY_REPORTED => ffi::HTTP_ALREADY_REPORTED,
            Status::HTTP_IM_USED => ffi::HTTP_IM_USED,
            Status::HTTP_MULTIPLE_CHOICES => ffi::HTTP_MULTIPLE_CHOICES,
            Status::HTTP_MOVED_PERMANENTLY => ffi::HTTP_MOVED_PERMANENTLY,
            Status::HTTP_MOVED_TEMPORARILY => ffi::HTTP_MOVED_TEMPORARILY,
            Status::HTTP_SEE_OTHER => ffi::HTTP_SEE_OTHER,
            Status::HTTP_NOT_MODIFIED => ffi::HTTP_NOT_MODIFIED,
            Status::HTTP_USE_PROXY => ffi::HTTP_USE_PROXY,
            Status::HTTP_TEMPORARY_REDIRECT => ffi::HTTP_TEMPORARY_REDIRECT,
            Status::HTTP_PERMANENT_REDIRECT => ffi::HTTP_PERMANENT_REDIRECT,
            Status::HTTP_BAD_REQUEST => ffi::HTTP_BAD_REQUEST,
            Status::HTTP_UNAUTHORIZED => ffi::HTTP_UNAUTHORIZED,
            Status::HTTP_PAYMENT_REQUIRED => ffi::HTTP_PAYMENT_REQUIRED,
            Status::HTTP_FORBIDDEN => ffi::HTTP_FORBIDDEN,
            Status::HTTP_NOT_FOUND => ffi::HTTP_NOT_FOUND,
            Status::HTTP_METHOD_NOT_ALLOWED => ffi::HTTP_METHOD_NOT_ALLOWED,
            Status::HTTP_NOT_ACCEPTABLE => ffi::HTTP_NOT_ACCEPTABLE,
            Status::HTTP_PROXY_AUTHENTICATION_REQUIRED => ffi::HTTP_PROXY_AUTHENTICATION_REQUIRED,
            Status::HTTP_REQUEST_TIME_OUT => ffi::HTTP_REQUEST_TIME_OUT,
            Status::HTTP_CONFLICT => ffi::HTTP_CONFLICT,
            Status::HTTP_GONE => ffi::HTTP_GONE,
            Status::HTTP_LENGTH_REQUIRED => ffi::HTTP_LENGTH_REQUIRED,
            Status::HTTP_PRECONDITION_FAILED => ffi::HTTP_PRECONDITION_FAILED,
            Status::HTTP_REQUEST_ENTITY_TOO_LARGE => ffi::HTTP_REQUEST_ENTITY_TOO_LARGE,
            Status::HTTP_REQUEST_URI_TOO_LARGE => ffi::HTTP_REQUEST_URI_TOO_LARGE,
            Status::HTTP_UNSUPPORTED_MEDIA_TYPE => ffi::HTTP_UNSUPPORTED_MEDIA_TYPE,
            Status::HTTP_RANGE_NOT_SATISFIABLE => ffi::HTTP_RANGE_NOT_SATISFIABLE,
            Status::HTTP_EXPECTATION_FAILED => ffi::HTTP_EXPECTATION_FAILED,
            Status::HTTP_IM_A_TEAPOT => ffi::HTTP_IM_A_TEAPOT,
            Status::HTTP_UNPROCESSABLE_ENTITY => ffi::HTTP_UNPROCESSABLE_ENTITY,
            Status::HTTP_LOCKED => ffi::HTTP_LOCKED,
            Status::HTTP_FAILED_DEPENDENCY => ffi::HTTP_FAILED_DEPENDENCY,
            Status::HTTP_UPGRADE_REQUIRED => ffi::HTTP_UPGRADE_REQUIRED,
            Status::HTTP_PRECONDITION_REQUIRED => ffi::HTTP_PRECONDITION_REQUIRED,
            Status::HTTP_TOO_MANY_REQUESTS => ffi::HTTP_TOO_MANY_REQUESTS,
            Status::HTTP_REQUEST_HEADER_FIELDS_TOO_LARGE => {
                ffi::HTTP_REQUEST_HEADER_FIELDS_TOO_LARGE
            }
            Status::HTTP_INTERNAL_SERVER_ERROR => ffi::HTTP_INTERNAL_SERVER_ERROR,
            Status::HTTP_NOT_IMPLEMENTED => ffi::HTTP_NOT_IMPLEMENTED,
            Status::HTTP_BAD_GATEWAY => ffi::HTTP_BAD_GATEWAY,
            Status::HTTP_SERVICE_UNAVAILABLE => ffi::HTTP_SERVICE_UNAVAILABLE,
            Status::HTTP_GATEWAY_TIME_OUT => ffi::HTTP_GATEWAY_TIME_OUT,
            Status::HTTP_VERSION_NOT_SUPPORTED => ffi::HTTP_VERSION_NOT_SUPPORTED,
            Status::HTTP_VARIANT_ALSO_VARIES => ffi::HTTP_VARIANT_ALSO_VARIES,
            Status::HTTP_INSUFFICIENT_STORAGE => ffi::HTTP_INSUFFICIENT_STORAGE,
            Status::HTTP_LOOP_DETECTED => ffi::HTTP_LOOP_DETECTED,
            Status::HTTP_NOT_EXTENDED => ffi::HTTP_NOT_EXTENDED,
            Status::HTTP_NETWORK_AUTHENTICATION_REQUIRED => {
                ffi::HTTP_NETWORK_AUTHENTICATION_REQUIRED
            }
            Status::HTTP_UNKNOWN(i) => i,
        }
    }
}

impl Into<Status> for c_int {
    fn into(self) -> Status {
        match self {
            ffi::OK => Status::OK,
            ffi::DECLINED => Status::DECLINED,
            ffi::DONE => Status::DONE,
            ffi::SUSPENDED => Status::SUSPENDED,

            ffi::HTTP_CONTINUE => Status::HTTP_CONTINUE,
            ffi::HTTP_SWITCHING_PROTOCOLS => Status::HTTP_SWITCHING_PROTOCOLS,
            ffi::HTTP_PROCESSING => Status::HTTP_PROCESSING,
            ffi::HTTP_OK => Status::HTTP_OK,
            ffi::HTTP_CREATED => Status::HTTP_CREATED,
            ffi::HTTP_ACCEPTED => Status::HTTP_ACCEPTED,
            ffi::HTTP_NON_AUTHORITATIVE => Status::HTTP_NON_AUTHORITATIVE,
            ffi::HTTP_NO_CONTENT => Status::HTTP_NO_CONTENT,
            ffi::HTTP_RESET_CONTENT => Status::HTTP_RESET_CONTENT,
            ffi::HTTP_PARTIAL_CONTENT => Status::HTTP_PARTIAL_CONTENT,
            ffi::HTTP_MULTI_STATUS => Status::HTTP_MULTI_STATUS,
            ffi::HTTP_ALREADY_REPORTED => Status::HTTP_ALREADY_REPORTED,
            ffi::HTTP_IM_USED => Status::HTTP_IM_USED,
            ffi::HTTP_MULTIPLE_CHOICES => Status::HTTP_MULTIPLE_CHOICES,
            ffi::HTTP_MOVED_PERMANENTLY => Status::HTTP_MOVED_PERMANENTLY,
            ffi::HTTP_MOVED_TEMPORARILY => Status::HTTP_MOVED_TEMPORARILY,
            ffi::HTTP_SEE_OTHER => Status::HTTP_SEE_OTHER,
            ffi::HTTP_NOT_MODIFIED => Status::HTTP_NOT_MODIFIED,
            ffi::HTTP_USE_PROXY => Status::HTTP_USE_PROXY,
            ffi::HTTP_TEMPORARY_REDIRECT => Status::HTTP_TEMPORARY_REDIRECT,
            ffi::HTTP_PERMANENT_REDIRECT => Status::HTTP_PERMANENT_REDIRECT,
            ffi::HTTP_BAD_REQUEST => Status::HTTP_BAD_REQUEST,
            ffi::HTTP_UNAUTHORIZED => Status::HTTP_UNAUTHORIZED,
            ffi::HTTP_PAYMENT_REQUIRED => Status::HTTP_PAYMENT_REQUIRED,
            ffi::HTTP_FORBIDDEN => Status::HTTP_FORBIDDEN,
            ffi::HTTP_NOT_FOUND => Status::HTTP_NOT_FOUND,
            ffi::HTTP_METHOD_NOT_ALLOWED => Status::HTTP_METHOD_NOT_ALLOWED,
            ffi::HTTP_NOT_ACCEPTABLE => Status::HTTP_NOT_ACCEPTABLE,
            ffi::HTTP_PROXY_AUTHENTICATION_REQUIRED => Status::HTTP_PROXY_AUTHENTICATION_REQUIRED,
            ffi::HTTP_REQUEST_TIME_OUT => Status::HTTP_REQUEST_TIME_OUT,
            ffi::HTTP_CONFLICT => Status::HTTP_CONFLICT,
            ffi::HTTP_GONE => Status::HTTP_GONE,
            ffi::HTTP_LENGTH_REQUIRED => Status::HTTP_LENGTH_REQUIRED,
            ffi::HTTP_PRECONDITION_FAILED => Status::HTTP_PRECONDITION_FAILED,
            ffi::HTTP_REQUEST_ENTITY_TOO_LARGE => Status::HTTP_REQUEST_ENTITY_TOO_LARGE,
            ffi::HTTP_REQUEST_URI_TOO_LARGE => Status::HTTP_REQUEST_URI_TOO_LARGE,
            ffi::HTTP_UNSUPPORTED_MEDIA_TYPE => Status::HTTP_UNSUPPORTED_MEDIA_TYPE,
            ffi::HTTP_RANGE_NOT_SATISFIABLE => Status::HTTP_RANGE_NOT_SATISFIABLE,
            ffi::HTTP_EXPECTATION_FAILED => Status::HTTP_EXPECTATION_FAILED,
            ffi::HTTP_IM_A_TEAPOT => Status::HTTP_IM_A_TEAPOT,
            ffi::HTTP_UNPROCESSABLE_ENTITY => Status::HTTP_UNPROCESSABLE_ENTITY,
            ffi::HTTP_LOCKED => Status::HTTP_LOCKED,
            ffi::HTTP_FAILED_DEPENDENCY => Status::HTTP_FAILED_DEPENDENCY,
            ffi::HTTP_UPGRADE_REQUIRED => Status::HTTP_UPGRADE_REQUIRED,
            ffi::HTTP_PRECONDITION_REQUIRED => Status::HTTP_PRECONDITION_REQUIRED,
            ffi::HTTP_TOO_MANY_REQUESTS => Status::HTTP_TOO_MANY_REQUESTS,
            ffi::HTTP_REQUEST_HEADER_FIELDS_TOO_LARGE => {
                Status::HTTP_REQUEST_HEADER_FIELDS_TOO_LARGE
            }
            ffi::HTTP_INTERNAL_SERVER_ERROR => Status::HTTP_INTERNAL_SERVER_ERROR,
            ffi::HTTP_NOT_IMPLEMENTED => Status::HTTP_NOT_IMPLEMENTED,
            ffi::HTTP_BAD_GATEWAY => Status::HTTP_BAD_GATEWAY,
            ffi::HTTP_SERVICE_UNAVAILABLE => Status::HTTP_SERVICE_UNAVAILABLE,
            ffi::HTTP_GATEWAY_TIME_OUT => Status::HTTP_GATEWAY_TIME_OUT,
            ffi::HTTP_VERSION_NOT_SUPPORTED => Status::HTTP_VERSION_NOT_SUPPORTED,
            ffi::HTTP_VARIANT_ALSO_VARIES => Status::HTTP_VARIANT_ALSO_VARIES,
            ffi::HTTP_INSUFFICIENT_STORAGE => Status::HTTP_INSUFFICIENT_STORAGE,
            ffi::HTTP_LOOP_DETECTED => Status::HTTP_LOOP_DETECTED,
            ffi::HTTP_NOT_EXTENDED => Status::HTTP_NOT_EXTENDED,
            ffi::HTTP_NETWORK_AUTHENTICATION_REQUIRED => {
                Status::HTTP_NETWORK_AUTHENTICATION_REQUIRED
            }
            i if i >= 100 => Status::HTTP_UNKNOWN(i),
            _ => Status::DECLINED,
        }
    }
}

pub enum ProxyReq {
    NONE,     // No proxy
    PROXY,    // Standard proxy
    REVERSE,  // Reverse proxy
    RESPONSE, // Origin response
}

impl From<c_int> for ProxyReq {
    fn from(c_int: c_int) -> ProxyReq {
        match c_int {
            ffi::PROXYREQ_NONE => ProxyReq::NONE,
            ffi::PROXYREQ_PROXY => ProxyReq::PROXY,
            ffi::PROXYREQ_REVERSE => ProxyReq::REVERSE,
            ffi::PROXYREQ_RESPONSE => ProxyReq::RESPONSE,
            _ => panic!("Unknown ProxyReq type"),
        }
    }
}

impl fmt::Display for ProxyReq {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display = match *self {
            ProxyReq::NONE => "No proxy",
            ProxyReq::PROXY => "Standard proxy",
            ProxyReq::REVERSE => "Reverse proxy",
            ProxyReq::RESPONSE => "Origin response",
        };

        write!(f, "{}", display)
    }
}

#[derive(Debug, PartialEq)]
pub enum ReadPolicy {
    REQUEST_NO_BODY,
    REQUEST_CHUNKED_ERROR,
    REQUEST_CHUNKED_DECHUNK,
}

impl From<ReadPolicy> for c_int {
    fn from(read_policy: ReadPolicy) -> c_int {
        match read_policy {
            ReadPolicy::REQUEST_NO_BODY => ffi::READ_POLICY_REQUEST_NO_BODY,
            ReadPolicy::REQUEST_CHUNKED_ERROR => ffi::READ_POLICY_REQUEST_CHUNKED_ERROR,
            ReadPolicy::REQUEST_CHUNKED_DECHUNK => ffi::READ_POLICY_REQUEST_CHUNKED_DECHUNK,
        }
    }
}

pub type Request = Wrapper<ffi::request_rec>;

impl Request {
    option_getter!(pool, Pool);

    option_getter!(connection, Conn);

    option_getter!(server, Server);

    str_getter!(the_request);

    bool_getter!(assbackwards);

    into_getter!(proxyreq, ProxyReq);

    bool_getter!(header_only);

    pub fn set_header_only(&mut self, header_only: bool) {
        set_field!(self, header_only, header_only as c_int);
    }

    str_getter!(protocol);

    str_getter!(hostname);

    type_getter!(request_time, i64);

    str_getter!(status_line);

    into_getter!(status, Status);

    pub fn set_status(&mut self, status: Status) {
        set_field!(self, status, status.into());
    }

    str_getter!(method);

    type_getter!(mtime, i64);

    pub fn set_last_modified(&mut self, mtime: i64) {
        unsafe {
            ffi::ap_update_mtime(self.ptr, mtime);
            ffi::ap_set_last_modified(self.ptr);
        }
    }

    str_getter!(range);

    type_getter!(clength, i64);

    option_getter!(body_table, Table);

    option_getter!(headers_in, Table);

    option_getter!(headers_out, Table);

    option_getter!(err_headers_out, Table);

    option_getter!(subprocess_env, Table);

    option_getter!(notes, Table);

    str_getter!(content_type);

    pub fn set_content_type<T: Into<Vec<u8>>>(&self, ct: T) {
        let c_str = ffi::strdup(field!(self, pool), ct);

        unsafe {
            ffi::ap_set_content_type(self.ptr, c_str);
        };
    }

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

    option_getter!(per_dir_config, ConfVector);

    option_getter!(request_config, ConfVector);

    str_getter!(log_id);

    str_getter!(useragent_ip);

    pub fn write<T: Into<Vec<u8>>>(&self, data: T) -> Result<(), ()> {
        let c_str_buf = match CString::new(data) {
            Ok(s) => s,
            Err(_) => return Err(()),
        };

        let sent = unsafe {
            ffi::ap_rwrite(
                c_str_buf.as_ptr() as *mut c_void,
                c_str_buf.to_bytes().len() as i32,
                self.ptr,
            )
        };

        match sent {
            -1 => Err(()),
            _ => Ok(()),
        }
    }

    pub fn setup_client_block(&self, read_policy: ReadPolicy) -> Result<i32, i32> {
        let setup = unsafe { ffi::ap_setup_client_block(self.ptr, read_policy.into()) };

        match setup {
            0 => Ok(setup),
            _ => Err(setup),
        }
    }

    pub fn should_client_block(&self) -> Result<i32, i32> {
        let result = unsafe { ffi::ap_should_client_block(self.ptr) };

        match result {
            1 => Ok(result),
            _ => Err(result),
        }
    }

    pub fn get_client_block(&self, buffer_ptr: *mut c_char, bufsize: u64) -> Result<i64, ()> {
        let result = unsafe { ffi::ap_get_client_block(self.ptr, buffer_ptr, bufsize) };
        Ok(result)
    }

    pub fn escape_html<'a, T: Into<Vec<u8>>>(&self, s: T) -> Result<&'a str, ()> {
        let c_str = match CString::new(s) {
            Ok(s) => s,
            Err(_) => return Err(()),
        };

        let escaped = unsafe { ffi::ap_escape_html2(field!(self, pool), c_str.as_ptr(), 0) };

        from_char_ptr(escaped).ok_or(())
    }

    //   pub fn escape_urlencoded<'a, T: Into<Vec<u8>>>(&self, s: T) -> Result<&'a str, ()> {
    //      let c_str = match CString::new(s) {
    //         Ok(s) => s,
    //         Err(_) => return Err(())
    //      };
    //
    //      let escaped = unsafe {
    //         ffi::ap_escape_urlencoded(field!(self, pool), c_str.as_ptr())
    //      };
    //
    //      from_char_ptr(escaped).ok_or(())
    //   }

    //   pub fn unescape_urlencoded<'a, T: Into<Vec<u8>>>(&self, query: T) -> Result<&'a str, ()> {
    //      let c_str = ffi::strdup(field!(self, pool), query);
    //
    //      let res = unsafe {
    //         ffi::ap_unescape_urlencoded(c_str)
    //      };
    //
    //      if res != 0 {
    //         return Err(());
    //      };
    //
    //      from_char_ptr(c_str).ok_or(())
    //   }

    pub fn server_name<'a>(&self) -> Option<&'a str> {
        from_char_ptr(unsafe { ffi::ap_get_server_name(self.ptr) })
    }

    pub fn server_port(&self) -> u16 {
        unsafe { ffi::ap_get_server_port(self.ptr) }
    }

    pub fn document_root<'a>(&self) -> Option<&'a str> {
        from_char_ptr(unsafe { ffi::ap_document_root(self.ptr) })
    }

    pub fn auth_name<'a>(&self) -> Option<&'a str> {
        from_char_ptr(unsafe { ffi::ap_auth_name(self.ptr) })
    }

    pub fn basic_auth_pw<'a>(&self) -> Option<&'a str> {
        let mut pw: *const c_char = ptr::null_mut();

        unsafe {
            ffi::ap_get_basic_auth_pw(self.ptr, &mut pw);
        }

        from_char_ptr(pw)
    }

    //   pub fn context_document_root<'a>(&self) -> Option<&'a str> {
    //      from_char_ptr(
    //         unsafe { ffi::ap_context_document_root(self.ptr) }
    //      )
    //   }

    //   pub fn context_prefix<'a>(&self) -> Option<&'a str> {
    //      from_char_ptr(
    //         unsafe { ffi::ap_context_prefix(self.ptr) }
    //      )
    //   }

    pub fn http_scheme<'a>(&self) -> Option<&'a str> {
        from_char_ptr(unsafe { ffi::ap_run_http_scheme(self.ptr) })
    }

    pub fn default_port(&self) -> u16 {
        unsafe { ffi::ap_run_default_port(self.ptr) }
    }

    pub fn is_initial_req(&self) -> bool {
        unsafe { ffi::ap_is_initial_req(self.ptr) == 1 }
    }

    pub fn some_auth_required(&self) -> bool {
        unsafe { ffi::ap_some_auth_required(self.ptr) == 1 }
    }

    //   pub fn cookie<'a, T: Into<Vec<u8>>>(&self, name: T) -> Option<&'a str> {
    //      let c_str_name = ffi::strdup(field!(self, pool), name);
    //      let mut val: *const c_char = ptr::null_mut();
    //
    //      unsafe {
    //         ffi::ap_cookie_read(self.ptr, c_str_name, &mut val, 0);
    //      }
    //
    //      from_char_ptr(val)
    //   }

    //   pub fn set_cookie(&self, cookie: Cookie) {
    //      let c_str_name = ffi::strdup(field!(self, pool), cookie.name);
    //      let c_str_val = ffi::strdup(field!(self, pool), cookie.value);
    //      let c_str_attrs = ffi::strdup(field!(self, pool), cookie.attrs(&self));
    //
    //      let null: *const ffi::apr_table_t = ptr::null();
    //
    //      unsafe {
    //         ffi::ap_cookie_write(self.ptr, c_str_name, c_str_val, c_str_attrs, 0,
    //                              field!(self, headers_out), null);
    //      }
    //   }

    pub fn base64_encode<'a, T: Into<Vec<u8>>>(&self, plain: T) -> Result<&'a str, ()> {
        let c_str_plain: CString = match CString::new(plain) {
            Ok(val) => val,
            Err(_) => return Err(()),
        };

        let plain_len: c_int = c_str_plain.to_bytes().len() as c_int;

        let mut encoded_len: c_int = unsafe { ffi::apr_base64_encode_len(plain_len) };

        if encoded_len == 0 {
            return Err(());
        };

        let encoded: *mut c_char = unsafe {
            ffi::apr_palloc(field!(self, pool), encoded_len as ffi::apr_size_t) as *mut c_char
        };

        encoded_len = unsafe { ffi::apr_base64_encode(encoded, c_str_plain.as_ptr(), plain_len) };

        if encoded_len == 0 {
            return Err(());
        };

        from_char_ptr(encoded).ok_or(())
    }

    pub fn base64_decode<'a, T: Into<Vec<u8>>>(&self, encoded: T) -> Result<&'a str, ()> {
        let c_str_encoded: CString = match CString::new(encoded) {
            Ok(val) => val,
            Err(_) => return Err(()),
        };

        let mut plain_len: c_int = unsafe { ffi::apr_base64_decode_len(c_str_encoded.as_ptr()) };

        if plain_len == 0 {
            return Err(());
        };

        let plain: *mut c_char = unsafe {
            ffi::apr_palloc(field!(self, pool), plain_len as ffi::apr_size_t) as *mut c_char
        };

        plain_len = unsafe { ffi::apr_base64_decode(plain, c_str_encoded.as_ptr()) };

        if plain_len == 0 {
            return Err(());
        };

        from_char_ptr(plain).ok_or(())
    }

    pub fn rfc822_date<'a>(&self, t: i64) -> Result<&'a str, ()> {
        let date: *mut c_char =
            unsafe { ffi::apr_palloc(field!(self, pool), ffi::APR_RFC822_DATE_LEN) as *mut c_char };

        unsafe {
            ffi::apr_rfc822_date(date, t);
        }

        from_char_ptr(date).ok_or(())
    }
}

pub type Conn = Wrapper<ffi::conn_rec>;

impl Conn {
    str_getter!(client_ip);

    type_field_getter!(client_port, client_addr, port, u16);

    str_getter!(remote_host);

    str_getter!(remote_logname);

    str_getter!(local_ip);

    str_getter!(local_host);

    type_field_getter!(local_port, local_addr, port, u16);

    str_getter!(log_id);
}

pub type Server = Wrapper<ffi::server_rec>;

impl Server {
    option_getter!(module_config, ConfVector);
    option_getter!(process, Process);
    str_getter!(path);
}

pub type Process = Wrapper<ffi::process_rec>;

impl Process {
    option_getter!(pool, APRPool);
}

pub type APRPool = Wrapper<ffi::apr_pool_t>;

pub type CmdParms = Wrapper<ffi::cmd_parms>;

impl CmdParms {
    option_getter!(server, Server);

    option_getter!(pool, Pool);
}

pub type Module = Wrapper<ffi::module>;

pub type ConfVector = Wrapper<ffi::ap_conf_vector_t>;

//pub type ListProviderGroup = Wrapper<ffi::ap_list_provider_groups_t>;
//
//
//impl ListProviderGroup {
//   str_getter!(provider_group);
//
//   str_getter!(provider_version);
//}

//pub type ListProviderName = Wrapper<ffi::ap_list_provider_names_t>;
//
//
//impl ListProviderName {
//   str_getter!(provider_name);
//}

//pub fn list_provider_groups(pool: &mut Pool) -> ArrayHeaderIter<ListProviderGroup> {
//   let ptr = unsafe { ffi::ap_list_provider_groups(pool.ptr) };
//
//   ArrayHeaderIter::<ListProviderGroup> {
//      phantom: PhantomData,
//      array_header: ptr,
//      next_idx: 0
//   }
//}

//pub fn list_provider_names<T: Into<Vec<u8>>>(pool: &mut Pool, provider_group: T, provider_version: T) -> ArrayHeaderIter<ListProviderName> {
//   let provider_group = ffi::strdup(pool.ptr, provider_group);
//   let provider_version = ffi::strdup(pool.ptr, provider_version);
//
//   let ptr = unsafe { ffi::ap_list_provider_names(pool.ptr, provider_group, provider_version) };
//
//   ArrayHeaderIter::<ListProviderName> {
//      phantom: PhantomData,
//      array_header: ptr,
//      next_idx: 0
//   }
//}

pub fn server_banner<'a>() -> Option<&'a str> {
    from_char_ptr(unsafe { ffi::ap_get_server_banner() })
}

pub fn server_description<'a>() -> Option<&'a str> {
    from_char_ptr(unsafe { ffi::ap_get_server_description() })
}

pub fn server_built<'a>() -> Option<&'a str> {
    from_char_ptr(unsafe { ffi::ap_get_server_built() })
}

pub fn show_mpm<'a>() -> Option<&'a str> {
    from_char_ptr(unsafe { ffi::ap_show_mpm() })
}
