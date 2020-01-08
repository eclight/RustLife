use sdl2::pixels::Color;

#[derive(Copy, Clone, Debug)]
pub struct CellState {
    pub color: Color,
    pub alive: bool,
}

impl CellState {
    pub fn new(alive: bool, c: Color) -> Self {
        CellState {
            color: c,
            alive: alive,
        }
    }

    pub fn combine_cells(parents: &[CellState]) -> CellState {
        let mut r: i32 = 0;
        let mut g: i32 = 0;
        let mut b: i32 = 0;

        for p in parents {
            match p {
                &CellState { color: Color { r: cr, b: cb, g: cg, a: _ } , .. } => {
                    r += cr as i32;
                    b += cb as i32;
                    g += cg as i32;
                }
            }
        }

        CellState::new(true,
                       Color::RGB((r / parents.len() as i32) as u8,
                                  (g / parents.len() as i32) as u8,
                                  (b / parents.len() as i32) as u8))
    }
}
