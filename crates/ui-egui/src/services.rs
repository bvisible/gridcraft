use super::printing;

type PickSave = Box<dyn Fn(&str) -> Option<String>>;
type OpenUrl = Box<dyn Fn(&str)>;
type CopyHtml = Box<dyn FnMut(&str, &str) -> Result<(), String>>;
type Download = Box<dyn Fn(&str, &[u8])>;
type PrintPdf = Box<dyn Fn(&[u8])>;

/// Host services the platform shell provides (file pickers, opening URLs).
#[derive(Default)]
pub struct Services {
    /// Desktop: discover configured printers on a worker.
    pub printers: Option<printing::Discover>,
    /// Desktop: submit the PDF with the chosen settings on a worker.
    pub print_native: Option<printing::Submit>,
    pub pick_open: Option<Box<dyn Fn() -> Option<String>>>,
    pub pick_save: Option<PickSave>,
    /// Native picture picker (PNG/JPEG), separate from workbook opening.
    pub pick_picture: Option<Box<dyn Fn() -> Option<String>>>,
    /// Browser picture picker. Each dialog owns its inbox, so canceled picks cannot
    /// insert into a subsequent dialog or open the image as a workbook.
    pub pick_picture_async: Option<Box<dyn Fn(Inbox)>>,
    pub open_url: Option<OpenUrl>,
    /// Publish HTML and its plain-text alternative together. `Ok` means the host owns the
    /// write (including any asynchronous fallback); an immediate error uses egui's text path.
    pub copy_html: Option<CopyHtml>,
    /// Read the HTML flavour of the system clipboard when the user pastes text, so cells the
    /// source spreadsheet recorded as numbers stay numbers ("Rp10,000.00" → 10000). `None` from
    /// the host pastes the plain text alone.
    pub paste_html: Option<Box<dyn FnMut() -> Option<String>>>,
    /// Read the system clipboard's plain text for the ribbon and menu Paste commands (Cmd/Ctrl+V
    /// gets it from egui's `Event::Paste`). `None` from the host, or no service (web), pastes
    /// GridCraft's own clipboard.
    pub paste_text: Option<Box<dyn FnMut() -> Option<String>>>,
    /// Web: deliver a file to the user (name, bytes).
    pub download: Option<Download>,
    /// Web: start an asynchronous file pick; the bytes arrive later in `inbox`.
    pub open_async: Option<Box<dyn Fn()>>,
    /// Files delivered asynchronously (name, bytes), opened on the next frame.
    pub inbox: Option<Inbox>,
    /// Web: hand PDF bytes to the browser's print flow directly (no download,
    /// no intermediate file) — opens the system print dialog on that PDF.
    pub print: Option<PrintPdf>,
    // //// Neoffice — the page that frames the editor (the Drive of the Suite, Neodesk's window) opens the workbooks:
    // File › Home, New and Open are not offered and what would bring another workbook in, or close this one, is
    // refused. The page's own file still opens. Off, as upstream has it, for every other build.
    pub host_owns_documents: bool,
    // //// Neoffice — Get Data in an editor whose page opens the workbooks imports into the page's workbook (hosted.rs): the
    // browser's picker for a text, CSV or JSON file, whose bytes arrive in the inbox given, never in `inbox`.
    /// Web, when a page opens the workbooks: start an asynchronous pick of a file to import; the bytes arrive in the given inbox.
    pub pick_import_async: Option<Box<dyn Fn(Inbox)>>,
    // //// Neoffice — the connection of the Nora assistant pane, as in WordCraft. Nora (the agent) lives on the Neoffice
    // server, so the pane needs the host's session and credentials to reach her: the host makes the HTTP call and
    // delivers the answer through `nora_inbox`. Without one the pane says that Nora isn't available and offers no text box.
    /// Make one call for the Nora pane: a `POST` of a JSON body to a path of the Neoffice instance. The host adds the
    /// origin and the credentials of its session.
    pub nora_http: Option<Box<dyn Fn(craft_nora::HttpRequest)>>,
    /// The answers to [`Services::nora_http`], delivered asynchronously by the host.
    pub nora_inbox: Option<NoraInbox>,
    // //// Neoffice — the host opens a page of the person's instance for the pane (a card's Open, a link of a reply): the
    // path only (`/drive/f/…`), already checked by craft-nora (`instance_path`). Without it the pane shows a link as its words.
    /// Open a page of the person's instance, by its path, the host's own way. (`OpenUrl` is upstream's alias: clippy's
    /// `type_complexity`, which upstream turned on at v0.15.0, flags the spelled-out type.)
    pub nora_open: Option<OpenUrl>,
}

/// Shared queue of files read asynchronously (browser file picker, drag and drop).
pub type Inbox = std::sync::Arc<std::sync::Mutex<Vec<(String, Vec<u8>)>>>;

// //// Neoffice — answers to the Nora pane's calls, delivered asynchronously by the host like `Inbox`.
pub type NoraInbox = std::sync::Arc<std::sync::Mutex<Vec<craft_nora::HttpResponse>>>;

/// Which parts of the app's own window chrome are drawn. Everything is on by default; a host
/// that embeds the sheet in its own window (and owns files itself) can turn off the title bar
/// and the File tab, whose Backstage view then never opens. Not persisted: it describes the
/// host, not a user preference.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Chrome {
    /// The title bar: AutoSave, Home, Save, Undo/Redo, the file name, Comments, Share, Search and
    /// the theme switch.
    pub title_bar: bool,
    /// The ribbon's File tab and the Backstage view it opens (also reached by keyboard and
    /// commands such as `ui.backstage`).
    pub file_tab: bool,
}

impl Default for Chrome {
    fn default() -> Self {
        Chrome { title_bar: true, file_tab: true }
    }
}
