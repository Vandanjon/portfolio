// Ombre du feuillage sur lui-même : transmittance de Beer-Lambert à travers
// la grille de densité foliaire, vers le soleil ou vers le ciel
use crate::foliage::Leaf;
use crate::math::{V3, v3};
use crate::voxels::Density;

/// Coefficient d'extinction : 0,5 en théorie (feuilles orientées au hasard), forcé
/// pour que les touffes se modèlent nettement, comme sur un feuillage réel plus dense
const G: f32 = 1.3;

/// Direction vers la source de lumière (soleil ou lune), commune à toutes les ambiances :
/// l'ombrage, le plus coûteux, se calcule une seule fois pour les huit
pub fn sun_dir() -> V3 {
    v3(-0.75, 0.55, 0.35).norm()
}

pub struct Shade {
    grid: Density,
}

impl Shade {
    pub fn new(leaves: &[Leaf], cell: f32) -> Self {
        Shade { grid: Density::new(leaves, cell) }
    }

    /// Normale de la « surface » du feuillage en `p` : vers où la densité décroît
    pub fn outward(&self, p: V3) -> V3 {
        let h = self.grid.cell * 2.0;
        let axis = |d: V3| self.grid.at(p - d) - self.grid.at(p + d);
        v3(axis(v3(h, 0.0, 0.0)), axis(v3(0.0, h, 0.0)), axis(v3(0.0, 0.0, h))).norm()
    }

    /// Fraction de lumière qui atteint `p` en venant de la direction `dir`
    pub fn transmittance(&self, p: V3, dir: V3) -> f32 {
        let Density { min, max, cell, .. } = self.grid;
        let step = cell * 0.5;
        // Au-delà de cette distance, le rayon a quitté la grille quel que soit son point de départ
        let reach = (p - (min + max) * 0.5).len() + (max - min).len() * 0.5;
        let mut t = cell * 0.6;
        let mut depth = 0.0;
        while t < reach {
            depth += self.grid.at(p + dir * t) * step;
            t += step;
        }
        (-G * depth).exp()
    }

    /// Part du ciel visible depuis `p`, pondérée vers le zénith
    pub fn sky(&self, p: V3) -> f32 {
        const DIRS: [(f32, f32); 9] = [(0.0, 90.0), (0.0, 50.0), (72.0, 50.0), (144.0, 50.0), (216.0, 50.0), (288.0, 50.0), (36.0, 20.0), (156.0, 20.0), (276.0, 20.0)];
        let (mut sum, mut weight) = (0.0, 0.0);
        for (az, el) in DIRS {
            let (az, el) = (az.to_radians(), el.to_radians());
            let dir = v3(el.cos() * az.cos(), el.sin(), el.cos() * az.sin());
            let w = 0.3 + 0.7 * dir.y;
            sum += w * self.transmittance(p, dir);
            weight += w;
        }
        sum / weight
    }
}
