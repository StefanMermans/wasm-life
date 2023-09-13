import init, { RustLife } from "./pkg/wasm_life.js";

init().then(setupGame);

function setupGame() {
  console.log('setupGame');
  const canvas = document.getElementById('rust-life-canvas');
  const rustLife = RustLife.new();
  canvas.width = rustLife.width();
  canvas.height = rustLife.height();
  canvas.addEventListener('click', (event) => {
    console.log('click', event.offsetX, event.offsetY)
    rustLife.click(event.offsetX, event.offsetY);
  })

  requestAnimationFrame(() => onFrame(rustLife, canvas.getContext('2d')));
}

/**
 * 
 * @param {RustLife} rustLife 
 * @param {CanvasRenderingContext2D} context 
 */
function onFrame(rustLife, context) {
    rustLife.on_frame(context);

    setTimeout(() => {
        requestAnimationFrame(() => onFrame(rustLife, context))
    }, 50)
}
