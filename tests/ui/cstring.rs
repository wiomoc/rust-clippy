#![deny(clippy::temporary_cstring_as_ptr)]

fn main() {}

fn temporary_cstring() {
    use std::ffi::CString;

    CString::new("foo").unwrap().as_ptr();
    CString::new("foo").expect("dummy").as_ptr();
}

mod issue4375 {
    use std::ffi::CString;
    use std::os::raw::c_char;

    extern "C" {
        fn foo(data: *const c_char);
    }

    pub fn bar_ok(v: &[u8]) {
        let cstr = CString::new(v);
        unsafe { foo(cstr.unwrap().as_ptr()) }
    }

    pub fn bar_bad(v: &[u8]) {
        let c_str = CString::new(v).unwrap().as_ptr();
        unsafe { foo(c_str) }
    }
}
