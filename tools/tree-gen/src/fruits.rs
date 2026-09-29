// Emplacements des fruits, un par étape de data/projects.toml : au bord visible du
// feuillage, bien répartis dans le houppier, du plus bas au plus haut (la chronologie monte)
use std::path::Path;

use crate::foliage::Leaf;
use crate::landmarks::{crown_box, frac};
use crate::light::Shade;
use crate::math::{V3, v3};
use crate::render::Camera;

/// Une feuille sur SAMPLE est candidate : bien assez pour quelques fruits, et rapide
const SAMPLE: usize = 25;
/// Part de la vue non masquée devant la feuille : le fruit pend au bord visible du feuillage
const VISIBLE: f32 = 0.5;
/// Zone retenue, en fraction des demi-axes du houppier : les étiquettes restent sur l'arbre
const INNER: f32 = 0.7;

type Spot = (f32, f32);

/// Nombre d'étapes du parcours : une table `[[career]]` par étape (sans dépendance TOML)
pub fn count(site: &Path) -> usize {
    let data = std::fs::read_to_string(site.join("data/projects.toml")).expect("lecture de data/projects.toml");
    data.lines().filter(|line| line.split_whitespace().collect::<String>() == "[[career]]").count()
}

/// `n` emplacements en fractions de l'image : le premier au plus près du cœur, chacun des
/// suivants le plus loin possible de ceux déjà pris
pub fn spots(leaves: &[Leaf], shade: &Shade, cam: &Camera, size: (u32, u32), heart: V3, n: usize) -> Vec<Spot> {
    let (l, t, r, b) = crown_box(leaves, cam, size);
    let (cx, cy, rx, ry) = ((l + r) / 2.0, (t + b) / 2.0, (r - l) / 2.0 * INNER, (b - t) / 2.0 * INNER);
    // Distances à l'écran : une fraction de largeur ne vaut pas une fraction de hauteur
    let aspect = size.0 as f32 / size.1 as f32;
    let dist = |a: Spot, b: Spot| ((a.0 - b.0) * aspect).hypot(a.1 - b.1);
    let toward_viewer = v3(0.0, 0.0, 1.0);
    let candidates: Vec<Spot> = leaves
        .iter()
        .step_by(SAMPLE)
        .filter(|leaf| leaf.base.z > 0.0 && shade.transmittance(leaf.base, toward_viewer) > VISIBLE)
        .map(|leaf| frac(cam, size, leaf.base))
        .filter(|&(x, y)| ((x - cx) / rx).powi(2) + ((y - cy) / ry).powi(2) < 1.0)
        .collect();
    let heart = frac(cam, size, heart);
    let nearest = candidates.iter().copied().min_by(|&a, &b| dist(a, heart).total_cmp(&dist(b, heart)));
    let mut kept: Vec<Spot> = nearest.into_iter().take(n).collect();
    while kept.len() < n {
        let gap = |c: Spot| kept.iter().map(|&k| dist(c, k)).fold(f32::INFINITY, f32::min);
        let farthest = candidates.iter().copied().max_by(|&a, &b| gap(a).total_cmp(&gap(b)));
        kept.push(farthest.expect("feuillage visible dans le houppier"));
    }
    kept.sort_by(|a, b| b.1.total_cmp(&a.1));
    kept
}
