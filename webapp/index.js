import { EmuWasm } from "chip8";

const SCALE = 16;
const TICKS_PER_FRAME = 10;
const GRID_COLOR = "#cccccc";
const OFF_COLOR = "#000000";
const ON_COLOR = "#ffffff";

let anim_frame = 0;

const width = 64;
const height = 32;

const canvas = document.getElementById("chip-8");
canvas.height = (SCALE + 1) * height + 1;
canvas.width = (SCALE + 1) * width + 1;

const ctx = canvas.getContext("2d");
ctx.fillStyle = "#000000";
ctx.fillRect(0, 0, canvas.width, canvas.height);

const input = document.getElementById("fileInput");

const FPS = 60;
let now;
let then = Date.now();
let interval = 1000 / FPS;
let delta;

async function run() {
  let chip8 = new EmuWasm();

  document.addEventListener("keydown", function (ev) {
    // chip8.keyDown(ev.keyCode);
  });
  document.addEventListener("keyup", function (ev) {
    // chip8.keyUp(ev.keyCode);
  });

  input.addEventListener("change", function (ev) {
    if (anim_frame != 0) {
      window.cancelAnimationFrame(anim_frame);
    }

    let file = ev.target.files[0];
    if (!file) {
      alert("Failed to read file");
    }

    let fr = new FileReader();
    fr.onload = function (e) {
      let buffer = fr.result;
      const rom = new Uint8Array(buffer);
      chip8.reset();
      chip8.load_game(rom);
      mainloop(chip8);
    };

    fr.readAsArrayBuffer(file);
  }, false);



  function mainloop(chip8) {
    anim_frame = window.requestAnimationFrame(() => { mainloop(chip8); });

    now = Date.now();
    delta = now - then;

    if (delta > interval) {

      for (let i = 0; i < TICKS_PER_FRAME; i++) {
        chip8.tick();
      }
      chip8.tick_timers();

      ctx.fillStyle = "#000000";
      ctx.fillRect(0, 0, canvas.width, canvas.height);

      ctx.fillStyle = "#ffffff";
      chip8.draw_screen(SCALE);
    }

  }
}

run().catch(console.error);

// const drawGrid = () => {
//   ctx.beginPath();
//   ctx.strokeStyle = GRID_COLOR;

//   // vertical lines
//   for (let i = 0; i < width; i++) {
//     ctx.moveTo(i * (SCALE + 1) + 1, 0);
//     ctx.lineTo(i * (SCALE + 1) + 1, (SCALE + 1) * height + 1);
//   }

//   // horizontal lines
//   for (let i = 0; i < height; i++) {
//     ctx.moveTo(0, i * (SCALE + 1) + 1);
//     ctx.lineTo((SCALE + 1) * width + 1, i * (SCALE + 1) + 1);
//   }

//   ctx.stroke();
// };

// const getIndex = (row, coloumn) => {
//   return row * width + coloumn;
// };

// const drawPixels = () => {
//   const pixelPtr = emulator.pixels();
//   const pixels = new Uint8Array(memory.buffer, pixelPtr, width * height);

//   ctx.beginPath();

//   for (let row = 0; row < height; row++) {
//     for (let col = 0; col < width; col++) {
//       const idx = getIndex(row, col);

//       ctx.fillStyle = pixels[idx] === 0 ? OFF_COLOR : ON_COLOR;

//       ctx.fillRect(
//         col * (SCALE + 1) + 1,
//         row * (SCALE + 1) + 1,
//         SCALE,
//         SCALE
//       );
//     }
//   }
// };



// const renderLoop = () => {
//   requestAnimationFrame(renderLoop);



//   if (delta > interval) {
//     then = now - (delta % interval);

//     emulator.tick();
//     emulator.tick_timers();

//     drawGrid();
//     drawPixels();
//   }
// };

// renderLoop();
