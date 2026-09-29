// Nuages : densité fractale étirée à l'horizontale, éclairés côté soleil (écart de densité
// vers la source), calculés en basse résolution puis agrandis pour des bords vaporeux
use tiny_skia::{Color, FilterQuality, Pixmap, PixmapPaint, Transform};

use crate::ambience::Scenery;
use crate::math::{smoothstep, v3};
use crate::random::fbm;

/// Facteur de sous-échantillonnage du calque de nuages
const DOWN: u32 = 4;
const SEED: u32 = 301;

pub fn paint(pix: &mut Pixmap, s: &Scenery, horizon: f32) {
    let (w, h) = (pix.width(), pix.height() as f32);
    let (cw, ch) = (w / DOWN + 1, (horizon / DOWN as f32) as u32 + 1);
    let Some(mut layer) = Pixmap::new(cw, ch) else { return };
    let (sx, sy) = (0.3 * h, 0.11 * h);
    let density = |x: f32, y: f32| fbm(v3(x / sx, y / sy, 0.0), 5, SEED);
    let sun = (s.sun.0 * w as f32, s.sun.1 * h);
    for j in 0..ch {
        let y = (j * DOWN) as f32;
        // Pas de nuage collé au zénith ni à l'horizon
        let band = smoothstep(0.0, 0.3 * horizon, y) * (1.0 - smoothstep(0.75 * horizon, horizon, y));
        if band < 0.01 {
            continue;
        }
        for i in 0..cw {
            let x = (i * DOWN) as f32;
            let n = density(x, y);
            let d = smoothstep(s.clouds.cover, s.clouds.cover + 0.12, n) * band;
            if d < 0.01 {
                continue;
            }
            let (vx, vy) = (sun.0 - x, sun.1 - y);
            let l = (vx * vx + vy * vy).sqrt().max(1.0);
            let toward = density(x + vx / l * 0.015 * h, y + vy / l * 0.015 * h);
            let c = s.clouds.shade.lerp(s.clouds.lit, (0.5 + (n - toward) * 8.0).clamp(0.0, 1.0));
            let a = d * s.clouds.alpha;
            if let Some(px) = Color::from_rgba(c.x, c.y, c.z, a) {
                layer.pixels_mut()[(j * cw + i) as usize] = px.premultiply().to_color_u8();
            }
        }
    }
    let paint = PixmapPaint { quality: FilterQuality::Bicubic, ..PixmapPaint::default() };
    pix.draw_pixmap(0, 0, layer.as_ref(), &paint, Transform::from_scale(DOWN as f32, DOWN as f32), None);
}
