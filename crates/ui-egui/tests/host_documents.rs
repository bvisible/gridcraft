// //// Neoffice — added file (no upstream equivalent): a page that frames the editor opens the workbooks.

//! A hosting page (the Drive of the Suite, Neodesk's window) gives the editor the workbook to edit and takes
//! what it saves. The editor must not bring another workbook in beside it, or close it: File › New or Open would
//! show the person a workbook the page knows nothing about, and its save would reach the page under another name.
//! The file the page hands over still opens, and without the host's setting everything is as upstream has it.

use std::cell::Cell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use egui_kittest::kittest::Queryable as _;
use gridcraft_engine::Session;
use gridcraft_engine::core::{CellRef, Value};
use gridcraft_ui_egui::{Inbox, Services, SheetApp};
use serde_json::json;

type Harness = egui_kittest::Harness<'static, SheetApp>;

fn harness(services: Services) -> Harness {
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
    h
}

fn page(h: &Harness) -> Option<&'static str> {
    h.state().backstage.as_ref().map(|b| b.page.id())
}

fn has_button(h: &Harness, label: &str) -> bool {
    h.query_by_role_and_label(egui::accesskit::Role::Button, label).is_some()
}

#[test]
fn a_host_that_opens_the_workbooks_keeps_the_editor_on_its_workbook() {
    let inbox = Inbox::default();
    let mut h = harness(Services { host_owns_documents: true, inbox: Some(inbox.clone()), ..Default::default() });
    let before = h.state().session.documents().len();
    for id in ["file.new", "file.open", "file.close"] {
        assert!(h.state_mut().run(id, json!({})).is_err(), "{id} is the page's, not the editor's");
    }
    assert_eq!(h.state().session.documents().len(), before, "still the workbook the page gave");

    // The File tab opens Info, and Home, New, Open and Close are not entries of the Backstage view.
    h.get_by_label("File").click();
    h.run_steps(3);
    assert_eq!(page(&h), Some("info"));
    for label in ["Home", "New", "Open", "Close"] {
        assert!(!has_button(&h, label), "{label} is not offered");
    }
    assert!(has_button(&h, "Save As"), "what saves the workbook is still there");
    h.state_mut().open_backstage(gridcraft_ui_egui::backstage::Page::Open);
    h.run_steps(2);
    assert_eq!(page(&h), Some("info"), "asking for Open shows Info");

    // The page's own file opens, then the guard is up again.
    inbox.lock().unwrap().push(("Page.csv".into(), b"a,b\n1,2\n".to_vec()));
    h.run_steps(3);
    let title = h.state().session.active().map(|d| d.display_title()).unwrap_or_default();
    assert!(title.contains("Page"), "the page's file opened: {title}");
    assert!(h.state_mut().run("file.new", json!({})).is_err());
}

#[test]
fn without_a_host_new_and_open_are_the_editors_as_upstream_has_them() {
    let mut h = harness(Services::default());
    let before = h.state().session.documents().len();
    h.state_mut().run("file.new", json!({})).unwrap();
    assert_eq!(h.state().session.documents().len(), before + 1);
    h.get_by_label("File").click();
    h.run_steps(3);
    assert_eq!(page(&h), Some("home"));
    for label in ["New", "Open", "Close"] {
        assert!(has_button(&h, label), "{label} is offered");
    }
}

#[test]
fn the_start_dialog_and_the_command_search_offer_no_other_workbook_to_a_host_that_opens_them() {
    let mut results = vec![];
    for hosted in [false, true] {
        let mut h = harness(Services { host_owns_documents: hosted, ..Default::default() });

        // The Start dialog (a new workbook, the recent files, Open) is the editor's start page: not for a host.
        h.state_mut().open_dialog("start", json!({}));
        h.run_steps(3);
        let started = h.state().dialog.as_ref().map(|d| d.name().to_string()).as_deref() == Some("start");
        assert_eq!(started, !hosted, "the Start dialog, hosted {hosted}");
        h.state_mut().dialog = None;

        // The command search lists commands by name, New, Open and Close among them (their ids start with `file.`).
        h.state_mut().open_dialog("commandSearch", json!({}));
        if let Some(d) = h.state_mut().dialog.as_mut() {
            d.search = "file.".into();
        }
        h.run_steps(4);
        let listed = h.query_all(egui_kittest::kittest::By::new().role(egui::accesskit::Role::Button)).count();
        results.push(listed);
    }
    // New, Open and Close are three of the commands of the File menu: three buttons fewer for a host.
    assert!(results[0] > results[1], "the command search lists fewer commands to a host: {results:?}");
    assert_eq!(results[0] - results[1], 3, "exactly New, Open and Close: {results:?}");
}

