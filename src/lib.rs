//! A library to safely and easily obtain the current locale on the system or for an application.
//!
//! This library currently supports the following platforms:
//! - Android
//! - iOS
//! - macOS
//! - Linux, BSD, and other UNIX variations
//! - WebAssembly on the web (via the `js` feature)
//! - Windows
#![cfg_attr(
    any(
        not(unix),
        target_os = "macos",
        target_os = "ios",
        target_os = "android"
    ),
    no_std
)]
extern crate alloc;
use alloc::string::String;

#[cfg(target_os = "android")]
mod android;
#[cfg(target_os = "android")]
use android as provider;

#[cfg(any(target_os = "macos", target_os = "ios"))]
mod apple;
#[cfg(any(target_os = "macos", target_os = "ios"))]
use apple as provider;

#[cfg(all(
    unix,
    not(any(target_os = "macos", target_os = "ios", target_os = "android"))
))]
mod unix;
#[cfg(all(
    unix,
    not(any(target_os = "macos", target_os = "ios", target_os = "android"))
))]
use unix as provider;

#[cfg(all(target_family = "wasm", feature = "js", not(unix)))]
mod wasm;
#[cfg(all(target_family = "wasm", feature = "js", not(unix)))]
use wasm as provider;

#[cfg(windows)]
mod windows;
#[cfg(windows)]
use windows as provider;

#[cfg(not(any(unix, all(target_family = "wasm", feature = "js", not(unix)), windows)))]
mod provider {
    pub fn get() -> Option<alloc::string::String> {
        None
    }
}

/// Returns the active locale for the system or application.
///
/// # Returns
///
/// Returns `Some(String)` with a BCP-47 language tag inside. If the locale
/// couldn't be obtained, `None` is returned instead.
///
/// # Example
///
/// ```no_run
/// use sys_locale::get_locale;
///
/// let current_locale = get_locale().unwrap_or_else(|| String::from("en-US"));
///
/// println!("The locale is {}", current_locale);
/// ```
pub fn get_locale() -> Option<String> {
    provider::get()
}

#[cfg(test)]
mod tests {
    use super::get_locale;
    extern crate std;

    #[test]
    fn can_obtain_locale() {
        let locale = get_locale().expect("locale should be present on most systems");
        assert!(!locale.is_empty(), "locale string was empty");
        assert!(!locale.ends_with('\0'), "locale contained trailing NUL");
    }
}
