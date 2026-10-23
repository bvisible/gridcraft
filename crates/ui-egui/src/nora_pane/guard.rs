// //// Neoffice — added file (no upstream equivalent): what a command of Nora's really wrote, read before it stays.

//! The links and formulas a change of Nora's left in the workbook, read once the change is made.
//!
//! The Neoffice server refuses a link to a file in what Nora asks for, but it reads only the parameters
//! of a command: a command can compose a link itself (a formula that builds its target, a replace that
//! turns a web address into a path, a fill that copies a link). GridCraft computes a `HYPERLINK` formula
//! as its text and refuses to open a link to a file when it is clicked, but the workbook it saves keeps
//! both, and Excel opens them. So after every change of Nora's the pane reads what the workbook holds
//! that leads outside it, and takes the change back when it holds more of it than before:
//!
//! - a cell's link that the file saves as an address outside the workbook (Insert › Link, `external`)
//!   and that is no web page or e-mail address ([`craft_nora::links::web_or_mail`]);
//! - a `HYPERLINK` formula whose target is not written out as a web page, an e-mail address or `#` and
//!   a place of the workbook (a target computed from cells or texts is not read, so it is refused), or
//!   `HYPERLINK` passed bare to a function (`MAP(…, HYPERLINK)`);
//! - a formula that calls outside the workbook when Excel computes it (`WEBSERVICE`, `IMAGE`, `RTD`,
//!   `CALL`, `REGISTER`, `REGISTER.ID`), when LibreOffice does (`DDE`) or Google Sheets (`IMPORTXML`,
//!   `IMPORTHTML`, `IMPORTDATA`, `IMPORTFEED`, `IMPORTRANGE`: a saved workbook is opened there too), or
//!   that reads another file (a sheet named with `[`, `]`, `\`, `/`, `:`, `?` or `*`, which no sheet of a
//!   workbook can be);
//! - a place of the workbook (`#…`, or a link the file saves as a place) that names another file
//!   (`#[Book.xlsx]Sheet1!A1`): Excel opens that file;
//! - a formula GridCraft cannot read that holds one of those, a `|` (a DDE call to a program) or a `[`.
//!
//! The cells, the defined names, the data validations, the conditional formats, the tables' calculated
//! columns, the charts' series and the pivot tables' sources and calculated fields are read. What was
//! already there (the person's own links, moved or not) is counted, not refused: only a change that adds
//! to it is.

use std::collections::BTreeMap;

use craft_nora::links;
use gridcraft_engine::formula::{Expr, Reference, SheetSel};
use gridcraft_engine::model::{CfRule, CfValueKind, Hyperlink, Sheet, Workbook};

/// The functions that reach outside the workbook when Excel computes them: a web service, a picture
/// from an address, a program (real-time data, a function of a library).
const OUTSIDE: &[&str] =
    &["WEBSERVICE", "IMAGE", "RTD", "CALL", "REGISTER", "REGISTER.ID", "DDE", "IMPORTXML", "IMPORTHTML", "IMPORTDATA", "IMPORTFEED", "IMPORTRANGE"];

/// What no sheet name can hold: a reference on such a "sheet" is to another file.
const NOT_IN_A_SHEET_NAME: &[char] = &['[', ']', '\\', '/', ':', '?', '*'];

/// What a place of the workbook (`#Sheet2!A1`, `Total`) cannot hold and a place in another file does
/// (`#[Book.xlsx]Sheet1!A1`, `#'\\srv\s\[x.xlsx]S'!A1`).
const ANOTHER_FILE: &[char] = &['[', ']', '\\', '/'];

/// The longest text of a target or a formula quoted back to Nora.
const MAX_QUOTED_CHARS: usize = 200;

/// The most of them listed in one refusal.
const MAX_LISTED: usize = 5;

/// One thing the workbook holds that leads outside it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Reach {
    /// What it is, to count it whatever cell holds it: the target, the function, the other file.
    key: String,
    /// What Nora is told it is.
    pub(super) what: String,
    /// Where it is: a cell, a defined name, a validation.
    pub(super) place: String,
}