/// Since v0.13 every workbook has a window of its own, and the windows' commands reach the workbooks as File › New, Open and
/// Close do: another window on the page's workbook, the page's workbook hidden or closed (an editor with no workbook starts a
/// blank one). For a host that opens the workbooks they are refused, not listed, and the View tab draws no button for them.
#[test]
fn a_host_that_opens_the_workbooks_leaves_its_workbook_the_only_window() {
    let windows = [
        "view.newWindow",
        "view.arrangeAll",
        "view.switchWindows",
        "view.hideWindow",
        "view.unhideWindow",
        "window.activate",
        "window.close",
        "window.list",
    ];
    let mut h = harness(Services { host_owns_documents: true, ..Default::default() });
    let (documents, shown) = (h.state().session.documents().len(), h.state().session.windows().len());
    for id in windows {
        assert!(h.state_mut().run(id, json!({})).is_err(), "{id} is the page's, not the editor's");
        assert!(h.state_mut().run(id, json!({"window": 1, "force": true})).is_err(), "{id} with parameters");
    }
    assert_eq!(
        (h.state().session.documents().len(), h.state().session.windows().len()),
        (documents, shown),
        "still the one workbook, in the one window"
    );
    assert!(h.state().session.active().is_some(), "the workbook is open");

    // The View tab has no button for them, and the command search does not find them.
    h.state_mut().ui.ribbon_tab = "View".into();
    h.run_steps(3);
    for label in ["New Window", "Arrange All", "Switch Windows"] {
        assert!(!has_button(&h, label), "{label} is not offered");
    }
    assert!(has_button(&h, "Freeze Panes"), "the rest of the View tab is there");
    h.state_mut().open_dialog("commandSearch", json!({}));
    if let Some(d) = h.state_mut().dialog.as_mut() {
        d.search = "window".into();
    }
    h.run_steps(4);
    for label in ["New Window", "Arrange All", "Switch Windows", "Hide", "Unhide…", "Switch Window", "Close Window", "Windows"] {
        assert!(!has_button(&h, label), "{label} is not listed");
    }

    // Without a host the windows are upstream's.
    let mut h = harness(Services::default());
    h.state_mut().ui.ribbon_tab = "View".into();
    h.run_steps(3);
    for label in ["New Window", "Arrange All", "Switch Windows"] {
        assert!(has_button(&h, label), "{label} is offered without a host");
    }
    h.state_mut().run("view.newWindow", json!({})).unwrap();
    assert_eq!(h.state().session.windows().len(), shown + 1, "a second window on the same workbook");
}

const PW: &str = "synthetic-pw-5531";

/// A workbook with 42 in A1, saved as `.xlsx` and encrypted with `PW`.
fn encrypted_bytes() -> Vec<u8> {
    let mut s = Session::new();
    s.new_workbook();
    s.run("cell.set", json!({"cell": "A1", "input": "42"})).unwrap();
    let xlsx = gridcraft_engine::io::save_bytes(&s.active().unwrap().wb, "book.xlsx").unwrap();
    gridcraft_xlsx::testing::encrypt_workbook(&xlsx, PW)
}

fn dialog(h: &Harness) -> Option<String> {
    h.state().dialog.as_ref().map(|d| d.name().to_string())
}

fn value(h: &Harness, cell: &str) -> Value {
    h.state().session.active().unwrap().wb.active().unwrap().value(CellRef::parse(cell).unwrap())
}

