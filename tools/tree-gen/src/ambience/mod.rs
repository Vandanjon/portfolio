// Palette d'une ambiance : l'éclairage calculé choisit une teinte dans des rampes
// peintes (de l'ombre à la lumière), pour un rendu doux plutôt que photographique
mod autumn_day;
mod autumn_night;
mod scenery;
mod spring_day;
mod spring_night;
mod summer_day;
mod summer_night;
mod winter_day;
mod winter_night;

pub use scenery::{Clouds, Scenery};

use crate::math::{V3, v3};

pub struct Ambience {
    /// Feuillage, de l'ombre la plus profonde au plus éclairé
    pub leaf: [V3; 5],
    /// Liseré de lumière sur les feuilles du bord tourné vers le soleil
    pub rim: V3,
    pub rim_strength: f32,
    /// Feuillage en flammes : la rampe s'inverse, le cœur du houppier devient le plus vif
    pub ember: bool,
    /// Halo lumineux autour du houppier (feuillage en flammes)
    pub aura: Option<V3>,
    /// Écorce, de l'ombre à la lumière, et rameaux de l'année
    pub bark: [V3; 4],
    pub twig: V3,
    /// Décor autour de l'arbre
    pub scenery: Scenery,
}

/// Couleur sRGB 0xRRVVBB, composantes dans [0, 1]
pub fn srgb(hex: u32) -> V3 {
    let c = |shift: u32| (hex >> shift & 0xff) as f32 / 255.0;
    v3(c(16), c(8), c(0))
}

/// Teinte à la position `t` ∈ [0, 1] d'une rampe
pub fn ramp(stops: &[V3], t: f32) -> V3 {
    let x = t.clamp(0.0, 1.0) * (stops.len() - 1) as f32;
    let i = (x as usize).min(stops.len() - 2);
    stops[i].lerp(stops[i + 1], x - i as f32)
}

/// Les huit ambiances du site : (thème, saison, palette), noms alignés sur `js/prefs.js`
pub fn all() -> [(&'static str, &'static str, Ambience); 8] {
    [
        ("light", "printemps", spring_day::ambience()),
        ("light", "ete", summer_day::ambience()),
        ("light", "automne", autumn_day::ambience()),
        ("light", "hiver", winter_day::ambience()),
        ("dark", "printemps", spring_night::ambience()),
        ("dark", "ete", summer_night::ambience()),
        ("dark", "automne", autumn_night::ambience()),
        ("dark", "hiver", winter_night::ambience()),
    ]
}
