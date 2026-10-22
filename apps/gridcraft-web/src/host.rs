// //// Neoffice — added file (no upstream equivalent on 14.10, when it was written): the bridge between the editor and the
// page that frames it. Upstream added its own `host.rs` in v0.7.0 (#237), a subset of this one: the same messages and
// `?author=`, without `gridcraft:saved`, the same-origin check when the bridge starts, `host_owns_documents` and the
// tests off wasm. At the v0.10.0 merge this file replaced upstream's whole; at a later merge, carry upstream's changes
// to its file over here by hand.

//! The `postMessage` bridge to a same-origin parent page (`?host=parent`), for pages that embed
//! GridCraft in an `<iframe>` (the Drive of the Suite, NeoDesk's window). It speaks the contract
//! upstream's own bridge uses (GridCraft since v0.7.0, and PdfCraft), so one host page drives every
//! Craft editor. Messages are plain objects with a `type`:
//!
//! - `gridcraft:ready` (to the parent): `{ version }`, the app is listening; send a file now.
//! - `gridcraft:open` (from the parent): `{ name, bytes }`, `bytes` an `ArrayBuffer` or
//!   `Uint8Array`. It opens like a workbook picked with File › Open or dropped on the window.
//! - `gridcraft:save` (to the parent): `{ name, bytes }` (a transferred `ArrayBuffer`) for every
//!   file GridCraft would otherwise download: Save and Save As (an .xlsx workbook) and the exports
//!   (PDF, CSV, web page). The parent stores it, or offers it, by the file's extension.
//! - `gridcraft:saved` (from the parent): `{ name, ok, error }`. Neoffice's addition: how storing
//!   `name` went. `ok: false` marks the workbook unsaved again and says why in the message bar. A
//!   parent that never answers leaves the workbook saved once the file has been handed over, as
//!   upstream's bridge does.
//! - `gridcraft:dirty` (to the parent): `{ dirty }` whenever unsaved changes appear or go away.
//! - `gridcraft:failed` (to the parent): `{ error }` when the app couldn't start.
//!
//! The bridge exists only when the page asks for it with `?host=parent` and a page of its own
//! origin frames it; otherwise the editor behaves as it always did. Only that parent is heard or
//! answered. In host mode the page starts with an empty workbook instead of a sample (the parent
//! sends the file to edit), and `?author=<name>` is the name that signs new notes and comments.
//!
//! What decides things (which files count as saved, what a refusal does, what the message says) is
//! plain Rust at the top of this file and is tested off wasm; the part that talks to the browser is
//! `Bridge` (module `wire`), below it.

use std::sync::Arc;

use gridcraft_ui_egui::{Notice, NoticeKind, SheetApp};

/// Saving one of these hands the whole workbook to the parent, which stores it: the workbook
/// counts as saved under that name, as it would after Save on the desktop.
const WORKBOOK_EXTS: &[&str] = &["xlsx"];

/// The longest user name taken from `?author=`.
const MAX_AUTHOR_CHARS: usize = 200;

/// The longest file name and the longest reason the message bar shows.
const MAX_NAME_CHARS: usize = 120;
const MAX_REASON_CHARS: usize = 300;

/// Workbooks handed over and not yet answered that are remembered; the oldest are forgotten beyond.
const MAX_AWAITING: usize = 32;

/// What the page said about a file it was handed (`gridcraft:saved`), or what the bridge found out
/// itself when it could not hand the file over.
#[derive(Clone, Debug, PartialEq)]
struct Answer {
    name: String,
    ok: bool,
    error: String,
}

/// What happens to the workbooks once the parent has their files.
#[derive(Default)]
struct Ledger {
    /// Workbooks handed to the parent whose answer hasn't come: (file name, document uid), oldest first.
    awaiting: Vec<(String, u64)>,
    /// The message we put up for a refused save, with the file it is about: a good save of that
    /// file takes it down again, and nothing else of the message bar is ever touched.
    shown: Option<(String, Notice)>,
}