/// What `after` holds that leads outside the workbook more often than `before` held it: what a change
/// of Nora's added. Only the sheets the change touched are read first (a sheet the change left alone is
/// the very same one); the whole workbook is counted only when one of them holds something.
pub(super) fn added(before: &Workbook, after: &Workbook) -> Vec<Reach> {
    let mut touched = Vec::new();
    for (at, sheet) in after.sheets.iter().enumerate() {
        if !before.sheets.get(at).is_some_and(|old| std::sync::Arc::ptr_eq(old, sheet)) {
            read_sheet(sheet, &mut touched);
        }
    }
    if before.names != after.names {
        read_names(after, &mut touched);
    }
    if touched.is_empty() {
        return Vec::new();
    }
    let mut held: BTreeMap<String, usize> = BTreeMap::new();
    for reach in all(before) {
        *held.entry(reach.key).or_default() += 1;
    }
    let mut new = Vec::new();
    for reach in all(after) {
        match held.get_mut(&reach.key) {
            Some(count) if *count > 0 => *count -= 1,
            _ => new.push(reach),
        }
    }
    new
}

/// What Nora is told when her change is taken back: what `what` wrote, and what she may write instead.
pub(super) fn refusal(what: &str, found: &[Reach]) -> String {
    let mut listed: Vec<String> = found.iter().take(MAX_LISTED).map(|reach| format!("{} ({})", reach.what, reach.place)).collect();
    if found.len() > MAX_LISTED {
        listed.push(format!("and {} more", found.len() - MAX_LISTED));
    }
    format!(
        "{what} was undone, nothing of it is left: it wrote {}. A link of yours goes to a web page, an e-mail address or a place of the \
workbook, never to a file; a formula of yours never calls outside the workbook (WEBSERVICE, IMAGE, RTD, CALL, REGISTER) nor reads \
another file nor names a place in one. Tell the person; do not look for another way to write it.",
        listed.join("; ")
    )
}

/// Everything `wb` holds that leads outside it.
fn all(wb: &Workbook) -> Vec<Reach> {
    let mut found = Vec::new();
    for sheet in &wb.sheets {
        read_sheet(sheet, &mut found);
    }
    read_names(wb, &mut found);
    found
}

fn read_sheet(sheet: &Sheet, found: &mut Vec<Reach>) {
    let on = |at: String| format!("{}!{at}", gridcraft_engine::formula::quote_sheet(&sheet.name));
    for (at, link) in &sheet.hyperlinks {
        if !link_stays(link) {
            let target = quoted(link.target.trim());
            found.push(Reach { key: format!("link:{}", link.target.trim()), what: format!("a link to {target}"), place: on(at.a1()) });
        }
    }
    for (at, cell) in sheet.cells.iter() {
        if let Some(formula) = &cell.formula {
            // Parsed again when the workbook came from a file (the tree is not kept there), so that a formula is
            // the same reach before and after the engine parses it.
            let expr = formula.expr_arc();
            read_formula(expr.as_deref(), &formula.text, &on(at.a1()), found);
        }
    }
    let first = |ranges: &[gridcraft_engine::core::RangeRef]| on(ranges.first().map(|r| r.a1()).unwrap_or_default());
    for validation in &sheet.validations {
        let place = format!("the validation of {}", first(validation.ranges.as_slice()));
        for text in std::iter::once(&validation.f1).chain(validation.f2.as_ref()) {
            read_text(text, &place, found);
        }
    }
    for conditional in &sheet.cond_formats {
        let place = format!("the conditional format of {}", first(conditional.ranges.as_slice()));
        match &conditional.rule {
            CfRule::CellIs { a, b, .. } => {
                for text in std::iter::once(a).chain(b.as_ref()) {
                    read_text(text, &place, found);
                }
            }
            CfRule::Expression { formula, .. } => read_text(formula, &place, found),
            // A colour scale's, a data bar's or an icon set's threshold may be a formula.
            CfRule::ColorScale { stops } => {
                for (value, _) in stops {
                    read_threshold(value, &place, found);
                }
            }
            CfRule::DataBar { min, max, .. } => {
                read_threshold(min, &place, found);
                read_threshold(max, &place, found);
            }
            CfRule::IconSet { thresholds, .. } => {
                for value in thresholds {
                    read_threshold(value, &place, found);
                }
            }
            _ => {}
        }
    }
    for table in &sheet.tables {
        for column in &table.columns {
            let place = format!("the column {} of the table {}", column.name, table.name);
            if let Some(formula) = &column.formula {
                read_text(formula, &place, found);
            }
            // The totals row's label, or its custom formula: a label is read only when it reads as a formula.
            if let Some(label) = &column.totals_label {
                read_reference(label, &place, found);
            }
        }
    }
    for (at, chart) in sheet.charts.iter().enumerate() {
        let place = format!("chart {} of {}", at + 1, gridcraft_engine::formula::quote_sheet(&sheet.name));
        for text in chart.source.iter().chain(chart.series.iter().flat_map(|series| {
            std::iter::once(&series.values).chain(series.name.as_ref()).chain(series.categories.as_ref()).chain(series.bubble_sizes.as_ref())
        })) {
            read_reference(text, &place, found);
        }
    }
    for pivot in &sheet.pivots {
        let place = format!("the pivot table {}", pivot.name);
        read_reference(&pivot.source, &place, found);
        for field in &pivot.calculated_fields {
            read_text(&field.formula, &place, found);
        }
    }
}

