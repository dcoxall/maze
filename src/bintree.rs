use crate::grid::{Grid, GridCell};
use rand::Rng;

pub struct Maze {
    pub width:  usize,
    pub height: usize,
    cells: Vec<GridCell>,
    index: Option<usize>,
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Self {
        let cells = vec![GridCell(false, false); width * height];
        Self { width, height, cells, index: None }
    }
}

impl Iterator for Maze {
    type Item = Grid;

    fn next(&mut self) -> Option<Self::Item> {
        let mut rng = rand::thread_rng();

        if self.index.is_none() {
            self.index.replace(0);
            return Some(Grid::new(self.width, self.height, &*self.cells, 0))
        }

        let i = self.index.unwrap();

        if let Some(cell) = self.cells.get_mut(i) {
            let has_cell_north = i >= self.width;
            let has_cell_east = ((i + 1) % self.width) != 0;

            match (has_cell_north, has_cell_east) {
                (true, false) => cell.0 = true,
                (false, true) => cell.1 = true,
                (true, true) => {
                    let b = rng.gen_bool(0.5);
                    cell.0 = b;
                    cell.1 = !b;
                },
                _ => {},
            }

            self.index.replace(i + 1);
            Some(Grid::new(self.width, self.height, &*self.cells, i + 1))
        } else {
            None
        }
    }
}
