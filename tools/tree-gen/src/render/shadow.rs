// Ombre portée au pied de l'arbre : ellipse très aplatie et floue, décalée à l'opposé
// du soleil ; elle fait partie du calque de l'arbre, qui peut ainsi se poser n'importe où
use tiny_skia::{FillRule, GradientStop, Paint, PathBuilder, Pixmap, Point, RadialGradient, SpreadMode, Transform};

use super::Camera;
use crate::light::sun_dir;
use crate::math::v3;
use crate::render::color;

const FLATTEN: f32 = 0.09;
const OPACITY: f32 = 0.35;

pub fn contact(pix: &mut Pixmap, cam: &Camera) {
    let rx = cam.half_span * 1.05 * cam.scale;
    let cx = cam.origin_x - sun_dir().x * cam.half_span * 0.3 * cam.scale;
    let dark = v3(0.08, 0.12, 0.06);
    let stops = vec![GradientStop::new(0.0, color(dark, OPACITY)), GradientStop::new(0.5, color(dark, OPACITY * 0.5)), GradientStop::new(1.0, color(dark, 0.0))];
    let (Some(shader), Some(path)) = (
        RadialGradient::new(Point::zero(), 0.0, Point::zero(), rx, stops, SpreadMode::Pad, Transform::identity()),
        PathBuilder::from_circle(0.0, 0.0, rx),
    ) else {
        return;
    };
    let place = Transform::from_scale(1.0, FLATTEN).post_translate(cx, cam.ground_y);
    pix.fill_path(&path, &Paint { shader, ..Paint::default() }, FillRule::Winding, place, None);
}
