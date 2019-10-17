mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Blank = 0,
    Head  = 1,
    Body  = 2,
    Apple = 3,
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Up = 0,
    Down = 1,
    Right = 2,
    Left = 3,
}

#[wasm_bindgen]
pub struct Snake {
    head: usize,
    bodies: Vec<usize>,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
    snake: Snake,
    apple: usize,
    score: u32,
}

#[wasm_bindgen]
impl Universe {
    fn calc_next(&self, direction: Direction, now: usize) -> usize {
        let w = self.width as usize;
        let h = self.height as usize;
        match (direction, now) {
            (Direction::Up, x) if x < w => now + w * (h - 1),
            (Direction::Up, _) => now - w,
            (Direction::Down, x) if w * (h - 1) <= x => now % w,
            (Direction::Down, _) => now + w,
            (Direction::Right, x) if (x + 1) % w == 0 => now - w + 1,
            (Direction::Right, _) => now + 1,
            (Direction::Left, x) if x % w == 0 => now + w - 1,
            (Direction::Left, _) => now - 1,
        }
    }

    fn forward_snake(&mut self) {
        self.snake.bodies.remove(0);
        self.snake.bodies.push(self.snake.head);
    }

    fn grow_up_snake(&mut self) {
        self.snake.bodies.insert(0, self.snake.head);
    }

    fn replace_apple(&mut self) {
        let size = (self.width * self.height) as usize;
        self.apple = utils::random_index(size, self.snake.head);
    }
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self, direction: Direction) {
        let mut next = self.cells.clone();
        let now = self.snake.head;

        self.forward_snake();
        self.snake.head = self.calc_next(direction, now);

        // apple was bitten?
        if self.apple == self.snake.head {
            self.grow_up_snake();
            self.replace_apple();
            self.score += 1;
        }

        for cell in next.iter_mut() {
            *cell = Cell::Blank;
        }
        next[self.apple] = Cell::Apple;
        for i in self.snake.bodies.iter() {
            next[*i] = Cell::Body;
        }
        next[self.snake.head] = Cell::Head;

        self.cells = next;
    }

    pub fn new() -> Universe {
        utils::set_panic_hook();

        let width  :u32 = 16;
        let height :u32 = 16;
        let size = (width * height) as usize;

        let snake = Snake {
            head: 1,
            bodies: vec![0],
        };
        let apple = utils::random_index(size, snake.head);

        let cells = (0..width * height)
            .map(|i| {
                if i as usize == apple {
                    Cell::Apple
                } else if i as usize == snake.head {
                    Cell::Head
                } else {
                    Cell::Blank
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
            snake,
            apple,
            score: 0,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }
    pub fn score(&self) -> u32 {
        self.score
    }
}
