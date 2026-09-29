// Habitants de la scène, selon l'ambiance : chaque rôle est un module chargé à la demande.
// Démarre une fois la page chargée (sans peser sur son affichage), jamais si l'utilisateur
// refuse les animations, et se met en pause au bouton (WCAG 2.2.2).
import { CAST } from "./cast.js";
import { Stage } from "./stage.js";

const PAUSE_KEY = "motion";
const root = document.documentElement;
const canvas = document.querySelector(".life");
const toggle = document.querySelector(".motion-toggle");
const marks = JSON.parse(document.getElementById("scene-marks").textContent);

let stage;
let actors = [];
let frame = 0;
let last = 0;

const stored = () => {
  try {
    return localStorage.getItem(PAUSE_KEY) === "paused";
  } catch {
    return false;
  }
};

function tick(now) {
  const dt = Math.min(0.05, (now - last) / 1000 || 0);
  last = now;
  stage.ctx.clearRect(0, 0, stage.w, stage.h);
  for (const actor of actors) {
    actor.update(dt);
    actor.draw(stage.ctx);
  }
  frame = requestAnimationFrame(tick);
}

function play(on) {
  cancelAnimationFrame(frame);
  toggle.setAttribute("aria-pressed", String(!on));
  if (on) frame = requestAnimationFrame((t) => ((last = t), tick(t)));
}

async function cast() {
  const roles = CAST[`${root.dataset.theme}-${root.dataset.season}`] ?? [];
  const modules = await Promise.all(roles.map(([name]) => import(`./${name}.js`)));
  actors = modules.map((module, i) => module.create(stage, roles[i][1] ?? {}));
  canvas.classList.remove("is-leaving");
}

function start() {
  stage = new Stage(canvas, document.querySelector(".scene__tree"), marks);
  toggle.hidden = false;
  cast().then(() => play(!stored()));
}

toggle.addEventListener("click", () => {
  const paused = toggle.getAttribute("aria-pressed") !== "true";
  play(!paused);
  try {
    localStorage.setItem(PAUSE_KEY, paused ? "paused" : "playing");
  } catch {
    // stockage bloqué : le choix vaut jusqu'au rechargement
  }
});

// Nouvelle ambiance : les habitants s'effacent pendant le halo, puis la nouvelle troupe arrive
addEventListener("change", ({ target }) => {
  if (!stage || (target.name !== "theme" && target.name !== "season")) return;
  canvas.classList.add("is-leaving");
  setTimeout(cast, 1200);
});

if (!matchMedia("(prefers-reduced-motion: reduce)").matches) {
  const idle = window.requestIdleCallback ?? ((fn) => setTimeout(fn, 200));
  if (document.readyState === "complete") idle(start);
  else addEventListener("load", () => idle(start), { once: true });
}