fn read_names(wb: &Workbook, found: &mut Vec<Reach>) {
    for name in &wb.names {
        read_text(&name.formula, &format!("the name {}", name.name), found);
    }
}

/// A formula kept as text (a defined name, a validation), read as GridCraft reads it.
fn read_text(text: &str, place: &str, found: &mut Vec<Reach>) {
    let body = text.trim().trim_start_matches('=');
    if body.is_empty() {
        return;
    }
    let expr = gridcraft_engine::formula::parse(body).ok();
    read_formula(expr.as_ref(), body, place, found);
}

/// A threshold of a conditional format, when it is a formula.
fn read_threshold(value: &CfValueKind, place: &str, found: &mut Vec<Reach>) {
    if let CfValueKind::Formula(text) = value {
        read_text(text, place, found);
    }
}

/// A range a chart or a pivot table reads (`Sheet1!$B$2:$B$9`), or a series' literal name (`Ventes [CHF]`): read as
/// a formula when GridCraft reads it as one; else by its text only when it reads as a reference (a `!`, or a `|`),
/// so that a name with brackets is not taken for another file.
fn read_reference(text: &str, place: &str, found: &mut Vec<Reach>) {
    let body = text.trim().trim_start_matches('=');
    match gridcraft_engine::formula::parse(body) {
        Ok(expr) => read_formula(Some(&expr), body, place, found),
        Err(_) if body.contains(['!', '|']) => read_formula(None, body, place, found),
        Err(_) => {}
    }
}

/// What a formula reaches outside the workbook: read in its syntax tree, or, for one GridCraft cannot
/// read (Excel will, its own way), in its text.
fn read_formula(expr: Option<&Expr>, text: &str, place: &str, found: &mut Vec<Reach>) {
    let Some(expr) = expr else {
        if unread_reaches(text) {
            let text = quoted(text);
            found.push(Reach {
                key: format!("formula:{text}"),
                what: format!("a formula that reaches outside the workbook ({text})"),
                place: place.to_string(),
            });
        }
        return;
    };
    let mut push = |key: String, what: String| found.push(Reach { key, what, place: place.to_string() });
    expr.walk(&mut |node| match node {
        Expr::Call(name, args) if name == "HYPERLINK" => match args.first().map(unparen) {
            Some(Expr::Text(target)) if formula_target_stays(target) => {}
            Some(Expr::Text(target)) => {
                push(format!("hyperlink:{target}"), format!("a HYPERLINK formula to {}", quoted(target)));
            }
            _ => push("hyperlink:".to_string(), "a HYPERLINK formula whose target is not written out in quotes".to_string()),
        },
        Expr::Call(name, _) if OUTSIDE.contains(&name.as_str()) => push(format!("call:{name}"), format!("{name}, which calls outside the workbook")),
        Expr::Name(name) if name.eq_ignore_ascii_case("HYPERLINK") => {
            push("hyperlink:".to_string(), "HYPERLINK passed to another function".to_string());
        }
        Expr::Ref(reference) => {
            if let Some(sheet) = other_file(reference) {
                push(format!("file:{sheet}"), format!("a reference to another file ({})", quoted(&sheet)));
            }
        }
        _ => {}
    });
}

