// Papillons d'été : ils errent autour du houppier et au-dessus du pré, d'un point à l'autre,
// en battant des ailes et en dansant de haut en bas
import { TAU, pick, rand } from "./util.js";

const COUNT = 5;
const COLORS = ["#ff9f1c", "#ffe066", "#ffffff", "#f7b2d9"];
const SPEED = 55; // unités par seconde
const SIZE = 1.8;

export function create(stage) {
  // Zone d'errance : le houppier élargi, jusqu'au pré sous l'arbre
  const target = () => {
    const c = stage.crown;
    return { x: rand(c.left - 120 * stage.unit, c.right + 160 * stage.unit), y: rand(c.top + 100 * stage.unit, stage.ground) };
  };
  const flock = Array.from({ length: COUNT }, () => ({ ...target(), goal: target(), color: pick(COLORS), clock: rand(0, 9), bob: rand(0, TAU) }));

  function drawOne(ctx, b) {
    const u = stage.unit * SIZE;
    const open = 0.25 + 0.75 * Math.abs(Math.sin(b.clock * 16));
    const y = b.y + Math.sin(b.clock * 3 + b.bob) * 8 * u;
    ctx.save();
    ctx.translate(b.x, y);
    ctx.rotate(Math.sin(b.clock * 2 + b.bob) * 0.25);
    ctx.fillStyle = b.color;
    for (const side of [-1, 1]) {
      ctx.beginPath();
      ctx.ellipse(side * 3.6 * u * open, -2 * u, 3.8 * u * open, 3.2 * u, side * 0.5, 0, TAU);
      ctx.ellipse(side * 2.6 * u * open, 2.6 * u, 2.6 * u * open, 2.2 * u, side * -0.4, 0, TAU);
      ctx.fill();
    }
    ctx.fillStyle = "#3a2e26";
    ctx.fillRect(-0.6 * u, -3.4 * u, 1.2 * u, 7 * u);
    ctx.restore();
  }

  return {
    update(dt) {
      for (const b of flock) {
        b.clock += dt;
        const dx = b.goal.x - b.x;
        const dy = b.goal.y - b.y;
        const d = Math.hypot(dx, dy);
        if (d < 10 * stage.unit) b.goal = target();
        const step = Math.min(d, SPEED * stage.unit * dt);
        b.x += (dx / (d || 1)) * step;
        b.y += (dy / (d || 1)) * step;
      }
    },
    draw(ctx) {
      for (const b of flock) drawOne(ctx, b);
    },
  };
}
