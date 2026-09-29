// Repères pour la faune du site, en fractions de l'image de l'arbre : perchoirs sur des
// branches bien visibles, contour du houppier, cœur (d'où part le halo) et pied
use std::fmt::Write;
use std::path::Path;

use crate::foliage::Leaf;
use crate::light::Shade;
use crate::math::{V3, v3};
use crate::render::Camera;
use crate::skeleton::{Node, ROOT};

/// Branches assez fortes pour un oiseau, assez fines pour qu'il s'y distingue (rayon, m)
const PERCH_RADIUS: (f32, f32) = (0.015, 0.06);
/// Part de la vue non masquée par le feuillage devant le perchoir
const VISIBLE: f32 = 0.35;
/// Écart minimal entre deux perchoirs, en mètres, et nombre maximal retenu
const SPACING: f32 = 1.2;
const MAX_PERCHES: usize = 20;

fn perches(nodes: &[Node], shade: &Shade) -> Vec<V3> {
    let toward_viewer = v3(0.0, 0.0, 1.0);
    let mut candidates: Vec<(f32, V3)> = nodes
        .iter()
        .filter(|n| n.parent != ROOT && n.radius > PERCH_RADIUS.0 && n.radius < PERCH_RADIUS.1 && n.pos.z > 0.0)
        // Branche presque horizontale : on peut s'y poser
        .filter(|n| (n.pos - nodes[n.parent].pos).norm().y.abs() < 0.6)
        .map(|n| (shade.transmittance(n.pos + v3(0.0, 0.08, 0.0), toward_viewer), n.pos))
        .filter(|&(seen, _)| seen > VISIBLE)
        .collect();
    candidates.sort_by(|a, b| b.0.total_cmp(&a.0));
    let mut kept: Vec<V3> = Vec::new();
    for (_, p) in candidates {
        let far = kept.iter().all(|k| (v3(k.x, k.y, 0.0) - v3(p.x, p.y, 0.0)).len() > SPACING);
        if far && kept.len() < MAX_PERCHES {
            kept.push(p);
        }
    }
    kept
}

/// Point de l'espace en fractions de l'image de l'arbre, au dix-millième
pub fn frac(cam: &Camera, (width, height): (u32, u32), p: V3) -> (f32, f32) {
    let (x, y) = cam.project(p);
    ((x / width as f32 * 1e4).round() / 1e4, (y / height as f32 * 1e4).round() / 1e4)
}

/// Contour du houppier (gauche, haut, droite, bas) : boîte des pointes de feuilles
pub fn crown_box(leaves: &[Leaf], cam: &Camera, size: (u32, u32)) -> (f32, f32, f32, f32) {
    let tips = leaves.iter().map(|l| frac(cam, size, l.base + l.along));
    tips.fold((1.0, 1.0, 0.0, 0.0), |(l, t, r, b), (x, y)| (l.min(x), t.min(y), r.max(x), b.max(y)))
}

pub fn json(nodes: &[Node], leaves: &[Leaf], shade: &Shade, cam: &Camera, size: (u32, u32), heart_y: f32, path: &Path) {
    let frac = |p: V3| frac(cam, size, p);
    let (l, t, r, b) = crown_box(leaves, cam, size);
    let list = perches(nodes, shade).into_iter().map(frac).map(|(x, y)| format!("[{x},{y}]")).collect::<Vec<_>>().join(",");
    let (hx, hy) = frac(v3(0.0, heart_y, 0.0));
    let (fx, fy) = frac(V3::default());
    let mut out = String::new();
    writeln!(out, r#"{{"perches":[{list}],"crown":[{l},{t},{r},{b}],"heart":[{hx},{hy}],"foot":[{fx},{fy}]}}"#).expect("écriture en mémoire");
    std::fs::write(path, out).expect("écriture des repères");
}
