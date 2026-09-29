// Squelette de l'arbre : nœuds reliés à leur parent, du pied vers les rameaux
use crate::math::{V3, v3};
use crate::random::Rng;
use crate::species::Species;

pub const ROOT: usize = usize::MAX;

pub struct Node {
    pub pos: V3,
    /// Indice du parent, toujours inférieur à celui du nœud (ROOT pour le pied)
    pub parent: usize,
    pub radius: f32,
}

/// Fût légèrement penché et sinueux, du sol jusqu'à la naissance de la couronne
pub fn trunk(sp: &Species, rng: &mut Rng) -> Vec<Node> {
    let steps = (sp.trunk_height / sp.step).ceil() as usize;
    let lean = v3(rng.range(-0.08, 0.08), 0.0, rng.range(-0.05, 0.05));
    let phase = rng.range(0.0, std::f32::consts::TAU);
    (0..=steps)
        .map(|i| {
            let y = sp.trunk_height * i as f32 / steps as f32;
            let sway = 0.05 * (y * 1.7 + phase).sin();
            Node {
                pos: v3(lean.x * y + sway, y, lean.z * y),
                parent: if i == 0 { ROOT } else { i - 1 },
                radius: 0.0,
            }
        })
        .collect()
}

/// Rayons par le modèle des tuyaux, des rameaux vers le pied, puis empattement au sol
pub fn thicken(nodes: &mut [Node], sp: &Species) {
    let e = sp.pipe_exponent;
    let mut flow = vec![0.0f32; nodes.len()];
    for j in (0..nodes.len()).rev() {
        let r = if flow[j] == 0.0 { sp.twig_radius } else { flow[j].powf(1.0 / e) };
        nodes[j].radius = r;
        if nodes[j].parent != ROOT {
            flow[nodes[j].parent] += r.powf(e);
        }
    }
    for n in nodes.iter_mut().filter(|n| n.pos.y < 1.0) {
        n.radius *= 1.0 + 0.55 * (-n.pos.y / 0.28).exp();
    }
}

/// Adoucit le tracé des grosses branches (le zigzag ne sied qu'aux rameaux) :
/// chaque nœud glisse vers le milieu de son parent et de son enfant principal
pub fn smooth(nodes: &mut [Node], min_radius: f32, passes: usize) {
    let mut main_child = vec![ROOT; nodes.len()];
    for j in 1..nodes.len() {
        let p = nodes[j].parent;
        if p != ROOT && (main_child[p] == ROOT || nodes[main_child[p]].radius < nodes[j].radius) {
            main_child[p] = j;
        }
    }
    for _ in 0..passes {
        let moved: Vec<V3> = nodes
            .iter()
            .zip(&main_child)
            .map(|(n, &c)| {
                if n.parent == ROOT || c == ROOT || n.radius < min_radius {
                    return n.pos;
                }
                let weight = (n.radius / min_radius - 1.0).clamp(0.0, 1.0) * 0.6;
                n.pos.lerp(nodes[n.parent].pos.lerp(nodes[c].pos, 0.5), weight)
            })
            .collect();
        for (n, p) in nodes.iter_mut().zip(moved) {
            n.pos = p;
        }
    }
}
