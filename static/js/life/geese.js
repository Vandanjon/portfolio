// Vol d'oies d'automne : de temps en temps, un V traverse le ciel en battant lentement des ailes
import { rand } from "./util.js";

const SPEED = 70; // unités par seconde
const GAP = [12, 28]; // secondes entre deux vols
const COLOR = "#3d3530";

function drawGoose(ctx, x, y, u, dir, flap) {
  ctx.save();
  ctx.translate(x, y);
  ctx.scale(dir * u, u);
  ctx.strokeStyle = COLOR;
  ctx.fillStyle = COLOR;
  ctx.lineWidth = 1.6;
  ctx.lineCap = "round";
  ctx.beginPath();
  ctx.ellipse(0, 0, 7, 2.4, 0, 0, Math.PI * 2);
  ctx.fill();
  ctx.beginPath();
  ctx.moveTo(6, -0.5);
  ctx.lineTo(12, -1.5);
  ctx.stroke();
  ctx.beginPath();
  ctx.arc(12.8, -1.7, 1.4, 0, Math.PI * 2);
  ctx.fill();
  ctx.beginPath();
  ctx.moveTo(-2, 0);
  ctx.quadraticCurveTo(-4, -9 * flap, -9, -12 * flap);
  ctx.moveTo(1, 0);
  ctx.quadraticCurveTo(-1, -7 * flap, -5, -10 * flap);
  ctx.stroke();
  ctx.restore();
}

export function create(stage) {
  let flight = null;
  let wait = rand(2, 6);

  function launch() {
    const dir = Math.random() < 0.5 ? -1 : 1;
    const count = 5 + Math.floor(rand(0, 5));
    flight = { dir, x: dir < 0 ? stage.w + 40 : -40, y: rand(0.16, 0.34) * stage.h, count, clock: 0 };
  }

  return {
    update(dt) {
      if (!flight && (wait -= dt) <= 0) launch();
      if (!flight) return;
      flight.clock += dt;
      flight.x += flight.dir * SPEED * stage.unit * dt;
      const tail = 26 * flight.count * stage.unit;
      if (flight.x < -tail - 60 || flight.x > stage.w + tail + 60) {
        flight = null;
        wait = rand(...GAP);
      }
    },
    draw(ctx) {
      if (!flight) return;
      const u = stage.unit;
      for (let i = 0; i < flight.count; i++) {
        // En V : les suivantes s'échelonnent en arrière, une fois à gauche, une fois à droite
        const rank = Math.ceil(i / 2);
        const side = i % 2 ? 1 : -1;
        const x = flight.x - flight.dir * rank * 26 * u;
        const y = flight.y + side * rank * 15 * u;
        drawGoose(ctx, x, y, 2 * u, flight.dir, Math.sin(flight.clock * 4.5 + i * 0.6));
      }
    },
  };
}
