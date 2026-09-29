// Hiboux de l'automne sombre, perchés sur les branches les plus dégagées : ils clignent
// des yeux (qui luisent dans la nuit) et tournent la tête de temps en temps
import { rand } from "./util.js";

const PERCHES = [0, 2];
const SIZE = 2.2;

function drawOwl(ctx, x, y, u, look, blink) {
  ctx.save();
  ctx.translate(x, y);
  ctx.scale(u, u);
  ctx.fillStyle = "#4a3c30";
  ctx.beginPath();
  ctx.ellipse(0, -8, 6.5, 8.5, 0, 0, Math.PI * 2);
  ctx.fill();
  ctx.fillStyle = "#7a6650";
  ctx.beginPath();
  ctx.ellipse(0, -6, 4.2, 6, 0, 0, Math.PI * 2);
  ctx.fill();
  ctx.translate(look * 1.4, 0);
  ctx.fillStyle = "#4a3c30";
  ctx.beginPath();
  ctx.moveTo(-5.5, -19);
  ctx.lineTo(-4, -24);
  ctx.lineTo(-1.5, -19);
  ctx.moveTo(5.5, -19);
  ctx.lineTo(4, -24);
  ctx.lineTo(1.5, -19);
  ctx.arc(0, -17, 5.8, 0, Math.PI * 2);
  ctx.fill();
  ctx.fillStyle = "#8f7a5e";
  ctx.beginPath();
  ctx.ellipse(-2.3, -17, 2.4, 2.6, 0, 0, Math.PI * 2);
  ctx.ellipse(2.3, -17, 2.4, 2.6, 0, 0, Math.PI * 2);
  ctx.fill();
  ctx.fillStyle = "#ffd36a";
  for (const side of [-1, 1]) {
    ctx.beginPath();
    ctx.ellipse(side * 2.3, -17, 1.5, 1.5 * (1 - blink), 0, 0, Math.PI * 2);
    ctx.fill();
  }
  ctx.restore();
}

export function create(stage) {
  const owls = PERCHES.map((perch) => ({ perch, clock: rand(0, 20), blinkAt: rand(1, 5), look: 0, lookTo: 0 }));

  return {
    update(dt) {
      for (const o of owls) {
        o.clock += dt;
        if (o.clock > o.blinkAt + 0.18) o.blinkAt = o.clock + rand(2.5, 6);
        if (Math.random() < dt * 0.25) o.lookTo = [-1, 0, 1][Math.floor(rand(0, 3))];
        o.look += (o.lookTo - o.look) * Math.min(1, dt * 4);
      }
    },
    draw(ctx) {
      for (const o of owls) {
        const p = stage.perch(o.perch);
        const blink = o.clock > o.blinkAt ? 1 : 0;
        drawOwl(ctx, p.x, p.y, SIZE * stage.unit, o.look, blink);
      }
    },
  };
}
