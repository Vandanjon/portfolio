// Fruits du parcours : à l'arrivée, ils s'allument un à un dans l'ordre du parcours, après
// l'intro ; au clic, le fruit rebondit et une onde part de lui. Rien si l'utilisateur
// refuse les animations
(() => {
  const root = document.documentElement;
  const reduced = matchMedia("(prefers-reduced-motion: reduce)");
  const orchard = document.querySelector(".orchard");
  const EASE = "cubic-bezier(0.16, 0.84, 0.24, 1)";
  const STAGGER = 320; // ms entre deux fruits à l'arrivée

  const wave = (fruit, delay = 0) =>
    fruit.animate([{ opacity: 0.9, scale: 1 }, { opacity: 0, scale: 3.6 }], { duration: 900, delay, easing: EASE, pseudoElement: "::after" });

  // Chaque partie part de rien et rejoint son état au repos (fin implicite = style courant)
  function light(fruit, i) {
    const delay = i * STAGGER;
    const from = (duration) => ({ duration, delay, easing: EASE, fill: "backwards" });
    return [
      fruit.querySelector(".fruit__body").animate([{ scale: 0 }, { scale: 1.4, offset: 0.5 }], from(700)),
      fruit.querySelector(".fruit__ring").animate([{ opacity: 0, scale: 0.4 }], from(900)),
      fruit.querySelector(".fruit__label").animate([{ opacity: 0 }], from(600)),
      fruit.animate([{ opacity: 0 }, { opacity: 1, offset: 0.3 }], { ...from(1400), pseudoElement: "::before" }),
      wave(fruit, delay),
    ];
  }

  // Fin de l'intro : son animation se termine, ou elle est sautée (js/intro.js)
  const introDone = new Promise((resolve) => {
    if (root.dataset.intro !== "play") return resolve();
    document.querySelector(".intro")?.addEventListener("animationend", ({ animationName }) => animationName === "intro-end" && resolve());
    new MutationObserver(resolve).observe(root, { attributeFilter: ["data-intro"] });
  });

  if (!orchard || reduced.matches) return;

  // En attente, chaque fruit est figé sur sa première image : éteint
  const entrance = [...orchard.querySelectorAll(".fruit")].flatMap(light);
  entrance.forEach((animation) => animation.pause());
  introDone.then(() => entrance.forEach((animation) => animation.play()));

  orchard.addEventListener("click", ({ target }) => {
    const fruit = target.closest(".fruit");
    if (!fruit) return;
    fruit.querySelector(".fruit__body").animate([{}, { scale: 0.85 }, { scale: 1.3 }, {}], 450);
    wave(fruit);
  });
})();
