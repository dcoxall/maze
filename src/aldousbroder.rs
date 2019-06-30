use crate::grid::{Grid, GridCell};
use std::collections::HashSet;
use rand::Rng;

pub struct Maze {
    pub width:  usize,
    pub height: usize,
    cells: Vec<GridCell>,
    index: Option<usize>,
    visited: HashSet<usize>,
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![GridCell(false, false); width * height],
            index: None,
            visited: HashSet::with_capacity(width * height),
        }
    }

    fn neighbours(&self, i: usize) -> Vec<Direction> {
        use Direction::*;

        let has_cell_north = i >= self.width;
        let has_cell_east = ((i + 1) % self.width) > 0;
        let has_cell_south = i + self.width < self.width * self.height;
        let has_cell_west = (i % self.width) > 0;
        let mut vec = Vec::with_capacity(4);

        if has_cell_north { vec.push(North(i - self.width)) };
        if has_cell_east { vec.push(East(i + 1)) };
        if has_cell_south { vec.push(South(i + self.width)) };
        if has_cell_west { vec.push(West(i - 1)) };

        vec
    }
}

#[derive(Debug)]
enum Direction {
    North(usize),
    East(usize),
    South(usize),
    West(usize),
}

impl Iterator for Maze {
    type Item = Grid;

    fn next(&mut self) -> Option<Self::Item> {
        use Direction::*;

        let mut rng = rand::thread_rng();

        if self.index.is_none() {
            let starting_i = rng.gen_range(0, self.width * self.height);
            self.index.replace(starting_i);
            return Some(Grid::new(self.width, self.height, &*self.cells, starting_i, vec![]))
        }

        if self.visited.len() == self.cells.len() {
            return None
        }

        let i = self.index.unwrap();

        let neighbours = self.neighbours(i);
        let neighbour_i = rng.gen_range(0, neighbours.len());
        let neighbour = neighbours.get(neighbour_i);

        if let Some(cell) = self.cells.get_mut(i) {
            self.visited.insert(i);

            let next_i = match neighbour {
                Some(North(i)) => {
                    if !self.visited.contains(i) { cell.0 = true };
                    *i
                },
                Some(East(i)) => {
                    if !self.visited.contains(i) { cell.1 = true };
                    *i
                },
                Some(South(i)) => {
                    let next_cell = self.cells.get_mut(*i).unwrap();
                    if !self.visited.contains(i) { next_cell.0 = true };
                    *i
                },
                Some(West(i)) => {
                    let next_cell = self.cells.get_mut(*i).unwrap();
                    if !self.visited.contains(i) { next_cell.1 = true };
                    *i
                },
                None => rng.gen_range(0, self.width * self.height),
            };

            self.index.replace(next_i);

            Some(Grid::new(self.width, self.height, &*self.cells, next_i, vec![]))
        } else {
            None
        }
    }
}
