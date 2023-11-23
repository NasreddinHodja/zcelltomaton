use nannou::prelude::Vec2;

type Cell = usize;

pub struct Board<const R: usize, const C: usize> {
    cells: [[Cell; C]; R],
}

impl<const R: usize, const C: usize> Board<R, C> {
    pub fn new() -> Board<R, C> {
        Board { cells: [[0; C]; R] }
    }

    pub fn position_to_rc(
        &self,
        x: f32,
        y: f32,
        cell_w: f32,
        cell_h: f32,
        window_w: f32,
        window_h: f32,
    ) -> Vec2 {
        let r = ((-y + window_h / 2.0) / cell_h).floor();
        let c = ((x + window_w / 2.0) / cell_w).floor();

        Vec2::new(r, c)
    }

    pub fn set_cell(&mut self, r: usize, c: usize, state: Cell) {
        self.cells[r][c] = state
    }

    pub fn get_cell(&self, r: usize, c: usize) -> Cell {
        self.cells[r][c]
    }
}
