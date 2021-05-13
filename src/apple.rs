use alloc::string::String;
use core::ptr::NonNull;
use cstr_core::CStr;
use libc::{c_char, free as c_free};

extern "C" {
    fn apple_fetch_locale() -> *mut c_char;
}

pub(crate) fn get() -> Option<String> {
    let ptr = NonNull::new(unsafe { apple_fetch_locale() })?;

    let locale = {
        // SAFETY: `ptr` is a valid pointer to a NUL terminated C string and isn't null.
        let raw = unsafe { CStr::from_ptr(ptr.as_ptr().cast()) };
        raw.to_str().ok().map(String::from)
    };

    // SAFETY: `ptr` is a non-null pointer that was allocated with `malloc` and can be freed by the caller.
    unsafe { c_free(ptr.as_ptr().cast()) };

    locale
}
