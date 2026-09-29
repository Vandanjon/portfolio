// Ciel en dégradé du zénith à l'horizon, étoiles la nuit, halo et disque du soleil ou de la lune
use tiny_skia::{FillRule, GradientStop, LinearGradient, Paint, PathBuilder, Pixmap, Point, RadialGradient, Rect, SpreadMode, Transform};

use crate::ambience::Scenery;
use crate::math::{V3, v3};
use crate::random::Rng;
use crate::render::color;

/// Halo radial autour de (x, y), de l'opacité `alpha` au centre à rien au rayon `r`
fn glow(pix: &mut Pixmap, (x, y): (f32, f32), r: f32, stops: &[(f32, V3, f32)]) {
    let stops = stops.iter().map(|&(t, c, a)| GradientStop::new(t, color(c, a))).collect();
    let center = Point::from_xy(x, y);
    if let Some(shader) = RadialGradient::new(center, 0.0, center, r, stops, SpreadMode::Pad, Transform::identity()) {
        let rect = Rect::from_xywh(x - r, y - r, 2.0 * r, 2.0 * r).expect("halo non vide");
        pix.fill_rect(rect, &Paint { shader, ..Paint::default() }, Transform::identity(), None);
    }
}

const STARS_SEED: u64 = 555;
const STARS_PER_1080P: f32 = 700.0;

/// Étoiles plus nombreuses et plus vives vers le zénith
fn stars(pix: &mut Pixmap, s: &Scenery, horizon: f32) {
    let (w, h) = (pix.width() as f32, pix.height() as f32);
    let mut rng = Rng::new(STARS_SEED);
    for _ in 0..(s.stars * STARS_PER_1080P * w * h / (1920.0 * 1080.0)) as usize {
        let (x, y) = (rng.range(0.0, w), rng.f().powf(1.4) * horizon * 0.95);
        let mut paint = Paint::default();
        paint.set_color(color(v3(1.0, 1.0, 1.0), rng.range(0.3, 1.0) * (1.0 - y / horizon)));
        if let Some(path) = PathBuilder::from_circle(x, y, rng.range(0.5, 1.5) * h / 1080.0) {
            pix.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);
        }
    }
}

pub fn paint(pix: &mut Pixmap, s: &Scenery, horizon: f32) {
    let (w, h) = (pix.width() as f32, pix.height() as f32);
    let last = (s.sky.len() - 1) as f32;
    let stops = s.sky.iter().enumerate().map(|(i, &c)| GradientStop::new(i as f32 / last, color(c, 1.0))).collect();
    let shader = LinearGradient::new(Point::from_xy(0.0, 0.0), Point::from_xy(0.0, horizon + 0.04 * h), stops, SpreadMode::Pad, Transform::identity())
        .expect("dégradé du ciel valide");
    pix.fill_rect(Rect::from_xywh(0.0, 0.0, w, h).expect("image non vide"), &Paint { shader, ..Paint::default() }, Transform::identity(), None);

    stars(pix, s, horizon);
    let sun = (s.sun.0 * w, s.sun.1 * h);
    let (c, g) = (s.sun_color, s.glow);
    glow(pix, sun, 0.9 * h, &[(0.0, c, 0.6 * g), (0.18, c, 0.28 * g), (0.5, c, 0.08 * g), (1.0, c, 0.0)]);
    let white = v3(1.0, 1.0, 1.0);
    glow(pix, sun, s.disc * h, &[(0.0, white, 1.0), (0.55, c.lerp(white, 0.6), 1.0), (1.0, c, 0.0)]);
}