impl Ledger {
    /// Settles what happened since the last frame. `handed` are the files handed to the parent
    /// (during that frame, in the order they went), `answers` what the parent said since. Returns
    /// whether the interface changed, so that a frame is due.
    fn settle(&mut self, app: &mut SheetApp, handed: &[String], answers: &[Answer]) -> bool {
        for name in handed.iter().filter(|name| is_workbook(name)) {
            // Handed over by a command of the frame that just ended, which ran on the active workbook.
            let Some(doc) = app.session.active_mut() else { continue };
            doc.saved = doc.wb.clone();
            if doc.path.is_none() {
                doc.title = name.clone();
            }
            self.awaiting.push((name.clone(), doc.uid));
        }
        let forgotten = self.awaiting.len().saturating_sub(MAX_AWAITING);
        self.awaiting.drain(..forgotten);
        let mut changed = false;
        for answer in answers {
            changed |= self.answer(app, answer);
        }
        changed
    }

    fn answer(&mut self, app: &mut SheetApp, answer: &Answer) -> bool {
        // The newest hand-over of that file is the one the parent means.
        let uid = self.awaiting.iter().rposition(|(name, _)| *name == answer.name).map(|at| self.awaiting.remove(at).1);
        if answer.ok {
            // Stored: the message we put up for this file is out of date, unless something else took its place.
            if self.shown.as_ref().is_some_and(|(name, notice)| *name == answer.name && app.notice.as_ref() == Some(notice)) {
                app.notice = None;
                self.shown = None;
                return true;
            }
            return false;
        }
        if is_workbook(&answer.name) {
            mark_unsaved(app, uid);
        }
        let reason = if answer.error.trim().is_empty() { "the page did not say why" } else { answer.error.as_str() };
        let text = fill(
            app.ui.language.tr("Couldn't save {name}: {error}"),
            &[("name", &tidy(&answer.name, MAX_NAME_CHARS)), ("error", &tidy(reason, MAX_REASON_CHARS))],
        );
        let notice = Notice { text, kind: NoticeKind::Warning, action: None };
        app.notice = Some(notice.clone());
        self.shown = Some((answer.name.clone(), notice));
        true
    }
}

/// The workbook has changes the parent doesn't have: `saved` stops being the workbook itself, which
/// is how a workbook is told from one with unsaved changes. `uid` is the document that was handed
/// over; without one (an answer about a file we have no record of) it is the active document, the
/// safe side of a doubt. A document closed since has nothing left to mark.
fn mark_unsaved(app: &mut SheetApp, uid: Option<u64>) {
    let session = &mut app.session;
    let active = session.active_index();
    let at = match uid {
        Some(uid) => session.documents().iter().position(|doc| doc.uid == uid),
        None => Some(active),
    };
    let Some(at) = at else { return };
    session.set_active(at);
    if let Some(doc) = session.active_mut() {
        doc.saved = Arc::new((*doc.wb).clone());
    }
    session.set_active(active);
}

/// Whether any open workbook has changes that aren't saved.
fn unsaved(app: &SheetApp) -> bool {
    app.session.documents().iter().any(|doc| doc.is_dirty())
}

/// `?author=`: the host's name for the user signs new notes and comments.
fn set_author(app: &mut SheetApp, raw: &str) {
    let author: String = raw.trim().chars().take(MAX_AUTHOR_CHARS).collect();
    if !author.is_empty() {
        app.session.prefs.user_name = author;
    }
}

fn is_workbook(name: &str) -> bool {
    name.rsplit_once('.').is_some_and(|(_, ext)| WORKBOOK_EXTS.iter().any(|e| ext.eq_ignore_ascii_case(e)))
}

/// The last component of `path`.
fn file_name(path: &str) -> String {
    std::path::Path::new(path).file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_else(|| path.to_string())
}

