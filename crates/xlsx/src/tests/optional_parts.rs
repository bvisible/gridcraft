// //// Neoffice — added file (no upstream equivalent): a workbook without a styles part opens without a warning.

//! ECMA-376 makes the styles part optional and Excel opens a package without one silently. Files that a
//! mail or a script writes often have none: they must open with the default style and no warning (the
//! editor shows every open warning in a message box), while a styles part the workbook names and that
//! is missing still warns.

use gridcraft_core::{CellRef, Value};
use gridcraft_model::StyleId;

use super::minimal;
use crate::{read_xlsx, write_xlsx};

const BODY: &str = r#"<sheetData><row r="1"><c r="A1" s="3"><v>42</v></c><c r="B1" t="inlineStr"><is><t>Total</t></is></c></row></sheetData>"#;

#[test]
fn a_workbook_without_a_styles_part_opens_without_a_warning() {
    let (wb, report) = read_xlsx(&minimal(BODY, &[], "", "")).unwrap();
    assert!(report.warnings.is_empty(), "{:?}", report.warnings);
    let sheet = wb.sheet(0).unwrap();
    assert_eq!(sheet.value(CellRef::new(0, 0)), Value::Number(42.0));
    assert_eq!(sheet.value(CellRef::new(0, 1)), Value::Text("Total".into()));
    // A style index with no styles part to point into is the default style.
    assert_eq!(sheet.cell(CellRef::new(0, 0)).map(|c| c.style), Some(StyleId::DEFAULT));
    // Saved again, it has a styles part, and still opens without a warning.
    let (again, report) = read_xlsx(&write_xlsx(&wb).unwrap()).unwrap();
    assert!(report.warnings.is_empty(), "{:?}", report.warnings);
    assert_eq!(again.sheet(0).unwrap().value(CellRef::new(0, 0)), Value::Number(42.0));
}

#[test]
fn a_styles_part_the_workbook_names_and_lacks_still_warns() {
    let rels = r#"<Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles" Target="styles.xml"/>"#;
    let (wb, report) = read_xlsx(&minimal(BODY, &[], rels, "")).unwrap();
    assert!(report.warnings.iter().any(|w| w.contains("xl/styles.xml")), "{:?}", report.warnings);
    assert_eq!(wb.sheet(0).unwrap().value(CellRef::new(0, 0)), Value::Number(42.0));
}