fn unparen(expr: &Expr) -> &Expr {
    match expr {
        Expr::Paren(inner) => unparen(inner),
        other => other,
    }
}

/// The "sheet" of a reference that no sheet can be: another file's.
fn other_file(reference: &Reference) -> Option<String> {
    let (first, last) = match &reference.sheet {
        SheetSel::Current => return None,
        SheetSel::Named(name) => (name, None),
        SheetSel::Span(first, last) => (first, Some(last)),
    };
    std::iter::once(first).chain(last).find(|name| name.contains(NOT_IN_A_SHEET_NAME)).cloned()
}

/// Whether a cell's link stays in the workbook or goes to a web page or an e-mail address: the file
/// saves it as an address when Insert › Link found it outside the workbook, or when it reads as an
/// address (the writer's own test), and as a place of the workbook otherwise.
fn link_stays(link: &Hyperlink) -> bool {
    let target = link.target.trim();
    if target.is_empty() {
        return true;
    }
    if links::hides_or_controls(target) {
        return false;
    }
    let lower = target.to_ascii_lowercase();
    let address = link.external || target.contains("://") || lower.starts_with("mailto:") || target.starts_with("\\\\") || lower.starts_with("file:");
    if address {
        return links::web_or_mail(target);
    }
    // Saved as a place of the workbook (`location`, without its `#`): no place of this workbook names another file.
    !target.contains(ANOTHER_FILE)
}

/// Whether a `HYPERLINK` formula's written target is a web page, an e-mail address or `#` and a place
/// of the workbook (without `#`, Excel reads a place as a file). An empty one leads nowhere.
fn formula_target_stays(target: &str) -> bool {
    let target = target.trim_matches([' ', '\t', '\n', '\r']);
    if target.is_empty() {
        return true;
    }
    if links::hides_or_controls(target) {
        return false;
    }
    match target.strip_prefix('#') {
        Some(place) => !place.contains("://") && !place.contains(ANOTHER_FILE),
        None => links::web_or_mail(target),
    }
}

/// Whether a formula GridCraft cannot read holds, outside its quoted texts, what could reach outside
/// the workbook once Excel reads it: a `|` (DDE), a `[` or a `\` (another file), `HYPERLINK` or a
/// function of [`OUTSIDE`].
fn unread_reaches(text: &str) -> bool {
    let bare = outside_quotes(text);
    if bare.contains(['|', '[', '\\']) {
        return true;
    }
    let upper = bare.to_ascii_uppercase();
    std::iter::once("HYPERLINK").chain(OUTSIDE.iter().copied()).any(|name| upper.contains(name))
}

/// `text` without its quoted texts (`"…"`, a `""` inside one being a quote).
fn outside_quotes(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut quoted = false;
    for c in text.chars() {
        if c == '"' {
            quoted = !quoted;
        } else if !quoted {
            out.push(c);
        }
    }
    out
}

/// `text` on one line, cut at [`MAX_QUOTED_CHARS`].
fn quoted(text: &str) -> String {
    let line = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if line.chars().count() <= MAX_QUOTED_CHARS {
        return line;
    }
    let mut cut: String = line.chars().take(MAX_QUOTED_CHARS).collect();
    cut.push('…');
    cut
}

#[cfg(test)]
mod tests {
    use super::*;
    use gridcraft_engine::Session;
    use serde_json::json;

    fn session() -> Session {
        let mut session = Session::new();
        session.new_workbook();
        session
    }

    fn workbook(session: &Session) -> std::sync::Arc<Workbook> {
        session.active().map(|d| d.wb.clone()).unwrap()
    }

    /// What `input` typed in B2 adds to an empty workbook.
    fn typed(input: &str) -> Vec<Reach> {
        let mut s = session();
        let before = workbook(&s);
        s.execute("cell.set", json!({"cell": "B2", "input": input})).unwrap();
        added(&before, &workbook(&s))
    }

