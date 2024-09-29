use std::{env, ffi::OsStr};

const LC_ALL: &str = "LC_ALL";
const LC_CTYPE: &str = "LC_CTYPE";
const LANG: &str = "LANG";

/// Environment variable access abstraction to allow testing without
/// mutating env variables.
///
/// Use [StdEnv] to query [std::env]
trait EnvAccess {
    /// See also [std::env::var]
    fn get(&self, key: impl AsRef<OsStr>) -> Option<String>;
}

/// Proxy to [std::env]
struct StdEnv;
impl EnvAccess for StdEnv {
    fn get(&self, key: impl AsRef<OsStr>) -> Option<String> {
        env::var(key).ok()
    }
}

pub(crate) fn get() -> impl Iterator<Item = String> {
    _get(&StdEnv).into_iter()
}

fn _get(env: &impl EnvAccess) -> Option<String> {
    let code = env
        .get(LC_ALL)
        .filter(|val| !val.is_empty())
        .or_else(|| env.get(LC_CTYPE))
        .filter(|val| !val.is_empty())
        .or_else(|| env.get(LANG))?;

    Some(posix_to_bcp47(&code))
}

/// Converts a POSIX locale string to a BCP 47 locale string.
///
/// This function processes the input `code` by removing any character encoding
/// (the part after the `.` character) and any modifiers (the part after the `@` character).
/// It replaces underscores (`_`) with hyphens (`-`) to conform to BCP 47 formatting.
///
/// If the locale is already in the BCP 47 format, no changes are made.
///
/// Useful links:
/// - [The Open Group Base Specifications Issue 8 - 7. Locale](https://pubs.opengroup.org/onlinepubs/9799919799/basedefs/V1_chap07.html)
/// - [The Open Group Base Specifications Issue 8 - 8. Environment Variables](https://pubs.opengroup.org/onlinepubs/9799919799/basedefs/V1_chap08.html)
/// - [BCP 47 specification](https://www.ietf.org/rfc/bcp/bcp47.html)
///
/// # Examples
///
/// ```ignore
/// let bcp47 = posix_to_bcp47("en-US"); // already BCP 47
/// assert_eq!(bcp47, "en-US"); // no changes
///
/// let bcp47 = posix_to_bcp47("en_US");
/// assert_eq!(bcp47, "en-US");
///
/// let bcp47 = posix_to_bcp47("ru_RU.UTF-8");
/// assert_eq!(bcp47, "ru-RU");
///
/// let bcp47 = posix_to_bcp47("fr_FR@dict");
/// assert_eq!(bcp47, "fr-FR");
///
/// let bcp47 = posix_to_bcp47("de_DE.UTF-8@euro");
/// assert_eq!(bcp47, "de-DE");
/// ```
///
/// # TODO
///
/// 1. Implement POSIX to BCP 47 modifier conversion (see https://github.com/1Password/sys-locale/issues/32).
/// 2. Optimize to avoid creating a new buffer (see https://github.com/1Password/sys-locale/pull/33).
fn posix_to_bcp47(locale: &str) -> String {
    locale
        .chars()
        .take_while(|&c| c != '.' && c != '@')
        .map(|c| if c == '_' { '-' } else { c })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{posix_to_bcp47, EnvAccess, _get, LANG, LC_ALL, LC_CTYPE};
    use std::{
        collections::HashMap,
        ffi::{OsStr, OsString},
    };

    type MockEnv = HashMap<OsString, String>;
    impl EnvAccess for MockEnv {
        fn get(&self, key: impl AsRef<OsStr>) -> Option<String> {
            self.get(key.as_ref()).cloned()
        }
    }

    const BCP_47: &str = "fr-FR";
    const POSIX: &str = "fr_FR";
    const POSIX_ENC: &str = "fr_FR.UTF-8";
    const POSIX_MOD: &str = "fr_FR@euro";
    const POSIX_ENC_MOD: &str = "fr_FR.UTF-8@euro";

    #[test]
    fn parse_identifier() {
        assert_eq!(posix_to_bcp47(BCP_47), BCP_47);
        assert_eq!(posix_to_bcp47(POSIX), BCP_47);
        assert_eq!(posix_to_bcp47(POSIX_ENC), BCP_47);
        assert_eq!(posix_to_bcp47(POSIX_MOD), BCP_47);
        assert_eq!(posix_to_bcp47(POSIX_ENC_MOD), BCP_47);
    }

    #[test]
    fn env_priority() {
        let mut env = MockEnv::new();
        assert_eq!(_get(&env), None);

        // These locale names are technically allowed and some systems may still
        // defined aliases such as these but the glibc sources mention that this
        // should be considered deprecated

        env.insert(LANG.into(), "invalid".to_owned());
        assert_eq!(_get(&env).as_deref(), Some("invalid"));

        env.insert(LC_CTYPE.into(), "invalid-also".to_owned());
        assert_eq!(_get(&env).as_deref(), Some("invalid-also"));

        env.insert(LC_ALL.into(), "invalid-again".to_owned());
        assert_eq!(_get(&env).as_deref(), Some("invalid-again"));
    }

    #[test]
    fn env_skips_empty_options() {
        let mut env = MockEnv::new();
        assert_eq!(_get(&env), None);

        // Skip the 1st of three variables.
        env.insert(LC_ALL.into(), String::new());
        env.insert(LC_CTYPE.into(), BCP_47.to_owned());

        let set_locale = _get(&env).unwrap();
        assert_eq!(set_locale, BCP_47);
        assert_eq!(posix_to_bcp47(&set_locale), BCP_47);

        // Ensure the 2nd will be skipped when empty as well.
        env.insert(LC_CTYPE.into(), String::new());
        env.insert(LANG.into(), BCP_47.to_owned());

        let set_locale = _get(&env).unwrap();
        assert_eq!(set_locale, BCP_47);
        assert_eq!(posix_to_bcp47(&set_locale), BCP_47);
    }
}
