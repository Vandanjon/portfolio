// Collines en trois plans, de plus en plus proches et saturées, séparées par des
// bandes de brume (perspective atmosphérique), avec des bosquets sur le plan du milieu
use tiny_skia::{FillRule, GradientStop, LinearGradient, Paint, PathBuilder, Pixmap, Point, Rect, SpreadMode, Transform};

use super::copses;
use crate::ambience::Scenery;
use crate::math::{V3, v3};
use crate::random::fbm;
use crate::render::color;

/// Espacement horizontal (px) des points d'une crête
const STEP: f32 = 4.0;

/// Profil d'une crête : hauteur à chaque pas le long de l'image
pub struct Ridge {
    heights: Vec<f32>,
}

impl Ridge {
    fn new(width: f32, base: f32, amp: f32, scale: f32, seed: u32) -> Self {
        let n = (width / STEP) as usize + 2;
        Ridge { heights: (0..n).map(|i| base - amp * fbm(v3(i as f32 * STEP / scale, 3.7, 0.0), 4, seed)).collect() }
    }

    pub fn y_at(&self, x: f32) -> f32 {
        let t = (x / STEP).clamp(0.0, (self.heights.len() - 2) as f32);
        let i = t as usize;
        self.heights[i] + (self.heights[i + 1] - self.heights[i]) * (t - i as f32)
    }

    fn fill(&self, pix: &mut Pixmap, c: V3) {
        let (w, h) = (pix.width() as f32, pix.height() as f32);
        let mut pb = PathBuilder::new();
        pb.move_to(0.0, h);
        for (i, &y) in self.heights.iter().enumerate() {
            pb.line_to((i as f32 * STEP).min(w), y);
        }
        pb.line_to(w, h);
        pb.close();
        let mut paint = Paint::default();
        paint.set_color(color(c, 1.0));
        if let Some(path) = pb.finish() {
            pix.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);
        }
    }
}

/// Bande de brume : transparente en haut, `alpha` aux 3/5, transparente en bas
fn haze(pix: &mut Pixmap, c: V3, y: f32, height: f32, alpha: f32) {
    let stops = vec![GradientStop::new(0.0, color(c, 0.0)), GradientStop::new(0.6, color(c, alpha)), GradientStop::new(1.0, color(c, 0.0))];
    let shader = LinearGradient::new(Point::from_xy(0.0, y), Point::from_xy(0.0, y + height), stops, SpreadMode::Pad, Transform::identity()).expect("dégradé de brume valide");
    let rect = Rect::from_xywh(0.0, y, pix.width() as f32, height).expect("bande non vide");
    pix.fill_rect(rect, &Paint { shader, ..Paint::default() }, Transform::identity(), None);
}

/// Peint les collines et renvoie la hauteur où commence le pré
pub fn paint(pix: &mut Pixmap, s: &Scenery, horizon: f32) -> f32 {
    let (w, h) = (pix.width() as f32, pix.height() as f32);
    Ridge::new(w, horizon - 0.01 * h, 0.1 * h, 0.42 * h, 31).fill(pix, s.hills[0]);
    haze(pix, s.haze, horizon - 0.12 * h, 0.2 * h, 0.5);
    let mid = Ridge::new(w, horizon + 0.035 * h, 0.06 * h, 0.26 * h, 57);
    mid.fill(pix, s.hills[1]);
    copses::paint(pix, &mid, &s.copses);
    Ridge::new(w, horizon + 0.075 * h, 0.035 * h, 0.17 * h, 83).fill(pix, s.hills[2]);
    haze(pix, s.haze, horizon + 0.02 * h, 0.12 * h, 0.3);
    horizon + 0.07 * h
}
