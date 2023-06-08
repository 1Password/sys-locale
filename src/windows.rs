use alloc::{string::String, vec::Vec};
use windows_sys::Win32::Globalization::{GetUserPreferredUILanguages, MUI_LANGUAGE_NAME};

#[allow(clippy::as_conversions)]
pub(crate) fn get() -> impl Iterator<Item = String> {
    let mut num_languages: u32 = 0;
    let mut buffer_length: u32 = 0;

    // Calling this with null buffer will retrieve the required buffer length
    unsafe {
        GetUserPreferredUILanguages(
            MUI_LANGUAGE_NAME,
            &mut num_languages,
            core::ptr::null_mut(),
            &mut buffer_length,
        )
    };

    let mut buffer = Vec::<u16>::new();
    buffer.resize(buffer_length as usize, 0);

    // Now that we have an appropriate buffer, we can query the names
    let mut result = Vec::with_capacity(num_languages as usize);
    let success = unsafe {
        GetUserPreferredUILanguages(
            MUI_LANGUAGE_NAME,
            &mut num_languages,
            buffer.as_mut_ptr(),
            &mut buffer_length,
        )
    } != 0;

    if success {
        // The buffer contains names split by null char (0), and ends with two null chars (00)
        for part in buffer.split(|i| i == &0) {
            if let Ok(locale) = String::from_utf16(part) {
                if !locale.is_empty() {
                    result.push(locale);
                }
            }
        }
    }

    result.into_iter()
}
