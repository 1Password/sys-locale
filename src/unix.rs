use alloc::string::String;
use core::ptr::NonNull;
use cstr_core::CStr;

const LC_CTYPE: &str = "LC_CTYPE";

pub(crate) fn get() -> Option<String> {
    // SAFETY: `setlocale` is being called with a known category and an empty, null-terminated string.
    let locale = NonNull::new(unsafe { libc::setlocale(libc::LC_ALL, b"\0".as_ptr().cast()) })?;

    // SAFETY: `setlocale` returns a pointer to a NUL terminated string which
    // is then cloned into an owned Rust string, and has otherwise been checked as non-null.
    let code = unsafe { CStr::from_ptr(locale.as_ptr()) };
    let code = code.to_str().ok()?;

    parse_locale_code(code)
}

fn parse_locale_code(code: &str) -> Option<String> {
    // Support two behaviors:
    //
    // - `LC_ALL` and `LANG` are set, so `setlocale` returns a single locale identifier.
    // - `LANG` is set but `LC_ALL` is not, so `setlocale` returns a list of all identifiers.
    //
    // In the first case we have a clear case of which locale to use. In the second, we will fallback to using `LC_CTYPE`
    // as its the most closely related to character localization.
    let code = if !code.contains(LC_CTYPE) {
        code
    } else {
        code.split(';')
            .find(|c| c.starts_with(LC_CTYPE))
            .and_then(|c| c.split('=').last())?
    };

    // Some locales are returned with the char encoding too: `en_US.UTF-8`
    code.splitn(2, '.').next().map(String::from)
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::{get, parse_locale_code};

    const LC_ALL_CONTENTS: &str = "LC_CTYPE=fr_FR.UTF-8;LC_NUMERIC=en_US.UTF-8;LC_TIME=en_US.UTF-8;LC_COLLATE=fr_FR.UTF-8;LC_MONETARY=en_US.UTF-8;LC_MESSAGES=fr_FR.UTF-8;LC_PAPER=en_US.UTF-8;LC_NAME=en_US.UTF-8;LC_ADDRESS=en_US.UTF-8;LC_TELEPHONE=en_US.UTF-8;LC_MEASUREMENT=en_US.UTF-8;LC_IDENTIFICATION=en_US.UTF-8";
    const PARSE_LOCALE: &str = "fr_FR";

    #[test]
    fn parse_lc_all() {
        assert_eq!(
            parse_locale_code(LC_ALL_CONTENTS).as_deref(),
            Some(PARSE_LOCALE)
        );
    }

    #[test]
    fn parse_identifier() {
        let identifier = "fr_FR.UTF-8";
        assert_eq!(parse_locale_code(identifier).as_deref(), Some(PARSE_LOCALE));
    }

    #[test]
    fn parse_non_suffixed_identifier() {
        assert_eq!(
            parse_locale_code(PARSE_LOCALE).as_deref(),
            Some(PARSE_LOCALE)
        )
    }

    #[test]
    fn invalid_locale_doesnt_crash() {
        std::env::set_var("LANG", "invalid");
        std::env::set_var("LC_ALL", "invalid-again");
        assert_eq!(get(), None);
    }
}
