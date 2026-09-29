// Enveloppe du houppier et points attracteurs qui y guident la croissance
use crate::math::{V3, v3};
use crate::random::{Rng, fbm};
use crate::species::Species;

/// Nombre de touffes, et part des attracteurs semés hors touffes
const CLUMPS: usize = 42;
const DIFFUSE: f32 = 0.08;
const OVERFLOW: f32 = 1.18;

pub struct Crown {
    center: V3,
    radii: V3,
    lumps: f32,
    seed: u32,
}

impl Crown {
    pub fn new(sp: &Species, seed: u32) -> Self {
        let (rx, ry, rz) = sp.crown_radii;
        Crown { center: v3(0.0, sp.crown_center_y, 0.0), radii: v3(rx, ry, rz), lumps: sp.crown_lumps, seed }
    }

    pub fn center(&self) -> V3 {
        self.center
    }

    /// Distance normalisée au centre : < 1 dans le houppier, 1 sur son contour bosselé
    pub fn level(&self, p: V3) -> f32 {
        let q = p - self.center;
        // Ovoïde : le bas est plus court que le haut
        let ry = if q.y < 0.0 { self.radii.y * 0.82 } else { self.radii.y };
        let d = v3(q.x / self.radii.x, q.y / ry, q.z / self.radii.z);
        let bump = 1.0 + self.lumps * (fbm(d.norm() * 2.2, 3, self.seed) - 0.5) * 2.0;
        d.len() / bump
    }

    /// Attracteurs groupés en touffes (boules de 1 à 2 m) le long du contour : chaque touffe
    /// s'éclaire dessus et s'ombre dessous, ce qui donne au houppier son relief ;
    /// une part diffuse relie les touffes à l'intérieur
    pub fn attractors(&self, count: usize, rng: &mut Rng) -> Vec<V3> {
        let span = self.radii * (1.0 + self.lumps);
        let inside = |rng: &mut Rng, lo: f32, hi: f32| loop {
            let q = v3(rng.range(-1.0, 1.0), rng.range(-1.0, 1.0), rng.range(-1.0, 1.0));
            let p = self.center + v3(q.x * span.x, q.y * span.y, q.z * span.z);
            let l = self.level(p);
            if l >= lo && l < hi {
                return p;
            }
        };
        let clumps: Vec<(V3, f32)> = (0..CLUMPS).map(|_| (inside(rng, 0.7, 1.0), rng.range(1.2, 2.2))).collect();
        let mut points = Vec::with_capacity(count);
        while points.len() < count {
            let p = if rng.f() < DIFFUSE {
                inside(rng, 0.0, 1.0)
            } else {
                let (c, r) = clumps[(rng.f() * CLUMPS as f32) as usize];
                c + rng.dir() * (r * rng.f().cbrt())
            };
            // Les touffes du bord débordent un peu : le contour se bosselle
            if self.level(p) < OVERFLOW {
                points.push(p);
            }
        }
        points
    }
}