/// Files the editor handed to the page (Save, Save As, the exports go there through `download`).
type Handed = Arc<Mutex<Vec<String>>>;

fn hosted(inbox: &Inbox, handed: &Handed) -> Services {
    let handed = handed.clone();
    Services {
        host_owns_documents: true,
        inbox: Some(inbox.clone()),
        download: Some(Box::new(move |name: &str, _bytes: &[u8]| handed.lock().unwrap().push(name.to_string()))),
        ..Default::default()
    }
}

/// The page's workbook has a password: the editor asks for it, as for a file opened without a page, and the password
/// opens that workbook in place of the blank one (the guard lets that one file through). Nothing goes to the page on
/// the way, and the guard is up again afterwards.
#[test]
fn the_pages_password_protected_workbook_opens_with_its_password() {
    let inbox = Inbox::default();
    let handed = Handed::default();
    let mut h = harness(hosted(&inbox, &handed));
    inbox.lock().unwrap().push(("Secret.xlsx".into(), encrypted_bytes()));
    h.run_steps(3);
    assert_eq!(dialog(&h).as_deref(), Some("openPassword"), "the editor asks for the password");
    assert!(h.state().message.is_none(), "no error box: the dialog asks instead");

    // A wrong password says so, and the dialog stays for another try: it is the password, not the guard, that refuses.
    h.state_mut().dialog.as_mut().unwrap().values.insert("password".into(), json!("wrong-guess"));
    h.get_by_label("   OK   ").click();
    h.run_steps(3);
    let said = h.state().message.as_ref().map(|(_, m)| m.clone()).unwrap_or_default();
    assert!(said.contains(gridcraft_engine::io::WRONG_PASSWORD), "{said}");
    assert_eq!(dialog(&h).as_deref(), Some("openPassword"));
    h.state_mut().message = None;

    h.state_mut().dialog.as_mut().unwrap().values.insert("password".into(), json!(PW));
    h.get_by_label("   OK   ").click();
    h.run_steps(3);
    assert_eq!(dialog(&h), None);
    assert!(h.state().message.is_none(), "{:?}", h.state().message);
    assert_eq!(h.state().session.documents().len(), 1, "the page's workbook, in place of the blank one");
    assert_eq!(value(&h, "A1"), Value::Number(42.0));
    assert!(h.state().session.active().unwrap().display_title().contains("Secret"));
    assert!(handed.lock().unwrap().is_empty(), "nothing went to the page: {:?}", handed.lock().unwrap());

    // The guard is up again: no other workbook opens, with a password or without.
    let other = gridcraft_engine::io::base64_encode(&encrypted_bytes());
    assert!(h.state_mut().run("file.open", json!({"name": "Other.xlsx", "base64": other, "password": PW})).is_err());
    assert_eq!(h.state().session.documents().len(), 1);
}

/// What the page was told of its files that did not open: (name, reason).
type Told = Arc<Mutex<Vec<(String, String)>>>;

fn telling(inbox: &Inbox, told: &Told) -> Services {
    let told = told.clone();
    Services {
        host_open_failed: Some(Box::new(move |name: &str, reason: &str| told.lock().unwrap().push((name.to_string(), reason.to_string())))),
        ..hosted(inbox, &Handed::default())
    }
}

/// Bytes no reader takes for a workbook or for text: NULs and control characters from the start.
const JUNK: &[u8] = &[0xA5, 0x4D, 0xCA, 0x18, 0x00, 0x25, 0x30, 0xBB, 0x1D, 0x00, 0x01, 0x02, 0xFF, 0xFE, 0x00, 0x07, 0x08, 0x00];

