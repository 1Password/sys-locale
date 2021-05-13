use alloc::{string::String, vec};
use winapi::um::{winnls::GetUserDefaultLocaleName, winnt::LOCALE_NAME_MAX_LENGTH};

#[allow(clippy::as_conversions)]
pub(crate) fn get() -> Option<String> {
    let mut locale = vec![0u16; LOCALE_NAME_MAX_LENGTH];
    // SAFETY: The buffer has sufficent capacity to have locales written and is valid to write to.
    let len = unsafe { GetUserDefaultLocaleName(locale.as_mut_ptr(), locale.len() as i32) };
    if len > 1 {
        let len = (len - 1) as usize;
        String::from_utf16(&locale[..len]).ok()
    } else {
        None
    }
}
