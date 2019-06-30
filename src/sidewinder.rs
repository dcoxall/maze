use crate::grid::{Grid, GridCell};
use rand::Rng;

pub struct Maze {
    pub width:  usize,
    pub height: usize,
    cells: Vec<GridCell>,
    index: Option<usize>,
    current_run: Vec<usize>,
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Self {
        let cells = vec![GridCell(false, false); width * height];
        Self {
            width,
            height,
            cells,
            index: None,
            current_run: Vec::with_capacity(width),
        }
    }

    fn close_run<R: rand::Rng>(&mut self, rng: &mut R, erase_north: bool) {
        let run_index = rng.gen_range(0, self.current_run.len());
        let run_cell_index = self.current_run.get(run_index).unwrap();
        if let Some(cell) = self.cells.get_mut(*run_cell_index) {
            if erase_north {
                cell.0 = true;
            }
        }
        self.current_run.clear();
    }
}

impl Iterator for Maze {
    type Item = Grid;

    fn next(&mut self) -> Option<Self::Item> {
        let mut rng = rand::thread_rng();

        if self.index.is_none() {
            self.index.replace(0);
            return Some(Grid::new(self.width, self.height, &*self.cells, 0, vec![]))
        }

        let i = self.index.unwrap();

        if let Some(cell) = self.cells.get_mut(i) {
            let has_cell_north = i >= self.width;
            let has_cell_east = ((i + 1) % self.width) != 0;

            self.current_run.push(i);

            match (has_cell_north, has_cell_east) {
                (false, false) => self.close_run(&mut rng, false),
                (true, false) => self.close_run(&mut rng, true),
                (false, true) => cell.1 = true,
                _ => {
                    if rng.gen_bool(0.5) {
                        self.close_run(&mut rng, true)
                    } else {
                        cell.1 = true
                    }
                }
            };

            self.index.replace(i + 1);
            Some(Grid::new(self.width, self.height, &*self.cells, i + 1, &*self.current_run))
        } else {
            None
        }
    }
}
