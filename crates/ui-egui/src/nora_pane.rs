// //// Neoffice — added file (no upstream equivalent): the Nora assistant pane, built on craft-nora and craft-nora-ui.

//! The Nora pane: Neoffice's assistant, in a pane at the right of the window, the one WordCraft has.
//!
//! Nora (the agent, her memory and her tools) lives on the Neoffice server. This pane is her chat
//! window: it keeps a [`NoraSession`] (the conversation, from `craft-nora`) and draws it with
//! [`NoraPanel`] (from `craft-nora-ui`). It is off until File › Options › Assistant, or the `ui.nora`
//! command, turns it on; the browser build turns it on when the page that hosts it offers Nora.
//!
//! The pane never opens a connection itself. The host makes the HTTP calls with its own session and
//! credentials ([`crate::Services::nora_http`]) and delivers the answers
//! ([`crate::Services::nora_inbox`]). Without a connection the pane says that Nora isn't available
//! and offers no text box.
//!
//! What Nora asks for in the workbook (`doc_*` actions) is played with GridCraft's own commands, the
//! ones its MCP tools run through [`gridcraft_engine::Session::execute`]:
//! - *Observe*: the workbook's name, the names of its sheets, the active sheet's used range as rows of
//!   cell values and as text (cut at a length that fits her), and the selection's address and values.
//! - *Execute* and *Batch*: the workbook's commands (its [`FAMILIES`], none of [`THE_PERSONS`]), never
//!   with a parameter that reaches a file of the computer or carries a password, and never one that
//!   would open a dialog.
//! - *SelectText*: a cell or a range by its address (`B4:D10`, `Sheet2!A1`), a defined name or a
//!   table, or else the n-th cell whose text contains the words. A label that is also an address
//!   (`Q1`, `FY25`) is the cell that shows it, when one does.
//! - *TypeText*: the text entered from the active cell as typed, a tab moving to the next cell and a
//!   line break to the next row (`range.setValues`, one undo step).
//! - *Save*: the title bar's Save.
//! - *RenderPage*: NeoSheet cannot draw a sheet as a picture without a window, so the action answers
//!   an error that says it does not exist here.
//! - *Print*: the workbook's PDF, as File › Print lays it out, in the browser's print dialog; where the editor has
//!   printers of its own (`print_native`), the Print page opens instead. The dialog is the person's yes.
//! - *ExportPdf*, *ExportFile*: the workbook's PDF, or the workbook in its own format as it is now, handed to Nora for
//!   an e-mail draft (`craft_nora::protocol::pdf_result`, `file_result`); never the page's workbook opened with its
//!   password, which would leave in clear.
//!
//! What the person is typing is entered before any action. An action that fails leaves the workbook as
//! it was, its history included. What Nora changed is undone a whole action at a time, and only while
//! it is still the newest thing in the workbook: the pane never takes back the person's own work.
//!
//! Once an action changed the workbook, what it really wrote is read ([`guard`]): a link to a file, a
//! formula that calls outside the workbook or reads another file, which the server could not see in
//! what Nora asked for, takes the whole action back, and Nora is told why.
//!
//! The page that hosts the editor names the file it opened (the Drive's file id, [`name_file`]): Nora
//! is told it with each message, and the server reads it again under the person's rights.

mod guard;

use std::sync::{Arc, Weak};

use craft_nora::{
    AppKind, CommandCall, CommandPolicy, Decision, DocActionKind, DocumentInfo, HttpRequest, HttpResponse, Input, NoraError, NoraSession, Output,
    PageContext, Selection, SessionConfig,
};
use craft_nora_ui::{CorrectionWords, LauncherWords, NoraLabels, NoraPanel, NoraStyle, PaneMemory, PanelAction, ResumeWords, drive};
use egui::{Color32, RichText, Ui, vec2};
use gridcraft_engine::core::{CellRef, RangeRef, col_to_letters};
use gridcraft_engine::model::{Sheet, Visibility, Workbook};
use gridcraft_engine::{DocState, HistoryEntry, UiRequest};
use serde_json::{Value as Json, json};

use crate::SheetApp;
use crate::i18n::Language;
use crate::theme::{self, Tokens};

/// The most characters of the selection told to Nora with a message; she reads the workbook herself
/// when she needs more.
const MAX_SELECTION_CHARS: usize = 2_000;

/// The most characters of the active sheet told to Nora when she reads the workbook, for its text and
/// for its values each (WordCraft's length for a document's text).
const MAX_OBSERVED_CHARS: usize = 50_000;

/// The same for the cells of the selection.
const MAX_OBSERVED_SELECTION_CHARS: usize = 10_000;

/// The most columns of a sheet read at once: a sheet is 16,384 columns wide.
const MAX_OBSERVED_COLUMNS: u32 = 200;

/// The longest text of one cell told to Nora; a longer one is cut and ends with `…`.
const MAX_CELL_CHARS: usize = 1_000;

/// The most commands of one batch: a longer one is refused whole (the server sends 50 at most).
const MAX_BATCH: usize = 200;

/// Nora's changes kept for Undo, as many as a workbook keeps undo steps.
const MAX_CHANGES: usize = 100;

/// The files the hosting page named that are remembered, the oldest forgotten beyond.
const MAX_NAMED_FILES: usize = 32;

/// Parameter names that would reach a file of the computer. A command given one is refused.
const FILE_KEYS: &[&str] = &["path", "paths", "file", "files", "filename", "folder", "directory", "dir"];

const NO_WORKBOOK: &str = "no workbook is open";

/// What Nora is told when she asks to look at a page.
const NO_RENDER: &str = "doc_render_page does not exist in NeoSheet: it cannot draw a sheet as a picture here. Read the cells instead, \
with doc_observe or doc_execute `sheet.read` {range, formatted: true}";

/// The command families Nora may play at all: the workbook's own. Every other family is refused: the
/// files (`file.`), the application and its language (`app.`), the windows (`window.`), the interface
/// (`ui.`), the scripts (`automate.`: a script stored in the workbook runs its own commands out of the
/// policy's sight, a file among them), and any family a later version adds, until someone looks at it.
const FAMILIES: &[&str] = &[
    "cell",
    "range",
    "selection",
    "sheet",
    "home",
    "insert",
    "draw",
    "pageLayout",
    "formulas",
    "data",
    "review",
    "view",
    "table",
    "chart",
    "pivot",
    "slicer",
    "object",
    "shape",
    "arrange",
    "edit",
    "document",
    "history",
];

/// Inside those families, the commands that are the person's rather than the workbook's: the settings
/// of the interface GridCraft keeps outside `ui.` (theme, size, animations, collapsed ribbon, formula
/// bar, rulers, the Comments pane, every note shown), the tools that take the person's next clicks (the
/// pen and the eraser swallow every click on the grid, the format painter paints the next selection),
/// the windows (a batch would go on in another open workbook, and read it), and Undo and Redo, which
/// walk the person's own history: the pane's Undo takes back what Nora did.
const THE_PERSONS: &[&str] = &[
    "view.darkMode",
    "view.theme",
    "view.interfaceScale",
    "view.animations",
    "view.collapseRibbon",
    "view.formulaBar",
    "view.ruler",
    "review.showComments",
    "review.showAllNotes",
    "draw.pen",
    "draw.eraser",
    "draw.lasso",
    "edit.formatPainter",
    "view.newWindow",
    "view.hideWindow",
    "view.unhideWindow",
    "view.switchWindows",
    "view.arrangeAll",
    "edit.undo",
    "edit.redo",
];

/// What the conversation refuses before an action reaches the pane, in the only form `CommandPolicy`
/// takes, prefixes: the families that are not the workbook's (the default refuses `file.` and `app.`)
/// and the person's commands. [`allowed`] adds the allow-list of [`FAMILIES`].
fn policy() -> CommandPolicy {
    let policy = CommandPolicy::default().deny_prefix("ui.").deny_prefix("window.").deny_prefix("automate.");
    THE_PERSONS.iter().fold(policy, |policy, id| policy.deny_prefix(*id))
}

/// Whether Nora may play `id`: a command of the workbook's families that is not the person's.
fn allowed(id: &str) -> bool {
    id.split_once('.').is_some_and(|(family, _)| FAMILIES.contains(&family)) && policy().check(id) != Decision::Deny
}

/// What the pane keeps between frames: the conversation, the panel that draws it, and what Nora
/// changed.
pub struct NoraState {
    session: NoraSession,
    panel: NoraPanel,
    /// Nora's changes that Undo can take back, the newest last.
    changes: Vec<Change>,
    /// The file each workbook is, by its uid, as the page that hosts the editor named it ([`name_file`]).
    files: Vec<(u64, String)>,
}

impl Default for NoraState {
    fn default() -> Self {
        let context = PageContext::editor(AppKind::Sheet, DocumentInfo::named(""));
        let config = SessionConfig { policy: policy(), ..SessionConfig::default() };
        NoraState { session: NoraSession::new(config, context), panel: NoraPanel::new(), changes: Vec::new(), files: Vec::new() }
    }
}

/// The page that hosts the editor says which of its files workbook `uid` is (`id`, the Drive's file id
/// sent with the file), or that it names none. Nora is told the id with each message about that
/// workbook ([`DocumentInfo::file_id`] keeps only what reads as one); the server never believes it as it
/// comes, it reads the file again under the person's rights. A host that names no file changes nothing.
pub fn name_file(app: &mut SheetApp, uid: u64, id: Option<&str>) {
    let files = &mut app.nora.files;
    files.retain(|(named, _)| *named != uid);
    if let Some(id) = id.and_then(DocumentInfo::file_id) {
        files.push((uid, id));
        let excess = files.len().saturating_sub(MAX_NAMED_FILES);
        files.drain(..excess);
    }
}

/// The file the hosting page named workbook `uid` ([`name_file`]), if it named one.
pub fn named_file(app: &SheetApp, uid: u64) -> Option<&str> {
    app.nora.files.iter().find(|(named, _)| *named == uid).map(|(_, id)| id.as_str())
}

/// One change of Nora's: the workbook it was made in, and the undo steps it took there, oldest first
/// (a batch takes several). A step is known by the workbook it restores, held weakly so that a step the
/// history has dropped is not kept alive here.
struct Change {
    doc: u64,
    steps: Vec<Weak<Workbook>>,
    /// The workbook's revision when the change began, and once it was made (or once the pane took back
    /// the change made right after it): the pane's Undo takes the change back only while the workbook is
    /// still at `after`. The steps alone can't tell: the person's own Undo puts back the very workbook a
    /// step restores, and the step of their next edit then restores it too.
    before: u64,
    after: u64,
}

/// The name the pane's state is remembered under in the browser (craft-nora-ui `PaneMemory`).
const MEMORY: &str = "gridcraft";

/// Turns the pane on where the page offers Nora, unless the person closed it on an earlier visit (remembered in the
/// browser, open when nothing is); off where Nora isn't offered (a pane left open would say she isn't available).
pub fn open_as_remembered(app: &mut SheetApp, offered: bool) -> Result<Json, String> {
    app.run("ui.nora", json!({"on": offered && PaneMemory::new(MEMORY).open()}))
}

/// The pane at the right of the window, outermost, while `ui.nora` has it on: GridCraft's own title
/// and close button above Nora's panel. Closed by its cross where Nora is offered, a rail with her sphere stays in its
/// place, beside the workbook, and opens it again on the same conversation (craft-nora-ui `launcher`).
pub fn pane(app: &mut SheetApp, ui: &mut Ui) {
    if !app.ui.nora_pane {
        if app.services.nora_http.is_some() {
            let t = Tokens::get(ui.ctx());
            let words = LauncherWords { open: app.ui.language.tr("Nora|Open Nora").to_string() };
            let frame = egui::Frame::NONE.fill(t.ribbon).inner_margin(egui::Margin::symmetric(0, 10));
            let response = craft_nora_ui::launcher::rail(ui, frame, &words);
            response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, true, &words.open));
            if response.clicked() && app.run("ui.nora", json!({"on": true})).is_ok() {
                PaneMemory::new(MEMORY).remember(true);
            }
        }
        return;
    }
    let t = Tokens::get(ui.ctx());
    egui::Panel::right("nora_pane").default_size(320.0).resizable(true).frame(egui::Frame::NONE.fill(t.ribbon).inner_margin(10)).show(ui, |ui| {
        let mut close = false;
        ui.horizontal(|ui| {
            ui.label(RichText::new("Nora").font(theme::ui_bold(15.0)));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                close = crate::widgets::icon_button(ui, crate::icons::Icon::Close, t.text_dim, "Close", vec2(22.0, 22.0)).clicked();
            });
        });
        ui.separator();
        if close {
            let _ = app.run("ui.nora", json!({"on": false}));
            PaneMemory::new(MEMORY).remember(false);
            return;
        }
        show(app, ui);
    });
}

/// Draws the pane's content and moves the conversation along.
fn show(app: &mut SheetApp, ui: &mut Ui) {
    let language = app.ui.language;
    // The frame's tick first, then the answers: after a gap without frames (a tab in the background) the session knows
    // it before it reads an answer that waited in the page, whose action it then refuses rather than plays late.
    let mut outputs = drive(ui.ctx(), &mut app.nora.session);
    outputs.extend(deliver_answers(app));
    app.nora.panel.labels = labels(language);
    app.nora.panel.resume_words = resume_words(language);
    app.nora.panel.correction_words = correction_words(language);
    // The panel takes its words for a failure of the connection once, when it is made: given again each frame, as
    // the labels, so that they follow the interface language.
    app.nora.panel = std::mem::take(&mut app.nora.panel).with_error_words(error_worder(language));
    app.nora.panel.style = style(ui);
    app.nora.panel.offline = app.services.nora_http.is_none().then(|| language.tr("Nora isn't available in this version yet").to_string());
    app.nora.panel.open_links = app.services.nora_open.is_some();
    for action in app.nora.panel.show(ui, &app.nora.session) {
        match &action {
            // Where the person is, as of the message: told to Nora with it, never read on every frame.
            PanelAction::Send(_) => {
                let context = context_of(app);
                outputs.extend(app.nora.session.handle(Input::SetContext(context)));
            }
            // A new conversation forgets what the last one changed, as the session does.
            PanelAction::NewConversation => app.nora.changes.clear(),
            // A Stop ends Nora's turn in the conversation; what the workbook already ran stays hers to undo.
            PanelAction::Confirm { .. }
            | PanelAction::Undo
            | PanelAction::Stop
            | PanelAction::OpenLink(_)
            | PanelAction::AnswerCard { .. }
            | PanelAction::ChooseOnCard { .. } => {}
        }
        outputs.extend(app.nora.session.handle(action.into_input()));
    }
    perform(app, outputs, ui.ctx());
}

