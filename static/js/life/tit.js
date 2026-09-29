// Mésange bleue de profil, bec vers la droite (facing = -1 la retourne) : dos vert-de-gris,
// ailes et calotte bleues, ventre jaune, joue blanche ; tailles en unités de la scène
const ellipse = (ctx, x, y, rx, ry, rot = 0) => {
  ctx.beginPath();
  ctx.ellipse(x, y, rx, ry, rot, 0, Math.PI * 2);
  ctx.fill();
};

function wing(ctx, flap) {
  ctx.fillStyle = "#4f82c6";
  if (flap === null) {
    ellipse(ctx, -1.5, -0.8, 4.6, 2.6, -0.2);
    return;
  }
  // En vol : l'aile bat de haut (flap = 1) en bas (flap = -1)
  ctx.beginPath();
  ctx.moveTo(-3, -1);
  ctx.quadraticCurveTo(-4, -1 - 9 * flap, -7, -1 - 8 * flap);
  ctx.lineTo(2, -1);
  ctx.fill();
}

/** `flap` : battement en vol dans [-1, 1], ou null quand l'oiseau est posé */
export function drawTit(ctx, x, y, unit, facing, flap) {
  ctx.save();
  ctx.translate(x, y);
  ctx.scale(facing * unit, unit);
  ctx.fillStyle = "#3d5f8f";
  ctx.beginPath();
  ctx.moveTo(-5, -1);
  ctx.lineTo(-12, 1.5);
  ctx.lineTo(-11, 3.5);
  ctx.lineTo(-4, 1.8);
  ctx.fill();
  ctx.fillStyle = "#7f9562";
  ellipse(ctx, 0, 0, 6.5, 4.6);
  ctx.fillStyle = "#f2d54a";
  ellipse(ctx, 1.2, 1.6, 5, 3.1);
  wing(ctx, flap);
  ctx.fillStyle = "#f4f2ea";
  ellipse(ctx, 5, -3.6, 3.6, 3.6);
  ctx.fillStyle = "#3f78c8";
  ctx.beginPath();
  ctx.arc(5, -3.6, 3.6, Math.PI * 1.05, Math.PI * 1.95);
  ctx.fill();
  ctx.strokeStyle = "#1f2a3a";
  ctx.lineWidth = 0.8;
  ctx.beginPath();
  ctx.moveTo(2.6, -3.5);
  ctx.lineTo(8.2, -3.1);
  ctx.stroke();
  ctx.fillStyle = "#3a3330";
  ctx.beginPath();
  ctx.moveTo(8.4, -3.8);
  ctx.lineTo(10.4, -3.2);
  ctx.lineTo(8.4, -2.6);
  ctx.fill();
  if (flap === null) {
    ctx.strokeStyle = "#5a5048";
    ctx.beginPath();
    ctx.moveTo(-0.5, 4);
    ctx.lineTo(-1, 6.5);
    ctx.moveTo(1.8, 4);
    ctx.lineTo(2, 6.5);
    ctx.stroke();
  }
  ctx.restore();
}
