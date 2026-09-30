// Contact : l'adresse n'est écrite en entier nulle part dans la page servie. Sans ce script,
// le bouton ouvre son encadré (popover natif), où elle se lit en toutes lettres. Avec lui,
// plus d'encadré : le clic lance directement la messagerie du visiteur
(() => {
  const opener = document.querySelector('[popovertarget="contact"]');
  const address = document.querySelector(".contact__address");
  if (!opener || !address) return;

  // Sans cible, le bouton n'est plus annoncé « réduit » par les lecteurs d'écran
  opener.removeAttribute("popovertarget");
  opener.addEventListener("click", () => {
    const { user, domain } = address.dataset;
    location.href = `mailto:${user}@${domain}?subject=${encodeURIComponent(`Contact depuis ${domain}`)}`;
  });
})();