    #[test]
    fn a_link_to_a_web_page_an_address_or_a_place_is_no_reach() {
        for input in [
            "=HYPERLINK(\"https://example.com\",\"Site\")",
            "=HYPERLINK(\"mailto:info@example.com\")",
            "=HYPERLINK(\"#Sheet1!A1\",\"Top\")",
            "=HYPERLINK(\"\")",
            "=SUM(A1:A3)",
            "Tarifs C:\\x.pdf",
            "=\"C:\\x.pdf\"",
        ] {
            assert!(typed(input).is_empty(), "{input}");
        }
    }

    #[test]
    fn a_formula_that_links_to_a_file_or_calls_outside_is_a_reach() {
        for (input, what) in [
            ("=HYPERLINK(\"C:\\Tarifs\\catalogue.pdf\",\"Tarifs\")", "a HYPERLINK formula to C:\\Tarifs\\catalogue.pdf"),
            ("=HYPERLINK(\"\\\\srv\\share\\x.xlsx\")", "a HYPERLINK formula to \\\\srv\\share\\x.xlsx"),
            ("=HYPERLINK(\"Sheet1!A1\")", "a HYPERLINK formula to Sheet1!A1"),
            ("=HYPERLINK(\"C:\"&\"\\x.pdf\")", "a HYPERLINK formula whose target is not written out in quotes"),
            ("=HYPERLINK(A1)", "a HYPERLINK formula whose target is not written out in quotes"),
            ("=WEBSERVICE(\"https://example.com/?\"&A1)", "WEBSERVICE, which calls outside the workbook"),
            ("=IMAGE(\"https://example.com/x.png\")", "IMAGE, which calls outside the workbook"),
            ("=IMPORTXML(\"https://example.com\",\"//a\")", "IMPORTXML, which calls outside the workbook"),
            ("=HYPERLINK(\"#[Book.xlsx]Sheet1!A1\")", "a HYPERLINK formula to #[Book.xlsx]Sheet1!A1"),
        ] {
            let found = typed(input);
            assert_eq!(found.len(), 1, "{input}: {found:?}");
            assert_eq!(found[0].what, what, "{input}");
            assert_eq!(found[0].place, "Sheet1!B2", "{input}");
        }
    }

    #[test]
    fn a_link_insert_link_saves_as_a_file_is_a_reach() {
        let mut s = session();
        let before = workbook(&s);
        s.execute("insert.link", json!({"cell": "A1", "target": "https://example.com"})).unwrap();
        s.execute("insert.link", json!({"cell": "A2", "target": "Sheet1!B2"})).unwrap();
        assert!(added(&before, &workbook(&s)).is_empty(), "a web page and a place stay");
        s.execute("insert.link", json!({"cell": "A3", "target": "C:\\Tarifs\\x.pdf"})).unwrap();
        let found = added(&before, &workbook(&s));
        assert_eq!(found.len(), 1, "{found:?}");
        assert_eq!(found[0].what, "a link to C:\\Tarifs\\x.pdf");
        assert_eq!(found[0].place, "Sheet1!A3");
        // A place in another workbook, which the file keeps as a place of this one: Excel opens that workbook.
        s.execute("insert.link", json!({"cell": "A4", "target": "#[Book.xlsx]Sheet1!A1"})).unwrap();
        let found = added(&before, &workbook(&s));
        assert_eq!(found.len(), 2, "{found:?}");
        assert_eq!(found[1].what, "a link to #[Book.xlsx]Sheet1!A1");
    }

