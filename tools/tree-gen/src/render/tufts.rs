// Touffes d'herbe devant le pied de l'arbre, dans son propre calque : le tronc sort
// de l'herbe du décor au lieu d'être posé dessus
use tiny_skia::{LineCap, Paint, PathBuilder, Pixmap, Stroke, Transform};

use super::{Camera, color};
use crate::ambience::Scenery;
use crate::random::Rng;

const SEED: u64 = 4242;
const BLADES: usize = 320;
/// Demi-largeur de la zone herbue et hauteur des brins, en mètres
const SPREAD: f32 = 1.4;
const HEIGHT: (f32, f32) = (0.3, 0.7);

pub fn grow(pix: &mut Pixmap, cam: &Camera, s: &Scenery) {
    let grass = &s.grass;
    let mut rng = Rng::new(SEED);
    let mut paths: Vec<PathBuilder> = grass.iter().map(|_| PathBuilder::new()).collect();
    for _ in 0..BLADES {
        // Plus dense près du tronc, clairsemé sur les bords
        let u = rng.range(-1.0, 1.0) * rng.f();
        let (x, y) = (cam.origin_x + u * SPREAD * cam.scale, cam.ground_y + rng.range(-0.02, 0.05) * cam.scale);
        let tall = rng.range(HEIGHT.0, HEIGHT.1) * (1.0 - 0.5 * u.abs()) * cam.scale;
        let bend = rng.range(-0.4, 0.4) * tall;
        let p = &mut paths[(rng.f() * grass.len() as f32) as usize];
        p.move_to(x, y);
        p.quad_to(x + bend * 0.2, y - tall * 0.6, x + bend, y - tall);
    }
    let stroke = Stroke { width: 0.025 * cam.scale, line_cap: LineCap::Round, ..Stroke::default() };
    for (pb, &c) in paths.into_iter().zip(grass) {
        // Mêmes teintes et même transparence que l'herbe du décor à cette profondeur
        let mut paint = Paint::default();
        paint.set_color(color(c.lerp(s.meadow[1], 0.35), 0.6));
        if let Some(path) = pb.finish() {
            pix.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
        }
    }
}