/// `text` on one line and at most `max` characters, for a bar that has room for neither.
fn tidy(text: &str, max: usize) -> String {
    let line = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if line.chars().count() <= max {
        return line;
    }
    let mut cut: String = line.chars().take(max.saturating_sub(1)).collect();
    cut.push('…');
    cut
}

/// `template` with each `{key}` replaced by its value, in one pass: a value that itself reads
/// `{other}` stays as it is, and a `{key}` with no value is left alone.
fn fill(template: &str, values: &[(&str, &str)]) -> String {
    let mut out = String::with_capacity(template.len());
    let mut rest = template;
    while let Some((before, after)) = rest.split_once('{') {
        out.push_str(before);
        let value = after.split_once('}').and_then(|(key, tail)| values.iter().find(|(k, _)| *k == key).map(|(_, value)| (*value, tail)));
        match value {
            Some((value, tail)) => {
                out.push_str(value);
                rest = tail;
            }
            None => {
                out.push('{');
                rest = after;
            }
        }
    }
    out.push_str(rest);
    out
}

#[cfg(target_arch = "wasm32")]
pub use wire::Bridge;

/// The part that talks to the browser.
#[cfg(target_arch = "wasm32")]
mod wire {
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    use gridcraft_ui_egui::{Inbox, Services, SheetApp};
    use wasm_bindgen::{JsCast as _, JsValue, closure::Closure};

    use super::{Answer, Ledger, file_name, set_author, unsaved};

    /// What the editor and the parent said since the last frame.
    #[derive(Default)]
    struct Mail {
        /// Files handed to the parent (`gridcraft:save` posted), oldest first.
        handed: Vec<String>,
        /// What the parent answered (`gridcraft:saved`), and files that could not be handed over.
        answers: Vec<Answer>,
    }

    #[derive(Clone)]
    pub struct Bridge {
        window: web_sys::Window,
        parent: web_sys::Window,
        origin: String,
        mail: Rc<RefCell<Mail>>,
        ledger: Rc<RefCell<Ledger>>,
        /// The unsaved-changes state last reported to the parent.
        reported: Rc<Cell<Option<bool>>>,
    }

    impl Bridge {
        /// The bridge, when the page asked for it (`?host=parent`) and a page of its own origin frames
        /// it; `None` otherwise, and the editor then behaves as it always did.
        pub fn requested() -> Option<Self> {
            if query_param("host").as_deref() != Some("parent") {
                return None;
            }
            let window = web_sys::window()?;
            let parent = window.parent().ok().flatten()?;
            if js_sys::Object::is(&parent, &window) {
                log::warn!("?host=parent ignored: this page is not in a frame");
                return None;
            }
            let origin = window.location().origin().ok()?;
            // A parent of another origin refuses to tell its own (reading it throws), and a page whose
            // origin is opaque (a file, a sandboxed frame) cannot be the target of a message.
            if origin == "null" || parent.location().origin().ok().as_deref() != Some(origin.as_str()) {
                log::warn!("?host=parent ignored: the page that frames this one is not of the same origin");
                return None;
            }
            Some(Self { window, parent, origin, mail: Rc::default(), ledger: Rc::default(), reported: Rc::default() })
        }

        /// Sends every file the editor would download to the parent instead, and hears the parent's
        /// files and its answers to the saves.
        pub fn connect(&self, services: &mut Services, inbox: Inbox, ctx: egui::Context) {
            // //// Neoffice — the page that frames the editor opens the workbooks (`Services::host_owns_documents`).
            services.host_owns_documents = true;
            self.listen(inbox, ctx);
            let bridge = self.clone();
            services.download = Some(Box::new(move |path: &str, bytes: &[u8]| bridge.post_save(path, bytes)));
        }

