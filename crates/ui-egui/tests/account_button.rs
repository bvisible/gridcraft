// //// Neoffice — added file (no upstream equivalent): the title bar's account button.

//! The account button at the right end of the title bar, the one WordCraft draws there: the initials of the
//! person in a round badge of the accent colour, and their name, nothing else, as the hover text. The name is
//! the one that signs notes and comments, which a hosting page passes as `?author=`. Without a name, or with
//! the engine's placeholder for a user nobody named, there is no button. Like WordCraft's, the button has no
//! accessible label: it is found here by its badge.

use egui::epaint::CircleShape;
use egui::{Color32, Rect, Shape, Vec2};
use egui_kittest::kittest::Queryable as _;
use gridcraft_engine::Session;
use gridcraft_ui_egui::SheetApp;
use gridcraft_ui_egui::theme::Tokens;

type Harness = egui_kittest::Harness<'static, SheetApp>;

const WIDTH: f32 = 1200.0;
/// `egui_kittest` lays the UI out inside an 8 point margin.
const HARNESS_MARGIN: f32 = 8.0;
/// The title bar and its tallest controls, as `ribbon.rs` sizes them.
const BAR_H: f32 = if cfg!(target_os = "macos") { 28.0 } else { 38.0 };
const CONTROL_H: f32 = if cfg!(target_os = "macos") { 22.0 } else { 26.0 };

fn harness(name: Option<&str>, dark: bool) -> Harness {
    let mut session = Session::new();
    session.new_workbook();
    if let Some(name) = name {
        session.prefs.user_name = name.into();
    }
    let mut app = SheetApp::new(session, Default::default());
    app.ui.system_theme = false;
    app.ui.dark = dark;
    let mut h = Harness::builder().with_size(egui::vec2(WIDTH, 800.0)).with_step_dt(1.0 / 60.0).build_ui_state(
        |ui, app: &mut SheetApp| {
            let ctx = ui.ctx().clone();
            app.logic(&ctx);
            app.ui(ui);
        },
        app,
    );
    h.run_steps(4);
    h
}

/// The badge: a circle in the accent colour in the title bar.
fn badge(h: &Harness, accent: Color32) -> Option<CircleShape> {
    h.output().shapes.iter().find_map(|s| match &s.shape {
        Shape::Circle(c) if c.fill == accent && c.center.y < HARNESS_MARGIN + BAR_H => Some(*c),
        _ => None,
    })
}

/// The square the badge fills (its circle keeps a point of margin).
fn square(badge: &CircleShape) -> Rect {
    Rect::from_center_size(badge.center, Vec2::splat(2.0 * (badge.radius + 1.0)))
}

/// The painted text `text`: its rectangle, font size and colour.
fn text(h: &Harness, text: &str) -> Option<(Rect, f32, Color32)> {
    h.output().shapes.iter().find_map(|s| match &s.shape {
        Shape::Text(t) if t.galley.job.text == text => {
            let format = &t.galley.job.sections.first()?.format;
            Some((t.galley.rect.translate(t.pos.to_vec2()), format.font_id.size, format.color))
        }
        _ => None,
    })
}

/// WCAG contrast ratio of two opaque colours.
fn contrast(a: Color32, b: Color32) -> f32 {
    fn luminance(c: Color32) -> f32 {
        let lin = |v: u8| {
            let v = f32::from(v) / 255.0;
            if v <= 0.04045 { v / 12.92 } else { ((v + 0.055) / 1.055).powf(2.4) }
        };
        0.2126 * lin(c.r()) + 0.7152 * lin(c.g()) + 0.0722 * lin(c.b())
    }
    let (la, lb) = (luminance(a), luminance(b));
    (la.max(lb) + 0.05) / (la.min(lb) + 0.05)
}

#[test]
fn the_title_bar_ends_with_the_initials_of_the_person_named() {
    for dark in [false, true] {
        let tokens = if dark { Tokens::dark() } else { Tokens::light() };
        let h = harness(Some("Jane Doe"), dark);
        // The badge, in the accent colour, fills a square of the bar's control height.
        let circle = badge(&h, tokens.accent).expect("the badge is painted");
        assert_eq!(circle.radius, CONTROL_H / 2.0 - 1.0, "dark={dark}");
        let button = square(&circle);
        // At the right end of the title bar, centred in it.
        assert_eq!(WIDTH - HARNESS_MARGIN - button.right(), 8.0, "dark={dark}: the last control of the bar");
        assert!((button.center().y - (HARNESS_MARGIN + BAR_H / 2.0)).abs() <= 1.0, "dark={dark}: centred in the bar: {button:?}");
        // The initials sit on it, small and legible.
        let (at, size, color) = text(&h, "JD").expect("the initials are painted");
        assert!(button.contains(at.center()), "dark={dark}: {at:?} in {button:?}");
        assert_eq!(size, 10.5, "dark={dark}");
        let ratio = contrast(color, tokens.accent);
        assert!(ratio >= 4.5, "dark={dark}: the initials read at {ratio:.2}:1");
    }
}

#[test]
fn hovering_the_button_shows_the_name_and_nothing_else() {
    let mut h = harness(Some("Jane Doe"), false);
    let circle = badge(&h, Tokens::light().accent).expect("the badge is painted");
    assert!(h.query_by_label_contains("Jane Doe").is_none(), "the name is not shown before the pointer comes");
    h.hover_at(circle.center);
    // Past egui's tooltip delay (half a second).
    h.run_steps(60);
    assert!(text(&h, "Jane Doe").is_some(), "the tooltip is the name itself");
    assert!(h.query_by_role_and_label(egui::accesskit::Role::Label, "Jane Doe").is_some(), "the tooltip is the name itself");
}

#[test]
fn no_button_without_a_known_name() {
    let accent = Tokens::light().accent;
    // No name (the engine's placeholder, "GridCraft User"), an empty one, a blank one.
    for name in [None, Some(""), Some("   ")] {
        let h = harness(name, false);
        assert!(badge(&h, accent).is_none(), "{name:?}: no badge");
        assert!(text(&h, "GU").is_none(), "{name:?}: no initials");
    }
    // The name is read each frame: once a name comes, the button does.
    let mut h = harness(None, false);
    h.state_mut().session.prefs.user_name = "Ada Lovelace".into();
    h.run_steps(2);
    assert!(badge(&h, accent).is_some());
    assert!(text(&h, "AL").is_some());
}