/// The answers the host delivered since the last frame, fed to the conversation.
fn deliver_answers(app: &mut SheetApp) -> Vec<Output> {
    let Some(inbox) = app.services.nora_inbox.clone() else { return Vec::new() };
    let answers = std::mem::take(&mut *inbox.lock().unwrap_or_else(|e| e.into_inner()));
    answers.into_iter().flat_map(|answer| app.nora.session.handle(Input::Http(answer))).collect()
}

/// Does what the conversation asks: a call goes to the host, a document action is played, a change
/// repaints. The results and the missing-connection answers come back as inputs and may ask for
/// more, so this goes round a few times at most.
fn perform(app: &mut SheetApp, outputs: Vec<Output>, ctx: &egui::Context) {
    let mut queue = outputs;
    for _ in 0..8 {
        if queue.is_empty() {
            return;
        }
        let mut next = Vec::new();
        for output in std::mem::take(&mut queue) {
            match output {
                Output::Http(request) => next.extend(send(app, request)),
                Output::Document(call) => {
                    let result = run_document(app, &call.kind);
                    next.extend(app.nora.session.handle(Input::DocumentResult { id: call.id, result }));
                }
                Output::Changed => ctx.request_repaint(),
                // A page of the instance (a card's Open), by the host.
                Output::Open(path) => {
                    if let Some(open) = &app.services.nora_open {
                        open(&path);
                    }
                }
            }
        }
        queue = next;
    }
}

/// Hands one call to the host. Without a connection the call fails at once, as an unavailable server.
fn send(app: &mut SheetApp, request: HttpRequest) -> Vec<Output> {
    match &app.services.nora_http {
        Some(http) => {
            http(request);
            Vec::new()
        }
        None => app.nora.session.handle(Input::Http(HttpResponse { id: request.id, status: 503, body: String::new() })),
    }
}

/// Plays one action of Nora's in the open workbook, once what the person is typing is entered. An
/// action that can change the workbook starts from a snapshot of it: when the action fails, the
/// workbook goes back to the snapshot, its history included, so that a failed action leaves it as it
/// was. When it changed something, the undo steps it took are remembered for the pane's Undo; a
/// reading (`sheet.read`, `cell.get`) leaves nothing to take back, and its answer says so (see
/// [`unchanged`]).
fn run_document(app: &mut SheetApp, kind: &DocActionKind) -> Result<Json, String> {
    enter_the_persons_drafts(app)?;
    if !kind.modifies() {
        let before = where_it_stands(app);
        let result = apply(app, kind, &mut Vec::new())?;
        return Ok(if where_it_stands(app) == before { unchanged(result) } else { result });
    }
    let snapshot = Snapshot::take(app).ok_or_else(|| NO_WORKBOOK.to_string())?;
    let mut steps = Vec::new();
    match apply(app, kind, &mut steps) {
        Ok(result) => {
            let after = revision(app, snapshot.uid).unwrap_or(snapshot.revision);
            if steps.is_empty() && after == snapshot.revision {
                return Ok(unchanged(result));
            }
            // What the action really wrote: a link or a formula that leads outside the workbook takes it all back.
            let written = app.session.documents().iter().find(|d| d.uid == snapshot.uid).map(|d| guard::added(&snapshot.wb, &d.wb));
            if let Some(found) = written.filter(|found| !found.is_empty()) {
                snapshot.restore(app);
                return Err(guard::refusal(&what_ran(kind), &found));
            }
            app.nora.changes.push(Change { doc: snapshot.uid, steps, before: snapshot.revision, after });
            let excess = app.nora.changes.len().saturating_sub(MAX_CHANGES);
            app.nora.changes.drain(..excess);
            Ok(result)
        }
        Err(error) => {
            snapshot.restore(app);
            Err(error)
        }
    }
}

/// How Nora is told which of her actions was taken back.
fn what_ran(kind: &DocActionKind) -> String {
    match kind {
        DocActionKind::Execute { command, .. } => format!("`{command}`"),
        DocActionKind::Batch { .. } => "The batch".to_string(),
        DocActionKind::TypeText { .. } => "The typed text".to_string(),
        _ => "The action".to_string(),
    }
}

/// The active workbook and its revision, to tell an action that changed nothing.
fn where_it_stands(app: &SheetApp) -> Option<(u64, u64)> {
    app.session.active().map(|d| (d.uid, d.revision))
}

/// The answer to an action that left the workbook's revision where it was, saying so: `"changed": false`,
/// from which Nora's session (craft-nora from 9d28340) keeps nothing of the action for the pane's Undo.
/// An answer that is not an object (a batch's list of results, a command's bare value) is put in one,
/// under `results` or `result`.
fn unchanged(answer: Json) -> Json {
    match answer {
        Json::Object(mut fields) => {
            fields.insert("changed".to_string(), Json::Bool(false));
            Json::Object(fields)
        }
        Json::Array(results) => json!({"results": results, "changed": false}),
        value => json!({"result": value, "changed": false}),
    }
}

/// What the person is typing is entered before any action of Nora's, as the interface enters it before
/// anything that works on the whole workbook: Nora reads what the person typed, her changes never join
/// it, and a sheet she goes to or a row she inserts never carries the draft elsewhere (the cell editor
/// enters its draft in the active sheet, at the address it started from). A draft that can't be entered
/// (an invalid formula) holds the action: the person's edit stays open as it was, the message their own
/// Enter will show is not shown now, and Nora is told to try again.
fn enter_the_persons_drafts(app: &mut SheetApp) -> Result<(), String> {
    if app.editor.is_some() {
        let shown = app.message.clone();
        if !app.commit_edit(0, 0, false, false) {
            let why = std::mem::replace(&mut app.message, shown).map(|(_, text)| text).unwrap_or_default();
            return Err(format!(
                "the person is typing in a cell and what they typed can't be entered yet ({why}): try again once they have finished"
            ));
        }
    }
    app.commit_text_box_edit()
}

/// The revision of workbook `uid`, which every change, undo and redo of it moves on.
fn revision(app: &SheetApp, uid: u64) -> Option<u64> {
    app.session.documents().iter().find(|d| d.uid == uid).map(|d| d.revision)
}

/// What Nora is told when she asks for the print dialog where the editor has none: the words the server reads as an
/// editor that does not do it (it then tells the person to use File, Print or Export).
const NO_PRINT_DIALOG: &str = "printing is not available in the editor here: the person prints with File, Print";
/// What Nora is told when she asks for the page's password-protected workbook for an e-mail: it would leave in clear.
const PROTECTED_STAYS: &str = "this workbook was opened with its password: it is never handed over in clear for an e-mail, the person joins \
it from NeoMail";

fn apply(app: &mut SheetApp, kind: &DocActionKind, steps: &mut Vec<Weak<Workbook>>) -> Result<Json, String> {
    match kind {
        DocActionKind::Observe => observe(app),
        DocActionKind::Execute { command: id, params } => command(app, id, params, steps),
        DocActionKind::Batch { commands } => batch(app, commands, steps),
        DocActionKind::SelectText { text, occurrence } => select(app, text, *occurrence),
        DocActionKind::TypeText { text, .. } => type_text(app, text, steps),
        DocActionKind::RenderPage { .. } => Err(NO_RENDER.to_string()),
        DocActionKind::Save => save(app),
        DocActionKind::Undo => undo(app),
        DocActionKind::Print => print(app),
        DocActionKind::ExportPdf => export_pdf(app),
        DocActionKind::ExportFile => export_file(app),
    }
}

/// The print dialog on the workbook: the browser's, on its PDF as File › Print lays it out (`file.print`), or, where
/// the editor has printers of its own, its Print page, where the person picks one and prints. Nothing reaches a printer
/// before the person says so.
fn print(app: &mut SheetApp) -> Result<Json, String> {
    if app.services.print_native.is_some() {
        app.open_backstage(crate::backstage::Page::Print);
        return Ok(json!({"dialog": true, "page": "print"}));
    }
    if app.services.print.is_none() {
        return Err(NO_PRINT_DIALOG.to_string());
    }
    app.submit_print(json!({}))?;
    Ok(json!({"dialog": true}))
}

/// The workbook's PDF, as File › Export › PDF makes it, handed to Nora for an e-mail draft: made by the engine itself,
/// so that nothing is downloaded on the way (the menu's export hands its PDF to the browser).
fn export_pdf(app: &mut SheetApp) -> Result<Json, String> {
    if app.keeps_the_pages_encrypted_original() {
        return Err(PROTECTED_STAYS.to_string());
    }
    let d = app.session.active().ok_or_else(|| NO_WORKBOOK.to_string())?;
    let name = format!("{}.pdf", stem_of(&d.display_title()));
    let made = app.session.run("file.exportPdf", json!({}))?;
    let bytes = made
        .get("base64")
        .and_then(Json::as_str)
        .and_then(gridcraft_engine::io::base64_decode)
        .ok_or_else(|| "the workbook could not be made into a PDF".to_string())?;
    craft_nora::protocol::pdf_result(&name, &bytes)
}

/// The workbook in its own format (the page's file's: `.xlsx`…), as it is now with what was not saved yet, handed to
/// Nora for an e-mail draft. Nothing is saved.
fn export_file(app: &mut SheetApp) -> Result<Json, String> {
    if app.keeps_the_pages_encrypted_original() {
        return Err(PROTECTED_STAYS.to_string());
    }
    let d = app.session.active().ok_or_else(|| NO_WORKBOOK.to_string())?;
    let title = d.display_title();
    // A workbook never saved has a title without a format (« Book1 »): it goes as a workbook.
    let name = if std::path::Path::new(&title).extension().is_some() { title } else { format!("{title}.xlsx") };
    let bytes = gridcraft_engine::io::save_bytes(&d.wb, &name).map_err(|e| format!("the workbook could not be written as {name}: {e}"))?;
    craft_nora::protocol::file_result(&name, &bytes)
}

/// A file name without its extension (`Ventes.xlsx` → `Ventes`).
fn stem_of(name: &str) -> String {
    std::path::Path::new(name).file_stem().and_then(|s| s.to_str()).filter(|s| !s.is_empty()).unwrap_or("Workbook").to_string()
}

/// One command of Nora's, run as GridCraft's MCP tools run it ([`gridcraft_engine::Session::execute`]):
/// one the policy [`allowed`], never with a parameter that reaches a file of the computer or carries a
/// password. A command that asks the interface for something (a dialog for the parameters it was not
/// given, an edit of the active cell, a link to open) needs the person: it is refused, and the caller
/// takes back whatever it did. The undo steps it took are added to `steps`.
fn command(app: &mut SheetApp, id: &str, params: &Json, steps: &mut Vec<Weak<Workbook>>) -> Result<Json, String> {
    if !allowed(id) {
        return Err(format!("`{id}` is refused by the editor's policy"));
    }
    if let Some((key, why)) = forbidden_key(params) {
        return Err(format!("`{id}` is refused: `{key}` {why}"));
    }
    let mark = Mark::take(app);
    let result = app.session.execute(id, params.clone()).map_err(|e| e.to_string());
    let asked = app.session.take_ui_requests().iter().find_map(needs_the_person);
    if let Some(mark) = mark {
        steps.extend(mark.steps_since(app));
    }
    let value = result?;
    match asked {
        Some(what) => Err(format!("`{id}` needs the person: it would {what}")),
        None => Ok(value),
    }
}

/// What a request a command made of the interface would have done, when it needs the person.
fn needs_the_person(request: &UiRequest) -> Option<String> {
    match request {
        UiRequest::Dialog(name, _) => Some(format!("open the `{name}` dialog to ask for what it was not given")),
        UiRequest::EditCell(_) => Some("start editing the active cell".to_string()),
        UiRequest::OpenUrl(_) => Some("open a link in the browser".to_string()),
        UiRequest::Message(_) => None,
    }
}

/// The first parameter, at any depth, that Nora may not pass, and why: a name that would reach a file
/// of the computer, or a password (`password`, `newPassword`…), which would lock the person out of their
/// own sheet or workbook with a password they never chose, or use one they gave to nobody.
fn forbidden_key(params: &Json) -> Option<(String, &'static str)> {
    match params {
        Json::Object(map) => map.iter().find_map(|(key, value)| {
            let lowered = key.to_ascii_lowercase();
            if FILE_KEYS.contains(&lowered.as_str()) {
                Some((key.clone(), "would reach a file of the computer"))
            } else if gridcraft_engine::cmd::is_secret_param(key) || lowered.ends_with("password") {
                Some((key.clone(), "is a password, and passwords are the person's"))
            } else {
                forbidden_key(value)
            }
        }),
        Json::Array(list) => list.iter().find_map(forbidden_key),
        _ => None,
    }
}

/// Several commands in order. The first that fails ends the batch with an error, and the caller takes
/// the whole batch back.
fn batch(app: &mut SheetApp, commands: &[CommandCall], steps: &mut Vec<Weak<Workbook>>) -> Result<Json, String> {
    if commands.len() > MAX_BATCH {
        return Err(format!("a batch of {} commands is refused: {MAX_BATCH} at most", commands.len()));
    }
    let mut results = Vec::new();
    for call in commands {
        match command(app, &call.command, &call.params, steps) {
            Ok(value) => results.push(json!({"command": call.command, "ok": true, "result": value})),
            Err(error) => return Err(format!("`{}` failed: {error}", call.command)),
        }
    }
    Ok(Json::Array(results))
}

/// Where the active workbook's history stood before a command: its newest undo step, to tell the steps
/// the command took from the older ones.
struct Mark {
    doc: u64,
    newest: Option<Arc<Workbook>>,
}

impl Mark {
    fn take(app: &SheetApp) -> Option<Mark> {
        let d = app.session.active()?;
        Some(Mark { doc: d.uid, newest: d.undo.last().map(|step| step.wb.clone()) })
    }

    /// The steps the workbook's history gained since: those above the newest step of then, all of them
    /// when there was none (or when so many came that the history dropped it). A command takes one
    /// step, and more when it runs other commands.
    fn steps_since(&self, app: &SheetApp) -> Vec<Weak<Workbook>> {
        let Some(d) = app.session.documents().iter().find(|d| d.uid == self.doc) else { return Vec::new() };
        let first = self.newest.as_ref().and_then(|newest| d.undo.iter().rposition(|step| Arc::ptr_eq(&step.wb, newest))).map_or(0, |at| at + 1);
        d.undo.iter().skip(first).map(|step| Arc::downgrade(&step.wb)).collect()
    }
}

/// The active workbook as it was before an action of Nora's, to go back to when the action fails.
struct Snapshot {
    uid: u64,
    revision: u64,
    wb: Arc<Workbook>,
    saved: Arc<Workbook>,
    selection: gridcraft_engine::Selection,
    sheet_selections: Vec<gridcraft_engine::Selection>,
    undo: Vec<HistoryEntry>,
    redo: Vec<HistoryEntry>,
    path: Option<String>,
    title: String,
    unlocked_ranges: Vec<(String, String)>,
}

