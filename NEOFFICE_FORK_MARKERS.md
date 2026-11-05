# Neoffice fork markers

Map of **our** divergence from upstream (`storytold/gridcraft`) in this fork (`bvisible/gridcraft`, branch
`version-15`). Everything that can carry a comment carries a `//// Neoffice — …` marker in place:
`// //// Neoffice — …` in Rust (a bare `////` is an error for clippy), `# //// Neoffice — …` in TOML
and YAML, `<!-- //// Neoffice — … -->` in Markdown. This file holds the base of the divergence and
what cannot carry a comment.

At the next upstream merge, `grep -rn "////"` in the source plus this file give the complete
picture of what is ours and why.

- **Upstream**: `storytold/gridcraft`, branch `main`.
- **Base of the divergence**: `30b210b4ef03b30616923465efa70766cefba379`, the tag `v0.15.0`, merged on 2026-11-02
  (before: `ac8da7d`, the tag `v0.11.1`, merged on 2026-10-24; `2ebdf16`, the tag `v0.10.0`, on 2026-10-22; `version-15` was cut
  on `fc45b7f`, the tag `v0.6.1`, on 2026-10-13). The tag, not upstream's `main`, whose unpublished work (a Python mode,
  equations) is not ours yet. Move it at each upstream merge. The check works it out itself (the common ancestor with
  upstream's `main`), so this line is for the reader.

## What the fork adds

Four workflow files and the features below, all marked in place. The workflow files are new files that
carry their own `//// Neoffice — added file` header. `neoffice-nora-catalogue.yml` (by hand) exports the editor's command list and
the Nora pane's verdict on each (`allowed` in `nora_pane.rs`) for the catalogue Neoffice's server keeps for Nora.

- **Where our code sits since the v0.15.0 merge**, which split upstream's files into modules (`grep -rn "////"` finds each):
  `ui-egui/src/lib.rs` (module lines, `UiState::nora_pane`, `SheetApp` fields, `hidden_from_a_host`), `services.rs` (the
  `Services` fields of the host and of Nora, `NoraInbox`), `app.rs` (constructor, the Nora pane in the layout),
  `app/commands.rs` (`run`'s guard, `ui.nora`, the dialogs the host does not offer, `open_backstage`), `app/files.rs` (the VBA
  notice, the inbox's guard, AutoSave), `theme/font_setup.rs` (the browser build's fonts, `has_ui_bold_face`),
  `ribbon/title_bar.rs`, `ribbon/view.rs`, `dialogs/*.rs`, `backstage/options.rs`, `engine/src/cmd/edit/cell.rs`
  (typed Swiss numbers), `engine/src/cmd/extra/data.rs` and `extra.rs` (Get Data's `base64`). `selection_statistics` in
  `tabs.rs` is upstream's English form, kept for upstream's tests; the status bar draws `selection_statistic_values`.
  `alert` (`app/commands.rs`) and `submit_print` (`app/print.rs`) are `pub(crate)` for `hosted.rs` and `nora_pane.rs`.
- **The browser build's fonts** (`crates/ui-egui/build.rs`, `web_fonts`; `theme/font_setup.rs`): Carlito for the cells and
  Inter for the interface, embedded from `CRAFT_FONTS_DIR` (`neoffice-web.yml`), beside upstream's own embedding of the
  same checkout (BIZ UDPGothic, NanumGothic and the Noto symbol faces, in required mode). The workflow pins craft-fonts at
  `aced4137`, which has both sets.

- `.github/workflows/neoffice-web.yml`: builds the browser version on a GitHub runner, started by
  hand, with the packaging script upstream's own release job runs. Upstream's workflows are
  disabled on this fork, because they need upstream's signing secrets (each new one is disabled through the API right
  after the merge that brings it: `nix.yml` with v0.10.0, `sonarcloud.yml`, which hands the secrets to a mutable
  upstream ref, with v0.11.1).
- `.github/workflows/neoffice-markers.yml`: fails a push to `version-15` when a change to
  upstream's code since the common ancestor carries no marker.
- `.github/workflows/neoffice-ci.yml`: runs upstream's own quality gate, `cargo xtask ci`, on a
  GitHub runner (by hand, or on a push that touches code), because the maintainer's Mac has too
  little disk to run it for every change.
- **The host bridge of the browser build** (new file `apps/gridcraft-web/src/host.rs`; marked changes in
  `apps/gridcraft-web/src/web.rs`, `apps/gridcraft-web/src/main.rs` and the web-sys features of
  `apps/gridcraft-web/Cargo.toml`): the `postMessage` contract of a page that frames the editor
  (`?host=parent`, same origin), the one upstream's GridCraft (its `main`, commit `6e2caa9`) and
  PdfCraft already use, so one host page (the Drive of the Suite, NeoDesk's window) drives every Craft
  editor: `gridcraft:ready`, `gridcraft:open` (a workbook to edit), `gridcraft:save` (every file the
  editor would download), `gridcraft:dirty` and `gridcraft:failed`, plus `?author=` (the name that signs
  notes and comments). Ours on top of upstream's: `gridcraft:saved` (`ok: false` marks the workbook
  unsaved again and says why in the message bar; a parent that never answers leaves it saved, as
  upstream's does), the same-origin check when the bridge starts (upstream's starts for any frame, then
  posts to nobody when the parent is of another origin), and a file that could not be posted counts as a
  refused save (upstream only logs it). Without `?host=parent` the editor behaves as before. What counts
  as saved and what a refusal does is plain Rust, tested off wasm (`host.rs` is built for `cfg(test)` too).
  Upstream added its own `host.rs` in v0.7.0 (#237), a subset of ours (same messages and `?author=`). At the
  v0.10.0 merge ours replaced it whole (its header says so) and upstream's `query_param` left `web.rs` (a marked
  line: ours is in `host.rs`). Since v0.11 upstream keeps the browser build's preferences in `localStorage`
  (`apps/gridcraft-web/src/prefs.rs`): under a hosting page they are restored first, then the page's choices apply
  (marked lines in `web.rs`): the name stored on an earlier visit does not sign (perhaps someone else's on a shared
  browser) and `?author=` is never stored over it (upstream's `keep_user_name`), the page's file names are kept out of
  the stored Recent list, and the Nora pane is open exactly when the page offers Nora. At a later merge, read upstream's changes to its file
  (`git diff <old base> <new tag> -- apps/gridcraft-web/src/host.rs`) and carry them over by hand; in `web.rs`,
  `main.rs` and `Cargo.toml` keep our calls (`Bridge::requested`, `connect`, `ready`, `sync(app, ctx)`).
- **The French line of the bridge's message** ("Couldn't save {name}: {error}"): new file
  `crates/ui-egui/src/i18n/neoffice.rs`, which `Language::tr` falls back to after upstream's French
  catalog (two marked lines in `crates/ui-egui/src/i18n.rs`). Every other language has it too, in the same
  file, asked after its own catalog (a third marked line, added with the Nora pane below).
- **No more corrections of upstream's French catalog**: until the v0.10.0 merge, `neoffice.rs` held a table
  (`FRENCH_FIXES`, asked before the catalog) with the 172 values upstream gave in storytold/gridcraft `c59e942`, for
  the entries of v0.6.1's `fr.rs` that read as the French of another string. v0.10.0 has that commit: 169 values
  are the same, the other three (`Options|General`, `Options|Save`, `Sheet Options|View`) were ours with their key's
  context, which upstream writes without it, so nothing on screen changes. The table went at that merge. What stays
  is a marked hunk in `i18n.rs` (test `a_key_s_context_never_shows`): `Language::tr` shows a translated
  `context|label` key's label alone, as it already did when falling back to English.
- **Cmd/Ctrl+S while a cell is being edited** (marked in `crates/ui-egui/src/formula_bar.rs`; tests in the
  new file `crates/ui-egui/tests/save_while_editing.rs`): the grid's keyboard handler ignores every key
  while an edit is open, and only the desktop app's menu bar carried the shortcut then, so in the browser
  build Cmd+S did nothing while a cell was being typed (the title bar's Save button, which commits the
  cell first, worked). The cell editor now commits the draft and saves, as the text-box editor already
  did. Drop it when upstream fixes the same.

- **No Discord link in the About dialog** (`crates/ui-egui/src/dialogs.rs`, one marked line): the link to upstream's
  community is not shown, for everyone who uses this build. The native menu's entry (`apps/gridcraft/src/native_menu.rs`)
  belongs to the desktop application, which this fork does not build: it stays as upstream has it.

- **A hosting page opens the workbooks** (`Services::host_owns_documents`, set by `apps/gridcraft-web/src/host.rs`, so
  only when a page frames the editor with `?host=parent`): the editor does not bring another workbook in beside the
  page's, or close it. `file.new`, `file.open` and `file.close` are refused with a message (`run` in
  `crates/ui-egui/src/app/commands.rs`, the list is `hidden_from_a_host` in `lib.rs`), the File menu has no Home, New,
  Open or Close and a request for those pages shows Info (`backstage.rs`, `open_backstage`), the "More commands" menu has
  no New or Open (`ribbon/title_bar.rs`), and a file dropped on the editor is not taken (`apps/gridcraft-web/src/web.rs`);
  the file the page hands over still opens (the inbox lifts the guard for it). **The windows too, since v0.13** (one window
  per workbook): `view.newWindow`, `view.arrangeAll`, `view.switchWindows`, `view.hideWindow`, `view.unhideWindow` and every
  `window.*` command (`activate`, `close`, `list`) are refused the same way, because a second window, a hidden one or a closed
  last window leaves the page without its workbook in view (an editor with no workbook starts a blank one); the View tab
  draws no New Window, Arrange All or Switch Windows (`ribbon/view.rs`) and the command search does not list them.
  Upstream's macros cannot reach the workbooks either way: their policy denies `file.*`, `window.*` and the window commands
  in a macro step (`engine/src/macros/policy.rs`). New test file `crates/ui-egui/tests/host_documents.rs`; the French line of the message is in
  `i18n/neoffice.rs`. Every other build behaves as before.
  Two things still reach a file there, and stay on the page's workbook (new file `crates/ui-egui/src/hosted.rs`, marked lines
  in `lib.rs`: the module, `Services::pick_import_async`, the `host_protected_file` and `imports` fields of `SheetApp`, the
  guard in `run`, the inbox, `open_dialog` and `logic`; tests in `crates/ui-egui/tests/host_documents.rs`):
  - **the page's workbook has a password**: the inbox asks for it with the `openPassword` dialog, as for a file opened
    without a page, and the guard lets the dialog's `file.open` through for that file only, told from any other by a
    fingerprint of its bytes. The password goes to `file.open` and nowhere else: the bridge never sees it. GridCraft
    cannot write an encrypted workbook, so that workbook is **never handed back to the page** (the Drive would replace
    the encrypted original with a copy in clear): Save, Save As, the title bar's Save and Nora's save send nothing
    (`open_dialog`'s `saveAs`, the field `host_encrypted_original`; `save` in `nora_pane.rs`), a message bar says why
    (one string, translated in `i18n/neoffice.rs` by Haiku), and the workbook stays marked as changed. Exports (PDF,
    CSV, web page) still go to the page: they are other files and replace nothing.
  - **AutoSave** (a preference upstream keeps since v0.11, `UiState::autosave`) never saves under a hosting page (one
    marked condition in `app/files.rs`): its direct `file.save` would skip the guards of File › Save; the page's workbooks
    have no file path, so it never fired there anyway.
  - **Data › Get Data** (and `data.getData` or `data.fromTextCsv` without a file, which ask for one): the browser's picker
    for a text, CSV or JSON file (`apps/gridcraft-web/src/host.rs`, a picker of its own whose file never reaches the inbox
    of the page's files) and the file imported into the page's workbook, in a new sheet named after it, as Excel's Get
    Data does. Upstream's opened it as a second workbook beside the page's. For that, the engine's `from_csv`
    (`crates/engine/src/cmd/extra.rs`, marked) takes a file's bytes as `base64` and finds their separator as File › Open
    finds a CSV's (comma, semicolon or tab) unless `delimiter` names it; `text` and `path` keep upstream's comma.

- **A workbook without a styles part opens without a warning** (`crates/xlsx/src/read.rs`, one marked hunk; tests in the
  new file `crates/xlsx/src/tests/optional_parts.rs`, a marked `mod` line in its `tests.rs`): ECMA-376 makes the part
  optional and Excel opens such a file silently, but upstream warned "no styles part", which the editor shows in a message
  box at every open (files that a mail or a script writes often have none). A styles part the workbook names and lacks
  still warns. Drop it when upstream stops warning.

- **The account button of the title bar** (new file `crates/ui-egui/src/account.rs`, one marked line in `lib.rs` and
  one in `ribbon/title_bar.rs`; new test file `crates/ui-egui/tests/account_button.rs`): at the right end of the
  title bar, the button WordCraft has there, so the four editors show the person the same way: a round badge in the
  accent colour, a square of the bar's control height, with up to two initials (the first letters of the words of
  `Prefs::user_name`, the name a hosting page passes as `?author=`) and the name, nothing else, as the hover text.
  It shows only when a name is known: not while the name is blank or the engine's placeholder ("GridCraft User").
  GridCraft's theme has no colour for text on the accent: the initials are white on the light theme's green and
  the title bar's grey on the dark theme's light green (white would read at about 2:1 there). No new text.

- **The Nora assistant pane** (Neoffice's assistant, in a pane at the right of the window, the one WordCraft has, so
  the same Nora works in every editor), off until File › Options › Assistant or the `ui.nora` command turns it on; the
  browser build turns it on by itself when the page offers Nora. It is built on `craft-nora` (the conversation with
  Nora, no I/O) and `craft-nora-ui` (the egui panel), two crates of Neoffice's `craft-libs` fork that depend on egui
  only, at WordCraft's revision. New file `crates/ui-egui/src/nora_pane.rs` (the pane, the conversation, Nora's
  document actions and their tests); marked changes in `Cargo.toml` and `crates/ui-egui/Cargo.toml` (the two
  dependencies), `crates/ui-egui/src/lib.rs` (the `nora_pane` preference, the `ui.nora` command, `Services::nora_http`
  and `Services::nora_inbox` for the host's connection, the pane's state, the pane in the layout, outermost at the
  right), `crates/ui-egui/src/backstage.rs` (the checkbox in Options) and `crates/ui-egui/src/control.rs` (`ui.nora` in
  the command list). Without a host connection the pane says that Nora isn't available and offers no text box. With
  one, what Nora asks for in the workbook is played with the engine's commands, the ones the MCP tools run
  (`Session::execute`), once what the person is typing (a cell, a text box) is entered: the commands of the
  workbook's families only (an allow-list, so `file.`, `app.`, `window.`, `ui.`, `automate.`, `macro.` (v0.15's recorder,
  editor and debugger) and any family a later version adds are refused) and none of the person's inside them (the
  interface's settings, `view.statusBar` among them since v0.15, the pen, the eraser and the format painter, the windows,
  Undo and Redo). The Solver (`data.solver`, `data.solverReport`) is allowed: it works on the page's workbook only, with no
  file and a bounded number of trials; a request that would type a defined name into the cell being edited
  (`formulas.pasteName`) needs the person, nothing that reaches a file or carries a password, nothing that
  would open a dialog; a failed action leaves the workbook, its history and its revision as they were, a reading is no
  change (an action that leaves the revision where it was answers `"changed": false`, from which craft-nora 9d28340
  keeps nothing for the Undo either; a batch's results then come in an object, under `results`), and the pane's Undo
  takes back a whole change only while the workbook's revision is still the one the change
  left (the person's own Undo puts back the very workbook a step restores, so the steps alone can't tell). Her words
  are translated in `crates/ui-egui/src/i18n/neoffice.rs`, every language (French included, since the v0.10.0 merge
  asked like the others, and Serbian, which v0.7.0 added), which `Language::tr` asks after each catalog (one marked
  line in `crates/ui-egui/src/i18n.rs`). `neoffice.rs`'s `table` and the pane's `describer` and `error_worder` match
  every `Language`: a language upstream adds does not compile until it has its table (Haiku translates it; v0.11 added
  Greek, Turkish, Polish, Italian, Ukrainian, Vietnamese, Dutch and Romanian). Three of the pane's buttons are asked
  as `Nora|Allow`, `Nora|Undo` and `Nora|Stop`: upstream's catalogs translate the bare words for other places (Japanese
  `Allow` is data validation's "type of input value") and a catalog wins over our tables; the context never shows.
  The browser build connects the pane with WordCraft's transport (new file `apps/gridcraft-web/src/nora.rs`, its
  decisions plain Rust tested off wasm as `host.rs`'s, the browser part in its module `wire`; marked changes in
  `apps/gridcraft-web/src/web.rs`, `apps/gridcraft-web/src/main.rs` and the web-sys features of
  `apps/gridcraft-web/Cargo.toml`, and a re-export in `crates/ui-egui/src/lib.rs`): a page that asks for it with
  `?nora`, or that offers `window.gridcraftNoraCall(path, body)` (Neodesk, which calls with the person's credential),
  gets the pane open and connected; every other page behaves exactly as before. The calls go to the page's own
  origin: a URL's `?nora=<origin>` is taken only when it is that origin, or a loopback one from a loopback page (the
  workshop's mock server); another origin in the URL is ignored (the pane still connects, to the page's own origin,
  the instance's Nora, and the console says why); any other origin only the hosting page sets, in
  `window.gridcraftNora`, and the page's session (its CSRF token, the headers of `window.gridcraftNora`) goes to its
  own origin only. The same rule holds for every editor.
- **The Nora pane redone, 2026-11-05** (`craft-nora-ui` at `7bc770d`: the Neoffice design system's look, Nora's sphere, what she is
  doing in words, and a microphone). The panel now draws its own canvas, head and close button, so `nora_pane::pane`
  (`crates/ui-egui/src/nora_pane.rs`, our file) gives it a frame with no fill and no margin, only a line, drops the old title row
  and reads the panel's close request; the editor's colours are no longer passed (`style` is gone, and with it the only reader of
  `theme::has_ui_bold_face`, `crates/ui-egui/src/theme/font_setup.rs`, kept for now). Dictation is offered only where the host says it
  carries a recording to the server: `Services::nora_dictation` (`crates/ui-egui/src/services.rs`), set from
  `window.gridcraftNoraDictation` on the page or the page that frames the editor (`apps/gridcraft-web/src/nora.rs`, `host_flag`); the
  session is told the interface language for it. While the window is hidden eframe runs no frame, only `logic`:
  `SheetApp::logic` (`crates/ui-egui/src/app.rs`) calls `nora_pane::pump_hidden`, so that Nora's queue is read and her actions played
  (`drive` reads the page's clock). New strings in every table of `crates/ui-egui/src/i18n/neoffice.rs` (French from Neoffice,
  the others by Haiku), among them `Nora|Cancel` for the microphone's Cancel: upstream's catalogs have no entry for the bare
  word in French and most other languages.
- **The browser build's URL flags read as URL parameters** (new file `apps/gridcraft-web/src/flags.rs`, marked changes
  in `apps/gridcraft-web/src/web.rs`): `?webgl`, `?sample`, `?nora`, `?locale` and (v0.15.0) `?chrome` are read by the
  browser's `URLSearchParams`, as the host bridge reads its own; upstream's `?chrome=` called its `query_param`, which is not
  in this web build. Upstream looks for the words anywhere in the query, so `?nora=https://sample.example`
  opened the sales sample. Drop it when upstream reads its flags as parameters.

