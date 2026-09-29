// Champ d'attracteurs de la colonisation, rangés dans une grille spatiale
// pour ne tester que les voisins d'un nœud
use std::collections::HashMap;

use crate::math::V3;
use crate::skeleton::{Node, ROOT};
use crate::species::Species;

type Cell = (i32, i32, i32);

fn cell_of(p: V3, size: f32) -> Cell {
    ((p.x / size).floor() as i32, (p.y / size).floor() as i32, (p.z / size).floor() as i32)
}

pub struct Field {
    pub points: Vec<V3>,
    pub alive: Vec<bool>,
    /// Nœud le plus proche dans la portée d'attraction, et le carré de sa distance
    pub nearest: Vec<(usize, f32)>,
    grid: HashMap<Cell, Vec<usize>>,
    size: f32,
    kill2: f32,
}

impl Field {
    pub fn new(points: Vec<V3>, sp: &Species) -> Self {
        let mut grid: HashMap<Cell, Vec<usize>> = HashMap::new();
        for (i, p) in points.iter().enumerate() {
            grid.entry(cell_of(*p, sp.influence)).or_default().push(i);
        }
        let n = points.len();
        Field { points, alive: vec![true; n], nearest: vec![(ROOT, sp.influence * sp.influence); n], grid, size: sp.influence, kill2: sp.kill * sp.kill }
    }

    /// Les nouveaux nœuds absorbent les attracteurs trop proches et captent ceux dont ils sont les plus proches
    pub fn link(&mut self, nodes: &[Node], from: usize) {
        let Field { points, alive, nearest, grid, size, kill2 } = self;
        for (j, node) in nodes.iter().enumerate().skip(from) {
            let (cx, cy, cz) = cell_of(node.pos, *size);
            for dx in -1..=1 {
                for dy in -1..=1 {
                    for dz in -1..=1 {
                        let Some(ids) = grid.get(&(cx + dx, cy + dy, cz + dz)) else { continue };
                        for &i in ids {
                            if !alive[i] {
                                continue;
                            }
                            let d = points[i] - node.pos;
                            let d2 = d.dot(d);
                            if d2 < *kill2 {
                                alive[i] = false;
                            } else if d2 < nearest[i].1 {
                                nearest[i] = (j, d2);
                            }
                        }
                    }
                }
            }
        }
    }
}
