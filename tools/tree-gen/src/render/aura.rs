// Halo lumineux autour du houppier en flammes : la silhouette de l'arbre, réduite,
// floutée et teintée, peinte sous l'arbre
use tiny_skia::{Color, FilterQuality, Pixmap, PixmapPaint, Transform};

use crate::math::V3;

const DOWN: u32 = 8;
const RADIUS: usize = 3;
const PASSES: usize = 3;
const STRENGTH: f32 = 1.4;

/// Flou de boîte séparable (horizontal puis vertical)
fn blur(src: &[f32], w: usize, h: usize) -> Vec<f32> {
    let pass = |src: &[f32], along: (usize, usize), len: usize, lines: usize| {
        let mut out = vec![0.0; src.len()];
        for line in 0..lines {
            for i in 0..len {
                let (lo, hi) = (i.saturating_sub(RADIUS), (i + RADIUS).min(len - 1));
                let sum: f32 = (lo..=hi).map(|k| src[line * along.1 + k * along.0]).sum();
                out[line * along.1 + i * along.0] = sum / (2 * RADIUS + 1) as f32;
            }
        }
        out
    };
    let horizontal = pass(src, (1, w), w, h);
    pass(&horizontal, (w, 1), h, w)
}

pub fn surround(tree: &Pixmap, c: V3) -> Pixmap {
    let (tw, th) = (tree.width() as usize, tree.height() as usize);
    let (w, h) = (tw / DOWN as usize + 1, th / DOWN as usize + 1);
    let mut alpha = vec![0.0f32; w * h];
    for (i, px) in tree.pixels().iter().enumerate() {
        alpha[(i / tw / DOWN as usize) * w + (i % tw) / DOWN as usize] += px.alpha() as f32 / 255.0 / (DOWN * DOWN) as f32;
    }
    for _ in 0..PASSES {
        alpha = blur(&alpha, w, h);
    }
    let mut layer = Pixmap::new(w as u32, h as u32).expect("calque de halo valide");
    for (px, a) in layer.pixels_mut().iter_mut().zip(alpha) {
        if let Some(col) = Color::from_rgba(c.x, c.y, c.z, (a * STRENGTH).min(1.0)) {
            *px = col.premultiply().to_color_u8();
        }
    }
    let mut out = Pixmap::new(tree.width(), tree.height()).expect("dimensions valides");
    let smooth = PixmapPaint { quality: FilterQuality::Bicubic, ..PixmapPaint::default() };
    out.draw_pixmap(0, 0, layer.as_ref(), &smooth, Transform::from_scale(DOWN as f32, DOWN as f32), None);
    out.draw_pixmap(0, 0, tree.as_ref(), &PixmapPaint::default(), Transform::identity(), None);
    out
}
