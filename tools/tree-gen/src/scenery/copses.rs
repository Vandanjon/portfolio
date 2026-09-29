// Bosquets lointains posés sur une crête : groupes de houppiers ronds,
// une ombre dessous et une face éclairée décalée vers le soleil (à gauche)
use tiny_skia::{FillRule, Paint, PathBuilder, Pixmap, Transform};

use super::hills::Ridge;
use crate::math::{V3, v3};
use crate::random::Rng;
use crate::render::color;

const SEED: u64 = 77;

fn disc(pix: &mut Pixmap, x: f32, y: f32, r: f32, c: V3) {
    let mut paint = Paint::default();
    paint.set_color(color(c, 1.0));
    if let Some(path) = PathBuilder::from_circle(x, y, r) {
        pix.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);
    }
}

pub fn paint(pix: &mut Pixmap, ridge: &Ridge, colors: &[V3; 4]) {
    let (w, k) = (pix.width() as f32, pix.height() as f32 / 1080.0);
    let mut rng = Rng::new(SEED);
    let mut x = rng.range(0.0, 40.0) * k;
    while x < w {
        let count = rng.range(2.0, 6.0) as usize;
        let lit = colors[(rng.f() * colors.len() as f32) as usize];
        let dark = lit.lerp(v3(0.08, 0.12, 0.08), 0.35);
        for j in 0..count {
            let r = rng.range(2.4, 6.0) * k;
            let cx = x + (j as f32 - count as f32 / 2.0) * r * 1.2 + rng.range(0.0, 2.0) * k;
            let base = ridge.y_at(cx) + 1.5 * k;
            disc(pix, cx, base - r * 0.8, r, dark);
            disc(pix, cx - r * 0.18, base - r, r * 0.82, lit);
        }
        x += rng.range(26.0, 116.0) * k;
    }
}
