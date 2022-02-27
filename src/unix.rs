use std::env;

const LC_ALL: &str = "LC_ALL";
const LC_CTYPE: &str = "LC_CTYPE";
const LANG: &str = "LANG";

pub(crate) fn get() -> Option<String> {
    let code = env::var(LC_ALL)
        .or_else(|_| env::var(LC_CTYPE))
        .or_else(|_| env::var(LANG))
        .ok()?;

    parse_locale_code(&code)
}

fn parse_locale_code(code: &str) -> Option<String> {
    // Some locales are returned with the char encoding too: `en_US.UTF-8`
    code.splitn(2, '.').next().map(String::from)
}

#[cfg(test)]
mod tests {
    use super::{get, parse_locale_code, LANG, LC_ALL, LC_CTYPE};
    use std::env;

    const PARSE_LOCALE: &str = "fr_FR";

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
    fn env_priority() {
        env::remove_var(LANG);
        env::remove_var(LC_ALL);
        env::remove_var(LC_CTYPE);
        assert_eq!(get(), None);

        // These locale names are technically allowed and some systems may still
        // defined aliases such as these but the glibc sources mention that this
        // should be considered deprecated

        env::set_var(LANG, "invalid");
        assert_eq!(get().as_deref(), Some("invalid"));

        env::set_var(LC_CTYPE, "invalid-also");
        assert_eq!(get().as_deref(), Some("invalid-also"));

        env::set_var(LC_ALL, "invalid-again");
        assert_eq!(get().as_deref(), Some("invalid-again"));
    }
}