/// A file of the page that is no workbook: the page hears of it, with its name and the reason. The editor stays on its
/// blank workbook, with the alert for a page that does not listen (the Drive covers both and refuses a save).
#[test]
fn a_file_of_the_page_that_is_no_workbook_is_told_to_the_page() {
    for name in ["Casse.ods", "Casse.xlsb", "Casse.xlsx", "Casse.xls", "Casse.csv", "Casse.tsv"] {
        let (inbox, told) = (Inbox::default(), Told::default());
        let mut h = harness(telling(&inbox, &told));
        inbox.lock().unwrap().push((name.into(), JUNK.to_vec()));
        h.run_steps(3);
        let heard = told.lock().unwrap().clone();
        assert_eq!(heard.len(), 1, "{name}: {heard:?}");
        assert_eq!(heard[0].0, name);
        assert!(!heard[0].1.trim().is_empty(), "{name}: the reason is given");
        assert!(h.state().message.as_ref().is_some_and(|(_, m)| m.contains(name)), "{name}: {:?}", h.state().message);
        assert_eq!(h.state().session.documents().len(), 1, "{name}");
        let doc = h.state().session.active().unwrap();
        assert!(doc.wb.sheets.iter().all(|sheet| sheet.cells.is_empty()) && !doc.is_dirty(), "{name}: nothing of the file went in");
    }
}

/// What the importers do read stays readable from a page: text under a text name or an .xls name (a web app's export),
/// an empty text file. Only an empty .xls, which holds no workbook, is refused with the binary ones.
#[test]
fn text_and_an_empty_text_file_still_open_from_a_page() {
    for (name, bytes) in
        [("Liste.csv", &b"a;b\n1;2\n"[..]), ("Liste.tsv", b"a\tb\n1\t2\n"), ("Export.xls", b"a\tb\n1\t2\n"), ("Vide.csv", b""), ("Vide.tsv", b"")]
    {
        let (inbox, told) = (Inbox::default(), Told::default());
        let mut h = harness(telling(&inbox, &told));
        inbox.lock().unwrap().push((name.into(), bytes.to_vec()));
        h.run_steps(3);
        assert!(told.lock().unwrap().is_empty(), "{name}: {:?}", told.lock().unwrap());
        assert!(h.state().message.is_none(), "{name}: {:?}", h.state().message);
    }
    let (inbox, told) = (Inbox::default(), Told::default());
    let mut h = harness(telling(&inbox, &told));
    inbox.lock().unwrap().push(("Vide.xls".into(), Vec::new()));
    h.run_steps(3);
    assert_eq!(told.lock().unwrap().len(), 1, "an empty .xls is no workbook");
}

/// A workbook that opens, and one that asks for its password, are not failures.
#[test]
fn a_workbook_that_opens_or_asks_for_its_password_is_not_told() {
    let (inbox, told) = (Inbox::default(), Told::default());
    let mut h = harness(telling(&inbox, &told));
    inbox.lock().unwrap().push(("Secret.xlsx".into(), encrypted_bytes()));
    h.run_steps(3);
    assert_eq!(dialog(&h).as_deref(), Some("openPassword"));
    assert!(told.lock().unwrap().is_empty());
}

/// Without a hosting page the alert is upstream's, and binary bytes under a text name are read as text as upstream has it.
#[test]
fn without_a_page_the_editor_says_it_itself() {
    let inbox = Inbox::default();
    let mut h = harness(Services { inbox: Some(inbox.clone()), ..Default::default() });
    inbox.lock().unwrap().push(("Casse.ods".into(), JUNK.to_vec()));
    h.run_steps(3);
    assert!(h.state().message.as_ref().is_some_and(|(_, m)| m.contains("Casse.ods")), "{:?}", h.state().message);
    h.state_mut().message = None;
    inbox.lock().unwrap().push(("Casse.csv".into(), JUNK.to_vec()));
    h.run_steps(3);
    assert!(h.state().message.is_none(), "{:?}", h.state().message);
    assert!(h.state().session.active().unwrap().display_title().contains("Casse"), "read as text");
}