        /// The editor is up: `?author=` names the user, and the parent is told to send its file.
        pub fn ready(&self, app: &mut SheetApp) {
            if let Some(author) = query_param("author") {
                set_author(app, &author);
            }
            self.send(&[("type", "gridcraft:ready".into()), ("version", env!("CARGO_PKG_VERSION").into())]);
        }

        pub fn post_failed(&self, error: &str) {
            self.send(&[("type", "gridcraft:failed".into()), ("error", error.into())]);
        }

        /// After each frame: a workbook handed to the parent counts as saved under the name it was
        /// saved as, the parent's answers to the saves are applied, and the parent hears when unsaved
        /// changes appear or go away.
        pub fn sync(&self, app: &mut SheetApp, ctx: &egui::Context) {
            let Mail { handed, answers } = std::mem::take(&mut *self.mail.borrow_mut());
            if self.ledger.borrow_mut().settle(app, &handed, &answers) {
                ctx.request_repaint();
            }
            let dirty = unsaved(app);
            if self.reported.get() != Some(dirty) && self.send(&[("type", "gridcraft:dirty".into()), ("dirty", dirty.into())]) {
                self.reported.set(Some(dirty));
            }
        }

        fn post_save(&self, path: &str, bytes: &[u8]) {
            let name = file_name(path);
            match self.hand_over(&name, bytes) {
                Ok(()) => self.mail.borrow_mut().handed.push(name),
                Err(error) => {
                    log::error!("couldn't hand {name} to the page: {error}");
                    // Nobody has the file: the workbook stays unsaved, and the user is told.
                    self.mail.borrow_mut().answers.push(Answer { name, ok: false, error });
                }
            }
        }

        fn hand_over(&self, name: &str, bytes: &[u8]) -> Result<(), String> {
            let buffer = js_sys::Uint8Array::from(bytes).buffer();
            let message = message(&[("type", "gridcraft:save".into()), ("name", name.into()), ("bytes", buffer.clone().into())])
                .ok_or_else(|| "couldn't build the message".to_string())?;
            self.parent.post_message_with_transfer(&message, &self.origin, &js_sys::Array::of1(&buffer)).map_err(|e| reason(&e))
        }

        fn listen(&self, inbox: Inbox, ctx: egui::Context) {
            let parent = self.parent.clone();
            let origin = self.origin.clone();
            let mail = self.mail.clone();
            let on_message = Closure::<dyn FnMut(web_sys::MessageEvent)>::new(move |event: web_sys::MessageEvent| {
                if event.origin() != origin || !event.source().is_some_and(|s| js_sys::Object::is(&s, &parent)) {
                    return;
                }
                let data = event.data();
                match string(&data, "type").as_deref() {
                    Some("gridcraft:open") => {
                        let name = string(&data, "name").filter(|n| !n.trim().is_empty()).unwrap_or_else(|| "Book.xlsx".to_string());
                        let Some(bytes) = bytes(&data) else { return };
                        inbox.lock().unwrap_or_else(|e| e.into_inner()).push((name, bytes));
                    }
                    Some("gridcraft:saved") => {
                        let Some(name) = string(&data, "name").filter(|n| !n.trim().is_empty()) else {
                            log::warn!("gridcraft:saved without the name of the file ignored");
                            return;
                        };
                        // Only an explicit `false` is a refusal.
                        let ok = js_sys::Reflect::get(&data, &JsValue::from_str("ok")).ok().and_then(|ok| ok.as_bool()) != Some(false);
                        let error = string(&data, "error").unwrap_or_default();
                        if !ok {
                            log::error!("the hosting page could not save {name}: {error}");
                        }
                        mail.borrow_mut().answers.push(Answer { name, ok, error });
                    }
                    _ => return,
                }
                ctx.request_repaint();
            });
            if self.window.add_event_listener_with_callback("message", on_message.as_ref().unchecked_ref()).is_ok() {
                // The listener lives as long as the page.
                on_message.forget();
            }
        }

