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
