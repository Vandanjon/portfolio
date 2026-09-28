// Préférences d'affichage (thème, saison), posées sur <html> et mémorisées au choix.
// Chargé de façon bloquante dans <head> : l'état est posé avant le premier rendu, sans flash.
// Portée isolée : les scripts classiques partagent la portée globale de la page
(() => {
  const root = document.documentElement;

  // Saisons astronomiques, dates arrondies (MMJJ)
  const seasonOf = (date) => {
    const day = (date.getMonth() + 1) * 100 + date.getDate();
    if (day < 320 || day >= 1221) return "hiver";
    if (day < 621) return "printemps";
    if (day < 922) return "ete";
    return "automne";
  };

  // Chaque préférence correspond à un groupe de radios de même name
  const PREFS = {
    theme: {
      values: ["light", "dark"],
      initial: () => (matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light"),
    },
    season: {
      values: ["printemps", "ete", "automne", "hiver"],
      initial: () => seasonOf(new Date()),
    },
  };

  const stored = (name) => {
    try {
      return localStorage.getItem(name);
    } catch {
      return null; // stockage bloqué : on retombe sur la valeur initiale
    }
  };

  for (const [name, { values, initial }] of Object.entries(PREFS)) {
    const value = stored(name);
    root.dataset[name] = values.includes(value) ? value : initial();
  }

  addEventListener("change", ({ target }) => {
    const { name, value } = target;
    if (!Object.hasOwn(PREFS, name) || !PREFS[name].values.includes(value)) return;
    root.dataset[name] = value;
    try {
      localStorage.setItem(name, value);
    } catch {
      // stockage bloqué : le choix vaut jusqu'au rechargement
    }
  });

  addEventListener("DOMContentLoaded", () => {
    for (const name of Object.keys(PREFS)) {
      const input = document.querySelector(`input[name="${name}"][value="${root.dataset[name]}"]`);
      if (input) input.checked = true;
    }
  });
})();
