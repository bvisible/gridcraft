// //// Neoffice — added file (no upstream equivalent): where the page command API (`window.gridcraft`, `api.rs`) is not put on
// the page. Plain Rust, built for the tests too, as `host.rs`'s decisions are.

//! Whether the page command API may be installed. `window.gridcraft` runs any control method and command of the editor for any
//! script of the page, or of a same-origin page that frames it: `file.open`, `view.newWindow`, `window.close`, a macro. The
//! guards that keep a hosting page's workbook the only one sit in the commands' dispatcher, but a control channel with no
//! person in front of it is a door that this build does not open for a hosted editor (the Drive of the Suite, NeoDesk's
//! window): the page already has `postMessage`, which our bridge (`host.rs`) answers with its refusals.

/// Whether `window.gridcraft` is installed. Not when our bridge is up (`?host=parent`, framed by a page of the same origin), nor
/// when the canvas carries `data-host`, which is how upstream's own host mode opts a page in: that page hosts the editor too, even
/// though this build does not speak its transport (it behaves as the plain app with downloads).
pub fn offered(bridge_is_up: bool, canvas_declares_a_host: bool) -> bool {
    !bridge_is_up && !canvas_declares_a_host
}

#[cfg(test)]
mod tests {
    use super::offered;

    #[test]
    fn no_page_command_api_where_a_page_hosts_the_editor() {
        assert!(!offered(true, false), "our bridge is up (`?host=parent`)");
        assert!(!offered(false, true), "upstream's `data-host` marks a hosting page");
        assert!(!offered(true, true));
    }

    #[test]
    fn the_plain_browser_app_keeps_upstreams_page_command_api() {
        assert!(offered(false, false));
    }
}
