// Une feuille : limbe ovale, teinte prise dans la rampe du feuillage selon la lumière
// reçue (soleil filtré par le feuillage, ciel visible), liseré clair côté soleil
use tiny_skia::{FillRule, Paint, PathBuilder, Pixmap, Transform};

use super::{Camera, color};
use crate::ambience::{Ambience, ramp};
use crate::foliage::Leaf;
use crate::math::{V3, smoothstep};

/// Demi-largeur relative le long du limbe (base → pointe) : ovale, pointe adoucie
const PROFILE: [(f32, f32); 9] = [(0.0, 0.0), (0.06, 0.6), (0.18, 0.88), (0.34, 1.0), (0.5, 0.98), (0.66, 0.86), (0.8, 0.64), (0.92, 0.34), (1.0, 0.0)];

/// Légère transparence : les feuilles superposées se fondent au lieu de se découper
const ALPHA: f32 = 0.92;

fn tint(light: f32, amb: &Ambience) -> V3 {
    // En flammes, le cœur ombragé du houppier devient le plus incandescent
    let light = if amb.ember { 1.0 - light } else { light };
    ramp(&amb.leaf, light).lerp(amb.rim, amb.rim_strength * smoothstep(0.8, 1.0, light))
}

pub fn draw(pixmap: &mut Pixmap, leaf: &Leaf, light: f32, amb: &Ambience, cam: &Camera) {
    let point = |t: f32, w: f32| cam.project(leaf.base + leaf.along * t + leaf.half_width * w);
    let mut pb = PathBuilder::new();
    let (x, y) = point(0.0, 0.0);
    pb.move_to(x, y);
    for &(t, w) in &PROFILE[1..] {
        let (x, y) = point(t, w);
        pb.line_to(x, y);
    }
    for &(t, w) in PROFILE[..PROFILE.len() - 1].iter().rev() {
        let (x, y) = point(t, -w);
        pb.line_to(x, y);
    }
    pb.close();
    let Some(path) = pb.finish() else { return };
    let mut paint = Paint::default();
    paint.set_color(color(tint(light, amb), ALPHA));
    pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);
}
