// //// Neoffice — added file (no upstream equivalent): the connection of the Nora pane in the browser, WordCraft's.

//! The Nora pane's connection for the browser build: a `fetch` made with the page's own session, the
//! same as WordCraft's, so that every editor reaches Nora alike.
//!
//! Nora lives on the Neoffice server, so the editor needs the session of whoever hosts it. The page
//! turns the connection on in its URL with `?nora`: the calls then go to the page's own origin, as for an
//! editor served by the instance itself. What a call carries (the person's messages, what they selected,
//! the cells Nora reads) must never go where a link says, so `?nora=<origin>` is taken only when it names
//! the page's own origin, or, for the workshop's mock server, a loopback origin from a loopback page
//! (`127.0.0.1`, `localhost`, `[::1]`). Another origin in the link is ignored: the pane still connects, to
//! the page's own origin (the instance's Nora), and the console says why, as in NeoWrite and NeoSlides.
//! Any other origin only the hosting page itself can set, in `window.gridcraftNora`, read at every call:
//! `{ origin?: string, headers?: { [name]: value }, credentials?: "include" | "same-origin" | "omit" }`.
//! The page's session goes to its own origin only: a Frappe page's CSRF token (`window.csrf_token` or
//! `frappe.csrf_token`, sent as `X-Frappe-CSRF-Token` without being asked) and the headers of
//! `window.gridcraftNora`. Nothing secret travels in the URL.
//!
//! A call that gets no answer from the network comes back as a failure the pane can show.
//!
//! A host that makes the calls itself offers `window.gridcraftNoraCall(path, body)`, on the page or on
//! the page of the same origin that frames it, a promise of `{ status, body }` (`body` is the JSON of the
//! call): the pane is then on without `?nora`, and every call goes through it. Neodesk does so: it calls
//! with the person's credential, which neither its page nor the editor ever holds (the same as
//! WordCraft's `wordcraftNoraCall` and NeoMail's `mailcraftNoraCall`).
//!
//! What decides things (which origin, which headers) is plain Rust at the top of this file, tested off
//! wasm; the part that talks to the browser is the module `wire`, below it.

/// The hosts that are this computer: a page served from one may send Nora's calls to another (the
/// workshop's mock server on its own port).
const LOOPBACK: &[&str] = &["127.0.0.1", "localhost", "[::1]"];

/// Where the URL sends Nora's calls.
#[derive(Debug, PartialEq, Eq)]
pub struct Requested {
    /// The origin the calls go to: `""` for the page's own (the calls go to paths of it).
    pub origin: String,
    /// Why the origin the URL named was ignored, for the console: `None` when it was taken.
    pub warning: Option<String>,
}

/// The origin Nora is asked at, as the URL asks for her: `value` is `?nora`'s value as
/// `URLSearchParams` read it (`None` when the URL has no `?nora`) and `page_origin` the page's own
/// (`location.origin`). `""` stands for the page's own origin (the calls go to paths of it): a bare
/// `?nora` (or `=1`, `=true`) or one that names it. Another origin is taken only when it and the page
/// are both loopback. Any other is ignored, whatever the link says: Nora is then the page's own (the
/// instance's), as in NeoWrite and NeoSlides, and the warning says why. `None` when the URL has no `?nora`.
pub fn origin_from_url(value: Option<&str>, page_origin: &str) -> Option<Requested> {
    let value = value?.trim();
    let taken = |origin: String| Some(Requested { origin, warning: None });
    if matches!(value, "" | "1" | "true") {
        return taken(String::new());
    }
    match (parse_origin(value), parse_origin(page_origin)) {
        (Some(named), Some(page)) if named == page => taken(String::new()),
        (Some(named), Some(page)) if LOOPBACK.contains(&named.host.as_str()) && LOOPBACK.contains(&page.host.as_str()) => taken(named.to_string()),
        _ => Some(Requested {
            origin: String::new(),
            warning: Some(format!(
                "?nora={value} ignored: a URL may name the page's own origin only (or a loopback origin from a loopback page), so Nora is asked at the page's own origin; a hosting page sets another one in window.gridcraftNora"
            )),
        }),
    }
}

/// Whether a call to `target` (an origin, or `""` for the page's own) may carry the page's session: its
/// CSRF token and the headers it set. Only to the page's own origin.
pub fn session_headers_allowed(target: &str, page_origin: &str) -> bool {
    target.is_empty() || parse_origin(target).zip(parse_origin(page_origin)).is_some_and(|(target, page)| target == page)
}

