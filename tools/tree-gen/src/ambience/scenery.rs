// Palette du décor : ciel (soleil ou lune, étoiles), nuages, collines, brume et pré
use crate::math::V3;

pub struct Clouds {
    pub lit: V3,
    pub shade: V3,
    /// Seuil du bruit au-dessus duquel il y a nuage : plus il est haut, plus le ciel est dégagé
    pub cover: f32,
    pub alpha: f32,
}

pub struct Scenery {
    /// Du zénith à l'horizon
    pub sky: [V3; 5],
    /// Position du soleil en fractions de la largeur et de la hauteur de l'image
    pub sun: (f32, f32),
    pub sun_color: V3,
    /// Intensité du halo (1 : plein soleil) et rayon du disque, en fraction de la hauteur
    pub glow: f32,
    pub disc: f32,
    /// Densité d'étoiles (0 : ciel de jour)
    pub stars: f32,
    pub clouds: Clouds,
    /// Collines, de la plus lointaine à la plus proche
    pub hills: [V3; 3],
    pub copses: [V3; 4],
    pub haze: V3,
    /// Pré, du haut (loin) vers le bas (près)
    pub meadow: [V3; 2],
    pub grass: [V3; 4],
    pub flowers: [V3; 4],
}
