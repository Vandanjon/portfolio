// Sombre d'automne : nuit tiède sous la lune rousse, feuillage cuivré, feuilles tombées dans le pré
use super::{Ambience, Clouds, Scenery, srgb};

pub fn ambience() -> Ambience {
    Ambience {
        leaf: [0x1e0e08, 0x3e1e0e, 0x6e3a16, 0xa4622a, 0xd49a52].map(srgb),
        rim: srgb(0xffe7c4),
        rim_strength: 0.2,
        ember: false,
        aura: None,
        bark: [0x120e0e, 0x241c1a, 0x3a302c, 0x5a4a42].map(srgb),
        twig: srgb(0x1e140e),
        scenery: Scenery {
            sky: [0x10142a, 0x1d1f3a, 0x2f2a48, 0x473757, 0x5e4560].map(srgb),
            sun: (0.2, 0.16),
            sun_color: srgb(0xffe7c4),
            glow: 0.45,
            disc: 0.035,
            stars: 0.8,
            clouds: Clouds { lit: srgb(0x8c7a9a), shade: srgb(0x2c2640), cover: 0.58, alpha: 0.6 },
            hills: [0x3a2e46, 0x2a2034, 0x1e1726].map(srgb),
            copses: [0x2a1a1a, 0x3a2218, 0x20140f, 0x4a2c1a].map(srgb),
            haze: srgb(0x5a4462),
            meadow: [0x3a2a1c, 0x1c140e].map(srgb),
            grass: [0x5a4428, 0x6e5432, 0x4a3620, 0x826440].map(srgb),
            flowers: [0x9c4a1c, 0xb8681e, 0x7a2e14, 0xc88a2e].map(srgb),
        },
    }
}
