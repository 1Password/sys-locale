use alloc::{string::String, vec::Vec};
use core_foundation_sys::{
    array::{CFArrayGetCount, CFArrayGetValueAtIndex, CFArrayRef},
    base::{Boolean, CFIndex, CFRange, CFRelease},
    string::{kCFStringEncodingUTF8, CFStringGetBytes, CFStringGetLength, CFStringRef},
};

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    fn CFLocaleCopyPreferredLanguages() -> CFArrayRef;
}

pub(crate) fn get() -> Option<String> {
    let preferred_langs = unsafe {
        // SAFETY: This function is safe to call and has no invariants. Any value inside the
        // array will be owned by us.
        let langs = CFLocaleCopyPreferredLanguages();
        if !langs.is_null() {
            let langs = CFArray(langs);
            // SAFETY: The returned array is a valid CFArray object.
            if CFArrayGetCount(langs.0) != 0 {
                langs
            } else {
                return None;
            }
        } else {
            return None;
        }
    };

    #[allow(clippy::as_conversions)]
    unsafe {
        // SAFETY: The array has been checked that it contains at least one value.
        let locale = CFArrayGetValueAtIndex(preferred_langs.0, 0) as CFStringRef;

        // SAFETY: `locale` is a valid CFString pointer because the array will always contain a value.
        let str_len = CFStringGetLength(locale);

        let range = CFRange {
            location: 0,
            length: str_len,
        };

        let mut capacity = 0;
        // SAFETY:
        // - `locale` is a valid CFString
        // - The supplied range is within the length of the string.
        // - `capacity` is writable.
        // Passing NULL and `0` is correct for the buffer to get the
        // encoded output length.
        CFStringGetBytes(
            locale,
            range,
            kCFStringEncodingUTF8,
            0,
            false as Boolean,
            core::ptr::null_mut(),
            0,
            &mut capacity,
        );

        // Guard against a zero-sized allocation, if that were to somehow occur.
        if capacity == 0 {
            return None;
        }

        // Note: This is the number of bytes (u8) that will be written to
        // the buffer, not the number of codepoints they would contain.
        let mut buffer = Vec::with_capacity(capacity as usize);

        // SAFETY:
        // - `locale` is a valid CFString
        // - The supplied range is within the length of the string.
        // - `buffer` is writable and has sufficent capacity to receive the data.
        // - `maxBufLen` is correctly based on `buffer`'s available capacity.
        // - `out_len` is writable.
        let mut out_len = 0;
        CFStringGetBytes(
            locale,
            range,
            kCFStringEncodingUTF8,
            0,
            false as Boolean,
            buffer.as_mut_ptr(),
            capacity as CFIndex,
            &mut out_len,
        );

        // Sanity check that both calls to `CFStringGetBytes`
        // were equivalent. If they weren't, the system is doing
        // something very wrong...
        assert!(out_len <= capacity);

        // SAFETY: The system has written `out_len` elements, so they are
        // initialized and inside the buffer's capacity bounds.
        buffer.set_len(out_len as usize);

        // This should always contain UTF-8 since we told the system to
        // write UTF-8 into the buffer, but the value is small enough that
        // using `from_utf8_unchecked` isn't worthwhile.
        String::from_utf8(buffer).ok()
    }
}

struct CFArray(CFArrayRef);

impl Drop for CFArray {
    fn drop(&mut self) {
        // SAFETY: This wrapper contains a valid CFArray.
        unsafe { CFRelease(self.0.cast()) }
    }
}
