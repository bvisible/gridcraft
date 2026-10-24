// //// Neoffice — added file (no upstream equivalent): a number typed with the thousands separator a Swiss editor shows.

//! The separator is the process's (`numfmt::set_display_grouping`, set once by the editor from its locale), so this
//! file is its own test binary and its one test sets it in turn: a Swiss editor, then any other.

use gridcraft_engine::core::{CellRef, Value};
use gridcraft_engine::display::{cell_text, edit_text};
use gridcraft_engine::{Session, numfmt};
use serde_json::json;

/// What a session shows for A1: its value, what the grid shows and what the cell editor starts from.
fn a1(s: &Session) -> (Value, String, String) {
    let d = s.doc().unwrap();
    let sheet = d.wb.active().unwrap();
    let at = CellRef::new(0, 0);
    (sheet.value(at), cell_text(&d.wb, sheet, at), edit_text(&d.wb, sheet, at))
}

/// Types `input` into A1 of a new workbook.
fn typed(input: &str) -> Session {
    let mut s = Session::new();
    s.new_workbook();
    s.execute("cell.set", json!({"cell": "A1", "input": input})).unwrap();
    s
}

fn enter(input: &str) -> (Value, String, String) {
    a1(&typed(input))
}

#[test]
fn a_number_typed_with_apostrophes_is_a_number_only_in_a_swiss_editor() {
    numfmt::set_display_grouping('\'');
    assert_eq!(enter("5'478.45"), (Value::Number(5478.45), "5'478.45".into(), "5478.45".into()));
    assert_eq!(enter("-1'000"), (Value::Number(-1000.0), "-1'000".into(), "-1000".into()));
    assert_eq!(enter("1'234'567.5"), (Value::Number(1_234_567.5), "1'234'567.50".into(), "1234567.5".into()));
    assert_eq!(enter("5\u{2019}478.45").0, Value::Number(5478.45), "the typographic apostrophe");
    // Not grouped in threes, or a leading apostrophe: text, as typed.
    assert_eq!(enter("12'3"), (Value::text("12'3"), "12'3".into(), "12'3".into()));
    assert_eq!(enter("'5478").0, Value::text("5478"));
    // Upstream's comma entry is a number still.
    assert_eq!(enter("5,478.45").1, "5'478.45");

    // Any other editor: an apostrophe is text.
    numfmt::set_display_grouping(',');
    assert_eq!(enter("5'478.45").0, Value::text("5'478.45"));
    assert_eq!(enter("-1'000").0, Value::text("-1'000"));
    assert_eq!(enter("1'234'567.5").0, Value::text("1'234'567.5"));
    assert_eq!(enter("5,478.45"), (Value::Number(5478.45), "5,478.45".into(), "5478.45".into()));

    // That text, opened in a Swiss editor, starts editing with the quote that keeps it text.
    let elsewhere = typed("5'478.45");
    numfmt::set_display_grouping('\'');
    assert_eq!(a1(&elsewhere), (Value::text("5'478.45"), "5'478.45".into(), "'5'478.45".into()));
    numfmt::set_display_grouping(',');
}