- **Our product's name in our builds** (new file `crates/ui-egui/src/brand.rs`): `brand::name()` is
  `NEOFFICE_PRODUCT_NAME` when the build sets it (only the build step of `neoffice-web.yml`: `NeoGrille`), upstream's
  name otherwise, so upstream's builds and every test run are unchanged. `Language::tr` passes its result through
  `brand::rebrand`, which swaps the whole word `GridCraft` in every language (the Help tab's About button, its
  tooltips) without touching the tables; the message box shows its default title `GridCraft` through
  `brand::alert_title` (`widgets.rs`, one line for the fourteen alerts); the About and Start dialogs, File › Account, the Agent Control
  and Macros texts, the VBA notice and the page that failed to start call `brand::name()` (marked lines in `lib.rs`,
  `i18n.rs`, `widgets.rs`, `dialogs.rs`, `backstage.rs` and `apps/gridcraft-web/src/web.rs`). The page title and the loading text are
  ours in `apps/gridcraft-web/index.html`. Stored or compared names (storage folders, `GridCraft User`, `gridcraft:*`
  messages, `ai.storyteller.*`) and the native app (window title, macOS menu) keep upstream's.

- **The thousands separator follows the editor's locale** (new file `crates/ui-egui/src/number_locale.rs`; marked lines in
  `crates/numfmt/src/lib.rs`, `crates/numfmt/src/render.rs`, `crates/numfmt/src/tests.rs`, `crates/ui-egui/src/lib.rs`,
  `apps/gridcraft-web/src/flags.rs`, `apps/gridcraft-web/src/web.rs` and `apps/gridcraft/src/main.rs`): `#,##0.00` asks for
  the locale's separator, as Excel takes it from the regional settings, and a Swiss locale (`fr-CH`, `de-CH`, `it-CH`,
  `rm-CH`, `de-LI`; the hosting page's `?locale=`, else the browser's or the system's) shows `CHF 5'478.45`; every other
  keeps the comma, the decimal point stays a point. Only what is shown changes (`numfmt::format_value`, through
  `set_display_grouping`): `TEXT()` and the saved workbook are the same for everyone. **What is typed back that way is
  a number** (new file `crates/engine/src/locale_input.rs`, a marked `mod` line in `crates/engine/src/lib.rs`, marked
  lines in `input_to_cell` of `crates/engine/src/cmd/edit.rs` and in `crates/engine/src/display.rs`; test file
  `crates/engine/tests/swiss_input.rs`): in a Swiss editor `5'478.45`, `-1'000` or `1'234'567.5` (apostrophes, or the
  typographic U+2019, grouping the whole part in threes) is read as the same entry with commas, so it gets upstream's
  automatic `#,##0` or `#,##0.00`; `12'3` and a leading apostrophe stay text, and a text cell that would read back as a
  number starts editing with the `'` that keeps it text. Every other locale reads an apostrophe as text, and nothing
  computed (`VALUE()`, arithmetic on text) depends on the locale.

