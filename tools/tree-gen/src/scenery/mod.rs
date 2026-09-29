// Décor derrière l'arbre, peint dans la palette de l'ambiance : ciel, soleil, nuages,
// collines et bosquets noyés de brume, puis pré. Toutes les tailles suivent la hauteur
// de l'image, pour qu'un même décor se décline en paysage comme en portrait.
mod clouds;
mod copses;
mod hills;
mod meadow;
mod sky;

use tiny_skia::Pixmap;

use crate::ambience::Scenery;

/// Peint le décor ; `horizon` est la hauteur de la ligne d'horizon, en fraction de l'image
pub fn paint(s: &Scenery, width: u32, height: u32, horizon: f32) -> Pixmap {
    let mut pix = Pixmap::new(width, height).expect("dimensions du décor valides");
    let y = horizon * height as f32;
    sky::paint(&mut pix, s, y);
    clouds::paint(&mut pix, s, y);
    let meadow_top = hills::paint(&mut pix, s, y);
    meadow::paint(&mut pix, s, meadow_top);
    pix
}
