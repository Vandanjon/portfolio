// Canvas plein écran au-dessus du décor, calé sur la boîte de l'image de l'arbre :
// les repères générés (fractions de l'image) deviennent des points à l'écran
export class Stage {
  constructor(canvas, tree, marks) {
    this.canvas = canvas;
    this.ctx = canvas.getContext("2d");
    this.tree = tree;
    this.marks = marks;
    this.resize();
    addEventListener("resize", () => this.resize());
  }

  resize() {
    const dpr = Math.min(devicePixelRatio || 1, 2);
    this.w = innerWidth;
    this.h = innerHeight;
    this.canvas.width = Math.round(this.w * dpr);
    this.canvas.height = Math.round(this.h * dpr);
    this.ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
    this.box = this.tree.getBoundingClientRect();
    // Unité de taille : la hauteur de l'image de l'arbre vaut 1000
    this.unit = this.box.height / 1000;
  }

  at([fx, fy]) {
    return { x: this.box.left + fx * this.box.width, y: this.box.top + fy * this.box.height };
  }

  perch(i) {
    return this.at(this.marks.perches[i % this.marks.perches.length]);
  }

  get heart() {
    return this.at(this.marks.heart);
  }

  get ground() {
    return this.at(this.marks.foot).y;
  }

  get crown() {
    const [l, t, r, b] = this.marks.crown;
    const a = this.at([l, t]);
    const z = this.at([r, b]);
    return { left: a.x, top: a.y, right: z.x, bottom: z.y };
  }

  /** Image de l'arbre affichée, pour repeindre le feuillage devant un oiseau */
  treeImage() {
    const src = getComputedStyle(this.tree).backgroundImage.match(/url\("?(.+?)"?\)/)?.[1];
    const img = new Image();
    if (src) img.src = src;
    return img;
  }

  /** Repeint l'arbre sur un disque : ce qui s'y trouve passe derrière le feuillage */
  cover(img, x, y, r, alpha) {
    if (!img.complete || !img.naturalWidth || alpha <= 0) return;
    const { left, top, width, height } = this.box;
    const kx = img.naturalWidth / width;
    const ky = img.naturalHeight / height;
    this.ctx.globalAlpha = alpha;
    this.ctx.drawImage(img, (x - r - left) * kx, (y - r - top) * ky, 2 * r * kx, 2 * r * ky, x - r, y - r, 2 * r, 2 * r);
    this.ctx.globalAlpha = 1;
  }
}