- **The status bar in the editor's language** (marked lines in `crates/ui-egui/src/tabs.rs`, `status_bar` and
  `selection_statistic_values`; the strings in
  `crates/ui-egui/src/i18n/neoffice.rs`; test file `crates/ui-egui/tests/status_bar_language.rs`): upstream writes Sum, Count and
  Average, the macro and circular-reference notices and the zoom buttons' tooltips in English. They are asked from `Language::tr`,
  with Neoffice's translations in French, German, Italian and Spanish (Haiku; every other language shows upstream's English, and a
  language's own catalog still wins), and the view buttons' tooltips use the ribbon's labels, which the catalogs translate.
  Sum and Average are shown in the number format of the selection's first numeric cell, as Excel shows them (upstream:
  the active cell's, so CHF amounts selected from a General cell or a label read `927.4083333333`); a General one keeps
  upstream's display (`first_numeric` in `tabs.rs`, marked). Since v0.15 the bar shows up to six statistics, chosen in its
  context menu (`view.statusBar`); the three that upstream added (Numerical Count, Minimum, Maximum) are in English
  in every language, and so are the menu's labels in the languages whose catalog lacks them.

## Files that cannot carry a comment

| File | Why it is ours |
|---|---|
| `Cargo.lock` | Lists `craft-nora` and `craft-nora-ui` (git, from Neoffice's `craft-libs` fork) for the Nora pane. On a merge conflict, take upstream's file and run `cargo metadata`: cargo writes our two entries back. |
