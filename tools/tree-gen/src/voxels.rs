// Grille de densité foliaire : surface de feuilles par voxel (m²/m³)
use crate::foliage::Leaf;
use crate::math::{V3, v3};

pub struct Density {
    pub min: V3,
    pub max: V3,
    pub cell: f32,
    dims: [usize; 3],
    values: Vec<f32>,
}

impl Density {
    pub fn new(leaves: &[Leaf], cell: f32) -> Self {
        let lo = leaves.iter().fold(v3(f32::MAX, f32::MAX, f32::MAX), |a, l| a.min(l.base));
        let hi = leaves.iter().fold(v3(f32::MIN, f32::MIN, f32::MIN), |a, l| a.max(l.base));
        let min = lo - v3(cell, cell, cell);
        let size = hi - min;
        let dims = [(size.x / cell) as usize + 2, (size.y / cell) as usize + 2, (size.z / cell) as usize + 2];
        let max = min + v3(dims[0] as f32, dims[1] as f32, dims[2] as f32) * cell;
        let mut grid = Density { min, max, cell, dims, values: vec![0.0; dims[0] * dims[1] * dims[2]] };
        let volume = cell * cell * cell;
        for leaf in leaves {
            if let Some(i) = grid.index(leaf.base) {
                grid.values[i] += leaf.area() / volume;
            }
        }
        grid
    }

    fn index(&self, p: V3) -> Option<usize> {
        let q = (p - self.min) * (1.0 / self.cell);
        let (x, y, z) = (q.x.floor(), q.y.floor(), q.z.floor());
        let [nx, ny, nz] = self.dims;
        let inside = x >= 0.0 && y >= 0.0 && z >= 0.0 && (x as usize) < nx && (y as usize) < ny && (z as usize) < nz;
        inside.then(|| (z as usize * ny + y as usize) * nx + x as usize)
    }

    /// Densité en `p`, nulle hors de la grille
    pub fn at(&self, p: V3) -> f32 {
        self.index(p).map_or(0.0, |i| self.values[i])
    }
}
