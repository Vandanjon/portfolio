// Écorce douce : un cylindre lisse dont la teinte suit la rampe d'écorce selon la lumière,
// sans texture ni contour, pour rester dans le registre peint du feuillage
use tiny_skia::Color;

use super::{VIEW, color};
use crate::ambience::{Ambience, ramp};
use crate::light::sun_dir;
use crate::math::{V3, smoothstep, v3};

/// Repère d'un segment vu de face : `side` pointe vers le bord s = +1, `facing` vers l'observateur
pub struct Frame {
    pub side: V3,
    pub facing: V3,
}

impl Frame {
    pub fn new(axis: V3) -> Self {
        let s = axis.cross(VIEW);
        let side = if s.len() < 1e-4 { v3(1.0, 0.0, 0.0) } else { s.norm() };
        Frame { side, facing: (VIEW - axis * VIEW.dot(axis)).norm() }
    }
}

/// Soleil et ciel reçus (fractions), interpolés le long du segment
#[derive(Clone, Copy)]
pub struct Lit {
    pub sun: f32,
    pub sky: f32,
}

/// Teinte de l'écorce à la position transversale `s` ∈ [-1, 1] d'une branche de rayon `radius`
pub fn tint(frame: &Frame, s: f32, pos: V3, radius: f32, lit: Lit, amb: &Ambience) -> Color {
    let s = s.clamp(-1.0, 1.0);
    let n = frame.facing * (1.0 - s * s).sqrt() + frame.side * s;
    let light = 0.12 + 0.55 * lit.sun * n.dot(sun_dir()).max(0.0) + 0.35 * lit.sky * (0.5 + 0.5 * n.y);
    // Pied plus sombre : l'herbe cache le ciel bas
    let foot = 0.75 + 0.25 * smoothstep(0.0, 0.6, pos.y);
    let bark = ramp(&amb.bark, light * foot);
    color(bark.lerp(amb.twig, smoothstep(0.03, 0.006, radius) * 0.6), 1.0)
}
