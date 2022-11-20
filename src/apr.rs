#![allow(non_camel_case_types)]
#![allow(clippy::result_unit_err)]

use std::ffi::{c_int, CString};
use std::marker::PhantomData;

use crate::{
    ffi,
    wrapper::{from_char_ptr, FromRaw, WrappedType, Wrapper},
};

pub enum HookOrder {
    REALLY_FIRST, // run this hook first, before ANYTHING
    FIRST,        // run this hook first
    MIDDLE,       // run this hook somewhere
    LAST,         // run this hook after every other hook which is defined
    REALLY_LAST,  // run this hook last, after EVERYTHING
}

impl Into<c_int> for HookOrder {
    fn into(self) -> c_int {
        match self {
            HookOrder::REALLY_FIRST => ffi::APR_HOOK_REALLY_FIRST as c_int,
            HookOrder::FIRST => ffi::APR_HOOK_FIRST as c_int,
            HookOrder::MIDDLE => ffi::APR_HOOK_MIDDLE as c_int,
            HookOrder::LAST => ffi::APR_HOOK_LAST as c_int,
            HookOrder::REALLY_LAST => ffi::APR_HOOK_REALLY_LAST as c_int,
        }
    }
}

pub type Table = Wrapper<ffi::apr_table_t>;

impl Table {
    pub fn get<'a, T: Into<Vec<u8>>>(&self, key: T) -> Option<&'a str> {
        let key = match CString::new(key) {
            Ok(s) => s,
            Err(_) => return None,
        };

        from_char_ptr(unsafe { ffi::apr_table_get(self.ptr, key.as_ptr()) })
    }

    pub fn set<T: Into<Vec<u8>>, U: Into<Vec<u8>>>(&mut self, key: T, val: U) -> Result<(), ()> {
        let key = match CString::new(key) {
            Ok(s) => s,
            Err(_) => return Err(()),
        };

        let val = match CString::new(val) {
            Ok(s) => s,
            Err(_) => return Err(()),
        };

        unsafe { ffi::apr_table_set(self.ptr, key.as_ptr(), val.as_ptr()) };

        Ok(())
    }

    pub fn add<T: Into<Vec<u8>>, U: Into<Vec<u8>>>(&mut self, key: T, val: U) -> Result<(), ()> {
        let key = match CString::new(key) {
            Ok(s) => s,
            Err(_) => return Err(()),
        };

        let val = match CString::new(val) {
            Ok(s) => s,
            Err(_) => return Err(()),
        };

        unsafe { ffi::apr_table_add(self.ptr, key.as_ptr(), val.as_ptr()) };

        Ok(())
    }

    pub fn iter(&self) -> TableIter {
        let ptr = unsafe { ffi::apr_table_elts(self.ptr) };
        let raw: &ffi::apr_array_header_t = unsafe { &*ptr };

        TableIter {
            array_header: raw,
            next_idx: 0,
        }
    }
}

pub type Pool = Wrapper<ffi::apr_pool_t>;

pub struct TableIter<'a> {
    pub array_header: &'a ffi::apr_array_header_t,
    pub next_idx: usize,
}

impl<'a> Iterator for TableIter<'a> {
    type Item = (&'a str, Option<&'a str>);

    fn next(&mut self) -> Option<(&'a str, Option<&'a str>)> {
        if self.next_idx != self.array_header.nelts as usize {
            let mut elts = self.array_header.elts as *const ffi::apr_table_entry_t;

            elts = unsafe { elts.offset(self.next_idx as isize) };
            self.next_idx += 1;

            let key = from_char_ptr(unsafe { (*elts).key }).unwrap();
            let val_result = from_char_ptr(unsafe { (*elts).val });

            Some((key, val_result))
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let rem = self.array_header.nelts as usize - self.next_idx;
        (rem, Some(rem))
    }
}

pub struct ArrayHeaderIter<T> {
    pub phantom: PhantomData<T>,
    pub array_header: *mut ffi::apr_array_header_t,
    pub next_idx: usize,
}

impl<T: Copy + WrappedType + FromRaw<*mut <T as WrappedType>::wrapped_type>> Iterator
    for ArrayHeaderIter<T>
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.array_header.is_null() {
            return None;
        }
        let array_header: &ffi::apr_array_header_t = unsafe { &*self.array_header };
        if self.next_idx != array_header.nelts as usize {
            let mut elt = array_header.elts as *const T::wrapped_type;

            elt = unsafe { elt.offset(self.next_idx as isize) };
            self.next_idx += 1;

            T::from_raw(elt as *mut <T as WrappedType>::wrapped_type)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.array_header.is_null() {
            return (0, None);
        }
        let array_header: &ffi::apr_array_header_t = unsafe { &*self.array_header };
        let rem = array_header.nelts as usize - self.next_idx;
        (rem, Some(rem))
    }
}

pub fn apr_version_string<'a>() -> Option<&'a str> {
    from_char_ptr(unsafe { ffi::apr_version_string() })
}

pub fn apu_version_string<'a>() -> Option<&'a str> {
    from_char_ptr(unsafe { ffi::apu_version_string() })
}

pub fn time_now() -> i64 {
    unsafe { ffi::apr_time_now() }
}

pub type AprBucket = Wrapper<ffi::apr_bucket>;

impl AprBucket {
    pub fn read(
        &self,
        str: *mut *const i8,
        len: *mut usize,
        block: ffi::apr_read_type_e,
    ) -> Result<i32, ()> {
        unsafe {
            let bucket = *self.ptr;
            if let Some(bucket_type) = bucket.type_.as_ref() {
                if let Some(read) = bucket_type.read {
                    let result = read(self.ptr, str, len, block);
                    return Ok(result);
                }
            }
            Err(())
        }
    }
}
