// Export pour le site : images AVIF (légères, transparentes pour l'arbre) et partiel Sass
// des repères de l'image de l'arbre, pour que la mise en page suive l'image générée
use std::fmt::Write;
use std::path::Path;

use ravif::{Encoder, Img, RGBA8};
use tiny_skia::{FilterQuality, Pixmap, PixmapPaint, Transform};

use crate::render::Camera;

/// Réduction de l'arbre à l'export, et qualités AVIF de l'arbre et des décors
pub const TREE_SCALE: f32 = 0.75;
pub const TREE_QUALITY: f32 = 52.0;
pub const DECOR_QUALITY: f32 = 60.0;
/// Effort d'encodage (1 lent et compact, 10 rapide)
const SPEED: u8 = 6;

/// Réduction bicubique : rendre grand puis réduire lisse les bords des feuilles
pub fn shrink(pix: &Pixmap, factor: f32) -> Pixmap {
    let (w, h) = ((pix.width() as f32 * factor).round() as u32, (pix.height() as f32 * factor).round() as u32);
    let mut out = Pixmap::new(w, h).expect("dimensions réduites valides");
    let paint = PixmapPaint { quality: FilterQuality::Bicubic, ..PixmapPaint::default() };
    out.draw_pixmap(0, 0, pix.as_ref(), &paint, Transform::from_scale(factor, factor), None);
    out
}

/// Encode en AVIF ; `quality` : qualité perçue, de 1 à 100
pub fn avif(pix: &Pixmap, quality: f32, path: &Path) {
    let rgba: Vec<RGBA8> = pix
        .pixels()
        .iter()
        .map(|p| {
            let c = p.demultiply();
            RGBA8::new(c.red(), c.green(), c.blue(), c.alpha())
        })
        .collect();
    let img = Img::new(rgba.as_slice(), pix.width() as usize, pix.height() as usize);
    let encoded = Encoder::new().with_quality(quality).with_alpha_quality(quality).with_speed(SPEED).encode_rgba(img).expect("encodage AVIF");
    std::fs::write(path, encoded.avif_file).expect("écriture de l'AVIF");
}

/// Dimensions de l'image de l'arbre, pied du tronc, cœur du houppier (d'où part le halo) et
/// emplacements des fruits, en fractions de cette image
pub fn scss(cam: &Camera, (width, height): (u32, u32), crown_center_y: f32, fruits: &[(f32, f32)], path: &Path) {
    let (w, h) = (width as f32, height as f32);
    let exported = |v: f32| (v * TREE_SCALE).round();
    let mut out = String::from("// Généré par tools/tree-gen : ne pas modifier à la main\n");
    let values = [
        ("tree-width", exported(w)),
        ("tree-height", exported(h)),
        ("tree-foot-x", cam.origin_x / w),
        ("tree-foot-y", cam.ground_y / h),
        ("tree-heart-y", (cam.ground_y - crown_center_y * cam.scale) / h),
    ];
    for (name, value) in values {
        writeln!(out, "${name}: {};", (value * 10_000.0).round() / 10_000.0).expect("écriture en mémoire");
    }
    // Virgule finale : une liste d'un seul fruit reste une liste de paires
    let spots: String = fruits.iter().map(|(x, y)| format!("{x} {y}, ")).collect();
    writeln!(out, "$fruit-spots: ({spots});").expect("écriture en mémoire");
    std::fs::write(path, out).expect("écriture du partiel Sass");
}
