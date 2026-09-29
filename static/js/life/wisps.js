// Feux follets de l'été sombre : des lueurs dansent en orbite autour du houppier en flammes,
// plus pâles quand elles passent derrière l'arbre
import { TAU, flicker, glow, rand } from "./util.js";

const COUNT = 6;
const COLOR = "#ffcf6a";

export function create(stage) {
  const wisps = Array.from({ length: COUNT }, () => ({
    angle: rand(0, TAU),
    speed: rand(0.25, 0.5) * (Math.random() < 0.5 ? -1 : 1),
    spread: rand(0.75, 1.1),
    lift: rand(-0.3, 0.3),
    seed: rand(0, 50),
    clock: 0,
  }));

  return {
    update(dt) {
      for (const w of wisps) {
        w.angle += w.speed * dt;
        w.clock += dt;
      }
    },
    draw(ctx) {
      const c = stage.crown;
      const h = stage.heart;
      const rx = ((c.right - c.left) / 2) * 1.05;
      const ry = (c.bottom - c.top) / 2;
      ctx.globalCompositeOperation = "lighter";
      for (const w of wisps) {
        const x = h.x + Math.cos(w.angle) * rx * w.spread;
        const y = h.y + (w.lift + Math.sin(w.angle * 2 + w.seed) * 0.35) * ry + Math.sin(w.clock * 1.7 + w.seed) * 12 * stage.unit;
        const behind = Math.sin(w.angle) < 0 ? 0.4 : 1;
        glow(ctx, x, y, 26 * stage.unit, COLOR, behind * (0.55 + 0.45 * flicker(w.clock, w.seed)));
      }
      ctx.globalCompositeOperation = "source-over";
    },
  };
}
