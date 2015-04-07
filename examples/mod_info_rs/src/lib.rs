extern crate libc;

#[macro_use]
extern crate apache2;

use apache2::{Request, Status, get_server_description, get_server_built, show_mpm,
   apr_version_string};

apache2_module!(info_rs_module, info_rs_handler, c_info_rs_handler, b"mod_info_rs\0");


fn info_rs_handler(r: &Request) -> Status {
   if r.handler().unwrap() != "server-info-rs" {
      return Status::DECLINED
   }

   let conn = r.connection().unwrap();

   r.set_content_type("text/html");

   r.write("<!doctype html><html><head><meta charset=\"utf-8\"><title>Apache Info</title></head><body>");

   r.write("<h1>Apache Server Information</h1>");

   let server_name = r.escape_html(r.server_name().unwrap()).unwrap();
   let local_ip = conn.local_ip().unwrap();
   r.write(format!("<p>Server: {} (via {})</p>", server_name, local_ip));

   let server_description = get_server_description().unwrap();
   r.write(format!("<p>Server Version: {}</p>", server_description));

   let mmp = show_mpm().unwrap();
   r.write(format!("<p>Server MPM (Multi-Processing Module): {}</p>", mmp));

   let server_built = get_server_built().unwrap();
   r.write(format!("<p>Server Built: {}</p>", server_built));

   let apr_version = apr_version_string().unwrap();
   r.write(format!("<p>Server loaded APR (Apache Portable Runtime) Version: {}</p>", apr_version));

   let client_ip = conn.client_ip().unwrap();
   r.write(format!("<p>Client IP: {}</p>", client_ip));

   r.write("</body></html>");

   Status::OK
}