/// The page's import is titled as the page says the save will name it (`Budget 2027.xlsx`), not `… (imported).xlsx`;
/// a file really called so keeps its name, and without a page the title is upstream's.
#[test]
fn an_import_of_the_page_bears_the_name_its_save_makes() {
    let title = |h: &Harness| h.state().session.active().unwrap().display_title();
    let inbox = Inbox::default();
    let mut h = harness(hosted(&inbox, &Handed::default()));
    inbox.lock().unwrap().push(("Budget 2027.xls".into(), b"a\tb\n1\t2\n".to_vec()));
    h.run_steps(3);
    assert_eq!(title(&h), "Budget 2027.xlsx");

    let inbox = Inbox::default();
    let mut h = harness(hosted(&inbox, &Handed::default()));
    inbox.lock().unwrap().push(("Report (imported).xlsx".into(), b"a,b\n1,2\n".to_vec()));
    h.run_steps(3);
    assert_eq!(title(&h), "Report (imported).xlsx", "that is the file's own name");

    let inbox = Inbox::default();
    let mut h = harness(Services { inbox: Some(inbox.clone()), ..Default::default() });
    inbox.lock().unwrap().push(("Budget 2027.xls".into(), b"a\tb\n1\t2\n".to_vec()));
    h.run_steps(3);
    assert_eq!(title(&h), "Budget 2027 (imported).xlsx");
}

/// A notice longer than the window wraps inside it: its end and its Dismiss button stay on screen (the French sentence of
/// an import ran off the window's edge), and the bar is as tall as its lines, then one line again for a short notice.
#[test]
fn a_long_notice_stays_inside_the_window() {
    let notice = |text: &str| Some(gridcraft_ui_egui::Notice { text: text.to_string(), kind: gridcraft_ui_egui::NoticeKind::Info, action: None });
    let grid_top = |h: &Harness| h.state().grid.rect.map(|r| r.min.y).unwrap_or_default();
    let mut h = harness(Services::default());
    let bare = grid_top(&h);
    let text = "Importé depuis un classeur Excel 97-2003 (.xls) : ".repeat(6);
    h.state_mut().notice = notice(text.trim());
    h.run_steps(4);
    let window = h.ctx.content_rect();
    let label = h.get_by_label_contains("Importé depuis").rect();
    assert!(label.max.x <= window.max.x && label.min.x >= window.min.x, "{label:?} in {window:?}");
    assert!(label.height() > 24.0, "it takes more than one line: {label:?}");
    let dismiss = h.get_by_label("Dismiss").rect();
    assert!(dismiss.max.x <= window.max.x && dismiss.min.x >= label.max.x - 1.0, "{dismiss:?} after {label:?}");
    assert!(grid_top(&h) - bare > 28.0, "the bar grows with its sentence: {}", grid_top(&h) - bare);

    h.state_mut().notice = notice("Enregistré.");
    h.run_steps(4);
    assert!((grid_top(&h) - bare - 20.0).abs() < 1.0, "one line again: {}", grid_top(&h) - bare);
}

/// Data › Get Data in an editor whose page opens the workbooks imports the file into the page's workbook, in a new
/// sheet named after it, as Excel's Get Data does, instead of opening it as a second workbook beside the page's. Its
/// separator is found as File › Open finds a CSV's. Without a page, Get Data opens the file as upstream's does.
#[test]
fn get_data_imports_into_the_pages_workbook_instead_of_opening_another() {
    for hosted_page in [true, false] {
        let inbox = Inbox::default();
        let handed = Handed::default();
        let workbook_picker = Rc::new(Cell::new(0));
        let asked = workbook_picker.clone();
        let mut services = if hosted_page { hosted(&inbox, &handed) } else { Services { inbox: Some(inbox.clone()), ..Default::default() } };
        services.open_async = Some(Box::new(move || asked.set(asked.get() + 1)));
        services.pick_import_async = Some(Box::new(|imports: Inbox| {
            imports.lock().unwrap().push(("Prices.csv".into(), b"Item;Price\nTea;3.5\nCoffee;4\n".to_vec()));
        }));
        let mut h = harness(services);
        let sheets = h.state().session.active().unwrap().wb.sheets.len();

        // The ribbon's Get Data button, then the command its key tip and the command search run.
        h.state_mut().open_dialog("open", json!({}));
        h.run_steps(3);
        if !hosted_page {
            assert_eq!(workbook_picker.get(), 1, "without a page, Get Data opens a file as upstream's does");
            continue;
        }
        assert_eq!(workbook_picker.get(), 0, "the workbook picker is never offered");
        let book = h.state().session.active().unwrap();
        assert_eq!(h.state().session.documents().len(), 1, "still only the page's workbook");
        assert_eq!(book.wb.sheets.len(), sheets + 1, "a new sheet");
        assert_eq!(book.wb.active().unwrap().name, "Prices");
        assert_eq!(value(&h, "A1"), Value::Text("Item".into()));
        assert_eq!(value(&h, "B2"), Value::Number(3.5), "the semicolon is the separator");
        assert_eq!(value(&h, "A3"), Value::Text("Coffee".into()));

        h.state_mut().run("data.getData", json!({})).unwrap();
        h.run_steps(3);
        assert_eq!(workbook_picker.get(), 0);
        assert_eq!(h.state().session.documents().len(), 1);
        assert_eq!(h.state().session.active().unwrap().wb.sheets.len(), sheets + 2, "a second import, a second sheet");
        assert!(handed.lock().unwrap().is_empty(), "an import saves nothing to the page");
    }
}

