import init, { RustLife } from "./pkg/wasm_life.js";

init().then(setupGame);

function setupGame() {
  console.log('setupGame');
  const canvas = document.getElementById('rust-life-canvas');
  const rustLife = RustLife.new();
  canvas.width = rustLife.width();
  canvas.height = rustLife.height();

  requestAnimationFrame(() => onFrame(rustLife, canvas.getContext('2d')));
}

/**
 * 
 * @param {RustLife} rustLife 
 * @param {CanvasRenderingContext2D} context 
 */
function onFrame(rustLife, context) {
    rustLife.on_frame(context);
}