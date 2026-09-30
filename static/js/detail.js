// Encadré d'une étape : popover natif, ouvert par son fruit même sans ce script. Ici, la
// caméra zoome sur le fruit tant que son encadré est ouvert, et « précédent / suivant »
// passent d'un encadré à l'autre
(() => {
  const root = document.documentElement;
  const orchard = document.querySelector(".orchard");
  if (!orchard?.showPopover) return; // navigateur sans popover

  const ZOOM = 1.45;
  const clamp = (v, min, max) => Math.min(max, Math.max(min, v));
  const fruitOf = (panel) => orchard.querySelector(`[popovertarget="${panel.id}"]`);

  function rest() {
    for (const name of ["--cam-x", "--cam-y", "--cam-zoom"]) root.style.removeProperty(name);
  }

  /** Amène le fruit au centre de la zone laissée libre par l'encadré, sans découvrir le fond */
  function focus(fruit, panel) {
    // offset* : positions de mise en page, insensibles au zoom en cours
    const [ox, oy] = getComputedStyle(orchard).transformOrigin.split(" ").map(parseFloat);
    const spot = fruit.parentElement;
    const beside = panel.offsetWidth < innerWidth; // encadré à droite, sinon feuille en bas
    const taken = { x: beside ? panel.offsetWidth : 0, y: beside ? 0 : panel.offsetHeight };
    // Bord fondu de l'encadré (sa marge côté scène) : la scène doit continuer dessous
    const fade = parseFloat(getComputedStyle(panel)[beside ? "paddingLeft" : "paddingTop"]);
    // Sur un axe : pied de l'arbre (origine du zoom), fruit, taille de l'écran, place prise
    const shift = (foot, at, size, used) =>
      clamp((size - used) / 2 - foot - ZOOM * (at - foot), size - Math.max(0, used - fade) - foot - ZOOM * (size - foot), (ZOOM - 1) * foot);
    root.style.setProperty("--cam-x", `${shift(orchard.offsetLeft + ox, orchard.offsetLeft + spot.offsetLeft, innerWidth, taken.x)}px`);
    root.style.setProperty("--cam-y", `${shift(orchard.offsetTop + oy, orchard.offsetTop + spot.offsetTop, innerHeight, taken.y)}px`);
    root.style.setProperty("--cam-zoom", ZOOM);
  }

  // Ambiance ou fenêtre qui change : l'encadré se ferme et la vue revient à plat d'un coup,
  // pour que le halo (js/scene.js) et les habitants (js/life) mesurent l'arbre hors zoom
  function cut() {
    document.querySelector(".detail:popover-open")?.hidePopover();
    root.classList.add("camera-cut");
    rest();
    void orchard.offsetWidth; // applique le retour avant de rendre la transition
    root.classList.remove("camera-cut");
  }

  // « toggle » ne remonte pas : écoute en capture
  document.addEventListener("toggle", ({ target, newState }) => {
    if (!target.matches?.(".detail")) return;
    fruitOf(target).classList.toggle("is-open", newState === "open");
    if (newState === "open") focus(fruitOf(target), target);
    else if (!document.querySelector(".detail:popover-open")) rest();
  }, true);

  // Sans ce script, le navigateur empile l'encadré voisin sur le premier ; ici il le remplace
  document.addEventListener("click", (event) => {
    const step = event.target.closest(".detail__step");
    if (!step) return;
    event.preventDefault();
    const panel = step.popoverTargetElement;
    fruitOf(panel).focus({ preventScroll: true }); // Échap y ramènera le focus
    panel.showPopover();
    (panel.querySelector(`.${step.classList[1]}`) ?? panel.querySelector(".detail__step")).focus();
  });

  addEventListener("change", ({ target }) => (target.name === "theme" || target.name === "season") && cut(), true);
  addEventListener("resize", cut);
})();
