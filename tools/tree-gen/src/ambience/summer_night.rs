// Sombre d'été : le feuillage brûle en flammes rouges, nuit rougeoyante, braises dans l'herbe
use super::{Ambience, Clouds, Scenery, srgb};

pub fn ambience() -> Ambience {
    Ambience {
        leaf: [0x2a0504, 0x6a0e08, 0xb82410, 0xf05a1c, 0xffb04a].map(srgb),
        rim: srgb(0xfff1b0),
        rim_strength: 0.0,
        ember: true,
        aura: Some(srgb(0xff3a14)),
        bark: [0x0e0a0a, 0x1e1614, 0x33251f, 0x4e362a].map(srgb),
        twig: srgb(0x1a0e0a),
        scenery: Scenery {
            sky: [0x0c070e, 0x1c0c16, 0x33121c, 0x521a20, 0x7a2a22].map(srgb),
            sun: (0.2, 0.16),
            sun_color: srgb(0xffc9a0),
            glow: 0.3,
            disc: 0.025,
            stars: 0.6,
            clouds: Clouds { lit: srgb(0x8a3a2a), shade: srgb(0x2a1016), cover: 0.6, alpha: 0.5 },
            hills: [0x3a1418, 0x260c10, 0x18080a].map(srgb),
            copses: [0x1a0a0c, 0x220c0c, 0x140808, 0x2a100c].map(srgb),
            haze: srgb(0x7a2818),
            meadow: [0x2a1410, 0x120806].map(srgb),
            grass: [0x4a2016, 0x5e2a1a, 0x3a1810, 0x7a3a1e].map(srgb),
            flowers: [0xff7a2a, 0xffb347, 0xff4a1a, 0xffd27a].map(srgb),
        },
    }
}
