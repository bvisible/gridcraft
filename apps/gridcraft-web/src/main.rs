//! GridCraft in the browser.
//!
//! Runs the same [`gridcraft_ui_egui::SheetApp`] as the desktop app through eframe's web runner
//! (wgpu: WebGPU where available, WebGL2 otherwise). Build with `trunk build --release` from
//! this directory (output in `dist/web`).
//!
//! Differences from the desktop app: no TCP control channel; Open uses the browser file picker
//! (bytes arrive asynchronously through `Services::inbox`); Save downloads the file; dropped
//! files are read asynchronously.
//!
//! URL query flags: `?webgl` forces WebGL2; `?sample=sales|budget|grades` opens a sample;
//! `?host=parent` (inside an `<iframe>`) talks to a same-origin parent page instead: it sends the
//! workbook to open and stores what GridCraft saves (`host`), and `&author=` names the user for
//! notes and comments. (Upstream's second host mode, `data-host="page"` on the canvas with DOM
//! `CustomEvent`s, is not spoken here: this build's bridge is `host`'s.) `window.gridcraft`
//! (`craft-web-api`) is the page command API: `request`, `execute`, `commands`, `docInspect`,
//! `uiInspect` and `screenshot` over the control methods (`api`).
// //// Neoffice — upstream puts that API on every page; this build does not put it where a page hosts the editor (api_policy.rs).
//!
//! `?chrome=none|notitlebar|nofiletab` leaves the title bar and/or the File tab undrawn for a host
//! that embeds the sheet in its own window (see `Chrome`). Preferences (theme, interface size,
//! editing options…) are kept in `localStorage` (`prefs`).
// //// Neoffice — the Nora pane's flags, documented for the next reader; the connection is in nora.rs.
//!
//! `?nora` opens the Nora pane, connected with the page's own session at the page's own origin
//! (`?nora=<origin>` names that origin, or a loopback one from a loopback page; any other origin only the
//! page itself sets, in `window.gridcraftNora`); a page that offers `window.gridcraftNoraCall` makes
//! Nora's calls itself and gets the pane the same way. The flags are read as URL parameters (flags.rs).
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

#[cfg(any(target_arch = "wasm32", test))]
mod api;
// //// Neoffice — added module: where the page command API is not installed (api_policy.rs), plain Rust built for the tests too.
#[cfg(any(target_arch = "wasm32", test))]
mod api_policy;
// //// Neoffice — the `postMessage` bridge to the page that frames the browser build (`?host=parent`): our host.rs, which
// replaces upstream's (same contract, plus `gridcraft:saved`). Its decisions are plain Rust, built for the tests too so
// that they run off wasm; upstream builds its bridge for wasm only. Since v0.19.0 upstream's bridge is `craft-web-host`'s
// (and its `hostmode.rs`, not built here: the rule it holds, the name of a nameless open, is in our `wire::listen`).
#[cfg(any(target_arch = "wasm32", test))]
mod host;
// //// Neoffice — added module: the page's URL flags read as parameters (flags.rs). Its decisions are plain Rust, built for the
// tests too, as host.rs's.
#[cfg(any(target_arch = "wasm32", test))]
mod flags;
// //// Neoffice — added module: the Nora pane's connection (nora.rs), plain Rust built for the tests too.
#[cfg(any(target_arch = "wasm32", test))]
mod nora;
#[cfg(any(target_arch = "wasm32", test))]
mod prefs;
#[cfg(target_arch = "wasm32")]
mod web;

#[cfg(target_arch = "wasm32")]
fn main() {
    web::start();
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    eprintln!("gridcraft-web only runs in the browser: build it with `trunk build --release` in apps/gridcraft-web");
}
