// Un segment de branche, tronc de cône vu de face : dégradé transversal pour le volume,
// teinte unie pour les rameaux trop fins pour qu'on y voie un modelé
use std::f32::consts::FRAC_PI_2;

use tiny_skia::{FillRule, GradientStop, LinearGradient, Paint, PathBuilder, Pixmap, Point, Shader, SpreadMode, Transform};

use super::Camera;
use super::bark_material::{Frame, Lit, tint};
use crate::ambience::Ambience;
use crate::skeleton::Node;

/// Arrêts du dégradé, répartis en sinus pour suivre la courbure du cylindre
const STOPS: usize = 9;

pub fn segment(pixmap: &mut Pixmap, a: &Node, b: &Node, lit: (Lit, Lit), amb: &Ambience, cam: &Camera) {
    let frame = Frame::new((b.pos - a.pos).norm());
    let (x0, y0) = cam.project(a.pos);
    let (x1, y1) = cam.project(b.pos);
    let (r0, r1) = ((a.radius * cam.scale).max(0.3), (b.radius * cam.scale).max(0.3));
    let rm = r0.max(r1);
    let mid = Lit { sun: (lit.0.sun + lit.1.sun) / 2.0, sky: (lit.0.sky + lit.1.sky) / 2.0 };
    let (pos, radius) = (a.pos.lerp(b.pos, 0.5), (a.radius + b.radius) / 2.0);
    let flat = tint(&frame, 0.0, pos, radius, mid, amb);

    let shader = if rm > 1.2 {
        let (sx, sy) = (frame.side.x, -frame.side.y);
        let (mx, my) = ((x0 + x1) / 2.0, (y0 + y1) / 2.0);
        let stops = (0..STOPS)
            .map(|k| (FRAC_PI_2 * (2.0 * k as f32 / (STOPS - 1) as f32 - 1.0)).sin())
            .map(|s| GradientStop::new((s + 1.0) / 2.0, tint(&frame, s, pos, radius, mid, amb)))
            .collect();
        LinearGradient::new(Point::from_xy(mx - sx * rm, my - sy * rm), Point::from_xy(mx + sx * rm, my + sy * rm), stops, SpreadMode::Pad, Transform::identity()).unwrap_or(Shader::SolidColor(flat))
    } else {
        Shader::SolidColor(flat)
    };
    let paint = Paint { shader, ..Paint::default() };

    let (dx, dy) = (x1 - x0, y1 - y0);
    let len = (dx * dx + dy * dy).sqrt().max(1e-3);
    let (px, py) = (-dy / len, dx / len);
    let mut pb = PathBuilder::new();
    pb.move_to(x0 + px * r0, y0 + py * r0);
    pb.line_to(x1 + px * r1, y1 + py * r1);
    pb.line_to(x1 - px * r1, y1 - py * r1);
    pb.line_to(x0 - px * r0, y0 - py * r0);
    pb.close();
    for path in [pb.finish(), PathBuilder::from_circle(x1, y1, r1)].into_iter().flatten() {
        pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);
    }
}