/// What the message bar says when a save of the page's password-protected workbook is refused (English, the harness's
/// language; `GridCraft` is what our builds show as NeoGrille).
const SAVE_IN_CLEAR_REFUSED: &str =
    "This workbook is protected by a password: GridCraft can't save it encrypted, so the original stays untouched in the Drive.";

fn press_cmd_s(h: &mut Harness) {
    let (key, modifiers) = (egui::Key::S, egui::Modifiers::COMMAND);
    h.input_mut().events.push(egui::Event::Key { key, physical_key: Some(key), pressed: true, repeat: false, modifiers });
    h.input_mut().events.push(egui::Event::Key { key, physical_key: Some(key), pressed: false, repeat: false, modifiers });
    h.run_steps(3);
}

/// A hosted editor with the page's workbook `bytes` open (typing `password` in the dialog when it asks), B1 then changed.
fn hosted_and_changed(bytes: Vec<u8>, password: Option<&str>) -> (Harness, Handed) {
    let inbox = Inbox::default();
    let handed = Handed::default();
    let mut h = harness(hosted(&inbox, &handed));
    inbox.lock().unwrap().push(("Secret.xlsx".into(), bytes));
    h.run_steps(3);
    if let Some(password) = password {
        h.state_mut().dialog.as_mut().unwrap().values.insert("password".into(), json!(password));
        h.get_by_label("   OK   ").click();
        h.run_steps(3);
    }
    assert_eq!(value(&h, "A1"), Value::Number(42.0), "the page's workbook is open");
    h.state_mut().notice = None;
    h.state_mut().run("cell.set", json!({"cell": "B1", "input": "7"})).unwrap();
    h.run_steps(2);
    assert!(h.state().session.active().unwrap().is_dirty());
    (h, handed)
}

/// GridCraft cannot write an encrypted workbook: the page's workbook opened with its password is never handed back to
/// the page, which would replace the encrypted original in the Drive with a copy in clear. Cmd+S, the title bar's Save
/// and Save As send nothing, a message bar says why, and the workbook stays marked as changed. The same workbook without
/// a password goes to the page as before.
#[test]
fn the_pages_password_protected_workbook_is_never_saved_back_in_clear() {
    let (mut h, handed) = hosted_and_changed(encrypted_bytes(), Some(PW));
    press_cmd_s(&mut h);
    h.state_mut().run_or_alert("file.save", json!({}));
    h.state_mut().run_or_alert("file.saveAs", json!({}));
    h.run_steps(3);
    assert!(handed.lock().unwrap().is_empty(), "nothing went to the page: {:?}", handed.lock().unwrap());
    assert_eq!(h.state().notice.as_ref().map(|n| n.text.as_str()), Some(SAVE_IN_CLEAR_REFUSED));
    assert!(h.query_by_label(SAVE_IN_CLEAR_REFUSED).is_some(), "the message bar shows it");
    assert!(h.state().message.is_none(), "a bar, not a message box: {:?}", h.state().message);
    assert!(h.state().session.active().unwrap().is_dirty(), "the changes stay marked as unsaved");

    // The page's workbook without a password is saved to the page as before.
    let mut plain = Session::new();
    plain.new_workbook();
    plain.run("cell.set", json!({"cell": "A1", "input": "42"})).unwrap();
    let bytes = gridcraft_engine::io::save_bytes(&plain.active().unwrap().wb, "book.xlsx").unwrap();
    let (mut h, handed) = hosted_and_changed(bytes, None);
    press_cmd_s(&mut h);
    assert_eq!(handed.lock().unwrap().as_slice(), ["Secret.xlsx".to_string()], "saved to the page");
    assert_ne!(h.state().notice.as_ref().map(|n| n.text.as_str()), Some(SAVE_IN_CLEAR_REFUSED));
}

