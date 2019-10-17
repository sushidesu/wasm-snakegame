import { Universe, Cell, Direction } from "wasm-game-of-snake"
import { memory } from "wasm-game-of-snake/wasm_game_of_snake_bg"

const CELL_SIZE = 10

const BLANK_COLOR = "#eeeeee"
const HEAD_COLOR = "#ff0040"
const BODY_COLOR = "#ffc4c4"
const APPLE_COLOR = "#ffbf00"

// create Univerce
const universe = Universe.new()
const width = universe.width()
const height = universe.height()

// create canvas
const canvas = document.getElementById("game-of-snake-canvas")
canvas.width = (CELL_SIZE + 1) * height + 1;
canvas.height = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext("2d")

const getIndex = (row, column) => {
  return row * width + column
}

const drawCells = () => {
  const cellsPtr = universe.cells()
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height)

  ctx.beginPath()

  draw(Cell.Blank, cells, BLANK_COLOR)
  draw(Cell.Head,  cells, HEAD_COLOR)
  draw(Cell.Body,  cells, BODY_COLOR)
  draw(Cell.Apple, cells, APPLE_COLOR)

  ctx.stroke()
}

const draw = (celltype, cells, color) => {
  ctx.fillStyle = color
  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = getIndex(row, col)
      if (cells[idx] != celltype) {
        continue
      }

      ctx.fillRect(
        col * (CELL_SIZE + 1) + 1,
        row * (CELL_SIZE + 1) + 1,
        CELL_SIZE,
        CELL_SIZE
      )
    }
  }
}

let v = Direction.Right
const on_keydown = (keyCode) => {
  switch (keyCode) {
    case 37:
      v = Direction.Left
      break;
    case 38:
      v = Direction.Up
      break;
    case 39:
      v = Direction.Right
      break;
    case 40:
      v = Direction.Down
      break;
  }
}
document.onkeydown = () => {
  on_keydown(event.keyCode)
}

let point
const score = document.getElementById("score")
const updateScore = (point) => {
  score.innerText = point.toString()
}

const renderLoop = () => {
  universe.tick(v)
  // drawGrid()
  drawCells()

  const nowpoint = universe.score()
  if (point != nowpoint) {
    updateScore(nowpoint)
    point = nowpoint
  }

  setTimeout(() => {
    requestAnimationFrame(renderLoop)
  }, 100);
}
renderLoop()
