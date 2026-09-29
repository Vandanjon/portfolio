// Braises qui montent du feuillage en flammes (rouges l'été, bleues l'hiver) :
// elles naissent dans le houppier, s'élèvent en ondulant, scintillent et s'éteignent
import { flicker, pick, rand } from "./util.js";

const RATE = 16; // naissances par seconde
const MAX = 70;

export function create(stage, { colors }) {
  const sparks = [];
  let due = 0;

  function spawn() {
    const c = stage.crown;
    const mid = (c.left + c.right) / 2;
    const half = (c.right - c.left) / 2;
    sparks.push({
      x: mid + (Math.random() + Math.random() - 1) * half * 0.9,
      // Haut du houppier : elles s'en échappent vite et se voient sur le ciel
      y: rand(c.top, c.top + (c.bottom - c.top) * 0.55),
      rise: rand(50, 110),
      life: rand(1.5, 3.2),
      age: 0,
      size: rand(1.2, 2.6),
      seed: rand(0, 50),
      color: pick(colors),
    });
  }

  return {
    update(dt) {
      due += RATE * dt;
      for (; due >= 1 && sparks.length < MAX; due--) spawn();
      due = Math.min(due, 1);
      for (const s of sparks) {
        s.age += dt;
        s.y -= s.rise * stage.unit * dt;
        s.x += Math.sin(s.age * 2.4 + s.seed) * 14 * stage.unit * dt;
      }
      for (let i = sparks.length - 1; i >= 0; i--) if (sparks[i].age > sparks[i].life) sparks.splice(i, 1);
    },
    draw(ctx) {
      ctx.globalCompositeOperation = "lighter";
      for (const s of sparks) {
        const k = s.age / s.life;
        ctx.globalAlpha = Math.sin(k * Math.PI) * (0.6 + 0.4 * flicker(s.age, s.seed));
        ctx.fillStyle = s.color;
        ctx.beginPath();
        ctx.arc(s.x, s.y, s.size * stage.unit * (1 - k * 0.5), 0, Math.PI * 2);
        ctx.fill();
      }
      ctx.globalAlpha = 1;
      ctx.globalCompositeOperation = "source-over";
    },
  };
}