/// An `http` or `https` origin, lower-cased, without the scheme's default port.
#[derive(Debug, PartialEq, Eq)]
struct Origin {
    scheme: String,
    host: String,
    port: Option<u16>,
}

impl std::fmt::Display for Origin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.port {
            Some(port) => write!(f, "{}://{}:{port}", self.scheme, self.host),
            None => write!(f, "{}://{}", self.scheme, self.host),
        }
    }
}

/// `text` as an origin (`scheme://host[:port]`, one `/` after it at most); `None` for anything else: a
/// path, a user, a query, another scheme, an opaque origin (`null`).
fn parse_origin(text: &str) -> Option<Origin> {
    let text = text.strip_suffix('/').unwrap_or(text);
    let (scheme, rest) = text.split_once("://")?;
    let scheme = scheme.to_ascii_lowercase();
    let default_port = match scheme.as_str() {
        "http" => 80,
        "https" => 443,
        _ => return None,
    };
    if rest.is_empty() || rest.contains(['/', '\\', '?', '#', '@']) {
        return None;
    }
    let (host, port) = match rest.strip_prefix('[') {
        // An IPv6 address, `[::1]:8080`.
        Some(inside) => {
            let (address, tail) = inside.split_once(']')?;
            if address.is_empty() || !address.chars().all(|c| c.is_ascii_hexdigit() || c == ':' || c == '.') {
                return None;
            }
            let port = if tail.is_empty() { "" } else { tail.strip_prefix(':')? };
            (format!("[{}]", address.to_ascii_lowercase()), port)
        }
        None => {
            let (host, port) = rest.split_once(':').unwrap_or((rest, ""));
            if host.is_empty() || !host.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '.') {
                return None;
            }
            (host.to_ascii_lowercase(), port)
        }
    };
    let port = if port.is_empty() { None } else { Some(port.parse::<u16>().ok()?) };
    Some(Origin { scheme, host, port: port.filter(|port| *port != default_port) })
}

#[cfg(target_arch = "wasm32")]
pub use wire::{connect, offered, requested};

/// The part that talks to the browser.
#[cfg(target_arch = "wasm32")]
mod wire {
    use std::sync::{Arc, Mutex};

    use gridcraft_ui_egui::{HttpRequest, HttpResponse, NoraInbox, Services};
    use wasm_bindgen::{JsCast as _, JsValue};
    use wasm_bindgen_futures::{JsFuture, spawn_local};

    use super::{origin_from_url, session_headers_allowed};

    /// The origin Nora is asked at when the page asked for her with `?nora` (`value`, as `URLSearchParams`
    /// read it): see [`origin_from_url`]. An origin the URL may not name is ignored, the console says why,
    /// and the pane is connected to the page's own origin.
    pub fn requested(value: Option<&str>) -> Option<String> {
        let page = web_sys::window().and_then(|window| window.location().origin().ok()).unwrap_or_default();
        let requested = origin_from_url(value, &page)?;
        if let Some(warning) = &requested.warning {
            log::warn!("{warning}");
        }
        Some(requested.origin)
    }

    /// Whether the page, or the page of the same origin that frames it, makes Nora's calls itself (`gridcraftNoraCall`):
    /// the pane is then on, as with `?nora`.
    pub fn offered() -> bool {
        host_call().is_some()
    }

    /// The host's own way to Nora, when it offers one: the window that offers it, and the function.
    fn host_call() -> Option<(JsValue, js_sys::Function)> {
        let window = web_sys::window()?;
        let own = JsValue::from(window.clone());
        let mut windows = vec![own.clone()];
        if let Ok(Some(parent)) = window.parent() {
            let parent = JsValue::from(parent);
            // A top page is its own parent; another origin's `document` can't be read.
            if parent != own && get(&parent, "document").is_some() {
                windows.push(parent);
            }
        }
        windows.into_iter().find_map(|w| get(&w, "gridcraftNoraCall").filter(JsValue::is_function).map(|f| (w, js_sys::Function::from(f))))
    }

