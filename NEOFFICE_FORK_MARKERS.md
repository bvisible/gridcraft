# Neoffice fork markers

Map of **our** divergence from upstream (`storytold/gridcraft`) in this fork (`bvisible/gridcraft`, branch
`version-15`). Everything that can carry a comment carries a `//// Neoffice — …` marker in place:
`// //// Neoffice — …` in Rust (a bare `////` is an error for clippy), `# //// Neoffice — …` in TOML
and YAML, `<!-- //// Neoffice — … -->` in Markdown. This file holds the base of the divergence and
what cannot carry a comment.

At the next upstream merge, `grep -rn "////"` in the source plus this file give the complete
picture of what is ours and why.

- **Upstream**: `storytold/gridcraft`, branch `main`.
- **Base of the divergence**: `fc45b7f6d8ab79ef20f74e3c8347a2fd731e6af5`, the tag `v0.6.1`, where `version-15` was cut on 2026-10-13.
  Move it at each upstream merge. The check works it out itself (the common ancestor with
  upstream's `main`), so this line is for the reader.

## What the fork adds

Three workflow files and the features below, all marked in place. The workflow files are new files that
carry their own `//// Neoffice — added file` header.

- `.github/workflows/neoffice-web.yml`: builds the browser version on a GitHub runner, started by
  hand, with the packaging script upstream's own release job runs. Upstream's workflows are
  disabled on this fork, because they need upstream's signing secrets.
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
  At the next upstream merge `host.rs` is added on both sides: take upstream's and put back what is
  marked `//// Neoffice`; in `web.rs`, `main.rs` and `Cargo.toml` take upstream's hunks, then adapt the
  calls (`Bridge::requested`, `connect`, `ready`, `sync(app, ctx)`).
- **The French line of the bridge's message** ("Couldn't save {name}: {error}"): new file
  `crates/ui-egui/src/i18n/neoffice.rs`, which `Language::tr` falls back to after upstream's French
  catalog (two marked lines in `crates/ui-egui/src/i18n.rs`). Every other language has it too, in the same
  file, asked after its own catalog (a third marked line, added with the Nora pane below).
- **Corrections of upstream's French catalog** (same file, `FRENCH_FIXES`, asked BEFORE the catalog): in
  v0.6.1 about 50 entries of `i18n/fr.rs` read as the French of another English string ("File" is "Nom du
  fichier", "Hide Detail" is "Insérer les cellules copiées"). Upstream fixed them in storytold/gridcraft
  `c59e942` (12.10.2026); the table holds its 172 changed values (wrappings and wordings too), so
  `fr.rs` stays untouched and a merge never conflicts with it. **Drop the table at the merge that brings
  that commit** (the values are the same), keeping the hook line of `i18n.rs` for the strings we add.
  Three of its values keep their key's context (`"Options|General"` is "Options|Général", `"Options|Save"`,
  `"Sheet Options|View"`), which File › Options and the Page Layout tab showed as is: `Language::tr` shows a
  translated `context|label` key's label alone, as it already did when falling back to English (a marked hunk in
  `i18n.rs`, test `a_key_s_context_never_shows`). Keep it at that merge: upstream's `fr.rs` then has the same values.
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
  `crates/ui-egui/src/lib.rs`), the File menu has no Home, New, Open or Close and a request for those pages shows Info
  (`backstage.rs`, `open_backstage`), the "More commands" menu has no New or Open (`ribbon.rs`), and a file dropped on
  the editor is not taken (`apps/gridcraft-web/src/web.rs`); the file the page hands over still opens (the inbox lifts
  the guard for it). New test file `crates/ui-egui/tests/host_documents.rs`; the French line of the message is in
  `i18n/neoffice.rs`. Every other build behaves as before.

- **The account button of the title bar** (new file `crates/ui-egui/src/account.rs`, one marked line in `lib.rs` and
  one in `ribbon.rs`'s `title_bar`; new test file `crates/ui-egui/tests/account_button.rs`): at the right end of the
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
  workbook's families only (an allow-list, so `file.`, `app.`, `window.`, `ui.`, `automate.` and any family a later
  version adds are refused) and none of the person's inside them (the interface's settings, the pen, the eraser and
  the format painter, the windows, Undo and Redo), nothing that reaches a file or carries a password, nothing that
  would open a dialog; a failed action leaves the workbook, its history and its revision as they were, a reading is no
  change (an action that leaves the revision where it was answers `"changed": false`, from which craft-nora 9d28340
  keeps nothing for the Undo either; a batch's results then come in an object, under `results`), and the pane's Undo
  takes back a whole change only while the workbook's revision is still the one the change
  left (the person's own Undo puts back the very workbook a step restores, so the steps alone can't tell). Her words
  are translated in `crates/ui-egui/src/i18n/neoffice.rs`, every language, which `Language::tr` asks after each catalog
  (one marked line in `crates/ui-egui/src/i18n.rs`).
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
- **The browser build's URL flags read as URL parameters** (new file `apps/gridcraft-web/src/flags.rs`, marked changes
  in `apps/gridcraft-web/src/web.rs`): `?webgl`, `?sample` and `?nora` are read by the browser's `URLSearchParams`, as
  the host bridge reads its own. Upstream looks for the words anywhere in the query, so `?nora=https://sample.example`
  opened the sales sample. Drop it when upstream reads its flags as parameters.

- **Our product's name in our builds** (new file `crates/ui-egui/src/brand.rs`): `brand::name()` is
  `NEOFFICE_PRODUCT_NAME` when the build sets it (only the build step of `neoffice-web.yml`: `NeoGrille`), upstream's
  name otherwise, so upstream's builds and every test run are unchanged. `Language::tr` passes its result through
  `brand::rebrand`, which swaps the whole word `GridCraft` in every language (the Help tab's About button, its
  tooltips) without touching the tables; the message box shows its default title `GridCraft` through
  `brand::alert_title` (`widgets.rs`, one line for the fourteen alerts); the About and Start dialogs, the Agent Control
  and Macros texts, the VBA notice and the page that failed to start call `brand::name()` (marked lines in `lib.rs`,
  `i18n.rs`, `widgets.rs`, `dialogs.rs` and `apps/gridcraft-web/src/web.rs`). The page title and the loading text are
  ours in `apps/gridcraft-web/index.html`. Stored or compared names (storage folders, `GridCraft User`, `gridcraft:*`
  messages, `ai.storyteller.*`) and the native app (window title, macOS menu) keep upstream's.

## Files that cannot carry a comment

| File | Why it is ours |
|---|---|
| `Cargo.lock` | Lists `craft-nora` and `craft-nora-ui` (git, from Neoffice's `craft-libs` fork) for the Nora pane. On a merge conflict, take upstream's file and run `cargo metadata`: cargo writes our two entries back. |
