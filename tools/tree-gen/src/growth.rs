// Croissance par colonisation de l'espace (Runions et al., 2007) :
// chaque attracteur tire le nœud le plus proche, qui pousse vers la moyenne de ses attracteurs
use crate::crown::Crown;
use crate::field::Field;
use crate::math::{V3, v3};
use crate::random::Rng;
use crate::skeleton::{Node, ROOT, trunk};
use crate::species::Species;

const MAX_ITERATIONS: usize = 400;
const MAX_CHILDREN: u8 = 4;

pub fn grow(sp: &Species, crown: &Crown, rng: &mut Rng) -> Vec<Node> {
    let mut nodes = trunk(sp, rng);
    let mut field = Field::new(crown.attractors(sp.attractors, rng), sp);
    field.link(&nodes, 0);
    // Le fût monte droit tant que la couronne est hors de portée
    while field.nearest.iter().all(|&(k, _)| k == ROOT) {
        let top = nodes.len() - 1;
        nodes.push(Node { pos: nodes[top].pos + v3(0.0, sp.step, 0.0), parent: top, radius: 0.0 });
        field.link(&nodes, top + 1);
    }
    let mut children = vec![0u8; nodes.len()];
    for _ in 0..MAX_ITERATIONS {
        let mut pull = vec![V3::default(); nodes.len()];
        for (i, &(k, _)) in field.nearest.iter().enumerate() {
            if field.alive[i] && k != ROOT {
                pull[k] += (field.points[i] - nodes[k].pos).norm();
            }
        }
        let start = nodes.len();
        for k in 0..start {
            if pull[k] == V3::default() || children[k] >= MAX_CHILDREN {
                continue;
            }
            let dir = (pull[k].norm() + v3(0.0, sp.tropism, 0.0) + rng.dir() * sp.zigzag).norm();
            nodes.push(Node { pos: nodes[k].pos + dir * sp.step, parent: k, radius: 0.0 });
            children[k] += 1;
            children.push(0);
        }
        if nodes.len() == start {
            break;
        }
        field.link(&nodes, start);
    }
    nodes
}
