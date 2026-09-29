// Génère les images du site : le charme (squelette 3D, feuillage, ombrage, rendu peint)
// et son décor, pour chacune des huit ambiances, en AVIF dans le site.
// Usage : tree-gen RACINE_DU_SITE [DOSSIER_D_APERÇUS]
mod ambience;
mod crown;
mod export;
mod field;
mod foliage;
mod growth;
mod light;
mod math;
mod preview;
mod random;
mod render;
mod scenery;
mod skeleton;
mod species;
mod voxels;

use std::path::PathBuf;
use std::time::Instant;

use crate::render::Camera;
use crate::species::CHARME;

/// Graine retenue pour le charme du site (choisie sur planche le 2026-09-29)
const SEED: u64 = 1984;
const TREE_SIZE: (u32, u32) = (1600, 1800);
/// Taille d'un voxel de la grille d'ombrage, en mètres
const SHADE_CELL: f32 = 0.3;

fn main() {
    let mut args = std::env::args().skip(1).map(PathBuf::from);
    let site = args.next().expect("usage : tree-gen RACINE_DU_SITE [DOSSIER_D_APERÇUS]");
    let previews = args.next();
    let scene = site.join("static/img/scene");
    std::fs::create_dir_all(&scene).expect("dossier des images");
    let clock = Instant::now();

    let mut rng = random::Rng::new(SEED);
    let crown = crown::Crown::new(&CHARME, SEED as u32);
    let mut nodes = growth::grow(&CHARME, &crown, &mut rng);
    skeleton::thicken(&mut nodes, &CHARME);
    skeleton::smooth(&mut nodes, 0.02, 6);
    let leaves = foliage::sprout(&nodes, &CHARME, crown.center(), &mut rng);
    let shade = light::Shade::new(&leaves, SHADE_CELL);
    let cam = Camera::frame(&nodes, &leaves, TREE_SIZE, 100.0, 110.0);
    let lighting = render::Lighting::new(&nodes, &leaves, &shade);
    export::scss(&cam, TREE_SIZE, crown.center().y, &site.join("sass/_scene-data.scss"));
    eprintln!("{} nœuds, {} feuilles, éclairage ({:.1?})", nodes.len(), leaves.len(), clock.elapsed());

    for (theme, season, amb) in ambience::all() {
        let name = format!("{theme}-{season}");
        let tree = render::draw(&nodes, &leaves, &lighting, &amb, &cam, TREE_SIZE);
        export::avif(&export::shrink(&tree, export::TREE_SCALE), export::TREE_QUALITY, &scene.join(format!("{name}-arbre.avif")));
        for layout in &preview::LAYOUTS {
            let decor = scenery::paint(&amb.scenery, layout.width, layout.height, layout.horizon);
            export::avif(&decor, export::DECOR_QUALITY, &scene.join(format!("{name}-{}.avif", layout.name)));
            if let Some(dir) = &previews {
                std::fs::create_dir_all(dir).expect("dossier d'aperçus");
                let view = preview::compose(&decor, &tree, &cam, layout);
                view.save_png(dir.join(format!("{name}-{}.png", layout.name))).expect("écriture de l'aperçu");
            }
        }
        eprintln!("{name} ({:.1?})", clock.elapsed());
    }
}
