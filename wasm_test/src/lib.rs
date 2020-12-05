mod utils;

use std::fmt;
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
    Dead = 0,
    Alive = 1,
}
#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Universe {
    fn get_index(&self, x: u32, y: u32) -> usize {
        (y * self.width + x) as usize
    }
    fn live_neighbour_count(&self, x: u32, y: u32) -> u8 {
        let mut count = 0;
        for delta_x in [self.width - 1, 0, 1].iter().cloned() {
            for delta_y in [self.height - 1, 0, 1].iter().cloned() {
                if delta_x == 0 && delta_y == 0 {
                    continue;
                }

                let neighbor_x = (x + delta_x) % self.width;
                let neighbor_y = (y + delta_y) % self.height;
                let idx = self.get_index(neighbor_x, neighbor_y);
                count += self.cells[idx] as u8;
            }
        }
        count
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for x in 0..self.width {
            for y in 0..self.height {
                let idx = self.get_index(x, y);

                next[idx] = match self.live_neighbour_count(x, y) {
                    3 => Cell::Alive,
                    2 => next[idx],
                    _ => Cell::Dead,
                };
            }
        }

        self.cells = next
    }

    pub fn new() -> Universe {
        let width = 64;
        let height = 64;

        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
