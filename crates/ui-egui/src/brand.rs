// //// Neoffice — added file (no upstream equivalent)
//! The product name the interface shows.
//!
//! Neoffice ships this editor under its own name. Our browser builds set `NEOFFICE_PRODUCT_NAME`
//! at compile time (the build step of `.github/workflows/neoffice-web.yml`); without it, as in
//! upstream's builds and every test run, [`name`] is upstream's name and nothing changes.
//!
//! Upstream has no single source for its name, and its English labels are the keys of the
//! translation tables. So [`crate::i18n::Language::tr`] passes its result through [`rebrand`],
//! which swaps the whole word for ours in every language, the message box shows its default
//! title through [`alert_title`], and the few literals left call [`name`]. Names that are stored
//! or compared (storage folders, `GridCraft User`, the `gridcraft:*` messages of a hosting page)
//! never come here.

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

/// Upstream's name for this app.
pub const UPSTREAM: &str = "GridCraft";

/// At most this many rebranded labels are kept (the tables have one per language). Past it, a new
/// label shows upstream's name rather than grow memory.
const MAX_REBRANDED: usize = 1024;

/// The name the interface shows: `NEOFFICE_PRODUCT_NAME` when the build sets it (not blank),
/// upstream's otherwise.
pub fn name() -> &'static str {
    match option_env!("NEOFFICE_PRODUCT_NAME").map(str::trim) {
        Some(ours) if !ours.is_empty() => ours,
        _ => UPSTREAM,
    }
}

/// A translated label with upstream's name (the whole word) as [`name`].
pub fn rebrand(text: &str) -> &str {
    let ours = name();
    if ours == UPSTREAM || !text.contains(UPSTREAM) {
        return text;
    }
    rebranded(text, ours).unwrap_or(text)
}

/// The title of a message box: the app's name, which is upstream's in the code (`("GridCraft",
/// message)`), shows as [`name`]; any other title (a dialog's, a validation rule's) as it is.
pub fn alert_title(title: &str) -> &str {
    if title == UPSTREAM { name() } else { title }
}

/// The rebranded form of `text`, made once and kept for the life of the app (bounded by
/// [`MAX_REBRANDED`]), so a label drawn every frame costs a lookup, not an allocation.
fn rebranded(text: &str, ours: &str) -> Option<&'static str> {
    static DONE: OnceLock<Mutex<HashMap<String, &'static str>>> = OnceLock::new();
    let mut done = DONE.get_or_init(Default::default).lock().ok()?;
    if let Some(&kept) = done.get(text) {
        return Some(kept);
    }
    if done.len() >= MAX_REBRANDED {
        return None;
    }
    let made: &'static str = replace_word(text, UPSTREAM, ours).leak();
    done.insert(text.to_owned(), made);
    Some(made)
}

/// `text` with every whole-word `from` replaced by `to`. Only ASCII letters and digits join a
/// word, so `GridCraftについて` (Japanese) is replaced and `GridCrafts` or `MyGridCraft` are not.
fn replace_word(text: &str, from: &str, to: &str) -> String {
    let joins = |c: Option<char>| c.is_some_and(|c| c.is_ascii_alphanumeric());
    let mut out = String::with_capacity(text.len() + to.len());
    let mut rest = text;
    let mut last: Option<char> = None;
    while let Some((before, after)) = rest.split_once(from) {
        out.push_str(before);
        let left = before.chars().next_back().or(last);
        out.push_str(if joins(left) || joins(after.chars().next()) { from } else { to });
        last = from.chars().next_back();
        rest = after;
    }
    out.push_str(rest);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn without_the_variable_nothing_changes() {
        if option_env!("NEOFFICE_PRODUCT_NAME").is_none_or(|n| n.trim().is_empty()) {
            assert_eq!(name(), UPSTREAM);
            assert_eq!(rebrand("Über\nGridCraft"), "Über\nGridCraft");
            assert_eq!(alert_title("GridCraft"), "GridCraft");
        }
        assert_eq!(alert_title("Sort"), "Sort");
    }

    #[test]
    fn replaces_the_whole_word_only() {
        assert_eq!(replace_word("About\nGridCraft", UPSTREAM, "NeoGrille"), "About\nNeoGrille");
        assert_eq!(replace_word("GridCraft について", UPSTREAM, "NeoGrille"), "NeoGrille について");
        assert_eq!(replace_word("GridCraft\n정보", UPSTREAM, "NeoGrille"), "NeoGrille\n정보");
        assert_eq!(replace_word("GridCrafts MyGridCraft", UPSTREAM, "NeoGrille"), "GridCrafts MyGridCraft");
        assert_eq!(replace_word("How agents drive GridCraft (MCP)", UPSTREAM, "NeoGrille"), "How agents drive NeoGrille (MCP)");
        assert_eq!(replace_word("getartcraft.com/apps/gridcraft", UPSTREAM, "NeoGrille"), "getartcraft.com/apps/gridcraft");
    }

    #[test]
    fn a_rebranded_label_is_made_once() {
        let first = rebranded("About\nGridCraft (test)", "NeoGrille");
        assert_eq!(first, Some("About\nNeoGrille (test)"));
        let again = rebranded("About\nGridCraft (test)", "NeoGrille");
        assert!(first.zip(again).is_some_and(|(a, b)| std::ptr::eq(a, b)), "the same string, not a new one");
    }
}
