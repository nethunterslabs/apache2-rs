//#![feature(prelude_import)]
#![no_std]
//#![feature(plugin)]
//#![plugin(interpolate_idents)]
//#![feature(fmt_internals)]

//#[prelude_import]
//use std::prelude::v1::*;

#[macro_use]
extern crate std as std;

#[macro_use]
extern crate apache2;

use apache2::{Request, Status};
use apache2::httpd;

//use std::os::raw::c_void;
use apache2::c_void;
use apache2::c_int;
use apache2::c_char;


use apache2::wrapper::FromRaw;

static mut hello_directives: [apache2::ffi::command_rec; 1] =
    [apache2::ffi::command_rec {
        name: 0 as *const u8 as *const ::c_char,
        func: apache2::ffi::cmd_func { _bindgen_data_: [0 as u64] },
        cmd_data: 0 as *mut ::c_void,
        req_override: 0,
        args_how: 0,
        errmsg: 0 as *const u8 as *const ::c_char,
    }];
#[allow(unused_unsafe)]
#[no_mangle]
pub static mut hello_module: apache2::ffi::module =
    apache2::ffi::module {
        version: apache2::ffi::MODULE_MAGIC_NUMBER_MAJOR,
        minor_version: apache2::ffi::MODULE_MAGIC_NUMBER_MINOR,
        module_index: -1,
        name: b"mod_hello\x00" as *const u8 as *const ::c_char,
        dynamic_load_handle: 0 as *mut ::c_void,
        next: 0 as *mut apache2::ffi::module,
        magic: apache2::ffi::MODULE_MAGIC_COOKIE,
        rewrite_args: None,
        create_dir_config: None,
        merge_dir_config: None,
        create_server_config: None,
        merge_server_config: None,
        cmds:
        unsafe {
            &hello_directives as *const apache2::ffi::command_rec
        },
        register_hooks: Some(hello_hooks),
    };

extern "C" fn hello_hooks(_: *mut apache2::ffi::apr_pool_t) {
    unsafe {
        apache2::ffi::ap_hook_handler(Some(c_hello_handler), std::ptr::null(),
                                      std::ptr::null(), apache2::HookOrder::MIDDLE.into());
    };
}

extern "C" fn c_hello_handler(r: *mut apache2::ffi::request_rec) -> ::c_int {
    match ::httpd::Request::from_raw(r) {
        None => ::httpd::Status::DECLINED.into(),
        Some(mut request) =>
            match hello_handler(&mut request) {
                Ok(status) => status,
                Err(_) => ::httpd::Status::HTTP_INTERNAL_SERVER_ERROR,
            }.into(),
    }
}

fn unwrap_str<'a>(wrapped: Option<&'a str>) -> &'a str {
    wrapped.unwrap_or("--")
}

fn hello_handler(r: &mut Request) -> Result<Status, ()> {
    r.set_content_type("text/html; charset=utf-8");
    match r.write("<html><body>Hello tCellio") {
        core::result::Result::Ok(val) => val,
        core::result::Result::Err(err) => {
            return core::result::Result::Err(core::convert::From::from(err));
        }
    };
    let headers_in =
        match r.headers_in() {
            Some(val) => val,
            None => {
                return core::result::Result::Err(core::convert::From::from(()));
            }
        };
    for (key, val) in headers_in.iter() {
        r.write(format!("<p>{:?}: {:?}</p><br>\n", key, val))?;
    }
    Ok(Status::OK)
}
