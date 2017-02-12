
use std::mem;
use rand;
use rand::Rng;
use sdl2::pixels::Color;
use color;
use cell_state::CellState;

#[derive(Copy, Clone, Debug)]
pub struct CellIndex {
    pub col: i32,
    pub row: i32,
}

const NEIGHBORS: [CellIndex; 8] = [CellIndex { col: -1, row: -1 },
                                   CellIndex { col: 0, row: -1 },
                                   CellIndex { col: 1, row: -1 },
                                   CellIndex { col: 1, row: 0 },
                                   CellIndex { col: 1, row: 1 },
                                   CellIndex { col: 0, row: 1 },
                                   CellIndex { col: -1, row: 1 },
                                   CellIndex { col: -1, row: 0 }];

pub struct World {
    pub cols: i32,
    pub rows: i32,
    front: Vec<CellState>,
    back: Vec<CellState>,
}


impl World {
    pub fn new(cols: i32, rows: i32) -> Self {
        let cells = (cols * rows) as usize;

        World {
            rows: rows,
            cols: cols,
            front: vec![CellState::new(false, Color::RGB(0, 0, 0)); cells],
            back: vec![CellState::new(false, Color::RGB(0, 0, 0)); cells],
        }
    }

    pub fn fill_random(&mut self) {
        let mut rng = rand::thread_rng();

        for row in 0..self.rows {
            for col in 0..self.cols {
                let state = if rng.gen::<i32>() % 2 == 0 {
                    CellState::new(true, color::random_color_hsv())
                } else {
                    CellState::new(false, Color::RGB(0, 0, 0))
                };
                self.set_state(CellIndex {
                                   row: row,
                                   col: col,
                               },
                               state);
            }
        }

        mem::swap(&mut self.front, &mut self.back);
    }

    pub fn tick(&mut self) {
        let mut alive_neighbors: [CellState; 8] = [CellState::new(false, Color::RGB(0, 0, 0)); 8];
        let mut num_alive_neighbors = 0;

        for row in 0..self.rows {
            for col in 0..self.cols {
                let idx = CellIndex {
                    col: col,
                    row: row,
                };
                let curr_state = self.get_state(idx);

                for offset in NEIGHBORS.iter() {
                    let neighbor = self.get_state(self.offset(idx, *offset));

                    if neighbor.alive {
                        alive_neighbors[num_alive_neighbors] = neighbor;
                        num_alive_neighbors += 1;
                    }
                }

                let new_state = match curr_state {
                    CellState { alive: true, color } => {
                        if num_alive_neighbors == 2 || num_alive_neighbors == 3 {
                            curr_state
                        } else {
                            CellState::new(false, color)
                        }
                    }
                    CellState { alive: false, .. } => {
                        if num_alive_neighbors == 3 {
                            CellState::combine_cells(&alive_neighbors[0..3])
                        } else {
                            curr_state
                        }
                    }
                };

                self.set_state(idx, new_state);
                num_alive_neighbors = 0;
            }
        }

        mem::swap(&mut self.front, &mut self.back);
    }

    pub fn get_state(&self, idx: CellIndex) -> CellState {
        let offset = (idx.row * self.cols + idx.col) as usize;
        self.front[offset]
    }

    pub fn set_state(&mut self, idx: CellIndex, state: CellState) {
        let offset = (idx.row * self.cols + idx.col) as usize;
        self.back[offset] = state;
    }

    fn offset(&self, idx: CellIndex, offset: CellIndex) -> CellIndex {
        CellIndex {
            row: (idx.row + offset.row + self.rows) % self.rows,
            col: (idx.col + offset.col + self.cols) % self.cols,
        }
    }
}
