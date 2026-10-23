// //// Neoffice — added file (no upstream equivalent): the thousands separator of the numbers the editor shows.

//! The thousands separator of the numbers the editor shows, from its locale, as Excel takes it from the regional
//! settings: in a format code, `#,##0.00` asks for the locale's separator, not for a comma. A Swiss locale (`fr-CH`,
//! `de-CH`, `it-CH`, `rm-CH`, and `de-LI`) shows `CHF 5'478.45`; every other keeps `5,478.45`, and the decimal point,
//! which the editor does not localise. The locale is the hosting page's (`?locale=` of the browser build, the Drive's
//! person and company), else the browser's or the system's. The workbook is untouched: the separator is chosen when a
//! value is shown (`gridcraft_numfmt::set_display_grouping`), never when it is computed (`TEXT()`) nor saved.

/// The separator a locale tag asks for (`fr-CH`, `de_CH.UTF-8`, `en`): its region decides.
pub fn grouping_for(tag: &str) -> char {
    match region(tag).as_deref() {
        Some("CH" | "LI") => '\'',
        _ => ',',
    }
}

/// The region of a locale tag, upper case: its first two-letter subtag after the language (`zh-Hant-TW` → `TW`).
fn region(tag: &str) -> Option<String> {
    let tag = tag.split(['.', '@']).next().unwrap_or_default();
    tag.split(['-', '_']).skip(1).find(|part| part.len() == 2 && part.chars().all(|c| c.is_ascii_alphabetic())).map(str::to_ascii_uppercase)
}

/// The locale of the system, or of the browser (`navigator.language`) in the browser build.
pub fn system_tag() -> Option<String> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        sys_locale::get_locale()
    }
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::window().and_then(|window| window.navigator().language())
    }
}

/// Shows numbers with the thousands separator of `tag` (the hosting page's locale), else of the system's.
pub fn apply(tag: Option<&str>) {
    let chosen = tag.map(str::trim).filter(|tag| !tag.is_empty()).map(str::to_string).or_else(system_tag);
    gridcraft_engine::numfmt::set_display_grouping(chosen.as_deref().map_or(',', grouping_for));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_swiss_locale_groups_with_an_apostrophe_and_every_other_with_a_comma() {
        for tag in ["fr-CH", "de-CH", "it-CH", "rm-CH", "de-LI", "de_CH.UTF-8", "fr_ch", "gsw-Latn-CH"] {
            assert_eq!(grouping_for(tag), '\'', "{tag}");
        }
        for tag in ["fr", "fr-FR", "de-DE", "en-US", "en", "zh-Hant-TW", "", "C", "POSIX"] {
            assert_eq!(grouping_for(tag), ',', "{tag}");
        }
    }
}