    /// One call made by the host: its answer read as a `fetch`'s (status 0 when it gave none).
    async fn call_host(owner: &JsValue, function: &js_sys::Function, request: &HttpRequest) -> Result<(u16, String), String> {
        let promise = function.call2(owner, &JsValue::from_str(&request.path), &JsValue::from_str(&request.body.to_string())).map_err(reason)?;
        let answer = JsFuture::from(js_sys::Promise::resolve(&promise)).await.map_err(reason)?;
        let status =
            get(&answer, "status").and_then(|value| value.as_f64()).filter(|status| (0.0..1000.0).contains(status)).map_or(0, |status| status as u16);
        let body = get(&answer, "body").and_then(|value| value.as_string()).unwrap_or_default();
        Ok((status, body))
    }

    /// Gives the app the means to reach Nora: each call is a `fetch`, each answer is delivered to the pane
    /// and wakes the interface.
    pub fn connect(services: &mut Services, origin: String, ctx: egui::Context) {
        let inbox: NoraInbox = Arc::new(Mutex::new(Vec::new()));
        services.nora_inbox = Some(inbox.clone());
        services.nora_http = Some(Box::new(move |request: HttpRequest| {
            let (inbox, ctx, origin) = (inbox.clone(), ctx.clone(), origin.clone());
            spawn_local(async move {
                let response = call(&origin, &request).await;
                inbox.lock().unwrap_or_else(|e| e.into_inner()).push(response);
                ctx.request_repaint();
            });
        }));
    }

    /// One call, answered whatever happens: a network failure becomes a refusal with its reason, which the
    /// session shows like any other.
    async fn call(default_origin: &str, request: &HttpRequest) -> HttpResponse {
        match fetch(default_origin, request).await {
            Ok((status, body)) => HttpResponse { id: request.id, status, body },
            Err(error) => {
                let body = serde_json::json!({ "exception": format!("No answer from Nora: {error}") }).to_string();
                HttpResponse { id: request.id, status: 0, body }
            }
        }
    }

    /// What the host said in `window.gridcraftNora`.
    struct Host {
        origin: Option<String>,
        headers: Vec<(String, String)>,
        credentials: Option<web_sys::RequestCredentials>,
    }

    fn get(target: &JsValue, key: &str) -> Option<JsValue> {
        js_sys::Reflect::get(target, &JsValue::from_str(key)).ok().filter(|value| !value.is_undefined() && !value.is_null())
    }

    fn host(window: &web_sys::Window) -> Host {
        let mut host = Host { origin: None, headers: Vec::new(), credentials: None };
        if let Some(settings) = get(window, "gridcraftNora").filter(JsValue::is_object) {
            host.origin = get(&settings, "origin").and_then(|value| value.as_string());
            host.credentials = get(&settings, "credentials").and_then(|value| value.as_string()).and_then(|name| match name.as_str() {
                "include" => Some(web_sys::RequestCredentials::Include),
                "same-origin" => Some(web_sys::RequestCredentials::SameOrigin),
                "omit" => Some(web_sys::RequestCredentials::Omit),
                _ => None,
            });
            if let Some(headers) = get(&settings, "headers").filter(JsValue::is_object) {
                for entry in js_sys::Object::entries(&js_sys::Object::from(headers)).iter() {
                    let pair = js_sys::Array::from(&entry);
                    if let (Some(name), Some(value)) = (pair.get(0).as_string(), pair.get(1).as_string()) {
                        host.headers.push((name, value));
                    }
                }
            }
        }
        host
    }

    /// The CSRF token of a Frappe page, if the page has one.
    fn csrf_token(window: &web_sys::Window) -> Option<String> {
        get(window, "csrf_token")
            .and_then(|token| token.as_string())
            .or_else(|| get(window, "frappe").and_then(|frappe| get(&frappe, "csrf_token")).and_then(|token| token.as_string()))
            .filter(|token| !token.is_empty())
    }

    fn reason(error: JsValue) -> String {
        error
            .as_string()
            .or_else(|| js_sys::Reflect::get(&error, &JsValue::from_str("message")).ok().and_then(|message| message.as_string()))
            .unwrap_or_else(|| format!("{error:?}"))
    }