impl Snapshot {
    fn take(app: &SheetApp) -> Option<Snapshot> {
        let d = app.session.active()?;
        Some(Snapshot {
            uid: d.uid,
            revision: d.revision,
            wb: d.wb.clone(),
            saved: d.saved.clone(),
            selection: d.selection.clone(),
            sheet_selections: d.sheet_selections.clone(),
            undo: d.undo.clone(),
            redo: d.redo.clone(),
            path: d.path.clone(),
            title: d.title.clone(),
            unlocked_ranges: d.unlocked_ranges.clone(),
        })
    }

    /// Puts the workbook back as it was, and makes it the active one again. Its revision too: the
    /// workbook is the very one it was then, and a change of Nora's made before the failed action stays
    /// the newest, for the pane's Undo.
    fn restore(self, app: &mut SheetApp) {
        let session = &mut app.session;
        let Some(at) = session.documents().iter().position(|d| d.uid == self.uid) else { return };
        session.set_active(at);
        let Some(d) = session.active_mut() else { return };
        let changed = !Arc::ptr_eq(&d.wb, &self.wb);
        d.wb = self.wb;
        d.saved = self.saved;
        d.selection = self.selection;
        d.sheet_selections = self.sheet_selections;
        d.undo = self.undo;
        d.redo = self.redo;
        d.path = self.path;
        d.title = self.title;
        d.unlocked_ranges = self.unlocked_ranges;
        d.revision = self.revision;
        if changed {
            d.calc.rebuild(&d.wb);
        }
    }
}

/// Takes back the last change Nora made, whole, when it is still the newest thing in the workbook: its
/// revision is the one the change left, and its newest undo steps are the change's. When the person
/// changed the workbook since, or undid the change, the pane leaves their work alone. The steps go to
/// the Redo list, as with the title bar's Undo, and the change made just before, when nothing came
/// between the two, is the newest again.
fn undo(app: &mut SheetApp) -> Result<Json, String> {
    let language = app.ui.language;
    let Some(change) = app.nora.changes.pop() else {
        return Err(language.tr("There is nothing of Nora's to undo.").to_string());
    };
    let d = app.session.active().ok_or_else(|| NO_WORKBOOK.to_string())?;
    if d.uid != change.doc {
        return Err(language.tr("Nora's change was made in another workbook.").to_string());
    }
    let count = change.steps.len();
    let newest = d.undo.len().checked_sub(count).and_then(|first| d.undo.get(first..));
    let ours = newest
        .is_some_and(|newest| newest.iter().zip(&change.steps).all(|(step, ours)| ours.upgrade().is_some_and(|ours| Arc::ptr_eq(&step.wb, &ours))));
    if d.revision != change.after || !ours {
        return Err(language.tr("The workbook has changed since Nora's change: undo it step by step with Undo.").to_string());
    }
    if count > 0 {
        app.run("edit.undo", json!({"steps": count}))?;
    }
    if let Some(now) = revision(app, change.doc)
        && let Some(previous) = app.nora.changes.last_mut()
        && previous.doc == change.doc
        && previous.after == change.before
    {
        previous.after = now;
    }
    Ok(json!({"undone": count}))
}

/// Selects what Nora names: a cell or a range by its address (`B4:D10`, `Sheet2!A1:C3`, `A1,C3`), a
/// defined name or a table, or else the `occurrence`-th cell whose text contains `text`. A label that
/// is also a cell's address (`Q1`, `FY25`, `H2`) is read as words when a cell shows exactly those words.
/// The result says which reading was taken: `by` is `address`, `name` or `text`.
fn select(app: &mut SheetApp, text: &str, occurrence: u32) -> Result<Json, String> {
    let wanted = text.trim();
    if wanted.is_empty() {
        return Err("there is nothing to select: name a cell, a range or a text".to_string());
    }
    let wb = &app.session.active().ok_or_else(|| NO_WORKBOOK.to_string())?.wb;
    // What to select, decided before anything runs: (the reading, the command, its parameters, the text of the cell found).
    let found_cell = |(sheet, cell, shown): (String, CellRef, String)| {
        ("text", "selection.set", json!({"range": format!("{}!{}", gridcraft_engine::formula::quote_sheet(&sheet), cell.a1())}), Some(shown))
    };
    let (by, id, params, found) = if is_reference(wb, wanted) {
        // One cell's address alone may be a label too; a range, several areas or a sheet's cell can only be an address.
        let label = !wanted.contains([':', ',', '!', '$']);
        match label.then(|| find_cell(wb, wanted, occurrence, true)) {
            Some(Ok(cell)) => found_cell(cell),
            Some(Err(seen)) if seen > 0 => return Err(not_found(wanted, occurrence, seen)),
            _ => ("address", "selection.set", json!({"range": wanted}), None),
        }
    } else if is_name(wb, wanted) {
        ("name", "edit.goTo", json!({"reference": wanted}), None)
    } else {
        found_cell(find_cell(wb, wanted, occurrence, false).map_err(|seen| not_found(wanted, occurrence, seen))?)
    };
    let mut result = command(app, id, &params, &mut Vec::new())?;
    result["by"] = json!(by);
    if let Some(shown) = found {
        result["text"] = json!(shown);
    }
    if let Some(sheet) = app.session.active().and_then(|d| d.wb.active()).map(|sh| sh.name.clone()) {
        result["sheet"] = json!(sheet);
    }
    app.grid.ensure_visible = true;
    Ok(result)
}

/// Whether `text` is an address the selection takes: areas separated by commas (`A1:B2`, `C:C`,
/// `3:5`), after one optional sheet prefix that names a sheet of the workbook (`'My Sheet'!A1`).
fn is_reference(wb: &Workbook, text: &str) -> bool {
    let (sheet, body) = match text.rsplit_once('!') {
        Some((sheet, body)) => (Some(sheet.trim_matches('\'').replace("''", "'")), body),
        None => (None, text),
    };
    sheet.is_none_or(|sheet| wb.sheet_index(&sheet).is_some()) && body.split(',').all(|area| RangeRef::parse(area).is_some())
}

/// Whether `text` names a defined name or a table of the workbook.
fn is_name(wb: &Workbook, text: &str) -> bool {
    wb.names.iter().any(|name| name.name.eq_ignore_ascii_case(text))
        || wb.sheets.iter().any(|sh| sh.tables.iter().any(|table| table.name.eq_ignore_ascii_case(text)))
}

/// The `occurrence`-th cell (from 1) whose text, as the cell shows it, contains `wanted` (is `wanted`,
/// when `whole`), ignoring case, in reading order: the active sheet first, then the other visible
/// sheets in the order of their tabs. Returns the sheet's name, the cell and its text, or else how many
/// cells show it.
fn find_cell(wb: &Workbook, wanted: &str, occurrence: u32, whole: bool) -> Result<(String, CellRef, String), u32> {
    let needle = wanted.to_lowercase();
    let order = std::iter::once(wb.active_sheet).chain((0..wb.sheets.len()).filter(|si| *si != wb.active_sheet));
    let mut seen = 0u32;
    for si in order {
        let Some(sh) = wb.sheet(si).filter(|sh| sh.visibility == Visibility::Visible) else { continue };
        for (cell, _) in sh.cells.iter() {
            let shown = gridcraft_engine::display::cell_text(wb, sh, cell);
            let lowered = shown.to_lowercase();
            let shows = if whole { lowered.trim() == needle } else { lowered.contains(&needle) };
            if shows {
                seen = seen.saturating_add(1);
                if seen == occurrence {
                    return Ok((sh.name.clone(), cell, shown));
                }
            }
        }
    }
    Err(seen)
}

/// Why no cell was selected, for Nora: `seen` cells show `wanted`.
fn not_found(wanted: &str, occurrence: u32, seen: u32) -> String {
    match seen {
        0 => format!("no cell shows \"{wanted}\""),
        n => format!("only {n} cell(s) show \"{wanted}\", not {occurrence}"),
    }
}

/// Types `text` from the active cell, as the person would type it cell by cell: a tab moves to the
/// next cell, a line break to the next row, under the active cell. Each cell takes its text as typed
/// input (`=` starts a formula, `12%` is a percentage), and the whole text is one undo step.
fn type_text(app: &mut SheetApp, text: &str, steps: &mut Vec<Weak<Workbook>>) -> Result<Json, String> {
    let d = app.session.active().ok_or_else(|| NO_WORKBOOK.to_string())?;
    let active = d.selection.active;
    // A merged area takes what is typed in its first cell, as the cell editor does.
    let at = d.wb.active().and_then(|sh| sh.merge_at(active)).map_or(active, |merge| merge.start);
    let rows = typed_rows(text);
    if rows.is_empty() {
        return Ok(json!({"cell": at.a1(), "cells": 0}));
    }
    let result = command(app, "range.setValues", &json!({"range": at.a1(), "values": rows}), steps)?;
    Ok(json!({"cell": at.a1(), "cells": result.get("cells").cloned().unwrap_or(Json::Null)}))
}

/// The rows of a typed text: lines split at line breaks (a last line break ends the text and adds no
/// empty row), cells at tabs.
fn typed_rows(text: &str) -> Vec<Json> {
    let text = text.strip_suffix('\n').map_or(text, |rest| rest.strip_suffix('\r').unwrap_or(rest));
    if text.is_empty() {
        return Vec::new();
    }
    text.split('\n')
        .map(|line| Json::Array(line.strip_suffix('\r').unwrap_or(line).split('\t').map(|cell| Json::String(cell.to_string())).collect()))
        .collect()
}

/// The title bar's Save: the cell being edited is committed first, then the workbook is written to its
/// file, or, when it has none, goes where this build's Save As sends it without asking (the page that
/// hosts the editor, or the browser's download). Never a file dialog of the computer: a workbook with
/// no file and no such place is not saved.
fn save(app: &mut SheetApp) -> Result<Json, String> {
    // The page's workbook opened with its password goes back to the page in clear never (hosted.rs): Nora is told so,
    // and the person reads it in the message bar.
    if app.keeps_the_pages_encrypted_original() {
        app.refuse_to_save_in_clear();
        return Err("this workbook was opened with its password and the editor cannot save it encrypted: nothing was saved, \
the original stays untouched"
            .to_string());
    }
    let d = app.session.active().ok_or_else(|| NO_WORKBOOK.to_string())?;
    let has_file = d.path.is_some();
    if !has_file && app.services.download.is_none() {
        return Err("this workbook has no file yet: the person saves it with File › Save As".to_string());
    }
    if !app.commit_edit(0, 0, false, false) {
        let why = app.message.as_ref().map(|(_, text)| text.clone()).unwrap_or_default();
        return Err(format!("the cell being edited could not be entered: {why}"));
    }
    app.run("file.save", json!({}))?;
    let name = app.session.active().map(|d| d.display_title()).unwrap_or_default();
    Ok(json!({"saved": true, "name": name, "handedOver": !has_file}))
}

/// What Nora learns when she reads the workbook: its name, the names of its sheets, the active sheet's
/// used range (its values row by row from its first cell, and the same cells as text with their column
/// letters and row numbers, cut at a length that fits her), and the selection with its values.
fn observe(app: &SheetApp) -> Result<Json, String> {
    let d = app.session.active().ok_or_else(|| NO_WORKBOOK.to_string())?;
    let wb = &d.wb;
    let sh = wb.active().ok_or_else(|| "the workbook has no sheet".to_string())?;
    let used = sh.used_range();
    let cells = used.map(|range| read_cells(wb, sh, range, MAX_OBSERVED_CHARS));
    let selected = read_cells(wb, sh, selected_area(d, sh), MAX_OBSERVED_SELECTION_CHARS);
    let (visible, hidden): (Vec<&Sheet>, Vec<&Sheet>) = wb.sheets.iter().map(|s| s.as_ref()).partition(|s| s.visibility == Visibility::Visible);
    let names = |sheets: Vec<&Sheet>| sheets.into_iter().map(|s| s.name.clone()).collect::<Vec<_>>();
    let (values, values_range, text, truncated) = match cells {
        Some(cells) => (cells.values, cells.range.map(|r| r.a1()), cells.text, cells.truncated),
        None => (Vec::new(), None, String::new(), false),
    };
    Ok(json!({
        "name": d.display_title(),
        "sheet": sh.name,
        "sheets": names(visible),
        "hiddenSheets": names(hidden),
        "usedRange": used.map(|r| r.a1()),
        "values": values,
        "valuesRange": values_range,
        "text": text,
        "textTruncated": truncated,
        "selection": {
            "range": d.selection.a1(),
            "activeCell": d.selection.active.a1(),
            "values": selected.values,
            "valuesRange": selected.range.map(|r| r.a1()),
            "truncated": selected.truncated,
        },
    }))
}

/// The current area of the selection, cut to the used range when it is whole rows or columns (to its
/// first cell when they hold nothing).
fn selected_area(d: &DocState, sh: &Sheet) -> RangeRef {
    let current = d.selection.current();
    if !current.is_full_rows() && !current.is_full_cols() {
        return current;
    }
    sh.used_range().and_then(|used| current.intersection(&used)).unwrap_or(RangeRef::cell(current.start))
}

/// Cells told to Nora: their values row by row, the same cells as text, the range both cover and
/// whether they stop short of the range they were read from.
struct Cells {
    values: Vec<Json>,
    text: String,
    range: Option<RangeRef>,
    truncated: bool,
}

/// The cells of `range` from its first row, as far as `budget` characters go for the values and for
/// the text each. The text starts with the column letters; each row with a cell to show is a line that
/// starts with the row's number, its cells separated by tabs.
fn read_cells(wb: &Workbook, sh: &Sheet, range: RangeRef, budget: usize) -> Cells {
    let width = range.width().min(MAX_OBSERVED_COLUMNS);
    let last_col = range.start.col.saturating_add(width.saturating_sub(1));
    let mut text: String = (range.start.col..=last_col).map(|col| format!("\t{}", col_to_letters(col))).collect();
    let mut text_chars = text.chars().count();
    let mut values = Vec::new();
    let mut values_chars = 2;
    let mut last_row = None;
    let mut truncated = width < range.width();
    for row in range.start.row..=range.end.row {
        let cells = (range.start.col..=last_col).map(|col| CellRef::new(row, col));
        let row_values = Json::Array(cells.clone().map(|cell| cut_value(gridcraft_engine::cmd::inspect::value_json(&sh.value(cell)))).collect());
        let mut shown: Vec<String> = cells.map(|cell| cut(&gridcraft_engine::display::cell_text(wb, sh, cell))).collect();
        while shown.last().is_some_and(String::is_empty) {
            shown.pop();
        }
        let line = if shown.is_empty() { String::new() } else { format!("\n{}\t{}", u64::from(row) + 1, shown.join("\t")) };
        let (value_chars, line_chars) = (row_values.to_string().chars().count() + 1, line.chars().count());
        if values_chars + value_chars > budget || text_chars + line_chars > budget {
            truncated = true;
            break;
        }
        values_chars += value_chars;
        text_chars += line_chars;
        values.push(row_values);
        text.push_str(&line);
        last_row = Some(row);
    }
    Cells { values, text, range: last_row.map(|row| RangeRef::new(range.start, CellRef::new(row, last_col))), truncated }
}

