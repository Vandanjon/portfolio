// Clair de printemps : ciel rosé du matin, feuillage vert tendre juste sorti, pré de fleurs pâles
use super::{Ambience, Clouds, Scenery, srgb};

pub fn ambience() -> Ambience {
    Ambience {
        leaf: [0x3f6e2e, 0x5a9040, 0x7fb155, 0xa8d27a, 0xd2ebaa].map(srgb),
        rim: srgb(0xfffaf0),
        rim_strength: 0.45,
        ember: false,
        aura: None,
        bark: [0x3d3935, 0x605a54, 0x898179, 0xb4aca2].map(srgb),
        twig: srgb(0x4a5236),
        scenery: Scenery {
            sky: [0x7fb2e5, 0xa8ccef, 0xd6e6f3, 0xf5dcd6, 0xfae8dc].map(srgb),
            sun: (0.2, 0.14),
            sun_color: srgb(0xfff1d6),
            glow: 0.9,
            disc: 0.04,
            stars: 0.0,
            clouds: Clouds { lit: srgb(0xffffff), shade: srgb(0xc9d3ea), cover: 0.56, alpha: 0.8 },
            hills: [0xa9c1d9, 0x98bd95, 0x7cab6a].map(srgb),
            copses: [0x5f8f55, 0x71a15f, 0xe8b4c5, 0xf3d0da].map(srgb),
            haze: srgb(0xffeeec),
            meadow: [0x94c768, 0x5b9942].map(srgb),
            grass: [0x6fae4e, 0x94c96a, 0x5a9a41, 0xaddb7c].map(srgb),
            flowers: [0xffffff, 0xfff1a8, 0xffc2d4, 0xdccbff].map(srgb),
        },
    }
}
