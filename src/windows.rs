use alloc::{string::String, vec, vec::Vec};
use windows_sys::Win32::{
    Globalization::GetUserDefaultLocaleName, System::SystemServices::LOCALE_NAME_MAX_LENGTH,
};

#[allow(clippy::as_conversions)]
pub(crate) fn get() -> Vec<String> {
    let mut locale = vec![0u16; LOCALE_NAME_MAX_LENGTH as usize];
    // SAFETY: The buffer has sufficent capacity to have locales written and is valid to write to.
    let len = unsafe { GetUserDefaultLocaleName(locale.as_mut_ptr(), locale.len() as i32) };
    if len > 1 {
        let len = (len - 1) as usize;
        if let Ok(locale) = String::from_utf16(&locale[..len]) {
            return vec![locale];
        }
    }
    vec![]
}
