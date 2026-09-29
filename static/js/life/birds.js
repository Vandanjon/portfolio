// Mésanges du printemps : elles arrivent en vol, se posent sur un perchoir, sautillent,
// se glissent dans le feuillage (l'arbre est repeint devant elles), puis repartent
import { drawTit } from "./tit.js";
import { bezier, ease, lerp, pick, rand } from "./util.js";

const COUNT = 3;
const SIZE = 2;
const NEXT = { wait: "in", in: "perch", perch: "hide", hide: "hidden", hidden: "emerge", emerge: "out", out: "wait" };
const DURATION = { in: [2.5, 3.5], perch: [2, 5], hide: [0.8, 0.8], hidden: [3, 7], emerge: [0.6, 0.6], out: [2.5, 3.5], wait: [3, 8] };

export function create(stage) {
  const img = stage.treeImage();
  const taken = new Set();
  const birds = Array.from({ length: COUNT }, (_, i) => ({ phase: "wait", t: 0, dur: 1 + i * 2.5, clock: rand(0, 9) }));
  const away = () => ({ x: Math.random() < 0.5 ? -40 : stage.w + 40, y: rand(0.05, 0.4) * stage.h });
  const seat = (b) => {
    const p = stage.perch(b.perch);
    return { x: p.x, y: p.y - 6.5 * SIZE * stage.unit };
  };
  // Cachette : un peu plus loin dans le houppier, vers son cœur
  const nook = (b) => {
    const p = seat(b);
    const h = stage.heart;
    const k = (28 * stage.unit) / (Math.hypot(h.x - p.x, h.y - p.y) || 1);
    return { x: lerp(p.x, h.x, k), y: lerp(p.y, h.y, k) };
  };
  const arc = (a, b) => ({ x: (a.x + b.x) / 2, y: Math.min(a.y, b.y) - 80 * stage.unit });

  function advance(b) {
    const free = stage.marks.perches.map((_, i) => i).filter((i) => !taken.has(i));
    let next = NEXT[b.phase];
    if (next === "in" && free.length === 0) next = "wait";
    if (next === "in") taken.add((b.perch = pick(free)));
    if (next === "in" || next === "out") b.far = away();
    if (next === "wait") taken.delete(b.perch);
    b.phase = next;
    b.t = 0;
    b.dur = rand(...DURATION[next]);
  }

  function pose(b) {
    const k = ease(Math.min(1, b.t / b.dur));
    const flap = Math.sin(b.clock * 28);
    const turn = Math.sin(b.clock * 0.9 + b.perch) > 0 ? 1 : -1;
    switch (b.phase) {
      case "in": return { ...bezier(b.far, arc(b.far, seat(b)), seat(b), k), facing: Math.sign(seat(b).x - b.far.x), flap, hidden: 0 };
      case "perch": return { ...seat(b), facing: turn, flap: null, hidden: 0 };
      case "hide": return { x: lerp(seat(b).x, nook(b).x, k), y: lerp(seat(b).y, nook(b).y, k), facing: turn, flap: null, hidden: k };
      case "emerge": return { x: lerp(nook(b).x, seat(b).x, k), y: lerp(nook(b).y, seat(b).y, k), facing: turn, flap: null, hidden: 1 - k };
      case "out": return { ...bezier(seat(b), arc(seat(b), b.far), b.far, k), facing: Math.sign(b.far.x - seat(b).x), flap, hidden: 0 };
      default: return null; // en attente, ou caché
    }
  }

  return {
    update(dt) {
      for (const b of birds) {
        b.t += dt;
        b.clock += dt;
        if (b.t >= b.dur) advance(b);
      }
    },
    draw(ctx) {
      for (const b of birds) {
        const p = pose(b);
        if (!p) continue;
        drawTit(ctx, p.x, p.y, SIZE * stage.unit, p.facing || 1, p.flap);
        stage.cover(img, p.x, p.y, 14 * SIZE * stage.unit, p.hidden);
      }
    },
  };
}
