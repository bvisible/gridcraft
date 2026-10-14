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

Three workflow files and three features, all marked in place. The workflow files are new files that
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
  catalog (two marked lines in `crates/ui-egui/src/i18n.rs`). Other languages show it in English.
- **Corrections of upstream's French catalog** (same file, `FRENCH_FIXES`, asked BEFORE the catalog): in
  v0.6.1 about 50 entries of `i18n/fr.rs` read as the French of another English string ("File" is "Nom du
  fichier", "Hide Detail" is "Insérer les cellules copiées"). Upstream fixed them in storytold/gridcraft
  `c59e942` (12.10.2026); the table holds its 172 changed values (wrappings and wordings too), so
  `fr.rs` stays untouched and a merge never conflicts with it. **Drop the table at the merge that brings
  that commit** (the values are the same), keeping the hook line of `i18n.rs` for the strings we add.
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

## Files that cannot carry a comment

None so far.
