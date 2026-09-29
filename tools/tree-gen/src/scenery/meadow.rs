// Pré au premier plan : dégradé du lointain au proche, herbes et fleurs dont la taille
// et l'espacement grandissent vers le bas (perspective)
use tiny_skia::{FillRule, GradientStop, LinearGradient, LineCap, Paint, PathBuilder, Pixmap, Point, SpreadMode, Stroke, Transform};

use crate::ambience::Scenery;
use crate::math::v3;
use crate::random::{Rng, fbm};
use crate::render::color;

const SEED: u64 = 911;
const FLOWERS_PER_1080P: f32 = 260.0;

pub fn paint(pix: &mut Pixmap, s: &Scenery, top: f32) {
    let (w, h) = (pix.width() as f32, pix.height() as f32);
    let k = h / 1080.0;
    let edge = |x: f32| top + 0.012 * h * (fbm(v3(x / (0.5 * h), 9.1, 0.0), 3, 5) - 0.5) * 2.0;
    let mut pb = PathBuilder::new();
    pb.move_to(0.0, h);
    let mut x = 0.0;
    while x <= w + 6.0 {
        pb.line_to(x, edge(x));
        x += 6.0;
    }
    pb.line_to(w, h);
    pb.close();
    let stops = vec![GradientStop::new(0.0, color(s.meadow[0], 1.0)), GradientStop::new(1.0, color(s.meadow[1], 1.0))];
    let shader = LinearGradient::new(Point::from_xy(0.0, top), Point::from_xy(0.0, h), stops, SpreadMode::Pad, Transform::identity()).expect("dégradé du pré valide");
    if let Some(path) = pb.finish() {
        pix.fill_path(&path, &Paint { shader, ..Paint::default() }, FillRule::Winding, Transform::identity(), None);
    }

    // Herbes : une touffe de chemins par couleur et par bande de profondeur (largeur de trait)
    let mut rng = Rng::new(SEED);
    let mut blades: Vec<PathBuilder> = (0..s.grass.len() * 3).map(|_| PathBuilder::new()).collect();
    let mut y = top;
    while y < h + 20.0 * k {
        let depth = ((y - top) / (h - top)).clamp(0.0, 1.0);
        let mut x = rng.range(0.0, 4.0);
        while x < w {
            let (base, tall) = (y.max(edge(x)) + rng.range(0.0, 3.0) * k, (4.0 + 34.0 * depth) * k * rng.range(0.6, 1.2));
            let bend = rng.range(-0.4, 0.4) * tall;
            let b = &mut blades[(rng.f() * s.grass.len() as f32) as usize * 3 + (depth * 2.99) as usize];
            b.move_to(x, base);
            b.quad_to(x + bend * 0.2, base - tall * 0.6, x + bend, base - tall);
            x += (1.6 + 9.0 * depth) * k * rng.range(0.6, 1.4);
        }
        y += (2.0 + 14.0 * depth) * k;
    }
    for (i, pb) in blades.into_iter().enumerate() {
        let mut paint = Paint::default();
        // Brins translucides : le pré reste une texture calme sous le contenu du site
        paint.set_color(color(s.grass[i / 3], 0.55));
        let stroke = Stroke { width: (0.9 + 0.8 * (i % 3) as f32) * k, line_cap: LineCap::Round, ..Stroke::default() };
        if let Some(path) = pb.finish() {
            pix.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
        }
    }

    for _ in 0..(FLOWERS_PER_1080P * w / 1920.0) as usize {
        let depth = rng.f().powf(1.5);
        let (x, y) = (rng.range(0.0, w), top + (6.0 * k) + depth * (h - top));
        let mut paint = Paint::default();
        paint.set_color(color(s.flowers[(rng.f() * s.flowers.len() as f32) as usize], 1.0));
        if let Some(path) = PathBuilder::from_circle(x, y, (1.0 + 3.0 * depth) * k) {
            pix.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);
        }
    }
}
