// Clair d'été : ciel bleu franc, soleil haut, feuillage vert tendre, pré fleuri
use super::{Ambience, Clouds, Scenery, srgb};

pub fn ambience() -> Ambience {
    Ambience {
        leaf: [0x244f22, 0x35692c, 0x4e8a37, 0x6fa945, 0x9cc95d].map(srgb),
        rim: srgb(0xfff8d8),
        rim_strength: 0.4,
        ember: false,
        aura: None,
        bark: [0x3b3733, 0x5d5852, 0x857e76, 0xb0a89e].map(srgb),
        twig: srgb(0x3c4230),
        scenery: Scenery {
            sky: [0x5796d8, 0x79b0e3, 0xa9cdec, 0xd9e8f1, 0xf1efe2].map(srgb),
            sun: (0.2, 0.14),
            sun_color: srgb(0xfff6dc),
            glow: 1.0,
            disc: 0.045,
            stars: 0.0,
            clouds: Clouds { lit: srgb(0xffffff), shade: srgb(0xb4c6de), cover: 0.55, alpha: 0.94 },
            hills: [0x91b1cf, 0x709d6b, 0x5a8a50].map(srgb),
            copses: [0x3f6e3a, 0x4c7d40, 0x5d8e47, 0x6f9d4d].map(srgb),
            haze: srgb(0xf6f4e6),
            meadow: [0xa9c65c, 0x5f8a33].map(srgb),
            grass: [0x8db64a, 0xb3cf62, 0x6e9a38, 0xc9d97a].map(srgb),
            flowers: [0xffe14d, 0xffffff, 0xe8452f, 0xffb13b].map(srgb),
        },
    }
}