    async fn fetch(default_origin: &str, request: &HttpRequest) -> Result<(u16, String), String> {
        if let Some((owner, function)) = host_call() {
            return call_host(&owner, &function, request).await;
        }
        let window = web_sys::window().ok_or("no window")?;
        let host = host(&window);
        let page = window.location().origin().unwrap_or_default();
        let origin = host.origin.as_deref().unwrap_or(default_origin).trim_end_matches('/');
        let url = format!("{origin}{}", request.path);
        let headers = web_sys::Headers::new().map_err(reason)?;
        headers.set("Content-Type", "application/json").map_err(reason)?;
        headers.set("Accept", "application/json").map_err(reason)?;
        if session_headers_allowed(origin, &page) {
            if let Some(token) = csrf_token(&window) {
                headers.set("X-Frappe-CSRF-Token", &token).map_err(reason)?;
            }
            for (name, value) in &host.headers {
                headers.set(name, value).map_err(reason)?;
            }
        }
        let init = web_sys::RequestInit::new();
        init.set_method("POST");
        init.set_headers(&headers);
        init.set_body(&JsValue::from_str(&request.body.to_string()));
        if let Some(credentials) = host.credentials {
            init.set_credentials(credentials);
        }
        let answer = JsFuture::from(window.fetch_with_str_and_init(&url, &init)).await.map_err(reason)?;
        let response: web_sys::Response = answer.dyn_into().map_err(|_| "the answer is not a response".to_string())?;
        let text = JsFuture::from(response.text().map_err(reason)?).await.map_err(reason)?;
        Ok((response.status(), text.as_string().unwrap_or_default()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PAGE: &str = "https://osiris.example";

    /// What the URL's `?nora=<named>` gives on `page`, when it asks for Nora.
    fn requested(named: &str, page: &str) -> Requested {
        origin_from_url(Some(named), page).unwrap_or_else(|| panic!("{named:?} asks for Nora"))
    }

    /// A link can turn Nora on, never send the person's messages and cells to another origin: only the
    /// page's own, or, for the workshop's mock server, a loopback origin from a loopback page. Any other
    /// origin it names is ignored, as NeoWrite and NeoSlides do: Nora is the page's own (the instance's),
    /// and the console says why.
    #[test]
    fn a_link_never_sends_nora_elsewhere() {
        let taken = |origin: &str| Requested { origin: origin.into(), warning: None };
        assert_eq!(origin_from_url(None, PAGE), None, "not asked");
        for own in ["", "1", "true", "https://osiris.example", "https://OSIRIS.example/", "https://osiris.example:443"] {
            assert_eq!(requested(own, PAGE), taken(""), "{own:?}: the page's own origin");
        }
        // The workshop's mock: a loopback page may name a loopback origin.
        assert_eq!(requested("http://127.0.0.1:8773", "http://localhost:8080"), taken("http://127.0.0.1:8773"));
        assert_eq!(requested("http://[::1]:8773/", "http://127.0.0.1:8080"), taken("http://[::1]:8773"));
        assert_eq!(requested("http://LOCALHOST:8773", "http://[::1]:8080"), taken("http://localhost:8773"));
        for (elsewhere, page) in [
            ("https://evil.example", PAGE),
            ("http://osiris.example", PAGE),
            ("https://osiris.example:8443", PAGE),
            ("https://osiris.example.evil.example", PAGE),
            ("https://osiris.example@evil.example", PAGE),
            ("https://evil.example/https://osiris.example", PAGE),
            ("//evil.example", PAGE),
            ("javascript:alert(1)", PAGE),
            ("http://127.0.0.1:8773", PAGE),
            ("https://osiris.example", "null"),
            // Not from a loopback page to another host; and loopback is these three names only.
            ("https://osiris.example", "http://127.0.0.1:8080"),
            ("http://127.0.0.2:8773", "http://127.0.0.1:8080"),
        ] {
            let ignored = requested(elsewhere, page);
            assert_eq!(ignored.origin, "", "{elsewhere:?} from {page:?}: the page's own origin instead");
            let warning = ignored.warning.unwrap_or_default();
            assert!(warning.contains(&format!("?nora={elsewhere} ignored")), "{elsewhere:?}: the console says which, in {warning:?}");
            assert!(warning.contains("page's own origin") && warning.contains("window.gridcraftNora"), "and why, in {warning:?}");
        }
    }

    /// The page's CSRF token and the headers it set in `window.gridcraftNora` go to the page's own origin
    /// only, even when the page sends Nora's calls elsewhere.
    #[test]
    fn the_page_s_session_goes_to_its_own_origin_only() {
        assert!(session_headers_allowed("", PAGE), "a call to the page's own origin");
        assert!(session_headers_allowed("https://osiris.example", PAGE));
        assert!(!session_headers_allowed("https://nora.example", PAGE));
        assert!(!session_headers_allowed("http://127.0.0.1:8773", "http://127.0.0.1:8080"), "another port is another origin");
        assert!(!session_headers_allowed("https://osiris.example", "null"), "an opaque page has no origin to match");
    }
}
