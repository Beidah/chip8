import { Screen } from "chip8";
import { memory } from "chip8/chip8_bg.wasm";

const CELL_SIZE = 16;
const GRID_COLOR = "#cccccc";
const OFF_COLOR = "#000000";
const ON_COLOR = "#ffffff";

const draw_screen = Screen.new();
const width = draw_screen.width();
const height = draw_screen.height();

const canvas = document.getElementById("chip-8");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext("2d");

const drawGrid = () => {
  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;

  // vertical lines
  for (let i = 0; i < width; i++) {
    ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
    ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
  }

  // horizontal lines
  for (let i = 0; i < height; i++) {
    ctx.moveTo(0, i * (CELL_SIZE + 1) + 1);
    ctx.lineTo((CELL_SIZE + 1) * width + 1, i * (CELL_SIZE + 1) + 1);
  }

  ctx.stroke();
}

const getIndex = (row, coloumn) => {
  return row * width + coloumn;
}

const drawPixels = () => {
  const pixelPtr = draw_screen.pixels();
  const pixels = new Uint8Array(memory.buffer, pixelPtr, width * height);

  ctx.beginPath();

  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = getIndex(row, col);

      ctx.fillStyle = pixels[idx] === 0 ? OFF_COLOR : ON_COLOR;
      
      ctx.fillRect(
        col * (CELL_SIZE + 1) + 1,
        row * (CELL_SIZE + 1) + 1,
        CELL_SIZE,
        CELL_SIZE
      )
    }
  }
}

const renderLoop = () => {
  drawGrid();
  drawPixels();

  requestAnimationFrame(renderLoop);
}

drawGrid();
drawPixels();
requestAnimationFrame(renderLoop)
