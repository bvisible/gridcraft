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
//! URL query flags: `?webgl` forces WebGL2; `?sample=sales|budget|grades` opens a sample.
// //// Neoffice — the host flags are documented here for the next reader; the contract is in host.rs.
//!
//! `?host=parent` (inside an `<iframe>`, with a parent page of the same origin) talks to that parent
//! instead: it sends the workbook to open and stores what GridCraft saves (`host`), and `&author=<name>`
//! names the user for notes and comments.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

// //// Neoffice — added module: the `postMessage` bridge to the page that frames the browser build (`?host=parent`, see host.rs).
// Its decisions are plain Rust, built for the tests too so that they run off wasm.
#[cfg(any(target_arch = "wasm32", test))]
mod host;
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