fn hosted_app() -> SheetApp {
    let mut session = Session::new();
    session.new_workbook();
    SheetApp::new(session, Services { host_owns_documents: true, ..Default::default() })
}

/// The workbooks open, the windows open, and the window in front.
fn layout(app: &SheetApp) -> (usize, usize, Option<u64>) {
    (app.session.documents().len(), app.session.windows().len(), app.session.active_window())
}

/// What a form button that came with a file would run: straight into the model, as a workbook's file puts it, past the
/// refusal `insert.button` and `control.assign` make when the action is given.
fn button_with(app: &mut SheetApp, id: &str, params: serde_json::Value) -> u64 {
    let control = app.run("insert.button", json!({"text": "From a file"})).unwrap()["control"].as_u64().unwrap();
    let doc = app.session.doc_mut().unwrap();
    let sheet = std::sync::Arc::make_mut(&mut doc.wb).sheet_mut(0).unwrap();
    let button = sheet.controls.iter_mut().find(|c| u64::from(c.id) == control).unwrap();
    button.action = Some(gridcraft_engine::model::ControlAction::Command { id: id.into(), params });
    control
}

/// Since v0.17 a form button runs a command inside the engine (`control.press`), below `SheetApp::run`'s guard, through the
/// macro policy, which let `view.switchWindows` and `view.arrangeAll` through. Every command the engine has, run in an editor
/// whose page opens the workbooks (as itself, with the parameters that name another window or workbook, and as the action of a
/// button that came with a file), leaves the page's workbook the only one, in the window it was in. A command a later upstream
/// adds that opens, closes or switches workbooks or windows fails here, whatever it is called.
#[test]
fn no_command_brings_another_workbook_or_window_to_a_host_that_opens_the_workbooks() {
    let ids: Vec<&str> = gridcraft_engine::cmd::command_specs().iter().map(|c| c.id).collect();
    assert!(ids.len() > 300, "the engine's commands are listed: {}", ids.len());
    let named = json!({"window": 1, "doc": 1, "force": true, "name": "Book1", "all": true});
    for id in ids {
        for params in [json!({}), named.clone()] {
            let mut app = hosted_app();
            let before = layout(&app);
            let _ = app.run(id, params.clone());
            assert_eq!(layout(&app), before, "{id} {params}: the page's workbook stays the only one, in its window");

            // The same command as a button's action. Pressing runs it through the macro policy and the sandbox, or refuses it.
            let mut app = hosted_app();
            let button = button_with(&mut app, id, params.clone());
            let before = layout(&app);
            let _ = app.run("control.press", json!({"id": button, "approve": true}));
            assert_eq!(layout(&app), before, "{id} {params} pressed from a button: the page's workbook stays the only one, in its window");
        }
    }
}

/// The refusal is for the commands that bring another workbook or window, not for every command a button may run: a
/// button on the page's workbook still works.
#[test]
fn a_button_still_runs_an_ordinary_command_for_a_host_that_opens_the_workbooks() {
    let mut app = hosted_app();
    let button = button_with(&mut app, "cell.set", json!({"cell": "B2", "input": "9"}));
    app.run("control.press", json!({"id": button, "approve": true})).unwrap();
    assert_eq!(app.session.active().unwrap().wb.active().unwrap().value(CellRef::parse("B2").unwrap()), Value::Number(9.0));
}
