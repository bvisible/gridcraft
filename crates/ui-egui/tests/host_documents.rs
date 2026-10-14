// //// Neoffice — added file (no upstream equivalent): a page that frames the editor opens the workbooks.

//! A hosting page (the Drive of the Suite, Neodesk's window) gives the editor the workbook to edit and takes
//! what it saves. The editor must not bring another workbook in beside it, or close it: File › New or Open would
//! show the person a workbook the page knows nothing about, and its save would reach the page under another name.
//! The file the page hands over still opens, and without the host's setting everything is as upstream has it.

use egui_kittest::kittest::Queryable as _;
use gridcraft_engine::Session;
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
