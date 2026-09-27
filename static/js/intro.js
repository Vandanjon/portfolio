// Intro jouée une fois par session, interruptible par Échap ou par le bouton « Sauter l'intro ».
// Chargé de façon bloquante dans <head> : l'état est posé avant le premier rendu, sans flash.
const root = document.documentElement;
const SEEN_KEY = "intro-seen";

try {
  root.dataset.intro = sessionStorage.getItem(SEEN_KEY) ? "off" : "play";
  sessionStorage.setItem(SEEN_KEY, "1");
} catch {
  root.dataset.intro = "play"; // stockage bloqué (navigation privée stricte) : on joue l'intro
}

const skip = () => {
  root.dataset.intro = "off";
};

addEventListener("keydown", (event) => {
  if (event.key === "Escape") skip();
});

addEventListener("click", (event) => {
  if (event.target.closest(".intro-skip")) skip();
});
