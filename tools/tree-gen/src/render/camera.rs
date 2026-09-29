// Projection orthographique vue de face : mètres du monde → pixels de l'image
use crate::foliage::Leaf;
use crate::math::V3;
use crate::skeleton::Node;

/// Projection monde (mètres) → image (pixels)
pub struct Camera {
    pub scale: f32,
    pub origin_x: f32,
    pub ground_y: f32,
    /// Demi-largeur du houppier, en mètres
    pub half_span: f32,
}

impl Camera {
    /// Cadre l'arbre entier dans l'image à `margin` des bords, pied posé à `foot` du bas
    /// (la marge du bas accueille l'ombre portée)
    pub fn frame(nodes: &[Node], leaves: &[Leaf], (width, height): (u32, u32), margin: f32, foot: f32) -> Self {
        let points = nodes.iter().map(|n| n.pos).chain(leaves.iter().map(|l| l.base + l.along));
        let (lo, hi, top) = points.fold((f32::MAX, f32::MIN, 0.0f32), |(lo, hi, top), p| (lo.min(p.x), hi.max(p.x), top.max(p.y)));
        let scale = ((width as f32 - 2.0 * margin) / (hi - lo)).min((height as f32 - margin - foot) / top);
        Camera { scale, origin_x: width as f32 / 2.0 - (lo + hi) / 2.0 * scale, ground_y: height as f32 - foot, half_span: (hi - lo) / 2.0 }
    }

    pub fn project(&self, p: V3) -> (f32, f32) {
        (self.origin_x + p.x * self.scale, self.ground_y - p.y * self.scale)
    }
}
