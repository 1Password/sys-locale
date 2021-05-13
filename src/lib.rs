//! A library to safely and easily obtain the current locale on the system or for an application.
//!
//! This library currently supports the following platforms:
//! - Android
//! - iOS
//! - macOS
//! - Linux, BSD, and other UNIX variations
//! - WebAssembly
//! - Windows
#![no_std]

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

#[cfg(target_arch = "wasm32")]
mod wasm;
#[cfg(target_arch = "wasm32")]
use wasm as provider;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
use windows as provider;

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

    #[ignore = "must be run seperately from other UNIX tests"]
    #[test]
    fn can_obtain_locale() {
        // Docker doesn't set these. Simulate it instead.
        #[cfg(target_os = "linux")]
        {
            std::env::set_var("LANG", "fr_FR.UTF-8");
            std::env::set_var("LC_ALL", "fr_FR.UTF-8");
        }

        assert!(
            get_locale().is_some(),
            "locale should be present on most systems"
        )
    }
}
