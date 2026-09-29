// Lumière du feuillage : soleil filtré et ciel visible, calculés sur la normale de touffe,
// moyennés par petits volumes (masses douces, pas de confettis), puis étirés sur la rampe
use std::collections::HashMap;

use super::VIEW;
use crate::foliage::Leaf;
use crate::light::{Shade, sun_dir};
use crate::math::smoothstep;

/// Part de la normale de touffe (le reste : normale propre de la feuille)
const BEND: f32 = 0.9;
/// Côté des volumes de moyenne, en mètres, et part de la moyenne dans la valeur finale
const BLUR_CELL: f32 = 0.45;
const BLUR: f32 = 0.8;

fn raw(leaf: &Leaf, shade: &Shade) -> f32 {
    let center = leaf.base + leaf.along * 0.45;
    let own = if leaf.normal.dot(VIEW) >= 0.0 { leaf.normal } else { -leaf.normal };
    let n = (own * (1.0 - BEND) + shade.outward(center) * BEND).norm();
    0.75 * shade.transmittance(center, sun_dir()) * n.dot(sun_dir()).max(0.0) + 0.45 * shade.sky(center)
}

/// Moyenne de chaque valeur avec celles des volumes voisins (3 × 3 × 3)
fn blur(leaves: &[Leaf], values: &[f32]) -> Vec<f32> {
    let key = |l: &Leaf| {
        let q = l.base * (1.0 / BLUR_CELL);
        (q.x.floor() as i32, q.y.floor() as i32, q.z.floor() as i32)
    };
    let mut cells: HashMap<(i32, i32, i32), (f32, u32)> = HashMap::new();
    for (l, &v) in leaves.iter().zip(values) {
        let c = cells.entry(key(l)).or_default();
        *c = (c.0 + v, c.1 + 1);
    }
    leaves
        .iter()
        .zip(values)
        .map(|(l, &v)| {
            let (x, y, z) = key(l);
            let (mut sum, mut count) = (0.0, 0);
            for (dx, dy, dz) in (0..27).map(|k| (k % 3 - 1, k / 3 % 3 - 1, k / 9 - 1)) {
                if let Some(&(s, c)) = cells.get(&(x + dx, y + dy, z + dz)) {
                    sum += s;
                    count += c;
                }
            }
            v + (sum / count as f32 - v) * BLUR
        })
        .collect()
}

/// Lumière de chaque feuille, étirée entre ses percentiles 5 et 97 : la rampe entière sert,
/// quelle que soit l'ambiance (ombres franches au cœur, pleine lumière au bord)
pub fn lights(leaves: &[Leaf], shade: &Shade) -> Vec<f32> {
    let raw: Vec<f32> = leaves.iter().map(|l| raw(l, shade)).collect();
    let soft = blur(leaves, &raw);
    let mut sorted = soft.clone();
    sorted.sort_by(f32::total_cmp);
    let (lo, hi) = (sorted[sorted.len() * 5 / 100], sorted[sorted.len() * 97 / 100]);
    // Exposant < 1 : la courbe penche vers la lumière, pour un feuillage plus gai
    soft.iter().zip(leaves).map(|(&v, leaf)| smoothstep(lo, hi, v + (leaf.tone - 0.5) * 0.12 * (hi - lo)).powf(0.8)).collect()
}