/// A cell's text, cut at [`MAX_CELL_CHARS`].
fn cut(text: &str) -> String {
    if text.chars().count() <= MAX_CELL_CHARS {
        return text.to_string();
    }
    let mut short: String = text.chars().take(MAX_CELL_CHARS).collect();
    short.push('…');
    short
}

fn cut_value(value: Json) -> Json {
    match value {
        Json::String(text) => Json::String(cut(&text)),
        other => other,
    }
}

/// Where the person is: the spreadsheet, the workbook's name, and the selection: its address on the
/// first line, then what its cells show, a line per row and a tab between cells.
fn context_of(app: &SheetApp) -> PageContext {
    let Some(d) = app.session.active() else { return PageContext::editor(AppKind::Sheet, DocumentInfo::named("")) };
    let selection = d.wb.active().map(|sh| Selection { text: selection_text(d, sh) });
    let id = named_file(app, d.uid).map(str::to_string);
    PageContext::editor(AppKind::Sheet, DocumentInfo { name: d.display_title(), id, pages: None, selection })
}

fn selection_text(d: &DocState, sh: &Sheet) -> String {
    let area = selected_area(d, sh);
    let mut text = format!("{}!{}", gridcraft_engine::formula::quote_sheet(&sh.name), d.selection.a1());
    let last_col = area.start.col.saturating_add(area.width().min(MAX_OBSERVED_COLUMNS).saturating_sub(1));
    for row in area.start.row..=area.end.row {
        if text.chars().count() >= MAX_SELECTION_CHARS {
            break;
        }
        let mut shown: Vec<String> =
            (area.start.col..=last_col).map(|col| cut(&gridcraft_engine::display::cell_text(&d.wb, sh, CellRef::new(row, col)))).collect();
        while shown.last().is_some_and(String::is_empty) {
            shown.pop();
        }
        text.push('\n');
        text.push_str(&shown.join("\t"));
    }
    text.chars().take(MAX_SELECTION_CHARS).collect::<String>().trim_end().to_string()
}

/// The pane's bubbles in GridCraft's own colours, light or dark. Its theme has no colour for text on
/// the accent: white on the light theme's green, the window's grey on the dark theme's light green, as
/// the account button's initials.
fn style(ui: &Ui) -> NoraStyle {
    let t = Tokens::get(ui.ctx());
    let on_accent = if t.dark { t.window } else { Color32::WHITE };
    NoraStyle {
        user_fill: Some(t.accent),
        user_text: Some(on_accent),
        nora_fill: Some(t.hover),
        nora_text: Some(t.text),
        // //// Neoffice — Nora's bold words in the interface's bold face, when it has one of its own (a system one, or the
        // //// browser build's Inter Bold, theme::has_ui_bold_face); without one, drawn twice a pixel apart (`None`).
        bold_family: theme::has_ui_bold_face().then_some(theme::UI_BOLD),
        ..NoraStyle::default()
    }
}

/// The words of the pane in the interface language. The pane's title bar is GridCraft's own, so the
/// panel's is left empty.
fn labels(language: Language) -> NoraLabels {
    let tr = |text: &'static str| language.tr(text).to_string();
    // `Nora|…`: the button's own word, not the catalog's for the same English elsewhere (`i18n/neoffice.rs`).
    NoraLabels {
        title: String::new(),
        new_conversation: tr("New conversation"),
        undo: tr("Nora|Undo"),
        undo_hint: tr("Undo the last change Nora made to the workbook"),
        placeholder: tr("Ask Nora…"),
        send: tr("Send"),
        stop: tr("Nora|Stop"),
        working: tr("Nora is working…"),
        stopped: tr("Stopped."),
        empty: tr("Ask Nora to fill in, calculate, sort or format your workbook."),
        confirm_heading: tr("Nora wants to:"),
        allow: tr("Nora|Allow"),
        refuse: tr("Refuse"),
        open: tr("Open"),
        describe: describer(language),
    }
}

/// What the pane says under a request lost to a tab in the background, and its button, in the interface language.
fn resume_words(language: Language) -> ResumeWords {
    ResumeWords {
        said: language.tr("This tab was in the background: Nora could not act in the workbook.").to_string(),
        resume: language.tr("Nora|Resume").to_string(),
    }
}

/// What the pane says of a reply of Nora's whose turn could not act in the workbook in time (written before she knew
/// that her change was refused): in its place when nothing was done, under it when part was. In the interface language.
fn correction_words(language: Language) -> CorrectionWords {
    CorrectionWords {
        nothing_done: language.tr("Nothing was done in the workbook: Nora could not act in it in time. Ask again if you still need it.").to_string(),
        partly_done: language.tr("Not all of this was done: part of it could not run in the workbook in time.").to_string(),
    }
}

/// [`describe`] in `language`, as the plain function the panel takes.
fn describer(language: Language) -> fn(&DocActionKind) -> String {
    match language {
        Language::En => |kind: &DocActionKind| describe(Language::En, kind),
        Language::Es => |kind: &DocActionKind| describe(Language::Es, kind),
        Language::Zh => |kind: &DocActionKind| describe(Language::Zh, kind),
        Language::Ja => |kind: &DocActionKind| describe(Language::Ja, kind),
        Language::Ko => |kind: &DocActionKind| describe(Language::Ko, kind),
        Language::Ru => |kind: &DocActionKind| describe(Language::Ru, kind),
        Language::PtBr => |kind: &DocActionKind| describe(Language::PtBr, kind),
        Language::De => |kind: &DocActionKind| describe(Language::De, kind),
        Language::Cs => |kind: &DocActionKind| describe(Language::Cs, kind),
        Language::Fr => |kind: &DocActionKind| describe(Language::Fr, kind),
        Language::Sr => |kind: &DocActionKind| describe(Language::Sr, kind),
        Language::El => |kind: &DocActionKind| describe(Language::El, kind),
        Language::Tr => |kind: &DocActionKind| describe(Language::Tr, kind),
        Language::Pl => |kind: &DocActionKind| describe(Language::Pl, kind),
        Language::It => |kind: &DocActionKind| describe(Language::It, kind),
        Language::Uk => |kind: &DocActionKind| describe(Language::Uk, kind),
        Language::Vi => |kind: &DocActionKind| describe(Language::Vi, kind),
        Language::Nl => |kind: &DocActionKind| describe(Language::Nl, kind),
        Language::Ro => |kind: &DocActionKind| describe(Language::Ro, kind),
    }
}

/// [`error_words`] in `language`, as the plain function the panel takes.
fn error_worder(language: Language) -> fn(&NoraError) -> String {
    match language {
        Language::En => |failure: &NoraError| error_words(Language::En, failure),
        Language::Es => |failure: &NoraError| error_words(Language::Es, failure),
        Language::Zh => |failure: &NoraError| error_words(Language::Zh, failure),
        Language::Ja => |failure: &NoraError| error_words(Language::Ja, failure),
        Language::Ko => |failure: &NoraError| error_words(Language::Ko, failure),
        Language::Ru => |failure: &NoraError| error_words(Language::Ru, failure),
        Language::PtBr => |failure: &NoraError| error_words(Language::PtBr, failure),
        Language::De => |failure: &NoraError| error_words(Language::De, failure),
        Language::Cs => |failure: &NoraError| error_words(Language::Cs, failure),
        Language::Fr => |failure: &NoraError| error_words(Language::Fr, failure),
        Language::Sr => |failure: &NoraError| error_words(Language::Sr, failure),
        Language::El => |failure: &NoraError| error_words(Language::El, failure),
        Language::Tr => |failure: &NoraError| error_words(Language::Tr, failure),
        Language::Pl => |failure: &NoraError| error_words(Language::Pl, failure),
        Language::It => |failure: &NoraError| error_words(Language::It, failure),
        Language::Uk => |failure: &NoraError| error_words(Language::Uk, failure),
        Language::Vi => |failure: &NoraError| error_words(Language::Vi, failure),
        Language::Nl => |failure: &NoraError| error_words(Language::Nl, failure),
        Language::Ro => |failure: &NoraError| error_words(Language::Ro, failure),
    }
}

/// A failure of the connection to Nora in `language` (the panel shows the workbook's own sentences, an action it
/// refused, as they come). The server's own words, when it gave some, are shown as they are: the server writes them
/// for the person.
fn error_words(language: Language, failure: &NoraError) -> String {
    match failure {
        NoraError::Timeout => language.tr("Nora did not answer in time.").to_string(),
        NoraError::Unreachable(_) => language.tr("Nora could not be reached. Check the connection and try again.").to_string(),
        NoraError::Http(401 | 403) => language.tr("Your session has expired. Sign in again.").to_string(),
        NoraError::Http(status) => {
            language.tr("The server could not answer (error {status}). Try again in a moment.").replace("{status}", &status.to_string())
        }
        NoraError::Server { message, .. } => message.clone(),
        NoraError::Malformed(_) => language.tr("Nora's answer could not be read.").to_string(),
    }
}

/// One sentence for an action waiting for the person's yes, in `language`.
fn describe(language: Language, kind: &DocActionKind) -> String {
    match kind {
        DocActionKind::Observe => language.tr("Read the workbook").to_string(),
        DocActionKind::Execute { command, .. } => language.tr("Run {command}").replace("{command}", command),
        DocActionKind::Batch { commands } => language.tr("Run {count} commands").replace("{count}", &commands.len().to_string()),
        DocActionKind::SelectText { text, .. } => language.tr("Select \"{text}\"").replace("{text}", &shorten(text)),
        DocActionKind::TypeText { text, .. } => language.tr("Type \"{text}\"").replace("{text}", &shorten(text)),
        DocActionKind::RenderPage { page, .. } => language.tr("Look at page {page}").replace("{page}", &page.to_string()),
        DocActionKind::Save => language.tr("Save the workbook").to_string(),
        DocActionKind::Undo => language.tr("Undo the last change").to_string(),
        DocActionKind::Print | DocActionKind::ExportPdf | DocActionKind::ExportFile => kind.summary(),
    }
}