        /// Posts a message to the parent; false if it couldn't be sent.
        fn send(&self, fields: &[(&str, JsValue)]) -> bool {
            message(fields).is_some_and(|m| self.parent.post_message(&m, &self.origin).is_ok())
        }
    }

    fn query_param(key: &str) -> Option<String> {
        let search = web_sys::window()?.location().search().ok()?;
        web_sys::UrlSearchParams::new_with_str(&search).ok()?.get(key)
    }

    fn reason(error: &JsValue) -> String {
        error.as_string().unwrap_or_else(|| format!("{error:?}"))
    }

    fn message(fields: &[(&str, JsValue)]) -> Option<js_sys::Object> {
        let m = js_sys::Object::new();
        for (key, value) in fields {
            js_sys::Reflect::set(&m, &JsValue::from_str(key), value).ok()?;
        }
        Some(m)
    }

    fn string(data: &JsValue, key: &str) -> Option<String> {
        js_sys::Reflect::get(data, &JsValue::from_str(key)).ok()?.as_string()
    }

    /// `bytes` as an `ArrayBuffer` or `Uint8Array`.
    fn bytes(data: &JsValue) -> Option<Vec<u8>> {
        let bytes = js_sys::Reflect::get(data, &JsValue::from_str("bytes")).ok()?;
        if let Some(array) = bytes.dyn_ref::<js_sys::Uint8Array>() {
            Some(array.to_vec())
        } else if bytes.is_instance_of::<js_sys::ArrayBuffer>() {
            Some(js_sys::Uint8Array::new(&bytes).to_vec())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use gridcraft_engine::Session;
    use gridcraft_ui_egui::Services;
    use gridcraft_ui_egui::i18n::Language;
    use serde_json::json;

    use super::*;

    /// An app with `books` empty workbooks (the last one active), none of them with unsaved changes.
    fn app_with(books: usize) -> SheetApp {
        let mut session = Session::new();
        for _ in 0..books {
            session.new_workbook();
        }
        SheetApp::new(session, Services::default())
    }

    /// Types `input` into A1 of the active workbook, so that it has unsaved changes.
    fn edit(app: &mut SheetApp, input: &str) {
        app.session.run("cell.set", json!({"cell": "A1", "input": input})).unwrap();
    }

    fn dirty(app: &SheetApp, at: usize) -> bool {
        app.session.documents()[at].is_dirty()
    }

    fn handed(names: &[&str]) -> Vec<String> {
        names.iter().map(|name| name.to_string()).collect()
    }

    fn refusal(name: &str, error: &str) -> Answer {
        Answer { name: name.into(), ok: false, error: error.into() }
    }

    fn stored(name: &str) -> Answer {
        Answer { name: name.into(), ok: true, error: String::new() }
    }

    fn notice(app: &SheetApp) -> Option<&str> {
        app.notice.as_ref().map(|n| n.text.as_str())
    }

    #[test]
    fn a_workbook_handed_over_counts_as_saved_under_that_name() {
        let mut app = app_with(1);
        edit(&mut app, "1");
        assert!(unsaved(&app));
        Ledger::default().settle(&mut app, &handed(&["devis.xlsx"]), &[]);
        assert!(!unsaved(&app));
        assert_eq!(app.session.active().unwrap().title, "devis.xlsx");
    }

    #[test]
    fn a_workbook_with_a_file_keeps_its_title() {
        let mut app = app_with(1);
        edit(&mut app, "1");
        app.session.active_mut().unwrap().path = Some("/work/budget.xlsx".into());
        Ledger::default().settle(&mut app, &handed(&["other.xlsx"]), &[]);
        assert!(!unsaved(&app));
        assert_eq!(app.session.active().unwrap().title, "Book1");
    }

    #[test]
    fn an_export_is_a_copy_and_leaves_the_unsaved_changes_alone() {
        let mut app = app_with(1);
        edit(&mut app, "1");
        let mut ledger = Ledger::default();
        ledger.settle(&mut app, &handed(&["devis.pdf", "devis.csv", "devis.html", "devis"]), &[]);
        assert!(unsaved(&app));
        assert_eq!(app.session.active().unwrap().title, "Book1");
    }

    #[test]
    fn a_parent_that_never_answers_leaves_the_workbook_saved() {
        let mut app = app_with(1);
        edit(&mut app, "1");
        let mut ledger = Ledger::default();
        ledger.settle(&mut app, &handed(&["devis.xlsx"]), &[]);
        for _ in 0..3 {
            assert!(!ledger.settle(&mut app, &[], &[]), "nothing changed");
        }
        assert!(!unsaved(&app));
        assert!(app.notice.is_none());
    }

    #[test]
    fn a_refused_save_makes_the_workbook_unsaved_again_and_says_why() {
        let mut app = app_with(1);
        edit(&mut app, "1");
        let mut ledger = Ledger::default();
        ledger.settle(&mut app, &handed(&["devis.xlsx"]), &[]);
        assert!(!unsaved(&app));
        assert!(ledger.settle(&mut app, &[], &[refusal("devis.xlsx", "quota exceeded")]), "a frame is due");
        assert!(unsaved(&app));
        assert_eq!(notice(&app), Some("Couldn't save devis.xlsx: quota exceeded"));
        assert_eq!(app.notice.as_ref().unwrap().kind, NoticeKind::Warning);
        // The workbook is still the one the user has: editing on, and saving again, work as before.
        edit(&mut app, "2");
        ledger.settle(&mut app, &handed(&["devis.xlsx"]), &[stored("devis.xlsx")]);
        assert!(!unsaved(&app));
    }

    #[test]
    fn the_message_speaks_the_language_of_the_interface() {
        let mut app = app_with(1);
        app.ui.language = Language::Fr;
        Ledger::default().settle(&mut app, &[], &[refusal("devis.xlsx", "quota exceeded")]);
        assert_eq!(notice(&app), Some("Impossible d'enregistrer devis.xlsx : quota exceeded"));
        // Every language of the interface has the line (`i18n/neoffice.rs` of the UI crate).
        app.ui.language = Language::De;
        Ledger::default().settle(&mut app, &[], &[refusal("devis.xlsx", "quota exceeded")]);
        assert_eq!(notice(&app), Some("devis.xlsx konnte nicht gespeichert werden: quota exceeded"));
        app.ui.language = Language::En;
        Ledger::default().settle(&mut app, &[], &[refusal("devis.xlsx", "quota exceeded")]);
        assert_eq!(notice(&app), Some("Couldn't save devis.xlsx: quota exceeded"));
    }

    #[test]
    fn a_refusal_without_a_reason_says_so_and_a_long_one_is_cut() {
        let mut app = app_with(1);
        let mut ledger = Ledger::default();
        ledger.settle(&mut app, &[], &[refusal("devis.xlsx", "  \n ")]);
        assert_eq!(notice(&app), Some("Couldn't save devis.xlsx: the page did not say why"));
        ledger.settle(&mut app, &[], &[refusal("devis.xlsx", &format!("line one\n{}", "x".repeat(1000)))]);
        let text = notice(&app).unwrap();
        assert!(!text.contains('\n') && text.starts_with("Couldn't save devis.xlsx: line one xxx"), "{text}");
        assert!(text.ends_with('…') && text.chars().count() < 350, "{text}");
    }

    #[test]
    fn a_refusal_marks_the_workbook_that_was_saved_not_the_one_in_front() {
        let mut app = app_with(2);
        app.session.set_active(0);
        edit(&mut app, "1");
        let mut ledger = Ledger::default();
        ledger.settle(&mut app, &handed(&["Book1.xlsx"]), &[]);
        assert!(!dirty(&app, 0));
        // The user moves on to the other workbook while the parent is still storing the first.
        app.session.set_active(1);
        ledger.settle(&mut app, &[], &[refusal("Book1.xlsx", "disk full")]);
        assert!(dirty(&app, 0), "the workbook that was handed over is unsaved again");
        assert!(!dirty(&app, 1), "the one in front is untouched");
        assert_eq!(app.session.active_index(), 1, "and stays in front");
    }

    #[test]
    fn a_refusal_about_a_workbook_closed_since_only_says_why() {
        let mut app = app_with(2);
        app.session.set_active(0);
        edit(&mut app, "1");
        let mut ledger = Ledger::default();
        ledger.settle(&mut app, &handed(&["Book1.xlsx"]), &[]);
        app.session.close_document(0);
        ledger.settle(&mut app, &[], &[refusal("Book1.xlsx", "disk full")]);
        assert!(!unsaved(&app), "the other workbook has nothing to do with it");
        assert_eq!(notice(&app), Some("Couldn't save Book1.xlsx: disk full"));
    }

    #[test]
    fn a_refusal_about_a_file_never_handed_over_marks_the_active_workbook() {
        let mut app = app_with(1);
        Ledger::default().settle(&mut app, &[], &[refusal("elsewhere.xlsx", "no way")]);
        assert!(unsaved(&app), "when in doubt the workbook is unsaved");
    }

    #[test]
    fn a_refused_export_says_why_and_leaves_the_workbook_alone() {
        let mut app = app_with(1);
        edit(&mut app, "1");
        let mut ledger = Ledger::default();
        ledger.settle(&mut app, &handed(&["devis.xlsx"]), &[]);
        ledger.settle(&mut app, &handed(&["devis.pdf"]), &[refusal("devis.pdf", "not a PDF folder")]);
        assert!(!unsaved(&app), "the workbook was saved, the PDF is a copy");
        assert_eq!(notice(&app), Some("Couldn't save devis.pdf: not a PDF folder"));
    }

    #[test]
    fn a_file_that_could_not_be_handed_over_is_not_saved() {
        // The bridge reports it as a refusal, without the file ever being counted as handed over.
        let mut app = app_with(1);
        edit(&mut app, "1");
        Ledger::default().settle(&mut app, &[], &[refusal("devis.xlsx", "DataCloneError")]);
        assert!(unsaved(&app));
        assert_eq!(notice(&app), Some("Couldn't save devis.xlsx: DataCloneError"));
    }

    #[test]
    fn a_good_save_takes_down_the_message_about_that_file_and_nothing_else() {
        let mut app = app_with(1);
        let mut ledger = Ledger::default();
        ledger.settle(&mut app, &[], &[refusal("devis.xlsx", "quota exceeded")]);
        // Another file stored: the message is still true.
        assert!(!ledger.settle(&mut app, &[], &[stored("other.xlsx")]));
        assert!(app.notice.is_some());
        // The same file stored: gone.
        assert!(ledger.settle(&mut app, &[], &[stored("devis.xlsx")]));
        assert!(app.notice.is_none());
        // A message that isn't ours (an open-time warning) stays, whatever is stored.
        ledger.settle(&mut app, &[], &[refusal("devis.xlsx", "again")]);
        let warning = Notice { text: "This workbook contains VBA macros.".into(), kind: NoticeKind::Warning, action: None };
        app.notice = Some(warning.clone());
        assert!(!ledger.settle(&mut app, &[], &[stored("devis.xlsx")]));
        assert_eq!(app.notice, Some(warning));
    }

    #[test]
    fn the_answer_to_each_hand_over_is_matched_to_the_newest_one_of_that_name() {
        let mut app = app_with(1);
        let mut ledger = Ledger::default();
        edit(&mut app, "1");
        ledger.settle(&mut app, &handed(&["devis.xlsx"]), &[]);
        edit(&mut app, "2");
        ledger.settle(&mut app, &handed(&["devis.xlsx"]), &[]);
        assert_eq!(ledger.awaiting.len(), 2);
        ledger.settle(&mut app, &[], &[stored("devis.xlsx")]);
        assert_eq!(ledger.awaiting.len(), 1);
        ledger.settle(&mut app, &[], &[stored("devis.xlsx"), stored("devis.xlsx"), stored("devis.xlsx")]);
        assert!(ledger.awaiting.is_empty(), "answers nobody waits for are harmless");
    }

    #[test]
    fn the_hand_overs_waiting_for_an_answer_are_bounded() {
        let mut app = app_with(1);
        let mut ledger = Ledger::default();
        let names: Vec<String> = (0..MAX_AWAITING * 2).map(|i| format!("book{i}.xlsx")).collect();
        ledger.settle(&mut app, &names, &[]);
        assert_eq!(ledger.awaiting.len(), MAX_AWAITING);
        assert_eq!(ledger.awaiting.last().map(|(name, _)| name.as_str()), Some(format!("book{}.xlsx", MAX_AWAITING * 2 - 1).as_str()));
    }

    #[test]
    fn any_workbook_with_unsaved_changes_counts() {
        let mut app = app_with(2);
        assert!(!unsaved(&app));
        app.session.set_active(0);
        edit(&mut app, "1");
        app.session.set_active(1);
        assert!(unsaved(&app), "not only the one in front");
    }

    #[test]
    fn only_a_workbook_file_counts_as_a_save() {
        assert!(is_workbook("devis.xlsx") && is_workbook("DEVIS.XLSX") && is_workbook("a.b.xlsx"));
        assert!(!is_workbook("devis.csv") && !is_workbook("devis.pdf") && !is_workbook("xlsx") && !is_workbook("devis.xlsx.pdf"));
    }

    #[test]
    fn a_saved_name_is_the_last_component_of_the_path() {
        assert_eq!(file_name("devis.xlsx"), "devis.xlsx");
        assert_eq!(file_name("/a/b/devis.xlsx"), "devis.xlsx");
        assert_eq!(file_name(""), "");
    }

    #[test]
    fn the_author_is_trimmed_capped_and_ignored_when_blank() {
        let mut app = app_with(1);
        assert_eq!(app.session.prefs.user_name, "GridCraft User");
        set_author(&mut app, "   ");
        assert_eq!(app.session.prefs.user_name, "GridCraft User");
        set_author(&mut app, "  Jane Doe ");
        assert_eq!(app.session.prefs.user_name, "Jane Doe");
        set_author(&mut app, &"é".repeat(500));
        assert_eq!(app.session.prefs.user_name.chars().count(), MAX_AUTHOR_CHARS);
    }

    #[test]
    fn text_for_the_bar_is_one_line_and_short() {
        assert_eq!(tidy("a\n  b\t c ", 20), "a b c");
        assert_eq!(tidy("abcdef", 6), "abcdef");
        assert_eq!(tidy("abcdefg", 6), "abcde…");
        assert_eq!(tidy("ééééééé", 4), "ééé…");
        assert_eq!(tidy("", 4), "");
        assert_eq!(tidy("abc", 0), "…");
    }

    #[test]
    fn placeholders_are_filled_in_one_pass() {
        assert_eq!(fill("{a} and {b}", &[("a", "1"), ("b", "2")]), "1 and 2");
        assert_eq!(fill("{b}{a}", &[("a", "1"), ("b", "2")]), "21");
        assert_eq!(fill("Couldn't save {name}: {error}", &[("name", "{error}"), ("error", "boom")]), "Couldn't save {error}: boom");
        assert_eq!(fill("x {unknown} y", &[("a", "1")]), "x {unknown} y");
        assert_eq!(fill("a { b } c {", &[]), "a { b } c {");
        assert_eq!(fill("", &[("a", "1")]), "");
        assert_eq!(fill("é{a}é", &[("a", "ü")]), "éüé");
    }
}