    #[test]
    fn the_formulas_a_workbook_keeps_beside_its_cells_are_read_too() {
        let s = session();
        let before = workbook(&s);
        let mut wb = (*workbook(&s)).clone();
        let mut sheet = (*wb.sheets[0]).clone();
        let range = gridcraft_engine::core::RangeRef::parse("A1:A9").unwrap();
        sheet.cond_formats.push(gridcraft_engine::model::CondFormat {
            ranges: vec![range],
            rule: CfRule::Expression { formula: "WEBSERVICE(\"https://example.com/?\"&A1)<>\"\"".into(), style: Box::default() },
            priority: 1,
            stop_if_true: false,
        });
        wb.sheets[0] = std::sync::Arc::new(sheet);
        let found = added(&before, &wb);
        assert_eq!(found.len(), 1, "{found:?}");
        assert_eq!(found[0].place, "the conditional format of Sheet1!A1:A9");
        // A data bar's threshold is a formula too.
        let mut sheet = (*wb.sheets[0]).clone();
        sheet.cond_formats.push(gridcraft_engine::model::CondFormat {
            ranges: vec![range],
            rule: CfRule::DataBar {
                min: CfValueKind::Min,
                max: CfValueKind::Formula("RTD(\"server\",,\"x\")".into()),
                color: gridcraft_engine::model::Color::default(),
                gradient: true,
                show_value: true,
            },
            priority: 2,
            stop_if_true: false,
        });
        wb.sheets[0] = std::sync::Arc::new(sheet);
        assert_eq!(added(&before, &wb).len(), 2);
    }

    #[test]
    fn what_was_there_is_counted_not_refused() {
        let mut s = session();
        s.execute("cell.set", json!({"cell": "A1", "input": "=HYPERLINK(\"C:\\x.pdf\")"})).unwrap();
        let before = workbook(&s);
        s.execute("cell.set", json!({"cell": "B1", "input": "5"})).unwrap();
        assert!(added(&before, &workbook(&s)).is_empty(), "another cell changed");
        s.execute("cell.set", json!({"cell": "A2", "input": "=HYPERLINK(\"C:\\x.pdf\")"})).unwrap();
        assert_eq!(added(&before, &workbook(&s)).len(), 1, "a second one is new");
    }

    #[test]
    fn a_defined_name_is_read_too() {
        let s = session();
        let before = workbook(&s);
        let mut wb = (*workbook(&s)).clone();
        wb.names.push(gridcraft_engine::model::DefinedName {
            name: "Feed".into(),
            scope: None,
            formula: "WEBSERVICE(\"https://example.com\")".into(),
            comment: String::new(),
            hidden: false,
        });
        let found = added(&before, &wb);
        assert_eq!(found.len(), 1, "{found:?}");
        assert_eq!(found[0].place, "the name Feed");
    }

    #[test]
    fn a_formula_on_a_sheet_of_another_file_is_a_reach() {
        // Read in its syntax tree, or by its text when GridCraft cannot read it: either way, one.
        assert_eq!(typed("='[Book.xlsx]Sheet1'!A1+1").len(), 1);
    }

    #[test]
    fn an_unread_formula_is_read_by_its_text() {
        assert!(unread_reaches("cmd|'/c calc'!A0"));
        assert!(unread_reaches("'C:\\x\\[Book.xlsx]Sheet1'!A1"));
        assert!(unread_reaches("hyperlink(x"));
        assert!(!unread_reaches("SUM(A1:A3"));
        assert!(!unread_reaches("CONCAT(\"a|b\",\"[c]\""));
    }

    #[test]
    fn a_reference_to_another_file_is_a_reach() {
        let reference = |name: &str| Reference {
            sheet: SheetSel::Named(name.into()),
            kind: gridcraft_engine::formula::RefKind::Cell(gridcraft_engine::formula::Anchor { row: 0, col: 0, row_abs: false, col_abs: false }),
        };
        assert_eq!(other_file(&reference("[Book.xlsx]Sheet1")), Some("[Book.xlsx]Sheet1".to_string()));
        assert_eq!(other_file(&reference("C:\\x\\Sheet1")), Some("C:\\x\\Sheet1".to_string()));
        assert_eq!(other_file(&reference("Ventes 2026")), None);
    }

    #[test]
    fn the_refusal_says_what_and_where() {
        let found = typed("=HYPERLINK(\"C:\\x.pdf\")");
        let text = refusal("`cell.set`", &found);
        assert!(text.starts_with("`cell.set` was undone"), "{text}");
        assert!(text.contains("a HYPERLINK formula to C:\\x.pdf (Sheet1!B2)"), "{text}");
    }
}
