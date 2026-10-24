// //// Neoffice — added file (no upstream equivalent): a number typed with the thousands separator the editor shows.

//! A Swiss editor shows `5'478.45` (the UI crate's `number_locale` sets the separator, numfmt's `display_grouping`),
//! so what the person types back that way is a number too, as Excel reads typed input by the regional settings:
//! `5'478.45`, `-1'000`, `1'234'567.5`. The apostrophes must group the whole part in threes (`12'3` stays text), and a
//! leading apostrophe still forces text. Upstream's parser (`gridcraft_core::parse`) reads commas: the entry is handed
//! to it with commas, so it gets the automatic format a comma entry gets (`#,##0` or `#,##0.00`), which a Swiss editor
//! shows with its apostrophe again. Every other locale keeps an apostrophe as text, and nothing computed changes:
//! `VALUE("5'478")` and arithmetic on text read the same for everyone.

/// The apostrophes a Swiss number is grouped with: the keyboard's, and the typographic one (U+2019) that macOS and
/// CLDR write for `de-CH` and that a pasted number may carry.
const APOSTROPHES: [char; 2] = ['\'', '\u{2019}'];

/// `input` with its thousands separators written as commas, when the editor groups with an apostrophe (`sep`) and
/// `input` is a number grouped with apostrophes: an optional sign, one to three digits, then groups of exactly three,
/// and an optional decimal part. `None` for anything else, which is then read as typed.
pub fn regroup(input: &str, sep: char) -> Option<String> {
    if sep != '\'' {
        return None;
    }
    let t = input.trim();
    let (sign, body) = match t.chars().next()? {
        c @ ('-' | '+') => (Some(c), &t[1..]),
        _ => (None, t),
    };
    let (whole, fraction) = match body.split_once('.') {
        Some((whole, fraction)) => (whole, Some(fraction)),
        None => (body, None),
    };
    if !whole.contains(APOSTROPHES) || fraction.is_some_and(|f| !f.bytes().all(|b| b.is_ascii_digit())) {
        return None;
    }
    let groups: Vec<&str> = whole.split(APOSTROPHES).collect();
    let digits = |g: &str| g.bytes().all(|b| b.is_ascii_digit());
    let first = groups.first()?;
    if first.is_empty() || first.len() > 3 || !digits(first) || groups[1..].iter().any(|g| g.len() != 3 || !digits(g)) {
        return None;
    }
    let mut out: String = sign.map(String::from).unwrap_or_default();
    out.push_str(&groups.join(","));
    if let Some(fraction) = fraction {
        out.push('.');
        out.push_str(fraction);
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_number_grouped_with_apostrophes_reads_with_commas_in_a_swiss_editor() {
        assert_eq!(regroup("5'478.45", '\'').as_deref(), Some("5,478.45"));
        assert_eq!(regroup("-1'000", '\'').as_deref(), Some("-1,000"));
        assert_eq!(regroup("+1'000", '\'').as_deref(), Some("+1,000"));
        assert_eq!(regroup("1'234'567.5", '\'').as_deref(), Some("1,234,567.5"));
        assert_eq!(regroup(" 128'016.30 ", '\'').as_deref(), Some("128,016.30"));
        assert_eq!(regroup("5\u{2019}478.45", '\'').as_deref(), Some("5,478.45"), "the typographic apostrophe");
    }

    #[test]
    fn anything_else_is_read_as_typed() {
        for text in ["12'3", "1'23", "1'2345", "'5478", "5'", "'", "5'478'", "1''000", "a'000", "1'000.4'5", "1'000.x", "", "-", "5478", "5,478"] {
            assert_eq!(regroup(text, '\''), None, "{text:?}");
        }
        assert_eq!(regroup("5'478.45", ','), None, "an editor that groups with a comma keeps the apostrophe as text");
    }
}
