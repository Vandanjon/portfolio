// Éclairage du squelette et du feuillage, indépendant de la palette :
// calculé une fois, il sert au rendu de toutes les ambiances
use super::leaf_light;
use crate::foliage::Leaf;
use crate::light::{Shade, sun_dir};
use crate::skeleton::{Node, ROOT};

pub enum Item {
    Segment(usize),
    Leaf(usize),
}

pub struct Lighting {
    /// Éléments à peindre, du plus loin au plus proche
    pub order: Vec<Item>,
    /// Soleil et ciel reçus par chaque nœud
    pub bark: Vec<(f32, f32)>,
    /// Lumière de chaque feuille, dans [0, 1]
    pub leaves: Vec<f32>,
}

/// Soleil et ciel reçus par chaque nœud, lissés le long des branches
/// pour que l'ombre ne découpe pas l'écorce en bandes d'un segment à l'autre
fn node_light(nodes: &[Node], shade: &Shade) -> Vec<(f32, f32)> {
    let mut light: Vec<(f32, f32)> = nodes.iter().map(|n| (shade.transmittance(n.pos, sun_dir()), shade.sky(n.pos))).collect();
    for _ in 0..3 {
        for j in 0..nodes.len() {
            if let Some(&(sun, sky)) = light.get(nodes[j].parent) {
                light[j] = ((light[j].0 + sun) / 2.0, (light[j].1 + sky) / 2.0);
            }
        }
    }
    light
}

impl Lighting {
    pub fn new(nodes: &[Node], leaves: &[Leaf], shade: &Shade) -> Self {
        let mut items: Vec<(f32, Item)> = nodes
            .iter()
            .enumerate()
            .filter(|(_, n)| n.parent != ROOT)
            .map(|(i, n)| ((n.pos.z + nodes[n.parent].pos.z) / 2.0, Item::Segment(i)))
            .chain(leaves.iter().enumerate().map(|(i, l)| (l.base.z + l.along.z / 2.0, Item::Leaf(i))))
            .collect();
        items.sort_by(|a, b| a.0.total_cmp(&b.0));
        Lighting { order: items.into_iter().map(|(_, item)| item).collect(), bark: node_light(nodes, shade), leaves: leaf_light::lights(leaves, shade) }
    }
}
