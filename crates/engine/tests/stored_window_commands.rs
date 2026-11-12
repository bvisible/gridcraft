// //// Neoffice — added file (no upstream equivalent): what a form button, a stored script and a macro may not run.

//! A button, a script and a macro run commands inside the engine, below the editor's own guard for a page that
//! hosts it (`SheetApp::run`). So the engine's lists of what they may not run have to hold the commands that bring
//! another workbook in, close the page's, or switch or rearrange its windows. v0.19.1's lists missed
//! `view.switchWindows` (macros and buttons) and `view.arrangeAll` (macros, buttons and scripts); the page's one
//! workbook could be put out of view by a button that came with a file.

use std::sync::Arc;

use gridcraft_engine::Session;
use gridcraft_engine::model::ControlAction;
use serde_json::json;

/// The commands that reach another workbook or another window.
const REACH_OTHER_WORKBOOKS: &[&str] = &[
    "file.new",
    "file.open",
    "file.close",
    "view.newWindow",
    "view.arrangeAll",
    "view.switchWindows",
    "view.hideWindow",
    "view.unhideWindow",
    "window.activate",
    "window.close",
    "window.list",
];

/// Two workbooks, so that switching windows has an effect to see.
fn two_workbooks() -> Session {
    let mut s = Session::new();
    s.new_workbook();
    s.new_workbook();
    s
}

fn state(s: &Session) -> (usize, usize, Option<u64>) {
    (s.documents().len(), s.windows().len(), s.active_window())
}

/// Puts a button on the active sheet whose action is `id`, as a file that carries one would: straight into the model,
/// past the refusal `control.assign` and `insert.button` make when the action is given.
fn forged_button(s: &mut Session, id: &str) -> u64 {
    let control = s.run("insert.button", json!({"text": "Forged"})).expect("a button without an action")["control"].as_u64().expect("its id");
    let doc = s.doc_mut().expect("a workbook");
    let sheet = Arc::make_mut(&mut doc.wb).sheet_mut(0).expect("a sheet");
    let button = sheet.controls.iter_mut().find(|c| u64::from(c.id) == control).expect("the button");
    button.action = Some(ControlAction::Command { id: id.into(), params: json!({}) });
    control
}

#[test]
fn a_button_is_refused_an_action_that_reaches_another_workbook_or_window() {
    for id in REACH_OTHER_WORKBOOKS {
        let mut s = two_workbooks();
        assert!(s.run("insert.button", json!({"command": id})).is_err(), "insert.button must refuse {id}");
        let button = s.run("insert.button", json!({})).expect("a button")["control"].as_u64().expect("its id");
        assert!(s.run("control.assign", json!({"id": button, "command": id})).is_err(), "control.assign must refuse {id}");
    }
}

#[test]
fn a_button_that_came_with_such_an_action_does_nothing_when_pressed() {
    for id in REACH_OTHER_WORKBOOKS {
        let mut s = two_workbooks();
        let before = state(&s);
        let button = forged_button(&mut s, id);
        let pressed = s.run("control.press", json!({"id": button, "approve": true}));
        assert!(pressed.is_err(), "{id}: the press must be refused, got {pressed:?}");
        assert_eq!(state(&s), before, "{id}: no workbook came or went and no window changed");
    }
}

#[test]
fn a_stored_script_is_refused_a_step_that_reaches_another_workbook_or_window() {
    for id in REACH_OTHER_WORKBOOKS {
        let mut s = two_workbooks();
        let before = state(&s);
        s.run("automate.newScript", json!({"name": "S", "commands": [{"command": id, "params": {}}]})).expect("the script is stored");
        let ran = s.run("automate.runScript", json!({"name": "S", "trusted": true}));
        assert!(ran.is_err(), "{id}: the script must be refused, got {ran:?}");
        assert_eq!(state(&s), before, "{id}: no workbook came or went and no window changed");
    }
}
