// Feuillage du charme : chaque rameau fin porte une « fronde », petit axe plat
// où les feuilles alternent de part et d'autre, limbe tourné vers le haut et l'extérieur
use std::f32::consts::PI;

use crate::math::{V3, v3};
use crate::random::{Rng, fbm};
use crate::skeleton::{Node, ROOT};
use crate::species::Species;

pub struct Leaf {
    pub base: V3,
    /// De la base à la pointe (longueur incluse)
    pub along: V3,
    /// Demi-largeur au plus large
    pub half_width: V3,
    pub normal: V3,
    /// Variation de teinte dans [0, 1], corrélée par touffes
    pub tone: f32,
}

impl Leaf {
    /// Aire d'un limbe ovale ≈ 0,7 × longueur × largeur
    pub fn area(&self) -> f32 {
        0.7 * self.along.len() * 2.0 * self.half_width.len()
    }
}

pub fn sprout(nodes: &[Node], sp: &Species, center: V3, rng: &mut Rng) -> Vec<Leaf> {
    let up = v3(0.0, 1.0, 0.0);
    let mut leaves = Vec::new();
    for node in nodes.iter().filter(|n| n.parent != ROOT && n.radius < sp.leafy_radius) {
        let twig = (node.pos - nodes[node.parent].pos).norm();
        let outward = (node.pos - center).norm();
        // Frondes étalées plutôt à l'horizontale, tournées vers l'extérieur
        let flat = v3(outward.x, 0.0, outward.z).norm();
        let axis = (twig * 0.35 + flat * 0.6 + rng.dir() * 0.35).norm();
        let hint = (up * 0.8 + outward * 0.5 + rng.dir() * 0.3).norm();
        let plane = (hint - axis * hint.dot(axis)).norm();
        let side = axis.cross(plane);
        let length = rng.range(sp.spray_length.0, sp.spray_length.1);
        let count = rng.range(sp.spray_leaves.0 as f32, sp.spray_leaves.1 as f32 + 1.0) as usize;
        let tone = fbm(node.pos * 0.6, 2, 97);
        for i in 0..count {
            let t = (i as f32 + 0.5) / count as f32;
            let flank = if i + 1 == count { 0.0 } else if i % 2 == 0 { 1.0 } else { -1.0 };
            let dir = (axis * 0.55 + side * (flank * rng.range(0.5, 1.1)) + rng.dir() * 0.35).norm();
            let tilt = (plane + rng.dir() * 0.35).norm();
            let normal = (tilt - dir * tilt.dot(dir)).norm();
            let size = sp.leaf_length * (0.7 + 0.4 * (PI * (0.3 + 0.7 * t)).sin()) * rng.range(0.85, 1.1);
            leaves.push(Leaf {
                base: node.pos + axis * (length * t),
                along: dir * size,
                half_width: dir.cross(normal).norm() * (size * 0.29),
                normal,
                tone: (tone * 0.7 + rng.f() * 0.3).clamp(0.0, 1.0),
            });
        }
    }
    leaves
}
