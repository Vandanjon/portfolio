// Qui habite quelle ambiance : [module, options] par « thème-saison »
// (clair : oiseaux, papillons, oies, rafales ; sombre : nuit calme, feux follets et braises,
// feuilles qui tombent et hiboux, braises bleues)
export const CAST = {
  "light-printemps": [["birds"]],
  "light-ete": [["butterflies"]],
  "light-automne": [["geese"]],
  "light-hiver": [["gusts"]],
  "dark-printemps": [],
  "dark-ete": [["wisps"], ["embers", { colors: ["#ff5a1e", "#ff8a2a", "#ffc24a"] }]],
  "dark-automne": [["leaves"], ["owls"]],
  "dark-hiver": [["embers", { colors: ["#3a90f0", "#6ab8ff", "#bfe4ff"] }]],
};
