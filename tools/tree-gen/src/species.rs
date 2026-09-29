// Paramètres d'essence, en mètres : tout ce qui fait qu'un charme ressemble à un charme
pub struct Species {
    /// Hauteur du fût nu, avant la couronne
    pub trunk_height: f32,
    /// Centre et demi-axes de l'ovoïde du houppier
    pub crown_center_y: f32,
    pub crown_radii: (f32, f32, f32),
    /// Irrégularité du contour (bosses et creux du houppier)
    pub crown_lumps: f32,
    /// Points attracteurs : nombre, portée d'attraction, distance d'absorption
    pub attractors: usize,
    pub influence: f32,
    pub kill: f32,
    /// Longueur d'un pas de croissance
    pub step: f32,
    /// Branches ascendantes (> 0) et rameaux en zigzag
    pub tropism: f32,
    pub zigzag: f32,
    /// Modèle des tuyaux : r_parent^e = Σ r_enfant^e
    pub twig_radius: f32,
    pub pipe_exponent: f32,
    /// Frondes feuillées portées par les rameaux plus fins que ce rayon
    pub leafy_radius: f32,
    pub spray_length: (f32, f32),
    pub spray_leaves: (usize, usize),
    pub leaf_length: f32,
}

pub const CHARME: Species = Species {
    trunk_height: 1.8,
    crown_center_y: 8.2,
    crown_radii: (5.4, 6.6, 5.0),
    crown_lumps: 0.22,
    attractors: 30000,
    influence: 0.9,
    kill: 0.22,
    step: 0.1,
    tropism: 0.18,
    zigzag: 0.25,
    twig_radius: 0.004,
    pipe_exponent: 2.15,
    leafy_radius: 0.012,
    spray_length: (0.18, 0.34),
    spray_leaves: (4, 8),
    leaf_length: 0.13,
};
