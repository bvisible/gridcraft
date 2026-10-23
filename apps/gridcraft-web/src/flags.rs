// //// Neoffice — added file (no upstream equivalent): the page's URL flags, read as parameters of the URL.

//! The flags the page passes in its URL (`?webgl`, `?sample=sales`, `?nora`), read as parameters by the
//! browser's `URLSearchParams`, as the host bridge reads `?host` and `?author`. A flag is a parameter's
//! name, never a word found somewhere in the query: upstream's `contains` took
//! `?nora=https://sample.example` for a request of the sales sample.

/// What the page's URL asks for.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Flags {
    /// `?webgl`: WebGL2 instead of WebGPU.
    pub webgl: bool,
    /// `?sample` (the sales sample) or `?sample=<name>`.
    pub sample: Option<String>,
    /// `?nora` or `?nora=<origin>`: the value as given, empty for a bare `?nora` (see
    /// `nora::origin_from_url`).
    pub nora: Option<String>,
    /// `?locale=fr-CH`: the locale of the hosting page, whose region chooses the thousands separator of the numbers
    /// shown (`number_locale`); the browser's otherwise.
    pub locale: Option<String>,
}

impl Flags {
    /// The flags, from `get`: a parameter's value by its name (`URLSearchParams.get`), `None` when the
    /// URL has no such parameter, `Some("")` for one without a value.
    pub fn read(get: impl Fn(&str) -> Option<String>) -> Flags {
        Flags {
            webgl: get("webgl").is_some(),
            sample: get("sample").map(|name| if name.trim().is_empty() { "sales".to_string() } else { name }),
            nora: get("nora"),
            locale: get("locale"),
        }
    }

    /// The flags of the query `search` (`location.search`), its parameters read by the browser.
    #[cfg(target_arch = "wasm32")]
    pub fn parse(search: &str) -> Flags {
        let params = web_sys::UrlSearchParams::new_with_str(search).ok();
        Flags::read(|name| params.as_ref().and_then(|params| params.get(name)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `URLSearchParams.get`, for the plain queries of these tests (nothing to decode in them).
    fn read(query: &str) -> Flags {
        Flags::read(|name| {
            query.trim_start_matches('?').split('&').filter(|item| !item.is_empty()).find_map(|item| {
                let (key, value) = item.split_once('=').unwrap_or((item, ""));
                (key == name).then(|| value.to_string())
            })
        })
    }

    #[test]
    fn a_flag_is_a_parameter_of_the_url_not_a_word_in_it() {
        // A value that happens to mention a flag asks for nothing.
        assert_eq!(read("?nora=https://sample.example"), Flags { nora: Some("https://sample.example".into()), ..Flags::default() });
        assert_eq!(read("?host=parent&author=webgl-fan"), Flags::default());
        // The flags themselves.
        assert_eq!(read("?webgl"), Flags { webgl: true, ..Flags::default() });
        // //// Neoffice — the hosting page's locale (number_locale).
        assert_eq!(read("?host=parent&locale=fr-CH"), Flags { locale: Some("fr-CH".into()), ..Flags::default() });
        assert_eq!(read("?sample"), Flags { sample: Some("sales".into()), ..Flags::default() });
        assert_eq!(read("?sample=budget&nora"), Flags { sample: Some("budget".into()), nora: Some(String::new()), ..Flags::default() });
        assert_eq!(read(""), Flags::default());
    }
}
