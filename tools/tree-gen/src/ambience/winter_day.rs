// Clair d'hiver : ciel bleuté, charme gardant ses feuilles sèches rousses, pré enneigé
use super::{Ambience, Clouds, Scenery, srgb};

pub fn ambience() -> Ambience {
    Ambience {
        leaf: [0x5a3a22, 0x7d5231, 0xa06c40, 0xbf8a58, 0xdcb488].map(srgb),
        rim: srgb(0xf4f7ff),
        rim_strength: 0.5,
        ember: false,
        aura: None,
        bark: [0x363a40, 0x585d64, 0x80868d, 0xaeb4ba].map(srgb),
        twig: srgb(0x4a4038),
        scenery: Scenery {
            sky: [0x8fb0d8, 0xb2c8e4, 0xd2def0, 0xe6edf6, 0xf2f5fa].map(srgb),
            sun: (0.2, 0.22),
            sun_color: srgb(0xfdfdf4),
            glow: 0.8,
            disc: 0.04,
            stars: 0.0,
            clouds: Clouds { lit: srgb(0xffffff), shade: srgb(0xb9c6d8), cover: 0.5, alpha: 0.85 },
            hills: [0xbccadc, 0xaebdcd, 0x9fb0c2].map(srgb),
            copses: [0x5d6f78, 0x6b7c84, 0x4f5f68, 0x7d8b90].map(srgb),
            haze: srgb(0xeef2f8),
            meadow: [0xeef3f9, 0xc3d0de].map(srgb),
            grass: [0xb8a98c, 0xa09177, 0xcfc2a6, 0x8e8068].map(srgb),
            flowers: [0xffffff, 0xf4f8ff, 0xe6eefa, 0xffffff].map(srgb),
        },
    }
}
