// //// Neoffice — added file (no upstream equivalent): the status bar in the editor's language.

//! With several cells selected, the status bar shows their Sum, Count and Average, as Excel does. Upstream writes those
//! words in English whatever the interface's language; Neoffice's table (`i18n/neoffice.rs`) has them in French, German,
//! Italian and Spanish, and the status bar asks for them in the editor's language.

use egui_kittest::kittest::Queryable as _;
use gridcraft_engine::Session;
use gridcraft_ui_egui::SheetApp;
use serde_json::json;

type Harness = egui_kittest::Harness<'static, SheetApp>;

/// A workbook with 12 and 7 in A1:A2, both selected, in an editor whose interface speaks `language`.
fn harness(language: &str) -> Harness {
    let mut session = Session::new();
    session.new_workbook();
    session.execute("cell.set", json!({"cell": "A1", "input": "12"})).unwrap();
    session.execute("cell.set", json!({"cell": "A2", "input": "7"})).unwrap();
    session.execute("selection.set", json!({"range": "A1:A2"})).unwrap();
    let mut app = SheetApp::new(session, Default::default());
    app.run("app.language.set", json!({"language": language})).unwrap();
    let mut h = Harness::builder().with_size(egui::vec2(1400.0, 800.0)).with_step_dt(1.0 / 60.0).build_ui_state(
        |ui, app: &mut SheetApp| {
            let ctx = ui.ctx().clone();
            app.logic(&ctx);
            app.ui(ui);
        },
        app,
    );
    h.run_steps(4);
    h
}

#[test]
fn sum_count_and_average_are_shown_in_the_editor_s_language() {
    for (language, words) in [
        ("fr", ["Somme : 19", "Nb (non vides) : 2", "Moyenne : 9.5"]),
        ("de", ["Summe: 19", "Anzahl: 2", "Mittelwert: 9.5"]),
        ("it", ["Somma: 19", "Conteggio: 2", "Media: 9.5"]),
        ("es", ["Suma: 19", "Recuento: 2", "Promedio: 9.5"]),
        ("en", ["Sum: 19", "Count: 2", "Average: 9.5"]),
    ] {
        let h = harness(language);
        for word in words {
            assert!(h.query_by_label(word).is_some(), "{language}: the status bar shows {word:?}");
        }
        if language != "en" {
            for english in ["Sum: 19", "Count: 2", "Average: 9.5"] {
                assert!(h.query_by_label(english).is_none(), "{language}: {english:?} is left in English");
            }
        }
    }
}
