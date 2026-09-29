// Halo de changement d'ambiance : quand le thème ou la saison change, l'ancienne scène
// reste au-dessus et s'efface en cercle depuis le cœur de l'arbre, une fois les nouvelles
// images prêtes. Sans animation si l'utilisateur la refuse.
(() => {
  const root = document.documentElement;
  const reduced = matchMedia("(prefers-reduced-motion: reduce)");
  const DURATION = 1800;
  const FEATHER = 0.16; // part du rayon final fondue sur le bord du disque

  let shown = { theme: root.dataset.theme, season: root.dataset.season };

  const imagesOf = (scene) =>
    [...scene.querySelectorAll(".scene__decor, .scene__tree")]
      .map((el) => getComputedStyle(el).backgroundImage.match(/url\("?(.+?)"?\)/)?.[1])
      .filter(Boolean);

  const decoded = (src) => {
    const img = new Image();
    img.src = src;
    return img.decode().catch(() => {}); // image manquante : le halo part quand même
  };

  addEventListener("change", ({ target }) => {
    // js/prefs.js, chargé avant, a déjà posé la nouvelle ambiance sur <html>
    if (target.name !== "theme" && target.name !== "season") return;
    const scene = document.querySelector(".scene:not(.scene--leaving)");
    // Halo interrompu : on repart de l'ambiance encore visible autour du disque
    const running = document.querySelector(".scene--leaving");
    const previous = running ? { ...running.dataset } : shown;
    shown = { theme: root.dataset.theme, season: root.dataset.season };
    running?.remove();
    if (!scene || reduced.matches) return;

    // Posée dans la même tâche que le changement : pas d'image intermédiaire à l'écran
    const leaving = scene.cloneNode(true);
    Object.assign(leaving.dataset, previous);
    leaving.classList.add("scene--leaving");
    scene.after(leaving);

    const tree = scene.querySelector(".scene__tree");
    const box = tree.getBoundingClientRect();
    const x = box.left + box.width / 2;
    const y = box.top + box.height * parseFloat(getComputedStyle(tree).getPropertyValue("--heart-at"));
    const reach = Math.hypot(Math.max(x, innerWidth - x), Math.max(y, innerHeight - y));
    leaving.style.setProperty("--heart-x", `${x}px`);
    leaving.style.setProperty("--heart-y", `${y}px`);
    leaving.style.setProperty("--feather", `${reach * FEATHER}px`);

    Promise.all(imagesOf(scene).map(decoded)).then(() => {
      if (!leaving.isConnected) return;
      const halo = leaving.animate([{ "--reveal": "0px" }, { "--reveal": `${reach * (1 + FEATHER)}px` }], {
        duration: DURATION,
        easing: "cubic-bezier(0.65, 0, 0.35, 1)",
        fill: "forwards",
      });
      halo.finished.then(() => leaving.remove(), () => {});
    });
  });
})();
