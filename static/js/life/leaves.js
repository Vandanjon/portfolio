// Feuilles d'automne qui se détachent du houppier et tombent en virevoltant jusqu'au pré
import { TAU, pick, rand } from "./util.js";

const COUNT = 14;
const COLORS = ["#9c4a1c", "#b8681e", "#7a2e14", "#c88a2e"];

export function create(stage) {
  const born = (leaf, anywhere) => {
    const c = stage.crown;
    Object.assign(leaf, {
      x: rand(c.left + 40 * stage.unit, c.right - 40 * stage.unit),
      y: anywhere ? rand(c.top, stage.ground) : rand(c.top + (c.bottom - c.top) * 0.3, c.bottom),
      fall: rand(22, 40),
      sway: rand(14, 28),
      spin: rand(1.5, 3.5),
      seed: rand(0, TAU),
      color: pick(COLORS),
      clock: 0,
    });
    return leaf;
  };
  const leaves = Array.from({ length: COUNT }, () => born({}, true));

  return {
    update(dt) {
      for (const leaf of leaves) {
        leaf.clock += dt;
        leaf.y += leaf.fall * stage.unit * dt;
        leaf.x += Math.cos(leaf.clock * 1.6 + leaf.seed) * leaf.sway * stage.unit * dt;
        if (leaf.y > stage.ground + 20 * stage.unit) born(leaf, false);
      }
    },
    draw(ctx) {
      const u = stage.unit * 1.6;
      for (const leaf of leaves) {
        // Proche du sol, la feuille s'estompe dans l'herbe
        ctx.globalAlpha = Math.min(1, Math.max(0, (stage.ground + 20 * u - leaf.y) / (40 * u)));
        ctx.save();
        ctx.translate(leaf.x, leaf.y);
        ctx.rotate(leaf.clock * leaf.spin + leaf.seed);
        ctx.scale(1, Math.cos(leaf.clock * leaf.spin * 1.3));
        ctx.fillStyle = leaf.color;
        ctx.beginPath();
        ctx.ellipse(0, 0, 5 * u, 2.4 * u, 0, 0, TAU);
        ctx.fill();
        ctx.restore();
      }
      ctx.globalAlpha = 1;
    },
  };
}
