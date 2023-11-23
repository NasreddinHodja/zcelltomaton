use nannou::prelude::Vec2;

use crate::automaton::Automaton;

type Cell = usize;

#[derive(Clone)]
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

    pub fn count_nbors(&self, nbors: &mut Vec<Cell>, r0: usize, c0: usize) {
        nbors.fill(0);

        for dr in -1..=1i32 {
            for dc in -1..=1i32 {
                if dr == 0 && dc == 0 {
                    continue;
                }
                let r = module(r0 as i32 + dr, R as i32) as usize;
                let c = module(c0 as i32 + dc, C as i32) as usize;
                nbors[self.get_cell(r, c)] += 1;
            }
        }
    }

    pub fn compute_next(&self, automaton: &Automaton, next: &mut Board<R, C>) {
        let mut nbors = vec![0; automaton.len()];

        for r in 0..R {
            for c in 0..C {
                self.count_nbors(&mut nbors, r, c);

                let state = match automaton.get_state(self.get_cell(r, c)) {
                    Some(state) => state,
                    None => panic!("State {}", self.cells[r][c]),
                };
                let transition = &nbors
                    .iter()
                    .map(|&x| x.to_string())
                    .collect::<Vec<String>>()
                    .join("");
                let next_state = state.next_state(transition);

                next.set_cell(r, c, next_state);
            }
        }
    }
}

fn module(a: i32, b: i32) -> i32 {
    ((a % b) + b) % b
}
