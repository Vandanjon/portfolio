// Sombre de printemps : nuit calme sous la lune, feuillage bleu-vert argenté, ciel étoilé
use super::{Ambience, Clouds, Scenery, srgb};

pub fn ambience() -> Ambience {
    Ambience {
        leaf: [0x0c1f1e, 0x153330, 0x214a42, 0x34655a, 0x5a8a80].map(srgb),
        rim: srgb(0xcfe0ff),
        rim_strength: 0.2,
        ember: false,
        aura: None,
        bark: [0x0f1318, 0x1f252c, 0x343c46, 0x55606c].map(srgb),
        twig: srgb(0x16201e),
        scenery: Scenery {
            sky: [0x0b1330, 0x16224a, 0x243463, 0x33447a, 0x46548a].map(srgb),
            sun: (0.2, 0.16),
            sun_color: srgb(0xe8eeff),
            glow: 0.45,
            disc: 0.03,
            stars: 1.0,
            clouds: Clouds { lit: srgb(0x8a95c0), shade: srgb(0x2a3358), cover: 0.6, alpha: 0.55 },
            hills: [0x2a3862, 0x1d2a48, 0x16213a].map(srgb),
            copses: [0x132034, 0x18283e, 0x1d3047, 0x101a2c].map(srgb),
            haze: srgb(0x3a4a7a),
            meadow: [0x1f3a3a, 0x0f2226].map(srgb),
            grass: [0x2c5048, 0x3a6258, 0x23423c, 0x4a7266].map(srgb),
            flowers: [0xcfd8ff, 0xffffff, 0xb8c4f0, 0xe6ecff].map(srgb),
        },
    }
}
