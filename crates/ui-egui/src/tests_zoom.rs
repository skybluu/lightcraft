//! 100 % means the photo's own pixels as it is shown: after the crop and the user's rotation.

use std::time::Duration;

use serde_json::json;

use crate::headless::Headless;
use crate::{LightcraftApp, Services};

const T: Duration = Duration::from_secs(20);
const SETTLE: Duration = Duration::from_secs(120);

/// The demo library in a 1400×900 window at 1 px per point, the first photo open in Detail.
fn detail() -> Headless {
    let services = Services { png: None, ..Default::default() };
    let app = LightcraftApp::new(lightcraft_engine::Session::with_demo(), services);
    let mut h = Headless::new(app, [1400.0, 900.0], 1.0);
    let r = h.request("ui.set", json!({"view": "detail", "right": "none", "filmstrip": false}), T);
    assert_eq!(r["ok"], true, "{r}");
    h.settle(SETTLE);
    h
}

fn edit(h: &mut Headless, f: impl FnOnce(&mut lightcraft_develop::DevelopSettings)) {
    let id = h.app.session.active().expect("a photo is open");
    let mut s = (*h.app.session.develop_of(id).expect("settings")).clone();
    f(&mut s);
    h.app.session.set_develop(id, s, "Test").expect("edit");
}

fn at_100(h: &mut Headless) -> (f32, f32) {
    h.request("engine.execute", json!({"command": "view.zoom100"}), T);
    h.settle(SETTLE);
    let r = h.app.image_rect.expect("the loupe drew the photo");
    (r.width(), r.height())
}

fn size_of(h: &Headless) -> (f32, f32) {
    let id = h.app.session.active().expect("a photo is open");
    let p = h.app.session.catalog.photo(id).expect("photo");
    (p.width as f32, p.height as f32)
}

// Given an uncropped photo, 100 % is its pixel size
#[test]
fn an_uncropped_photo_at_100_is_its_pixel_size() {
    let mut h = detail();
    let (nw, nh) = size_of(&h);
    let (w, hh) = at_100(&mut h);
    assert!((w - nw).abs() < 1.5 && (hh - nh).abs() < 1.5, "{w}×{hh} for {nw}×{nh}");
}

// Given a crop to half the width and a third of the height, 100 % is the cropped pixels
#[test]
fn a_cropped_photo_at_100_is_the_crops_pixel_size() {
    let mut h = detail();
    let (nw, nh) = size_of(&h);
    edit(&mut h, |s| s.crop.geometry.rect = lightcraft_geom::Rect { x0: 0.25, y0: 0.2, x1: 0.75, y1: 0.2 + 1.0 / 3.0 });
    let (w, hh) = at_100(&mut h);
    assert!((w - nw / 2.0).abs() < 1.5 && (hh - nh / 3.0).abs() < 1.5, "{w}×{hh} for a crop of {}×{}", nw / 2.0, nh / 3.0);
}

// Given a photo rotated a quarter turn, 100 % is its pixel size with the axes swapped
#[test]
fn a_rotated_photo_at_100_is_its_pixel_size_swapped() {
    let mut h = detail();
    let (nw, nh) = size_of(&h);
    edit(&mut h, |s| s.orientation = lightcraft_geom::Orientation::Rotate90);
    let (w, hh) = at_100(&mut h);
    assert!((w - nh).abs() < 1.5 && (hh - nw).abs() < 1.5, "{w}×{hh} for {nh}×{nw}");
}

/// A scrub keeps the pressed image point fixed, reverses without drift and doesn't click on release.
#[test]
fn scrub_zoom_tracks_horizontal_travel_and_preserves_the_anchor() {
    use crate::state::Zoom;
    let mut h = detail();
    h.request("engine.execute", json!({"command": "tool.zoom"}), T);
    h.request("engine.execute", json!({"command": "view.zoom100"}), T);
    h.step();
    let image = h.app.image_rect.unwrap();
    let at = image.center() + egui::vec2(35.0, 20.0);
    let point = (at - image.min) / image.size();
    let modifiers = egui::Modifiers::default();
    h.app.synthetic.push(egui::Event::PointerMoved(at));
    h.step();
    h.app.synthetic.push(egui::Event::PointerButton { pos: at, button: egui::PointerButton::Primary, pressed: true, modifiers });
    h.step();
    let to = at + egui::vec2(200.0, 40.0);
    h.app.synthetic.push(egui::Event::PointerMoved(to));
    h.step();
    let Zoom::Percent(percent) = h.app.ui.zoom else { panic!("scrub must zoom in") };
    assert!((percent - 200.0).abs() < 0.01, "{percent}");
    let image = h.app.image_rect.unwrap();
    assert!((image.min + image.size() * point).distance(at) < 0.05, "the press position is the zoom anchor");
    h.app.synthetic.push(egui::Event::PointerMoved(at));
    h.step();
    assert!(matches!(h.app.ui.zoom, Zoom::Percent(p) if (p - 100.0).abs() < 0.01));
    let left = at - egui::vec2(200.0, 0.0);
    h.app.synthetic.push(egui::Event::PointerMoved(left));
    h.step();
    assert!(matches!(h.app.ui.zoom, Zoom::Percent(p) if (p - 50.0).abs() < 0.01));
    h.app.synthetic.push(egui::Event::PointerButton { pos: left, button: egui::PointerButton::Primary, pressed: false, modifiers });
    h.step();
    assert!(h.app.gesture.is_none());
    assert!(matches!(h.app.ui.zoom, Zoom::Percent(p) if (p - 50.0).abs() < 0.01), "release must not toggle Fit");
}

/// Shift temporarily scrubs, ordinary dragging still pans, and editing tools retain their input.
#[test]
fn shift_scrub_does_not_replace_pan_or_editing_tools() {
    use crate::state::Zoom;
    let mut h = detail();
    h.request("engine.execute", json!({"command": "view.zoom100"}), T);
    h.step();
    let at = h.app.image_rect.unwrap().center();
    let drag = |h: &mut Headless, shift| {
        let r = h.request("ui.drag", json!({"x": at.x, "y": at.y, "toX": at.x + 80.0, "toY": at.y, "shift": shift, "steps": 8}), T);
        assert_eq!(r["ok"], true, "{r}");
        h.step();
    };
    let pan = h.app.ui.pan;
    drag(&mut h, false);
    assert_eq!(h.app.ui.zoom, Zoom::Percent(100.0));
    assert!(h.app.ui.pan.0 < pan.0, "an ordinary drag pans the zoomed photo");
    drag(&mut h, true);
    assert!(matches!(h.app.ui.zoom, Zoom::Percent(p) if p > 100.0));
    assert!(h.app.gesture.is_none());
    h.request("engine.execute", json!({"command": "tool.wbPicker"}), T);
    let zoom = h.app.ui.zoom;
    drag(&mut h, true);
    assert_eq!(h.app.ui.zoom, zoom, "a colour picker must not become scrub zoom");
}
