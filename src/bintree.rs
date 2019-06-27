use crate::grid::{Grid, GridCell};
use rand::Rng;

pub struct Maze {
    pub width:  usize,
    pub height: usize,
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
}

pub struct MazeIter {
    width:              usize,
    height:             usize,
    cells:              Vec<GridCell>,
    current_cell_index: Option<usize>,
}

impl MazeIter {
    pub fn new(width: usize, height: usize) -> Self {
        let cells: Vec<GridCell> = (0..(width * height))
            .map(|_| GridCell(false, false))
            .collect();
        Self { width, height, cells, current_cell_index: None }
    }
}

impl From<Maze> for Grid {
    fn from(generator: Maze) -> Self {
        generator.into_iter()
            .last()
            .unwrap()
    }
}

impl From<Maze> for MazeIter {
    fn from(generator: Maze) -> Self {
        MazeIter::new(generator.width, generator.height)
    }
}

impl IntoIterator for Maze {
    type Item = Grid;
    type IntoIter = MazeIter;

    fn into_iter(self) -> Self::IntoIter {
        MazeIter::from(self)
    }
}

impl Iterator for MazeIter {
    type Item = Grid;

    fn next(&mut self) -> Option<Self::Item> {
        let mut rng = rand::thread_rng();

        if self.current_cell_index.is_none() {
            self.current_cell_index.replace(0);
            return Some(Grid::new(self.width, self.height, self.cells.clone(), 0))
        }

        let i = self.current_cell_index.unwrap_or(0);
        self.current_cell_index.replace(i + 1);

        if let Some(cell) = self.cells.get_mut(i) {
            let has_cell_north = i >= self.width;
            let has_cell_east = ((i + 1) % self.width) != 0;

            match (has_cell_north, has_cell_east) {
                (true,  false) => cell.0 = true,
                (false,  true) => cell.1 = true,
                (true, true) => {
                    let b = rng.gen_bool(0.5);
                    cell.0 = b;
                    cell.1 = !b;
                },
                _ => {},
            }

            Some(Grid::new(self.width, self.height, self.cells.clone(), i + 1))
        } else {
            None
        }
    }
}
