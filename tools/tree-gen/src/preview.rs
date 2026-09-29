// Mises en page de contrôle : le calque de l'arbre posé sur le décor, comme sur le site
// (arbre décalé à gauche en paysage, centré en portrait sur mobile)
use tiny_skia::{FilterQuality, Pixmap, PixmapPaint, Transform};

use crate::render::Camera;

pub struct Layout {
    pub name: &'static str,
    pub width: u32,
    pub height: u32,
    /// Toutes les positions en fractions de l'image : ligne d'horizon, axe du tronc,
    /// pied de l'arbre, hauteur occupée par le calque de l'arbre
    pub horizon: f32,
    pub tree_x: f32,
    pub foot_y: f32,
    pub tree_height: f32,
}

pub const LAYOUTS: [Layout; 2] = [
    Layout { name: "paysage", width: 1920, height: 1080, horizon: 0.64, tree_x: 0.36, foot_y: 0.9, tree_height: 0.9 },
    Layout { name: "portrait", width: 1080, height: 1920, horizon: 0.6, tree_x: 0.5, foot_y: 0.84, tree_height: 0.59 },
];

pub fn compose(decor: &Pixmap, tree: &Pixmap, cam: &Camera, layout: &Layout) -> Pixmap {
    let mut out = decor.clone();
    let k = layout.tree_height * layout.height as f32 / tree.height() as f32;
    let (x, y) = (layout.tree_x * layout.width as f32 - cam.origin_x * k, layout.foot_y * layout.height as f32 - cam.ground_y * k);
    let paint = PixmapPaint { quality: FilterQuality::Bicubic, ..PixmapPaint::default() };
    out.draw_pixmap(0, 0, tree.as_ref(), &paint, Transform::from_scale(k, k).post_translate(x, y), None);
    out
}
