// Rendu orthographique vu de face (+z vers l'observateur), par l'algorithme du peintre :
// segments d'écorce et feuilles triés du plus loin au plus proche
mod aura;
mod bark;
mod bark_material;
mod camera;
mod leaf;
mod leaf_light;
mod lighting;
mod shadow;
mod tufts;

pub use camera::Camera;
pub use lighting::Lighting;

use bark_material::Lit;
use lighting::Item;

use tiny_skia::{Color, Pixmap};

use crate::ambience::Ambience;
use crate::foliage::Leaf;
use crate::math::{V3, v3};
use crate::skeleton::Node;

/// Couleur sRGB (composantes dans [0, 1]) vers tiny-skia
pub fn color(c: V3, alpha: f32) -> Color {
    Color::from_rgba(c.x.clamp(0.0, 1.0), c.y.clamp(0.0, 1.0), c.z.clamp(0.0, 1.0), alpha).unwrap_or(Color::BLACK)
}

pub const VIEW: V3 = v3(0.0, 0.0, 1.0);

pub fn draw(nodes: &[Node], leaves: &[Leaf], lighting: &Lighting, amb: &Ambience, cam: &Camera, (width, height): (u32, u32)) -> Pixmap {
    let mut pixmap = Pixmap::new(width, height).expect("dimensions d'image valides");
    shadow::contact(&mut pixmap, cam);
    for item in &lighting.order {
        match *item {
            Item::Segment(i) => {
                let lit = |(sun, sky): (f32, f32)| Lit { sun, sky };
                let ends = (lit(lighting.bark[nodes[i].parent]), lit(lighting.bark[i]));
                bark::segment(&mut pixmap, &nodes[nodes[i].parent], &nodes[i], ends, amb, cam);
            }
            Item::Leaf(i) => leaf::draw(&mut pixmap, &leaves[i], lighting.leaves[i], amb, cam),
        }
    }
    tufts::grow(&mut pixmap, cam, &amb.scenery);
    match amb.aura {
        Some(c) => aura::surround(&pixmap, c),
        None => pixmap,
    }
}
