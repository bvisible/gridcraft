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
- **Cmd/Ctrl+S while a cell is being edited** (marked in `crates/ui-egui/src/formula_bar.rs`; tests in the
  new file `crates/ui-egui/tests/save_while_editing.rs`): the grid's keyboard handler ignores every key
  while an edit is open, and only the desktop app's menu bar carried the shortcut then, so in the browser
  build Cmd+S did nothing while a cell was being typed (the title bar's Save button, which commits the
  cell first, worked). The cell editor now commits the draft and saves, as the text-box editor already
  did. Drop it when upstream fixes the same.

## Files that cannot carry a comment

None so far.
