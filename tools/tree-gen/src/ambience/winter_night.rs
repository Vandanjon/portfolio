// Sombre d'hiver : les feuilles sèches brûlent en flammes bleues, nuit glacée, neige sous les étoiles
use super::{Ambience, Clouds, Scenery, srgb};

pub fn ambience() -> Ambience {
    Ambience {
        leaf: [0x040a30, 0x0c2470, 0x1850c0, 0x3a90f0, 0x9ad8ff].map(srgb),
        rim: srgb(0xffffff),
        rim_strength: 0.0,
        ember: true,
        aura: Some(srgb(0x4aa8ff)),
        bark: [0x080a12, 0x141a26, 0x232c3e, 0x384660].map(srgb),
        twig: srgb(0x0c1020),
        scenery: Scenery {
            sky: [0x050a1a, 0x0a1430, 0x10204a, 0x183062, 0x224078].map(srgb),
            sun: (0.2, 0.16),
            sun_color: srgb(0xdcecff),
            glow: 0.4,
            disc: 0.03,
            stars: 1.0,
            clouds: Clouds { lit: srgb(0x5a78b0), shade: srgb(0x101c3a), cover: 0.6, alpha: 0.5 },
            hills: [0x14244a, 0x0e1a36, 0x0a1428].map(srgb),
            copses: [0x081224, 0x0a162a, 0x060e1e, 0x0c1a30].map(srgb),
            haze: srgb(0x20407a),
            meadow: [0x2e4266, 0x141e36].map(srgb),
            grass: [0x3a4a6a, 0x4a5c80, 0x2c3a56, 0x5a6e92].map(srgb),
            flowers: [0xbfe4ff, 0xffffff, 0x8ccfff, 0xe6f6ff].map(srgb),
        },
    }
}
