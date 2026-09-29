// Clair d'automne : lumière dorée d'après-midi, charme jaune d'or, pré roux jonché de feuilles
use super::{Ambience, Clouds, Scenery, srgb};

pub fn ambience() -> Ambience {
    Ambience {
        leaf: [0x6e3a10, 0xa0601a, 0xcf932c, 0xe9bf4c, 0xf7df8a].map(srgb),
        rim: srgb(0xffe2b0),
        rim_strength: 0.45,
        ember: false,
        aura: None,
        bark: [0x3a322c, 0x5e524a, 0x877a6e, 0xb2a391].map(srgb),
        twig: srgb(0x4a3a2a),
        scenery: Scenery {
            sky: [0x8e96d0, 0xb99bc6, 0xeaa99a, 0xf7c887, 0xfbe2b0].map(srgb),
            sun: (0.2, 0.3),
            sun_color: srgb(0xffdca8),
            glow: 1.1,
            disc: 0.05,
            stars: 0.0,
            clouds: Clouds { lit: srgb(0xffc796), shade: srgb(0x9a7ea6), cover: 0.53, alpha: 0.85 },
            hills: [0xb79bb9, 0xa8775b, 0x8c5b3d].map(srgb),
            copses: [0xc0592b, 0xd9822b, 0x8f3f22, 0xe0a43a].map(srgb),
            haze: srgb(0xffd6aa),
            meadow: [0xc99a4b, 0x7a5230].map(srgb),
            grass: [0xb88a3e, 0xd4a855, 0x9c7236, 0xe0bf6a].map(srgb),
            flowers: [0xd9541e, 0xe8892a, 0xb83a1a, 0xf0b43c].map(srgb),
        },
    }
}
