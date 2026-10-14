// //// Neoffice — added file (no upstream equivalent): Neoffice's own interface strings, kept out of upstream's catalogs.

//! The strings Neoffice added to GridCraft (the message the hosting page's bridge shows, see
//! `apps/gridcraft-web/src/host.rs`), kept outside upstream's catalogs. [`super::Language::tr`] asks
//! here when the language's own catalog has no entry for a string, so an upstream merge never
//! touches these, and a catalog that gains the same English string wins over this table.
//!
//! French is the first language here: Neoffice's customers read it. The other languages show these
//! strings in English until someone adds them.

/// Neoffice's strings in French, English first.
const FRENCH: &[(&str, &str)] = &[("Couldn't save {name}: {error}", "Impossible d'enregistrer {name} : {error}")];

/// `text` in French, when Neoffice has translated it.
pub(super) fn french(text: &str) -> Option<&'static str> {
    FRENCH.iter().find(|(english, _)| *english == text).map(|(_, french)| *french)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::Language;

    /// The `{placeholders}` of a string, in order.
    fn placeholders(text: &str) -> Vec<&str> {
        text.split('{').skip(1).filter_map(|rest| rest.split_once('}').map(|(name, _)| name)).collect()
    }

    #[test]
    fn every_french_string_keeps_the_placeholders_of_its_english() {
        for (english, french) in FRENCH {
            assert_eq!(placeholders(english), placeholders(french), "{english:?}");
            assert_eq!(french.trim(), *french, "stray whitespace in {french:?}");
            assert!(!french.contains("..."), "use … rather than three dots: {french:?}");
        }
    }

    #[test]
    fn the_english_strings_are_listed_once() {
        let mut seen = std::collections::HashSet::new();
        for (english, _) in FRENCH {
            assert!(seen.insert(*english), "{english:?} is listed twice");
        }
    }

    #[test]
    fn only_french_has_them_for_now() {
        let refusal = "Couldn't save {name}: {error}";
        assert_eq!(Language::Fr.tr(refusal), "Impossible d'enregistrer {name} : {error}");
        for language in Language::ALL.into_iter().filter(|l| *l != Language::Fr) {
            assert_eq!(language.tr(refusal), refusal, "{language:?}");
        }
        assert_eq!(french("A string upstream has"), None);
    }
}
