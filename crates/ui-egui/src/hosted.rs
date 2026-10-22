// //// Neoffice — added file (no upstream equivalent): what reaches a file in an editor whose page opens the workbooks.

//! A hosting page (the Drive of the Suite, Neodesk's window) opens the workbooks (`Services::host_owns_documents`):
//! [`SheetApp::run`] refuses `file.new`, `file.open` and `file.close`, and File › Home, New and Open are not offered.
//! Two things still reach a file, and this module keeps both on the page's workbook:
//!
//! - **The page's workbook has a password.** The inbox asks for it with the `openPassword` dialog, as for a file
//!   opened without a page, and the dialog's `file.open` is let through for that file only: the one the page handed
//!   over, told from any other by a fingerprint of its bytes. The password goes to `file.open` and nowhere else; the
//!   bridge to the page never sees it.
//! - **Data › Get Data**, and every command that asks for a file to import (`data.getData`, `data.fromTextCsv`), opens
//!   the browser's file picker, which would bring the file in as a second workbook beside the page's. Here it imports
//!   the text, CSV or JSON file into the page's workbook, in a new sheet, as Excel's Get Data does.
//!
//! Without a hosting page nothing here runs: the editor behaves as upstream's.

use std::hash::{Hash as _, Hasher as _};

use serde_json::{Value as Json, json};

use crate::{Inbox, SheetApp};

/// Excel's longest sheet name.
const MAX_SHEET_NAME_CHARS: usize = 31;

/// A fingerprint of a file's bytes as `file.open` carries them (`base64`), to tell the page's file from another.
pub(crate) fn fingerprint(base64: &str) -> u64 {
    let mut hasher = std::hash::DefaultHasher::new();
    base64.hash(&mut hasher);
    hasher.finish()
}

/// The new sheet's name for an imported file: its name without the extension, as Excel names it.
fn sheet_name(file: &str) -> String {
    let stem = std::path::Path::new(file).file_stem().and_then(|s| s.to_str()).unwrap_or_default().trim();
    stem.chars().take(MAX_SHEET_NAME_CHARS).collect()
}

impl SheetApp {
    /// Whether `file.open` with these `params` opens the page's password-protected workbook with the password the person
    /// typed: the one open the guard of `host_owns_documents` lets through besides the inbox's own.
    pub(crate) fn opens_the_pages_protected_workbook(&self, id: &str, params: &Json) -> bool {
        id == "file.open"
            && params.get("password").is_some_and(Json::is_string)
            && self.host_protected_file.is_some()
            && params.get("base64").and_then(Json::as_str).map(fingerprint) == self.host_protected_file
    }

    /// The inbox could not open the page's workbook without its password: remember which file the password dialog is
    /// for, when a page opens the workbooks.
    pub(crate) fn remember_the_pages_protected_workbook(&mut self, base64: &str) {
        self.host_protected_file = self.services.host_owns_documents.then(|| fingerprint(base64));
    }

    /// Data › Get Data in an editor whose page opens the workbooks: the browser asks for a text, CSV or JSON file, which
    /// [`SheetApp::import_arrived_data`] imports into this workbook. A host with no picker for it refuses with the
    /// page's sentence, rather than doing nothing.
    pub(crate) fn ask_for_data_to_import(&mut self) {
        if self.services.pick_import_async.is_none() {
            let refusal = self.ui.language.tr("This workbook is opened from the page that hosts the editor.").to_string();
            self.alert(refusal);
            return;
        }
        // A picker of its own, as Insert › Picture has: a file picked here never reaches the inbox of the workbooks.
        let inbox = Inbox::default();
        if let Some(pick) = &self.services.pick_import_async {
            pick(inbox.clone());
        }
        self.imports = Some(inbox);
    }

    /// Imports each file the browser read for Get Data into a new sheet of the active workbook.
    pub(crate) fn import_arrived_data(&mut self) {
        let Some(inbox) = &self.imports else { return };
        let arrived: Vec<(String, Vec<u8>)> = std::mem::take(&mut *inbox.lock().unwrap_or_else(std::sync::PoisonError::into_inner));
        if arrived.is_empty() {
            return;
        }
        self.imports = None;
        for (name, bytes) in arrived {
            let params = json!({"base64": gridcraft_engine::io::base64_encode(&bytes), "sheet": sheet_name(&name)});
            if let Err(e) = self.run("data.getData", params) {
                self.alert(e);
            }
            self.after_engine();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_imported_file_names_its_sheet_without_its_extension() {
        assert_eq!(sheet_name("Prices.csv"), "Prices");
        assert_eq!(sheet_name("folder/Report 2026.tsv"), "Report 2026");
        assert_eq!(sheet_name(&format!("{}.csv", "x".repeat(40))).chars().count(), MAX_SHEET_NAME_CHARS);
        assert_eq!(sheet_name(".csv"), ".csv", "a name that is all extension keeps it");
    }

    #[test]
    fn a_fingerprint_tells_one_file_from_another() {
        assert_eq!(fingerprint("UEsDBA=="), fingerprint("UEsDBA=="));
        assert_ne!(fingerprint("UEsDBA=="), fingerprint("UEsDBB=="));
    }
}
