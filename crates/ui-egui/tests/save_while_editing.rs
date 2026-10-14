// //// Neoffice — added file (no upstream equivalent): Cmd/Ctrl+S while a cell is being edited.

//! Cmd/Ctrl+S (with Shift: Save As) saves the workbook with the cell draft in it, also while the cell
//! editor or the formula bar has the keyboard. The grid's keyboard handler ignores every key while an
//! edit is open, so in the browser build, which has no menu bar to carry the shortcut, it did nothing
//! until the cell was committed; the title bar's Save button, which commits first, did work.

use std::sync::{Arc, Mutex};

use egui::{Event, Key, Modifiers};
use gridcraft_engine::Session;
use gridcraft_engine::core::{CellRef, Value};
use gridcraft_ui_egui::{Services, SheetApp};

type Harness = egui_kittest::Harness<'static, SheetApp>;
type Files = Arc<Mutex<Vec<(String, Vec<u8>)>>>;

/// A workbook open in a window that hands every file it would download to `Files`, as the browser build
/// does when a hosting page takes the saves.
fn harness() -> (Harness, Files) {
    let files = Files::default();
    let capture = files.clone();
    let services = Services {
        download: Some(Box::new(move |name, bytes| capture.lock().unwrap().push((name.to_string(), bytes.to_vec())))),
        ..Default::default()
    };
    let mut session = Session::new();
    session.new_workbook();
    let mut h = Harness::builder().with_size(egui::vec2(1200.0, 800.0)).build_ui_state(
        |ui, app: &mut SheetApp| {
            let ctx = ui.ctx().clone();
            app.logic(&ctx);
            app.ui(ui);
        },
        SheetApp::new(session, services),
    );
    h.run_steps(4);
    (h, files)
}

fn press(h: &mut Harness, key: Key, modifiers: Modifiers) {
    h.input_mut().events.push(Event::Key { key, physical_key: Some(key), pressed: true, repeat: false, modifiers });
    h.input_mut().events.push(Event::Key { key, physical_key: Some(key), pressed: false, repeat: false, modifiers });
    h.run_steps(3);
}

/// What A1 holds in the workbook a file carries.
fn a1(bytes: &[u8]) -> Value {
    let (wb, _) = gridcraft_engine::io::open_bytes("saved.xlsx", bytes).unwrap();
    wb.active().unwrap().value(CellRef::new(0, 0))
}

/// Cmd on a Mac, Ctrl elsewhere: both set `command`, which is what the shortcut asks for.
const CMD: Modifiers = Modifiers::COMMAND;
const CMD_SHIFT: Modifiers = Modifiers { shift: true, ..Modifiers::COMMAND };

#[test]
fn cmd_s_saves_when_no_cell_is_being_edited() {
    let (mut h, files) = harness();
    h.state_mut().run("cell.set", serde_json::json!({"cell": "A1", "input": "7"})).unwrap();
    press(&mut h, Key::S, CMD);
    let saved = files.lock().unwrap();
    assert_eq!(saved.len(), 1, "one save, not one per frame");
    assert_eq!(saved[0].0, "Book1.xlsx");
    assert_eq!(a1(&saved[0].1), Value::Number(7.0));
}

#[test]
fn cmd_s_commits_the_cell_being_typed_and_saves() {
    let (mut h, files) = harness();
    h.input_mut().events.push(Event::Text("123".into()));
    h.run_steps(3);
    assert!(h.state().editor.is_some(), "typing starts an edit");
    assert!(files.lock().unwrap().is_empty());
    press(&mut h, Key::S, CMD);
    assert!(h.state().editor.is_none(), "the draft was committed");
    let saved = files.lock().unwrap();
    assert_eq!(saved.len(), 1, "one save, not one per frame");
    assert_eq!(saved[0].0, "Book1.xlsx");
    assert_eq!(a1(&saved[0].1), Value::Number(123.0), "the saved workbook has what was typed");
}

#[test]
fn cmd_s_commits_a_formula_bar_edit_and_saves() {
    let (mut h, files) = harness();
    h.state_mut().begin_edit(Some("456".into()), true);
    h.run_steps(3);
    assert!(h.state().editor.as_ref().is_some_and(|e| e.from_formula_bar));
    press(&mut h, Key::S, CMD);
    assert!(h.state().editor.is_none());
    let saved = files.lock().unwrap();
    assert_eq!(saved.len(), 1);
    assert_eq!(a1(&saved[0].1), Value::Number(456.0));
}

#[test]
fn cmd_shift_s_commits_the_cell_being_typed_and_asks_for_a_file() {
    let (mut h, files) = harness();
    h.input_mut().events.push(Event::Text("8".into()));
    h.run_steps(3);
    press(&mut h, Key::S, CMD_SHIFT);
    assert!(h.state().editor.is_none());
    let saved = files.lock().unwrap();
    assert_eq!(saved.len(), 1, "Save As hands the file over once, and Save did not run as well");
    assert_eq!(a1(&saved[0].1), Value::Number(8.0));
}

#[test]
fn a_draft_that_cannot_be_committed_keeps_the_edit_open_and_saves_nothing() {
    let (mut h, files) = harness();
    h.input_mut().events.push(Event::Text("=SUM(".into()));
    h.run_steps(3);
    press(&mut h, Key::S, CMD);
    assert!(files.lock().unwrap().is_empty(), "an invalid formula is not saved behind the user's back");
    assert!(h.state().editor.is_some(), "the edit stays open for the user to fix");
    assert!(h.state().message.is_some(), "and the user is told why");
}

#[test]
fn the_other_command_shortcuts_still_wait_for_the_edit_to_end() {
    // Only the save shortcuts reach through an edit: Cmd+B must not restyle the cell from inside it.
    let (mut h, files) = harness();
    h.input_mut().events.push(Event::Text("5".into()));
    h.run_steps(3);
    press(&mut h, Key::B, CMD);
    assert!(h.state().editor.is_some(), "the edit goes on");
    press(&mut h, Key::Enter, Modifiers::NONE);
    assert!(h.state().editor.is_none());
    let doc = h.state().session.active().unwrap();
    let sheet = doc.wb.active().unwrap();
    assert!(!doc.wb.styles.get(sheet.style_id(CellRef::new(0, 0))).font.bold);
    assert!(files.lock().unwrap().is_empty());
}
