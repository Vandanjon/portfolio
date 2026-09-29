// Rafales d'hiver : de temps en temps, un coup de vent soulève la poudreuse du pré,
// trace quelques filets d'air et fait ployer l'arbre un instant
import { rand } from "./util.js";

const GAP = [6, 12]; // secondes entre deux rafales
const LENGTH = 2.2; // durée d'une rafale, en secondes
const FLAKES = 140;

export function create(stage) {
  let wait = rand(2, 4);
  let gust = null;

  function blow() {
    const dir = Math.random() < 0.5 ? -1 : 1;
    const from = dir > 0 ? -20 : stage.w + 20;
    const top = stage.crown.top;
    gust = {
      t: 0,
      flakes: Array.from({ length: FLAKES }, () => ({ x: from - dir * rand(0, stage.w * 0.4), y: rand(top, stage.h), v: rand(0.6, 1.2), r: rand(1.5, 3.5), phase: rand(0, 6) })),
      streaks: Array.from({ length: 5 }, () => ({ x: from - dir * rand(0, stage.w * 0.3), y: rand(stage.crown.top, stage.h), len: rand(80, 180) })),
      dir,
    };
    stage.tree.animate([{ rotate: "0deg" }, { rotate: `${dir * 1.2}deg` }, { rotate: `${-dir * 0.3}deg` }, { rotate: "0deg" }], { duration: LENGTH * 1000, easing: "ease-in-out" });
  }

  return {
    update(dt) {
      if (!gust && (wait -= dt) <= 0) blow();
      if (!gust) return;
      gust.t += dt;
      const speed = stage.w * 0.55 * dt * gust.dir;
      for (const f of gust.flakes) f.x += speed * f.v;
      for (const s of gust.streaks) s.x += speed * 1.3;
      if (gust.t > LENGTH) {
        gust = null;
        wait = rand(...GAP);
      }
    },
    draw(ctx) {
      if (!gust) return;
      const fade = Math.sin(Math.min(1, gust.t / LENGTH) * Math.PI);
      // Flocons cernés de bleu-gris : lisibles sur le ciel comme sur la neige du pré
      ctx.fillStyle = "#ffffff";
      ctx.strokeStyle = "#7d8fa8";
      ctx.lineWidth = 0.6;
      for (const f of gust.flakes) {
        ctx.globalAlpha = fade * 0.9;
        ctx.beginPath();
        ctx.arc(f.x, f.y + Math.sin(gust.t * 6 + f.phase) * 6, f.r * stage.unit, 0, Math.PI * 2);
        ctx.fill();
        ctx.stroke();
      }
      ctx.strokeStyle = "#ffffff";
      ctx.lineWidth = 1.8;
      for (const s of gust.streaks) {
        ctx.globalAlpha = fade * 0.6;
        ctx.beginPath();
        ctx.moveTo(s.x, s.y);
        ctx.quadraticCurveTo(s.x - gust.dir * s.len * 0.5, s.y - 10, s.x - gust.dir * s.len, s.y + 4);
        ctx.stroke();
      }
      ctx.globalAlpha = 1;
    },
  };
}