/// The start of a text, for a sentence that quotes it.
fn shorten(text: &str) -> String {
    let mut short: String = text.chars().take(40).collect();
    if text.chars().count() > 40 {
        short.push('…');
    }
    short
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Services;
    use egui_kittest::kittest::Queryable;
    use gridcraft_engine::Session;
    use std::sync::Mutex;

    type Outbox = Arc<Mutex<Vec<HttpRequest>>>;
    type Downloads = Arc<Mutex<Vec<(String, usize)>>>;

    fn app(services: Services) -> SheetApp {
        let mut session = Session::new();
        session.new_workbook();
        SheetApp::new(session, services)
    }

    /// A host that records the calls and lets the test answer them.
    fn connected() -> (Services, Outbox, crate::NoraInbox) {
        let outbox: Outbox = Arc::default();
        let inbox: crate::NoraInbox = Arc::default();
        let sent = outbox.clone();
        let services = Services {
            nora_http: Some(Box::new(move |request| sent.lock().unwrap_or_else(|e| e.into_inner()).push(request))),
            nora_inbox: Some(inbox.clone()),
            ..Services::default()
        };
        (services, outbox, inbox)
    }

    fn harness(app: SheetApp) -> egui_kittest::Harness<'static, SheetApp> {
        egui_kittest::Harness::builder().with_size(egui::vec2(1440.0, 900.0)).build_ui_state(
            |ui, app: &mut SheetApp| {
                let ctx = ui.ctx().clone();
                app.logic(&ctx);
                app.ui(ui);
            },
            app,
        )
    }

    fn answer(inbox: &crate::NoraInbox, request: &HttpRequest, message: Json) {
        let body = json!({ "message": message }).to_string();
        inbox.lock().unwrap_or_else(|e| e.into_inner()).push(HttpResponse { id: request.id, status: 200, body });
    }

    fn take(outbox: &Outbox) -> Vec<HttpRequest> {
        std::mem::take(&mut *outbox.lock().unwrap_or_else(|e| e.into_inner()))
    }

    /// Sends a message from the pane and answers the start of the conversation, so that the pane reads
    /// the actions; returns the read of the actions to answer.
    fn ask(h: &mut egui_kittest::Harness<'static, SheetApp>, outbox: &Outbox, inbox: &crate::NoraInbox, message: &str) -> HttpRequest {
        h.state_mut().nora.panel.set_draft(message);
        h.run_steps(2);
        h.get_by_label("Send").click();
        h.run_steps(4);
        let first = take(outbox);
        answer(inbox, &first[0], json!({"thread_id": "t1"}));
        h.run_steps(6);
        take(outbox).into_iter().find(|r| r.path.ends_with("get_browser_actions")).expect("the actions are read")
    }

    fn run(app: &mut SheetApp, kind: DocActionKind) -> Result<Json, String> {
        run_document(app, &kind)
    }

    fn execute(app: &mut SheetApp, command: &str, params: Json) -> Result<Json, String> {
        run(app, DocActionKind::Execute { command: command.into(), params })
    }

    fn call(command: &str, params: Json) -> CommandCall {
        CommandCall { command: command.into(), params }
    }

    fn set(app: &mut SheetApp, cell: &str, input: &str) {
        app.run("cell.set", json!({"cell": cell, "input": input})).unwrap();
    }

    fn value(app: &mut SheetApp, cell: &str) -> Json {
        app.session.run("cell.get", json!({"cell": cell})).unwrap()["value"].clone()
    }

    fn history(app: &SheetApp) -> (Vec<String>, Vec<String>) {
        let d = app.session.active().unwrap();
        (d.undo.iter().map(|e| e.label.clone()).collect(), d.redo.iter().map(|e| e.label.clone()).collect())
    }

    /// The Nora pane is a UI command, off by default and kept between runs.
    #[test]
    fn nora_is_a_ui_command_kept_between_runs() {
        let mut a = app(Services::default());
        assert!(!a.ui.nora_pane, "off by default");
        assert_eq!(a.run("ui.nora", json!({})).unwrap(), json!({"on": true}), "no value toggles it");
        assert_eq!(a.run("ui.nora", json!({})).unwrap(), json!({"on": false}));
        assert_eq!(a.run("ui.nora", json!({"on": true})).unwrap(), json!({"on": true}));
        assert_eq!(a.run("ui.nora", json!({"on": true})).unwrap(), json!({"on": true}), "a value sets it");
        assert_eq!(a.run("ui.nora", json!({"value": false})).unwrap(), json!({"on": false}), "WordCraft's `value` too");
        a.run("ui.nora", json!({"on": true})).unwrap();
        for junk in [json!({"on": "yes"}), json!({"on": 1}), json!({"on": null}), json!({"value": [true]})] {
            assert!(a.run("ui.nora", junk).is_err());
        }
        assert!(a.ui.nora_pane, "a rejected value keeps the setting");
        let saved = serde_json::to_string(&a.ui).unwrap();
        assert!(saved.contains(r#""noraPane":true"#), "{saved}");
        let restored: crate::UiState = serde_json::from_str(&saved).unwrap();
        assert!(restored.nora_pane);
        // Settings saved before the pane existed leave it off.
        let old: crate::UiState = serde_json::from_str(r#"{"ribbonTab":"Insert"}"#).unwrap();
        assert!(!old.nora_pane);
    }

    /// Closed, the pane leaves a rail with Nora's sphere at the right edge where Nora is offered; its click opens the pane
    /// again on the same conversation. Where Nora isn't offered, no rail. Outside a browser nothing is remembered: the
    /// pane opens as before.
    #[test]
    fn a_closed_pane_leaves_a_rail_that_opens_it_again() {
        let (services, outbox, inbox) = connected();
        let mut a = app(services);
        assert_eq!(open_as_remembered(&mut a, true).unwrap(), json!({"on": true}), "open where Nora is offered");
        let mut h = harness(a);
        h.run();
        let _ = ask(&mut h, &outbox, &inbox, "Bonjour");
        assert_eq!(h.state().nora.session.thread_id(), Some("t1"));
        assert!(h.query_by_label("Open Nora").is_none(), "no rail while the pane is open");
        h.state_mut().run("ui.nora", json!({"on": false})).unwrap();
        h.run();
        let rail = h.get_by_label("Open Nora").rect();
        // At the right edge (the test window keeps a margin of its own).
        assert!(rail.min.x >= 1440.0 - craft_nora_ui::launcher::RAIL_WIDTH - 16.0, "at the right edge: {rail:?}");
        h.get_by_label("Open Nora").click();
        h.run();
        assert!(h.state().ui.nora_pane, "open again");
        assert_eq!(h.state().nora.session.thread_id(), Some("t1"), "on the same conversation");
        assert!(h.query_by_label("Open Nora").is_none());
        // Where Nora isn't offered: no pane, no rail.
        let mut alone = app(Services::default());
        assert_eq!(open_as_remembered(&mut alone, false).unwrap(), json!({"on": false}));
        let mut h = harness(alone);
        h.run();
        assert!(h.query_by_label("Open Nora").is_none());
    }

    /// The rail's words are the interface language's.
    #[test]
    fn the_rail_speaks_the_interface_language() {
        assert_eq!(Language::Fr.tr("Nora|Open Nora"), "Ouvrir Nora");
        assert_eq!(Language::En.tr("Nora|Open Nora"), "Open Nora");
        for language in Language::ALL {
            assert!(!language.tr("Nora|Open Nora").contains('|'), "{language:?}");
        }
    }

    /// File › Options › Assistant is the `ui.nora` setting.
    #[test]
    fn the_options_page_shows_and_hides_the_pane() {
        let mut a = app(Services::default());
        a.run("ui.backstage", json!({"page": "options"})).unwrap();
        let mut h = harness(a);
        h.run_steps(4);
        assert!(h.query_by_label("Assistant").is_some(), "the section is there");
        h.get_by_label("Show the Nora assistant pane (experimental)").click();
        h.run_steps(4);
        assert!(h.state().ui.nora_pane, "the checkbox turns it on");
        h.get_by_label("Show the Nora assistant pane (experimental)").click();
        h.run_steps(4);
        assert!(!h.state().ui.nora_pane, "and off again");
    }

    /// Nothing is drawn while the pane is off, and without a connection the pane says why it can't be
    /// used instead of offering a text box.
    #[test]
    fn without_a_connection_the_pane_says_nora_isnt_available() {
        let mut h = harness(app(Services::default()));
        h.run_steps(4);
        assert!(h.query_by_label("Nora isn't available in this version yet").is_none(), "off by default");
        h.state_mut().run("ui.nora", json!({"on": true})).unwrap();
        h.run_steps(4);
        assert!(h.query_by_label("Nora").is_some(), "the pane's title");
        assert!(h.query_by_label("Nora isn't available in this version yet").is_some());
        assert!(h.query_by_role(egui::accesskit::Role::MultilineTextInput).is_none(), "no text box to lose a message in");
        assert!(h.query_by_label("Ask Nora to fill in, calculate, sort or format your workbook.").is_some());
        // The title bar's close button is a painted icon that runs this very command.
        h.state_mut().run("ui.nora", json!({"on": false})).unwrap();
        h.run_steps(4);
        assert!(h.query_by_label("Nora isn't available in this version yet").is_none(), "closed, it draws nothing");
    }

    /// The words of the pane follow the interface language.
    #[test]
    fn the_pane_speaks_french_when_the_interface_does() {
        let mut a = app(Services::default());
        a.run("app.language.set", json!({"language": "fr"})).unwrap();
        a.run("ui.nora", json!({"on": true})).unwrap();
        let mut h = harness(a);
        h.run_steps(4);
        assert!(h.query_by_label("Nora n'est pas encore disponible dans cette version").is_some());
        assert!(h.query_by_label("Nouvelle conversation").is_some());
        assert!(h.query_by_label("Demandez à Nora de remplir, de calculer, de trier ou de mettre en forme votre classeur.").is_some());
        assert!(h.query_by_label("New conversation").is_none());
    }

    /// A message typed in the pane leaves as the first call of the conversation, carrying where the
    /// person is; the host's answers show up in the pane; the workbook is left alone.
    #[test]
    fn a_connected_pane_sends_the_message_with_its_context_and_shows_the_reply() {
        let (services, outbox, inbox) = connected();
        let mut a = app(services);
        set(&mut a, "B2", "1200");
        a.run("selection.set", json!({"range": "B2"})).unwrap();
        a.run("ui.nora", json!({"on": true})).unwrap();
        let mut h = harness(a);
        h.run_steps(4);
        assert!(h.query_by_label("Nora isn't available in this version yet").is_none());
        let revision = h.state().session.active().map(|d| d.revision);

        h.get_by_role(egui::accesskit::Role::MultilineTextInput).focus();
        h.run_steps(2);
        h.get_by_role(egui::accesskit::Role::MultilineTextInput).type_text("Total the column");
        h.run_steps(2);
        h.key_press(egui::Key::Enter);
        h.run_steps(4);

        let calls = take(&outbox);
        assert_eq!(calls.len(), 1, "{calls:?}");
        assert!(calls[0].path.ends_with("nora.api.chat.start_or_resume_thread"), "{}", calls[0].path);
        assert_eq!(calls[0].body["message"], "Total the column");
        let context: Json = serde_json::from_str(calls[0].body["context"].as_str().unwrap_or_default()).unwrap();
        assert_eq!(context["surface"], "editor");
        assert_eq!(context["app"], "sheet");
        assert_eq!(context["document"]["name"], "Book1");
        assert_eq!(context["document"]["selection"]["text"], "Sheet1!B2\n1200", "the selection's address, then what it shows");
        assert!(h.query_by_label("Total the column").is_some(), "the person's own message is in the transcript");
        assert_eq!(h.state().session.active().map(|d| d.revision), revision, "typing in the pane never edits the workbook");
        assert_eq!(value(h.state_mut(), "A1"), Json::Null);

        // The server gives the conversation an id; the pane then reads the replies and the actions.
        answer(&inbox, &calls[0], json!({"thread_id": "t1", "sent_at": 1000.0}));
        h.run_steps(6);
        let polls = take(&outbox);
        let reads = polls.iter().find(|r| r.path.ends_with("get_messages")).expect("the replies are read");
        let actions = polls.iter().find(|r| r.path.ends_with("get_browser_actions")).expect("the actions are read");
        let replies = json!([
            {"role": "assistant", "text": "An answer from an earlier conversation.", "ts": 900.0},
            {"role": "assistant", "text": "The total is 1200.", "ts": 1001.0}
        ]);
        answer(&inbox, reads, json!({"messages": replies}));
        answer(&inbox, actions, json!({"actions": []}));
        h.run_steps(4);
        assert!(h.query_by_label("The total is 1200.").is_some(), "Nora's reply is shown");
        assert!(h.query_by_label("An answer from an earlier conversation.").is_none(), "a reply older than the send is not");
    }

    /// What Nora asks for in the workbook is played, and she gets the result by callback.
    #[test]
    fn a_document_action_runs_and_nora_gets_the_observation() {
        let (services, outbox, inbox) = connected();
        let mut a = app(services);
        a.run("ui.nora", json!({"on": true})).unwrap();
        let mut h = harness(a);
        h.run_steps(4);
        let actions = ask(&mut h, &outbox, &inbox, "Write a greeting");
        answer(&inbox, &actions, json!({"actions": [{"action": "doc_type_text", "text": "Bonjour\tà tous", "callback_id": "c1"}]}));
        h.run_steps(4);
        assert_eq!(value(h.state_mut(), "A1"), json!("Bonjour"), "typed in the active cell");
        assert_eq!(value(h.state_mut(), "B1"), json!("à tous"), "a tab moves to the next cell");
        let told = take(&outbox);
        let observation = told.iter().find(|r| r.path.ends_with("receive_observation")).expect("Nora gets an answer");
        assert_eq!(observation.body["callback_id"], "c1");
        assert_eq!(observation.body["observation"]["ok"], true);
        assert!(h.state().nora.session.can_undo(), "the panel offers to undo it");
    }

    /// Saving waits for the person's yes, asked in the interface language; a no reaches Nora and nothing
    /// is saved. With `GRIDCRAFT_NORA_SCREENSHOT=<path.png>` the pane is also drawn to that file (and to a
    /// `-dark` one), to look at it.
    #[test]
    fn saving_waits_for_the_persons_answer_in_their_language() {
        let (services, outbox, inbox) = connected();
        let mut a = app(services);
        a.run("app.language.set", json!({"language": "fr"})).unwrap();
        a.run("range.setValues", json!({"range": "A1", "values": [["Poste", "Montant"], ["Loyer", 1200], ["Total", "=SUM(B2:B2)"]]})).unwrap();
        a.run("ui.nora", json!({"on": true})).unwrap();
        let mut h = harness(a);
        h.run_steps(4);
        h.state_mut().nora.panel.set_draft("Enregistre le classeur");
        h.run_steps(2);
        h.get_by_label("Envoyer").click();
        h.run_steps(4);
        let first = take(&outbox);
        answer(&inbox, &first[0], json!({"thread_id": "t1", "sent_at": 1000.0}));
        h.run_steps(6);
        let polls = take(&outbox);
        let reads = polls.iter().find(|r| r.path.ends_with("get_messages")).expect("the replies are read");
        let actions = polls.iter().find(|r| r.path.ends_with("get_browser_actions")).expect("the actions are read");
        answer(&inbox, reads, json!({"messages": [{"role": "assistant", "text": "Je l'enregistre dès que vous l'autorisez.", "ts": 1001.0}]}));
        answer(&inbox, actions, json!({"actions": [{"action": "doc_save", "callback_id": "c3"}]}));
        h.run_steps(6);
        assert!(h.query_by_label("Je l'enregistre dès que vous l'autorisez.").is_some(), "Nora's reply is shown");
        assert!(h.query_by_label("Nora souhaite :").is_some());
        assert!(h.query_by_label("Enregistrer le classeur").is_some());
        assert!(h.query_by_label("Autoriser").is_some());
        assert!(take(&outbox).iter().all(|r| !r.path.ends_with("receive_observation")), "nothing is played before the answer");
        if let Ok(path) = std::env::var("GRIDCRAFT_NORA_SCREENSHOT") {
            h.render().unwrap().save(&path).unwrap();
            h.state_mut().ui.dark = true;
            h.run_steps(4);
            h.render().unwrap().save(path.replace(".png", "-dark.png")).unwrap();
        }
        h.get_by_label("Refuser").click();
        h.run_steps(4);
        let told = take(&outbox);
        let observation = told.iter().find(|r| r.path.ends_with("receive_observation")).expect("Nora gets an answer");
        assert_eq!(observation.body["callback_id"], "c3");
        assert_eq!(observation.body["observation"]["error"], "the user declined");
        assert!(h.state().session.active().is_some_and(|d| d.path.is_none() && d.is_dirty()), "nothing was saved");
    }

    /// A command the policy refuses never reaches the workbook or the interface, and Nora is told why.
    #[test]
    fn a_refused_command_never_runs() {
        let (services, outbox, inbox) = connected();
        let mut a = app(services);
        a.run("ui.nora", json!({"on": true})).unwrap();
        let mut h = harness(a);
        h.run_steps(4);
        let actions = ask(&mut h, &outbox, &inbox, "Switch to dark");
        answer(
            &inbox,
            &actions,
            json!({"actions": [{"action": "doc_execute", "command": "view.darkMode", "params": {"on": true}, "callback_id": "c2"}]}),
        );
        h.run_steps(4);
        let told = take(&outbox);
        let observation = told.iter().find(|r| r.path.ends_with("receive_observation")).expect("Nora gets an answer");
        assert_eq!(observation.body["observation"]["ok"], false);
        assert!(observation.body["observation"]["error"].as_str().unwrap_or_default().contains("refused"));
        assert!(!h.state().ui.dark, "the person's theme is theirs");
    }

    #[test]
    fn nora_reads_the_sheet_its_selection_and_the_names_of_the_sheets() {
        let mut a = app(Services::default());
        set(&mut a, "A1", "Item");
        set(&mut a, "B1", "Cost");
        set(&mut a, "A2", "Rent");
        set(&mut a, "B2", "1200");
        set(&mut a, "C3", "=B2*2");
        a.run("home.insertSheet", json!({"name": "Notes"})).unwrap();
        a.run("sheet.activate", json!({"sheet": 0})).unwrap();
        a.run("selection.set", json!({"range": "A2:B2"})).unwrap();
        let seen = run(&mut a, DocActionKind::Observe).unwrap();
        assert_eq!(seen["name"], "Book1");
        assert_eq!(seen["sheet"], "Sheet1");
        assert_eq!(seen["sheets"], json!(["Sheet1", "Notes"]));
        assert_eq!(seen["hiddenSheets"], json!([]));
        assert_eq!(seen["usedRange"], "A1:C3");
        assert_eq!(seen["valuesRange"], "A1:C3");
        assert_eq!(seen["values"], json!([["Item", "Cost", null], ["Rent", 1200.0, null], [null, null, 2400.0]]));
        assert_eq!(seen["text"], "\tA\tB\tC\n1\tItem\tCost\n2\tRent\t1200\n3\t\t\t2400", "the document's text, with its columns and rows");
        assert_eq!(seen["textTruncated"], false);
        assert_eq!(seen["selection"]["range"], "A2:B2");
        assert_eq!(seen["selection"]["activeCell"], "A2");
        assert_eq!(seen["selection"]["values"], json!([["Rent", 1200.0]]));
        assert_eq!(seen["selection"]["truncated"], false);
        // An empty sheet reads as empty, not as an error.
        a.run("sheet.activate", json!({"sheet": 1})).unwrap();
        let empty = run(&mut a, DocActionKind::Observe).unwrap();
        assert_eq!(empty["sheet"], "Notes");
        assert_eq!(empty["usedRange"], Json::Null);
        assert_eq!(empty["values"], json!([]));
        assert_eq!(empty["selection"]["values"], json!([[null]]));
    }

    #[test]
    fn what_nora_reads_is_cut_at_a_length_that_fits_her() {
        let mut a = app(Services::default());
        let long = "é".repeat(MAX_CELL_CHARS + 10);
        let rows: Vec<Json> = (0..3_000).map(|i| json!([long.clone(), format!("row {i}")])).collect();
        a.run("range.setValues", json!({"range": "A1", "values": rows})).unwrap();
        let seen = run(&mut a, DocActionKind::Observe).unwrap();
        assert_eq!(seen["usedRange"], "A1:B3000");
        assert_eq!(seen["textTruncated"], true);
        let text = seen["text"].as_str().unwrap();
        assert!(text.chars().count() <= MAX_OBSERVED_CHARS, "{}", text.chars().count());
        assert!(seen["values"].to_string().chars().count() <= MAX_OBSERVED_CHARS);
        let shown = seen["values"].as_array().map(Vec::len).unwrap();
        assert!(shown > 10 && shown < 3_000, "{shown}");
        assert_eq!(seen["valuesRange"], format!("A1:B{shown}"));
        let first = seen["values"][0][0].as_str().unwrap();
        assert_eq!(first.chars().count(), MAX_CELL_CHARS + 1, "a long text is cut");
        assert!(first.ends_with('…'));
        // A whole column selected reads as far as the used range goes, within its own budget.
        a.run("selection.set", json!({"range": "B:B"})).unwrap();
        let seen = run(&mut a, DocActionKind::Observe).unwrap();
        assert_eq!(seen["selection"]["range"], "B:B");
        assert_eq!(seen["selection"]["values"][0], json!(["row 0"]));
        assert!(seen["selection"]["values"].to_string().chars().count() <= MAX_OBSERVED_SELECTION_CHARS);
        assert_eq!(seen["selection"]["truncated"], true);
    }

    #[test]
    fn the_context_of_a_message_is_the_workbook_and_the_selection() {
        let mut a = app(Services::default());
        set(&mut a, "A1", "Item");
        set(&mut a, "B1", "Cost");
        set(&mut a, "B2", "5");
        a.run("selection.set", json!({"range": "A1:C2"})).unwrap();
        let context = context_of(&a);
        assert_eq!(context.app, AppKind::Sheet);
        assert_eq!(context.document.name, "Book1");
        assert_eq!(context.document.selection.map(|s| s.text).as_deref(), Some("Sheet1!A1:C2\nItem\tCost\n\t5"));
        // A whole sheet selected is told within its length.
        let rows: Vec<Json> = (0..5_000).map(|i| json!([format!("line {i}")])).collect();
        a.run("range.setValues", json!({"range": "A1", "values": rows})).unwrap();
        a.run("edit.selectAll", json!({})).unwrap();
        a.run("edit.selectAll", json!({})).unwrap();
        let text = context_of(&a).document.selection.map(|s| s.text).unwrap_or_default();
        assert!(text.chars().count() <= MAX_SELECTION_CHARS, "{}", text.chars().count());
        assert!(text.contains("line 0"), "{text}");
    }

    #[test]
    fn whole_columns_of_an_empty_sheet_read_as_their_first_cell() {
        let mut a = app(Services::default());
        a.run("selection.set", json!({"range": "B:D"})).unwrap();
        assert_eq!(context_of(&a).document.selection.map(|s| s.text).as_deref(), Some("Sheet1!B:D"));
        let seen = run(&mut a, DocActionKind::Observe).unwrap();
        assert_eq!(seen["selection"]["range"], "B:D");
        assert_eq!(seen["selection"]["values"], json!([[null]]));
        assert_eq!(seen["selection"]["valuesRange"], "B1");
    }

    #[test]
    fn nora_selects_by_address_by_name_or_by_text() {
        let mut a = app(Services::default());
        set(&mut a, "A1", "Total due");
        set(&mut a, "C5", "Monthly total");
        a.run("home.insertSheet", json!({"name": "Sheet 2"})).unwrap();
        set(&mut a, "D4", "total of totals");
        a.run("sheet.activate", json!({"sheet": 0})).unwrap();
        let select = |a: &mut SheetApp, text: &str, occurrence: u32| run(a, DocActionKind::SelectText { text: text.into(), occurrence });
        let selection =
            |a: &SheetApp| a.session.active().map(|d| (d.wb.active().map(|s| s.name.clone()).unwrap_or_default(), d.selection.a1())).unwrap();

        select(&mut a, "B4:D10", 1).unwrap();
        assert_eq!(selection(&a), ("Sheet1".into(), "B4:D10".into()), "an address");
        select(&mut a, " A1,C3 ", 1).unwrap();
        assert_eq!(selection(&a), ("Sheet1".into(), "A1,C3".into()), "several areas");
        let seen = select(&mut a, "'Sheet 2'!A1:B2", 1).unwrap();
        assert_eq!(selection(&a), ("Sheet 2".into(), "A1:B2".into()), "on another sheet");
        assert_eq!(seen["sheet"], "Sheet 2");
        a.run("sheet.activate", json!({"sheet": 0})).unwrap();

        a.run("formulas.defineName", json!({"name": "Header", "refersTo": "=Sheet1!$A$1:$C$1"})).unwrap();
        select(&mut a, "header", 1).unwrap();
        assert_eq!(selection(&a), ("Sheet1".into(), "A1:C1".into()), "a defined name");

        let seen = select(&mut a, "TOTAL", 1).unwrap();
        assert_eq!(selection(&a), ("Sheet1".into(), "A1".into()), "the first cell that shows the words, ignoring case");
        assert_eq!(seen["text"], "Total due");
        select(&mut a, "total", 2).unwrap();
        assert_eq!(selection(&a), ("Sheet1".into(), "C5".into()), "in reading order");
        select(&mut a, "total", 3).unwrap();
        assert_eq!(selection(&a), ("Sheet 2".into(), "D4".into()), "then on the other sheets");
        let error = select(&mut a, "total", 4).unwrap_err();
        assert!(error.contains("only 3"), "{error}");
        assert!(select(&mut a, "nowhere", 1).unwrap_err().contains("no cell"));
        assert!(select(&mut a, "  ", 1).is_err());
        assert!(a.nora.changes.is_empty(), "a selection is no change to undo");
    }

    /// A label such as Q1, FY25 or H2 is also a cell's address: a cell that shows exactly those words wins, and the
    /// result says which reading was taken.
    #[test]
    fn a_label_that_reads_like_an_address_selects_the_cell_that_shows_it() {
        let mut a = app(Services::default());
        set(&mut a, "A1", "Q1");
        set(&mut a, "B1", "FY25");
        set(&mut a, "C1", "h2");
        set(&mut a, "A5", "Q3 total");
        let select = |a: &mut SheetApp, text: &str| run(a, DocActionKind::SelectText { text: text.into(), occurrence: 1 }).unwrap();
        let selection = |a: &SheetApp| a.session.active().map(|d| d.selection.a1()).unwrap_or_default();
        for (label, cell) in [("Q1", "A1"), ("FY25", "B1"), ("H2", "C1")] {
            let seen = select(&mut a, label);
            assert_eq!(selection(&a), cell, "{label}: the cell that shows it");
            assert_eq!(seen["by"], "text", "{label}");
        }
        // No cell shows exactly "Q3" ("Q3 total" only contains it): the address.
        let seen = select(&mut a, "Q3");
        assert_eq!(selection(&a), "Q3");
        assert_eq!(seen["by"], "address");
        // A range or a sheet's cell can only be an address.
        assert_eq!(select(&mut a, "A1:B2")["by"], "address");
        assert_eq!(selection(&a), "A1:B2");
        assert_eq!(select(&mut a, "total")["by"], "text");
    }

    #[test]
    fn typing_fills_the_cells_from_the_active_cell() {
        let mut a = app(Services::default());
        a.run("selection.set", json!({"range": "B2"})).unwrap();
        let before = history(&a).0.len();
        let typed = run(&mut a, DocActionKind::TypeText { text: "Item\tShare\n=2+3\t12%\n".into(), paragraphs: false }).unwrap();
        assert_eq!(typed["cell"], "B2");
        assert_eq!(value(&mut a, "B2"), json!("Item"));
        assert_eq!(value(&mut a, "C2"), json!("Share"));
        assert_eq!(value(&mut a, "B3"), json!(5.0), "typed input: a formula");
        let share = value(&mut a, "C3").as_f64().unwrap_or_default();
        assert!((share - 0.12).abs() < 1e-12, "typed input: a percentage, not {share}");
        assert_eq!(value(&mut a, "B4"), Json::Null, "the last line break adds no row");
        assert_eq!(history(&a).0.len(), before + 1, "one undo step for the whole text");
        assert_eq!(a.session.active().unwrap().selection.a1(), "B2", "the selection stays");
        // Nothing to type changes nothing.
        let revision = a.session.active().map(|d| d.revision);
        assert_eq!(run(&mut a, DocActionKind::TypeText { text: String::new(), paragraphs: false }).unwrap()["cells"], 0);
        assert_eq!(a.session.active().map(|d| d.revision), revision);
    }

    #[test]
    fn typing_on_a_locked_cell_fails_and_changes_nothing() {
        let mut a = app(Services::default());
        set(&mut a, "A1", "kept");
        a.run("review.protectSheet", json!({})).unwrap();
        let before = history(&a);
        assert!(run(&mut a, DocActionKind::TypeText { text: "x".into(), paragraphs: false }).is_err());
        assert_eq!(value(&mut a, "A1"), json!("kept"));
        assert_eq!(history(&a), before);
        assert!(a.nora.changes.is_empty());
    }

    #[test]
    fn undo_takes_back_a_whole_batch_and_only_what_nora_did() {
        let mut a = app(Services::default());
        set(&mut a, "A1", "mine");
        let commands = vec![
            call("cell.set", json!({"cell": "A2", "input": "a"})),
            call("cell.set", json!({"cell": "A3", "input": "b"})),
            call("home.bold", json!({"range": "A2:A3"})),
        ];
        let results = run(&mut a, DocActionKind::Batch { commands }).unwrap();
        assert_eq!(results.as_array().map(Vec::len), Some(3));
        assert_eq!(a.nora.changes.last().map(|c| c.steps.len()), Some(3), "a step per command");
        assert_eq!(run(&mut a, DocActionKind::Undo).unwrap(), json!({"undone": 3}));
        assert_eq!(value(&mut a, "A2"), Json::Null);
        assert_eq!(value(&mut a, "A3"), Json::Null);
        assert_eq!(value(&mut a, "A1"), json!("mine"), "the person's own typing stays");
        assert_eq!(history(&a).1.len(), 3, "what was undone can be redone, as with the title bar's Undo");
        assert!(run(&mut a, DocActionKind::Undo).is_err(), "nothing left of Nora's to undo");
        assert_eq!(value(&mut a, "A1"), json!("mine"));
    }

    #[test]
    fn a_command_that_runs_others_is_undone_whole() {
        let mut a = app(Services::default());
        set(&mut a, "A1", "1");
        set(&mut a, "A2", "2");
        a.run("selection.set", json!({"range": "A3"})).unwrap();
        // AutoSum enters its formula with a command of its own: two steps, one action.
        execute(&mut a, "formulas.autoSum", json!({})).unwrap();
        assert_eq!(value(&mut a, "A3"), json!(3.0));
        run(&mut a, DocActionKind::Undo).unwrap();
        assert_eq!(value(&mut a, "A3"), Json::Null);
        assert_eq!(history(&a).0.len(), 2, "the person's two steps are what is left");
    }

    #[test]
    fn undo_leaves_the_persons_later_changes_alone() {
        let mut a = app(Services::default());
        execute(&mut a, "cell.set", json!({"cell": "A1", "input": "Nora's"})).unwrap();
        set(&mut a, "A2", "the person's");
        let error = run(&mut a, DocActionKind::Undo).unwrap_err();
        assert!(error.contains("changed since"), "{error}");
        assert_eq!(value(&mut a, "A1"), json!("Nora's"));
        assert_eq!(value(&mut a, "A2"), json!("the person's"));
        // The refused change is forgotten, as the session forgets its own record of it: the title bar's
        // Undo still takes it back, step by step.
        assert!(a.nora.changes.is_empty());
    }

    #[test]
    fn undo_still_finds_nora_s_change_when_the_history_is_full() {
        let mut a = app(Services::default());
        for i in 0..110 {
            set(&mut a, "A1", &i.to_string());
        }
        assert_eq!(history(&a).0.len(), 100, "the workbook keeps 100 steps");
        execute(&mut a, "cell.set", json!({"cell": "B1", "input": "Nora's"})).unwrap();
        assert_eq!(a.nora.changes.last().map(|c| c.steps.len()), Some(1));
        run(&mut a, DocActionKind::Undo).unwrap();
        assert_eq!(value(&mut a, "B1"), Json::Null);
        assert_eq!(value(&mut a, "A1"), json!(109.0), "and only Nora's");
    }

    /// The person's own Undo puts back the very workbook Nora's change started from, so the undo step of their next
    /// edit restores that same workbook and looks like hers: the document's revision tells them apart.
    #[test]
    fn undo_never_takes_back_the_persons_edit_after_their_own_undo() {
        let mut a = app(Services::default());
        execute(&mut a, "cell.set", json!({"cell": "B2", "input": "Nora's"})).unwrap();
        a.run("edit.undo", json!({})).unwrap();
        set(&mut a, "C5", "mine");
        let error = run(&mut a, DocActionKind::Undo).unwrap_err();
        assert!(error.contains("changed since"), "{error}");
        assert_eq!(value(&mut a, "C5"), json!("mine"), "the person's edit stays");
        assert_eq!(value(&mut a, "B2"), Json::Null);
    }

    #[test]
    fn undo_takes_back_nora_s_changes_one_after_the_other_but_never_past_the_person() {
        let mut a = app(Services::default());
        execute(&mut a, "cell.set", json!({"cell": "A1", "input": "one"})).unwrap();
        execute(&mut a, "cell.set", json!({"cell": "A2", "input": "two"})).unwrap();
        run(&mut a, DocActionKind::Undo).unwrap();
        run(&mut a, DocActionKind::Undo).unwrap();
        assert_eq!((value(&mut a, "A1"), value(&mut a, "A2")), (Json::Null, Json::Null), "two changes in a row, both taken back");
        // Between two changes of Nora's, the person undoes the first and types: the first stays theirs to undo.
        execute(&mut a, "cell.set", json!({"cell": "B1", "input": "first"})).unwrap();
        a.run("edit.undo", json!({})).unwrap();
        set(&mut a, "C1", "mine");
        execute(&mut a, "cell.set", json!({"cell": "B2", "input": "second"})).unwrap();
        run(&mut a, DocActionKind::Undo).unwrap();
        assert_eq!(value(&mut a, "B2"), Json::Null, "the newest change is taken back");
        assert!(run(&mut a, DocActionKind::Undo).is_err(), "the older one is behind the person's edit");
        assert_eq!(value(&mut a, "C1"), json!("mine"));
    }

    #[test]
    fn a_failed_action_keeps_the_change_before_it_undoable() {
        let mut a = app(Services::default());
        execute(&mut a, "cell.set", json!({"cell": "A1", "input": "Nora's"})).unwrap();
        let commands = vec![call("cell.set", json!({"cell": "A2", "input": "more"})), call("no.such.command", json!({}))];
        assert!(run(&mut a, DocActionKind::Batch { commands }).is_err());
        run(&mut a, DocActionKind::Undo).unwrap();
        assert_eq!(value(&mut a, "A1"), Json::Null);
    }

    /// A reading (`sheet.read`, `cell.get`) changes nothing: it leaves nothing for the pane's Undo to take back.
    #[test]
    fn reading_is_no_change_to_undo() {
        let mut a = app(Services::default());
        execute(&mut a, "cell.set", json!({"cell": "A1", "input": "Nora's"})).unwrap();
        execute(&mut a, "sheet.read", json!({})).unwrap();
        run(&mut a, DocActionKind::Batch { commands: vec![call("cell.get", json!({"cell": "A1"})), call("document.inspect", json!({}))] }).unwrap();
        run(&mut a, DocActionKind::TypeText { text: String::new(), paragraphs: false }).unwrap();
        assert_eq!(a.nora.changes.len(), 1, "only the change is remembered");
        run(&mut a, DocActionKind::Undo).unwrap();
        assert_eq!(value(&mut a, "A1"), Json::Null, "the pane's Undo takes back the change, not a reading");
    }

    /// An action that leaves the workbook's revision where it was answers `"changed": false`, so that Nora's
    /// session (craft-nora from 9d28340) keeps nothing of it for the pane's Undo either; a change says nothing
    /// of the kind. A batch's list of results is then put in an object, under `results`.
    #[test]
    fn a_reading_answers_that_it_changed_nothing_and_a_change_does_not() {
        let mut a = app(Services::default());
        set(&mut a, "A1", "mine");
        let reads = [
            execute(&mut a, "sheet.read", json!({})),
            execute(&mut a, "cell.get", json!({"cell": "A1"})),
            run(&mut a, DocActionKind::Batch { commands: vec![call("cell.get", json!({"cell": "A1"})), call("document.inspect", json!({}))] }),
            run(&mut a, DocActionKind::Observe),
            run(&mut a, DocActionKind::SelectText { text: "mine".into(), occurrence: 1 }),
            run(&mut a, DocActionKind::TypeText { text: String::new(), paragraphs: false }),
        ];
        for read in reads {
            let read = read.unwrap();
            assert_eq!(read.get("changed"), Some(&json!(false)), "{read}");
        }
        let batch = run(&mut a, DocActionKind::Batch { commands: vec![call("cell.get", json!({"cell": "A1"}))] }).unwrap();
        assert_eq!(batch["results"][0]["result"]["value"], json!("mine"), "the batch's results stay readable: {batch}");
        let changes = [
            execute(&mut a, "cell.set", json!({"cell": "A2", "input": "Nora's"})),
            run(&mut a, DocActionKind::Batch { commands: vec![call("cell.set", json!({"cell": "A3", "input": "b"}))] }),
            run(&mut a, DocActionKind::TypeText { text: "c".into(), paragraphs: false }),
        ];
        for change in changes {
            let change = change.unwrap();
            assert_eq!(change.get("changed"), None, "{change}");
        }
        assert_eq!(a.nora.changes.len(), 3, "the three changes, and none of the readings, are left for the pane's Undo");
    }

    /// The person is typing in a cell when Nora acts: what they typed is entered first, where they typed it. The cell
    /// editor enters its draft in the active sheet at the address it started from, so before, a sheet Nora moved to
    /// took the person's words, and so did a cell a row of hers had moved.
    #[test]
    fn the_persons_draft_is_entered_where_they_typed_it_before_nora_acts() {
        let mut a = app(Services::default());
        a.run("home.insertSheet", json!({"name": "Sheet 2"})).unwrap();
        a.run("sheet.activate", json!({"sheet": 0})).unwrap();
        a.run("selection.set", json!({"range": "B2"})).unwrap();
        a.begin_edit(Some("draft".into()), false);
        run(&mut a, DocActionKind::SelectText { text: "'Sheet 2'!A1".into(), occurrence: 1 }).unwrap();
        assert!(a.editor.is_none(), "the draft was entered first");
        // The person's Enter: nothing is left to enter.
        a.commit_edit(1, 0, false, false);
        a.run("sheet.activate", json!({"sheet": 0})).unwrap();
        assert_eq!(value(&mut a, "B2"), json!("draft"), "in the sheet and the cell the person typed in");
        a.run("sheet.activate", json!({"sheet": 1})).unwrap();
        assert_eq!(value(&mut a, "B2"), Json::Null);
    }

    #[test]
    fn a_row_nora_inserts_moves_the_persons_entry_with_it() {
        let mut a = app(Services::default());
        a.run("selection.set", json!({"range": "B2"})).unwrap();
        a.begin_edit(Some("draft".into()), false);
        execute(&mut a, "home.insertRows", json!({"rows": "1:1"})).unwrap();
        a.commit_edit(1, 0, false, false);
        assert_eq!(value(&mut a, "B3"), json!("draft"), "the entry moved down with its row");
        assert_eq!(value(&mut a, "B2"), Json::Null);
    }

    #[test]
    fn a_draft_that_cannot_be_entered_holds_nora_s_action() {
        let mut a = app(Services::default());
        a.run("selection.set", json!({"range": "B2"})).unwrap();
        a.begin_edit(Some("=1+*2".into()), false);
        let error = execute(&mut a, "cell.set", json!({"cell": "A1", "input": "Nora's"})).unwrap_err();
        assert!(error.contains("typing"), "{error}");
        assert_eq!(value(&mut a, "A1"), Json::Null, "Nora's action waits");
        assert!(a.editor.as_ref().is_some_and(|editor| editor.text == "=1+*2"), "the person's edit stays open as it was");
        assert!(a.message.is_none(), "and no message box comes up for it");
    }

    #[test]
    fn a_failing_batch_leaves_the_workbook_and_its_history_as_they_were() {
        let mut a = app(Services::default());
        set(&mut a, "A1", "before");
        set(&mut a, "A9", "undone");
        a.run("edit.undo", json!({})).unwrap();
        a.run("selection.set", json!({"range": "C3"})).unwrap();
        let (undo, redo) = history(&a);
        assert_eq!(redo.len(), 1);
        let remembered = a.nora.changes.len();
        let commands = vec![
            call("cell.set", json!({"cell": "A2", "input": "more"})),
            call("selection.set", json!({"range": "D4"})),
            call("no.such.command", json!({})),
        ];
        let error = run(&mut a, DocActionKind::Batch { commands }).unwrap_err();
        assert!(error.contains("no.such.command"), "{error}");
        assert_eq!(value(&mut a, "A2"), Json::Null, "the command that ran was taken back");
        assert_eq!(value(&mut a, "A1"), json!("before"));
        assert_eq!(history(&a), (undo, redo), "the history too: the person can still redo their step");
        assert_eq!(a.session.active().unwrap().selection.a1(), "C3", "and the selection");
        assert_eq!(a.nora.changes.len(), remembered, "nothing new is left to undo");
    }

    #[test]
    fn a_command_that_would_open_a_dialog_is_refused_and_changes_nothing() {
        let mut a = app(Services::default());
        let error = execute(&mut a, "insert.picture", json!({})).unwrap_err();
        assert!(error.contains("dialog"), "{error}");
        let commands = vec![call("cell.set", json!({"cell": "A1", "input": "x"})), call("insert.picture", json!({}))];
        assert!(run(&mut a, DocActionKind::Batch { commands }).is_err());
        assert_eq!(value(&mut a, "A1"), Json::Null);
        assert!(a.dialog.is_none(), "no dialog opened");
        assert!(execute(&mut a, "edit.beginEdit", json!({})).unwrap_err().contains("editing"));
        assert!(a.editor.is_none(), "and no cell editor");
    }

    #[test]
    fn no_command_of_nora_reaches_a_file_of_the_computer() {
        let mut a = app(Services::default());
        for (command, params) in [
            ("insert.picture", json!({"path": "/etc/hosts"})),
            ("data.fromTextCsv", json!({"path": "/etc/passwd"})),
            ("insert.picture", json!({"options": {"FileName": "x"}})),
            ("home.borders", json!({"list": [{"directory": "/"}]})),
        ] {
            let error = execute(&mut a, command, params).unwrap_err();
            assert!(error.contains("refused"), "{error}");
        }
        let batch = vec![call("cell.set", json!({"cell": "A1", "input": "x"})), call("insert.picture", json!({"path": "x.png"}))];
        assert!(run(&mut a, DocActionKind::Batch { commands: batch }).is_err());
        assert_eq!(value(&mut a, "A1"), Json::Null);
    }

    #[test]
    fn the_policy_keeps_the_interface_the_files_and_the_history_to_the_person() {
        let policy = policy();
        for allowed in [
            "cell.set",
            "range.setValues",
            "home.bold",
            "data.sort",
            "view.freezePanes",
            "view.zoom",
            "sheet.read",
            "insert.chart",
            "draw.stroke",
            "review.protectSheet",
            "review.newComment",
        ] {
            assert_eq!(policy.check(allowed), Decision::Allow, "{allowed}");
        }
        for denied in [
            "ui.nora",
            "ui.dialog",
            "view.darkMode",
            "view.theme",
            "view.interfaceScale",
            "view.animations",
            "view.collapseRibbon",
            "view.formulaBar",
            "app.language.set",
            "file.save",
            "file.open",
            "file.exportPdf",
            "automate.runScript",
            "edit.undo",
            "edit.redo",
            // Tools that take the person's next clicks: the pen and the eraser swallow every click on the grid.
            "draw.pen",
            "draw.eraser",
            "draw.lasso",
            "edit.formatPainter",
            // The person's own display.
            "view.ruler",
            "review.showComments",
            "review.showAllNotes",
            // The windows: a batch would go on in another workbook, and read it.
            "window.activate",
            "view.newWindow",
            "view.hideWindow",
            "view.unhideWindow",
            "view.switchWindows",
            "view.arrangeAll",
        ] {
            assert_eq!(policy.check(denied), Decision::Deny, "{denied}");
        }
        // Played directly, without the session's check, a denied command is refused all the same, and so is a family
        // the workbook has no commands in (one a later version adds, or none at all).
        let mut a = app(Services::default());
        for id in ["automate.runScript", "draw.pen", "window.activate", "ai.summarize", "nothing"] {
            let error = execute(&mut a, id, json!({"name": "x"})).unwrap_err();
            assert!(error.contains("policy"), "{id}: {error}");
        }
        assert!(a.session.draw_tool.is_empty(), "the pen stays down");
    }

    #[test]
    fn no_password_of_nora_s_locks_the_person_out() {
        let mut a = app(Services::default());
        for (command, params) in [
            ("review.protectSheet", json!({"password": "secret"})),
            ("review.protectWorkbook", json!({"password": "secret"})),
            ("review.allowEditRanges", json!({"action": "add", "title": "Mine", "range": "A1", "password": "x"})),
            ("review.protectSheet", json!({"options": {"newPassword": "x"}})),
        ] {
            let error = execute(&mut a, command, params).unwrap_err();
            assert!(error.contains("password"), "{command}: {error}");
        }
        let protected = |a: &SheetApp| a.session.active().and_then(|d| d.wb.active()).is_some_and(|sh| sh.is_protected());
        assert!(!protected(&a), "nothing was protected");
        // Without a password the person takes the protection off as easily: allowed.
        execute(&mut a, "review.protectSheet", json!({})).unwrap();
        assert!(protected(&a));
    }

    #[test]
    fn a_batch_has_a_length() {
        let mut a = app(Services::default());
        let commands = (0..=MAX_BATCH).map(|i| call("cell.set", json!({"cell": format!("A{}", i + 1), "input": "x"}))).collect();
        assert!(run(&mut a, DocActionKind::Batch { commands }).unwrap_err().contains("refused"));
        assert_eq!(value(&mut a, "A1"), Json::Null, "nothing ran");
    }

    #[test]
    fn looking_at_a_page_does_not_exist_in_neosheet() {
        let mut a = app(Services::default());
        let error = run(&mut a, DocActionKind::RenderPage { page: 1, scale: 1.0 }).unwrap_err();
        assert!(error.contains("does not exist in NeoSheet"), "{error}");
    }

    #[test]
    fn save_hands_a_workbook_without_a_file_to_the_host_and_never_opens_a_dialog() {
        // Without a file and without a place to put it, nothing is saved and no dialog opens.
        let mut a = app(Services::default());
        set(&mut a, "A1", "x");
        assert!(run(&mut a, DocActionKind::Save).unwrap_err().contains("Save As"));
        assert!(a.dialog.is_none());
        // The browser build (or the page that hosts it) takes the workbook.
        let downloads: Downloads = Arc::default();
        let got = downloads.clone();
        let mut a = app(Services {
            download: Some(Box::new(move |name: &str, bytes: &[u8]| {
                got.lock().unwrap_or_else(|e| e.into_inner()).push((name.to_string(), bytes.len()))
            })),
            ..Services::default()
        });
        set(&mut a, "A1", "x");
        let saved = run(&mut a, DocActionKind::Save).unwrap();
        assert_eq!(saved["handedOver"], true);
        let handed = downloads.lock().unwrap().clone();
        assert_eq!(handed.len(), 1, "{handed:?}");
        assert_eq!(handed[0].0, "Book1.xlsx");
        assert!(handed[0].1 > 0);
    }

    /// The sentences of the pane are words of the interface language, not the core's English.
    /// A reply that said a refused change was done is corrected in the interface language (`correction_words`).
    #[test]
    fn the_correction_of_a_refused_change_is_said_in_the_interface_language() {
        let french = correction_words(Language::Fr);
        assert!(french.nothing_done.starts_with("Rien n'a été fait dans le classeur"), "{}", french.nothing_done);
        assert!(french.partly_done.starts_with("Tout n'a pas été fait"), "{}", french.partly_done);
        for language in Language::ALL {
            let words = correction_words(language);
            assert!(!words.nothing_done.is_empty() && !words.partly_done.is_empty(), "{language:?}");
            assert!(language == Language::En || !words.nothing_done.starts_with("Nothing was done"), "{language:?}: {}", words.nothing_done);
        }
        let (services, _outbox, _inbox) = connected();
        let mut a = app(services);
        a.run("app.language.set", json!({"language": "de"})).unwrap();
        a.run("ui.nora", json!({"on": true})).unwrap();
        let mut h = harness(a);
        h.run_steps(2);
        assert_eq!(h.state().nora.panel.correction_words.partly_done, correction_words(Language::De).partly_done);
    }

    #[test]
    fn actions_are_described_with_the_pane_words() {
        assert_eq!(describe(Language::En, &DocActionKind::Save), "Save the workbook");
        assert_eq!(describe(Language::Fr, &DocActionKind::Save), "Enregistrer le classeur");
        assert_eq!(describer(Language::Fr)(&DocActionKind::Observe), "Lire le classeur");
        assert_eq!(describe(Language::En, &DocActionKind::Execute { command: "home.bold".into(), params: json!({}) }), "Run home.bold");
        let long = "é".repeat(60);
        let typed = describe(Language::En, &DocActionKind::TypeText { text: long, paragraphs: false });
        assert!(typed.starts_with("Type \"") && typed.ends_with("…\""), "{typed}");
        for language in Language::ALL {
            let labels = labels(language);
            for (word, english) in [(&labels.send, "Send"), (&labels.allow, "Allow"), (&labels.new_conversation, "New conversation")] {
                assert!(language == Language::En || word != english, "{language:?}: {word}");
            }
            assert!(!(labels.describe)(&DocActionKind::Undo).is_empty());
            assert!(language == Language::En || labels.stop != "Stop", "{language:?}: {}", labels.stop);
            assert!(language == Language::En || error_worder(language)(&NoraError::Timeout) != "Nora did not answer in time.", "{language:?}");
        }
    }

    /// While Nora works, the pane's Send button is a Stop button: a click ends her turn there and then, the pane says
    /// so, and the next message can go.
    #[test]
    fn stop_ends_nora_s_turn() {
        let (services, outbox, inbox) = connected();
        let mut a = app(services);
        a.run("ui.nora", json!({"on": true})).unwrap();
        let mut h = harness(a);
        ask(&mut h, &outbox, &inbox, "Sum column B");
        assert!(h.state().nora.session.is_busy(), "Nora is at work");
        h.get_by_label("Stop").click();
        h.run_steps(4);
        assert!(!h.state().nora.session.is_busy(), "her turn ended");
        assert!(h.query_by_label("Stopped.").is_some(), "the pane says Nora was stopped");
        assert!(h.query_by_label("Send").is_some(), "the next message can go");
    }

    /// A call that got no answer at all is said in the interface language, not with the browser's English reason.
    #[test]
    fn no_answer_from_nora_is_said_in_the_interface_language() {
        let (services, outbox, inbox) = connected();
        let mut a = app(services);
        a.run("app.language.set", json!({"language": "fr"})).unwrap();
        a.run("ui.nora", json!({"on": true})).unwrap();
        let mut h = harness(a);
        h.state_mut().nora.panel.set_draft("Bonjour");
        h.run_steps(2);
        h.get_by_label("Envoyer").click();
        h.run_steps(4);
        let sent = take(&outbox);
        let body = json!({"exception": "No answer from Nora: TypeError: Failed to fetch"}).to_string();
        inbox.lock().unwrap_or_else(|e| e.into_inner()).push(HttpResponse { id: sent[0].id, status: 0, body });
        h.run_steps(4);
        assert!(h.query_by_label("Nora n'a pas pu être jointe. Vérifiez la connexion et réessayez.").is_some());
        assert!(h.query_by_label("No answer from Nora: TypeError: Failed to fetch").is_none());
    }

    /// Rows Nora writes just below the block a chart plots stay out of the chart, as in Excel, where only a table's new
    /// rows join the charts on it: a total row typed under a block is not plotted. She brings them in with
    /// `chart.selectData` and the block's new address (the chart id comes back from `insert.chart`), which the pane
    /// plays: the recipe the server's instructions give her.
    #[test]
    fn rows_written_below_a_chart_s_block_join_it_through_select_data() {
        let mut a = app(Services::default());
        let block = json!([["Month", "Sales"], ["Jan", 10], ["Feb", 12], ["Mar", 9], ["Apr", 14]]);
        execute(&mut a, "range.setValues", json!({"range": "A1", "values": block})).unwrap();
        let chart = execute(&mut a, "insert.chart", json!({"range": "A1:B5", "type": "column"})).unwrap()["chart"].clone();
        assert!(chart.is_u64(), "{chart}");
        let series = |a: &SheetApp| {
            let charts = a.session.active().and_then(|d| d.wb.active()).map(|sh| sh.charts.clone()).unwrap_or_default();
            charts.iter().flat_map(|c| c.series.iter().map(|s| (s.values.clone(), s.categories.clone()))).collect::<Vec<_>>()
        };
        let before = series(&a);
        assert_eq!(before.len(), 1, "{before:?}");
        assert!(before[0].0.ends_with("$B$2:$B$5"), "{before:?}");

        execute(&mut a, "range.setValues", json!({"range": "A6", "values": [["May", 11], ["Jun", 16]]})).unwrap();
        assert_eq!(series(&a), before, "a plain range keeps its chart where it was");

        execute(&mut a, "chart.selectData", json!({"chart": chart, "range": "A1:B7"})).unwrap();
        let after = series(&a);
        assert_eq!(after.len(), 1, "{after:?}");
        assert!(after[0].0.ends_with("$B$2:$B$7"), "{after:?}");
        assert!(after[0].1.as_deref().is_some_and(|c| c.ends_with("$A$2:$A$7")), "{after:?}");
    }

    /// The page's workbook opened with its password is never saved back to the page in clear (hosted.rs): when Nora
    /// asks to save it, nothing is handed over, she is told why, and the person reads it in the message bar.
    #[test]
    fn nora_cannot_save_the_page_s_password_protected_workbook_in_clear() {
        let handed: Downloads = Arc::default();
        let sent = handed.clone();
        let mut a = app(Services {
            host_owns_documents: true,
            download: Some(Box::new(move |name: &str, bytes: &[u8]| {
                sent.lock().unwrap_or_else(|e| e.into_inner()).push((name.to_string(), bytes.len()))
            })),
            ..Services::default()
        });
        a.host_encrypted_original = a.session.active().map(|d| d.uid);
        set(&mut a, "A1", "7");
        let error = run(&mut a, DocActionKind::Save).unwrap_err();
        assert!(error.contains("cannot save it encrypted"), "{error}");
        assert!(handed.lock().unwrap_or_else(|e| e.into_inner()).is_empty(), "nothing went to the page");
        assert_eq!(a.notice.as_ref().map(|n| n.text.as_str()), Some(crate::hosted::SAVE_IN_CLEAR_REFUSED));
        assert!(a.session.active().is_some_and(|d| d.is_dirty()), "the change stays unsaved");
    }

    /// Nora's print opens the browser's print dialog on the workbook's PDF, through the host's `print`; a host without
    /// one is told so, and nothing is printed.
    #[test]
    fn nora_s_print_opens_the_print_dialog_on_the_workbook_s_pdf() {
        let printed: Arc<Mutex<Vec<usize>>> = Arc::default();
        let got = printed.clone();
        let mut a = app(Services {
            print: Some(Box::new(move |bytes: &[u8]| {
                assert!(bytes.starts_with(b"%PDF-"));
                got.lock().unwrap_or_else(|e| e.into_inner()).push(bytes.len())
            })),
            ..Services::default()
        });
        set(&mut a, "A1", "7");
        let result = run(&mut a, DocActionKind::Print).unwrap();
        assert_eq!(result["dialog"], true, "{result}");
        assert_eq!(printed.lock().unwrap_or_else(|e| e.into_inner()).len(), 1);
        let mut bare = app(Services::default());
        set(&mut bare, "A1", "7");
        assert!(run(&mut bare, DocActionKind::Print).unwrap_err().contains("not available"));
    }

    /// For an e-mail draft Nora is handed the workbook's PDF and the workbook in its format, as they are now: nothing is
    /// downloaded on the way, nothing is saved, and the page's password-protected workbook is never handed in clear.
    #[test]
    fn nora_is_handed_the_workbook_s_pdf_and_file_without_a_download_or_a_save() {
        let handed: Downloads = Arc::default();
        let sent = handed.clone();
        let services = |sent: Downloads| Services {
            host_owns_documents: true,
            download: Some(Box::new(move |name: &str, bytes: &[u8]| {
                sent.lock().unwrap_or_else(|e| e.into_inner()).push((name.to_string(), bytes.len()))
            })),
            ..Services::default()
        };
        let mut a = app(services(sent));
        set(&mut a, "A1", "7");
        let pdf = run(&mut a, DocActionKind::ExportPdf).unwrap();
        assert!(pdf["name"].as_str().is_some_and(|n| n.ends_with(".pdf")), "{pdf}");
        let bytes = gridcraft_engine::io::base64_decode(pdf["data"].as_str().unwrap_or_default()).unwrap_or_default();
        assert!(bytes.starts_with(b"%PDF-"));
        let file = run(&mut a, DocActionKind::ExportFile).unwrap();
        assert!(file["name"].as_str().is_some_and(|n| n.ends_with(".xlsx")), "{file}");
        let bytes = gridcraft_engine::io::base64_decode(file["data"].as_str().unwrap_or_default()).unwrap_or_default();
        assert!(bytes.starts_with(b"PK"), "a workbook package");
        assert_eq!(file["size"], bytes.len());
        assert!(handed.lock().unwrap_or_else(|e| e.into_inner()).is_empty(), "nothing went to the browser or the page");
        assert!(a.session.active().is_some_and(|d| d.is_dirty()), "nothing was saved");
        let mut protected = app(services(Arc::default()));
        protected.host_encrypted_original = protected.session.active().map(|d| d.uid);
        set(&mut protected, "A1", "7");
        for kind in [DocActionKind::ExportPdf, DocActionKind::ExportFile] {
            assert!(run(&mut protected, kind).unwrap_err().contains("never handed over in clear"));
        }
    }

    /// The page that hosts the editor names the file a workbook is: Nora is told its id with each message about that
    /// workbook and no other, and what does not read as an id is never passed on.
    #[test]
    fn the_context_carries_the_file_the_page_named() {
        let mut a = app(Services::default());
        let uid = a.session.active().unwrap().uid;
        assert_eq!(context_of(&a).document.id, None, "nothing named, nothing told");
        name_file(&mut a, uid, Some("3fcaa7893c"));
        assert_eq!(context_of(&a).document.id.as_deref(), Some("3fcaa7893c"));
        let value: Json = serde_json::from_str(&context_of(&a).to_json_string()).unwrap();
        assert_eq!(value["document"]["id"], "3fcaa7893c", "{value}");
        a.session.new_workbook();
        assert_eq!(context_of(&a).document.id, None, "another workbook is no file of the page's");
        a.session.set_active(0);
        assert_eq!(context_of(&a).document.id.as_deref(), Some("3fcaa7893c"));
        name_file(&mut a, uid, Some("../etc/passwd"));
        assert_eq!(context_of(&a).document.id, None, "what is no id is not passed on, and the last one is forgotten");
        name_file(&mut a, uid, Some("abc"));
        name_file(&mut a, uid, None);
        assert_eq!(context_of(&a).document.id, None, "a file the page names no more");
        for at in 0..(MAX_NAMED_FILES as u64 + 5) {
            name_file(&mut a, 1_000 + at, Some("x"));
        }
        assert_eq!(a.nora.files.len(), MAX_NAMED_FILES);
    }

    /// A change of Nora's that leaves a link to a file, or a formula that calls outside the workbook, is taken back
    /// whole, and her answer says why; a link to a web page stays.
    #[test]
    fn a_change_that_leads_outside_the_workbook_is_taken_back_and_said() {
        let mut a = app(Services::default());
        set(&mut a, "A1", "Prix");
        let workbook = a.session.active().unwrap().wb.clone();
        let kept = history(&a);
        let error = execute(&mut a, "cell.set", json!({"cell": "B2", "input": "=HYPERLINK(\"C:\\Tarifs\\x.pdf\",\"Tarifs\")"})).unwrap_err();
        assert!(error.starts_with("`cell.set` was undone"), "{error}");
        assert!(error.contains("a HYPERLINK formula to C:\\Tarifs\\x.pdf (Sheet1!B2)"), "{error}");
        assert!(Arc::ptr_eq(&a.session.active().unwrap().wb, &workbook), "the workbook is the one before");
        assert_eq!(history(&a), kept, "and so is its history");
        assert!(a.nora.changes.is_empty(), "nothing of Nora's to undo");

        let error = execute(&mut a, "insert.link", json!({"cell": "C3", "target": "\\\\srv\\share\\x.xlsx"})).unwrap_err();
        assert!(error.contains("a link to \\\\srv\\share\\x.xlsx (Sheet1!C3)"), "{error}");

        let typed = DocActionKind::TypeText { text: "Flux\t=WEBSERVICE(\"https://example.com/?\"&A1)".into(), paragraphs: false };
        let error = run(&mut a, typed).unwrap_err();
        assert!(error.starts_with("The typed text was undone"), "{error}");
        assert!(error.contains("WEBSERVICE, which calls outside the workbook"), "{error}");

        let batch = vec![
            call("cell.set", json!({"cell": "D1", "input": "1"})),
            call("cell.set", json!({"cell": "D2", "input": "=IMAGE(\"https://example.com/x.png\")"})),
        ];
        let error = run(&mut a, DocActionKind::Batch { commands: batch }).unwrap_err();
        assert!(error.starts_with("The batch was undone"), "{error}");
        assert!(Arc::ptr_eq(&a.session.active().unwrap().wb, &workbook), "the whole batch is taken back");

        execute(&mut a, "cell.set", json!({"cell": "B2", "input": "=HYPERLINK(\"https://example.com\",\"Site\")"})).unwrap();
        execute(&mut a, "insert.link", json!({"cell": "C3", "target": "mailto:info@example.com"})).unwrap();
        assert_eq!(a.nora.changes.len(), 2, "a web page and an address stay");
    }

    /// A replace composes a link of its own: the server could not see it in what Nora asked, the pane reads it.
    #[test]
    fn a_link_a_command_composes_is_read_once_written() {
        let mut a = app(Services::default());
        set(&mut a, "A1", "=HYPERLINK(\"https://example.com/tarifs.pdf\",\"Tarifs\")");
        let error = execute(&mut a, "edit.replace", json!({"what": "https://example.com/", "with": "C:\\", "all": true})).unwrap_err();
        assert!(error.contains("C:\\tarifs.pdf"), "{error}");
        // The person's own link to a file stays where it is when Nora works around it.
        set(&mut a, "A5", "=HYPERLINK(\"C:\\x.pdf\")");
        execute(&mut a, "cell.set", json!({"cell": "B1", "input": "5"})).unwrap();
    }
}
