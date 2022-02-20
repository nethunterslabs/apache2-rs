#![allow(non_camel_case_types)]

use libc::c_char;
use std::ffi::CStr;
use std::str;

pub trait FromRaw<T>: Sized {
    fn from_raw(_: T) -> Option<Self>;
}

#[derive(Copy, Clone)]
pub struct Wrapper<T: Copy + Clone> {
    pub ptr: *mut T,
}

impl<T: Copy + Clone> FromRaw<*mut T> for Wrapper<T> {
    fn from_raw(ptr: *mut T) -> Option<Wrapper<T>> {
        if ptr.is_null() {
            None
        } else {
            Some(Wrapper::<T> { ptr })
        }
    }
}

pub trait WrappedType {
    type wrapped_type;
}

impl<T: Copy + Clone> WrappedType for Wrapper<T> {
    type wrapped_type = T;
}

#[inline]
pub fn from_char_ptr<'a>(ptr: *const c_char) -> Option<&'a str> {
    if ptr.is_null() {
        return None;
    }

    let slice = unsafe { CStr::from_ptr(ptr) };
    str::from_utf8(slice.to_bytes()).ok()
}

/*
CStrBuf

Taken from:
- https://dean.serenevy.net/blog/2021/Feb/c-string-buffers/
- https://github.com/duelafn/blog-code/tree/master/2021/c-string-buffer

License for CStrBuf:

Unless otherwise noted, all code released under an MIT license:


Permission is hereby granted, free of charge, to any person obtaining a
copy of this software and associated documentation files (the "Software"),
to deal in the Software without restriction, including without limitation
the rights to use, copy, modify, merge, publish, distribute, sublicense,
and/or sell copies of the Software, and to permit persons to whom the
Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
*/

pub struct CStrBuf {
    pub vec: Vec<u8>,
}

impl CStrBuf {
    /// New buffer. Since this buffer is intended to be used with C
    /// functions, we panic if the requested length is larger than
    /// i32::MAX.
    pub fn new(len: usize) -> CStrBuf {
        // Safety, The intended use is for passing to C functions which
        // expect an i32 length. Check the length at construction. I panic,
        // but one could instead change return type to Option<CStrBuf>.
        if len > (std::i32::MAX as usize) {
            panic!("Expected length <= i32::MAX");
        }
        // Fully initialized buffer to protect against undefined behavior
        // of reading uninitialized memory. See:
        //    https://www.ralfj.de/blog/2019/07/14/uninit.html
        //    https://users.rust-lang.org/t/is-it-ub-to-read-uninitialized-integers-in-a-vector-why/39682
        CStrBuf { vec: vec![0; len] }
    }

    #[inline]
    pub fn as_ptr(&self) -> *const i8 {
        self.vec.as_ptr() as *const i8 // u8 -> i8 is a safe cast
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut i8 {
        self.vec.as_mut_ptr() as *mut i8 // u8 -> i8 is a safe cast
    }

    #[inline]
    pub fn buffer_len(&self) -> i32 {
        self.vec.len() as i32 // cast to i32 is safe because of checks in new()
    }

    /// C-style string length, search for the first null byte.
    /// Fall back to full vector length.
    #[inline]
    pub fn strlen(&self) -> usize {
        // match self.vec.iter().position(|&x| 0 == x) {
        match memchr::memchr(0, &self.vec) {
            Some(n) => n,
            None => self.vec.len(),
        }
    }

    /// Copy null-terminated contents to a new String
    #[inline]
    pub fn to_string(&self) -> Result<String, std::string::FromUtf8Error> {
        let len = self.strlen();
        String::from_utf8(self.vec[0..len].to_vec())
    }

    /// Zero-copy, consume the buffer into a string
    #[inline]
    pub fn into_string(mut self) -> Result<String, std::string::FromUtf8Error> {
        let len = self.strlen();
        self.vec.truncate(len);
        String::from_utf8(self.vec)
    }

    /// Zero-copy borrow of the data as a &str
    #[inline]
    pub fn to_str(&self) -> Result<&str, std::str::Utf8Error> {
        let len = self.strlen();
        std::str::from_utf8(&self.vec[0..len])
    }

    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        let len = self.strlen();
        &self.vec[0..len]
    }

    #[inline]
    pub fn as_bytes_owned(&self) -> Vec<u8> {
        let len = self.strlen();
        self.vec[0..len].to_vec()
    }
}
