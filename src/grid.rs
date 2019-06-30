use std::borrow::Cow;
use std::fmt;
use gif;

#[derive(Debug, Clone)]
pub struct GridCell(
    pub bool, // north
    pub bool, // east
);

#[derive(Debug, Clone)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<GridCell>,
    pub highlights: Vec<usize>,
    pub current_cell_index: usize,
}

impl Grid {
    pub fn new<T: Into<Vec<GridCell>>, H: Into<Vec<usize>>>(width: usize, height: usize, cells: T, current_cell_index: usize, highlights: H) -> Self {
        Self { width, height, cells: cells.into(), current_cell_index, highlights: highlights.into() }
    }

    pub fn at(&self, x: usize, y: usize) -> Option<&GridCell> {
        self.cells.get(y * self.width + x)
    }
}

impl<T: Iterator<Item=Grid>> From<T> for Grid {
    fn from(t: T) -> Self {
        t.last().unwrap()
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const CORNER: &str       = "+";
        const WALL: &str         = "|";
        const NORTH_OPEN: &str   = "   +";
        const EAST_OPEN: &str    = "    ";
        const NORTH_CLOSED: &str = "---+";
        const EAST_CLOSED: &str  = "   |";

        let mut output = String::from("");
        let mut top_row: String;
        let mut middle_row: String;
        for y in 0..self.height {
            top_row    = String::from(CORNER);
            middle_row = String::from(WALL);

            for x in 0..self.width {
                match self.at(x, y) {
                    Some(GridCell(true, false)) => {
                        top_row.push_str(NORTH_OPEN);
                        middle_row.push_str(EAST_CLOSED);
                    },
                    Some(GridCell(false, true)) => {
                        top_row.push_str(NORTH_CLOSED);
                        middle_row.push_str(EAST_OPEN);
                    },
                    Some(GridCell(true, true)) => {
                        top_row.push_str(NORTH_OPEN);
                        middle_row.push_str(EAST_OPEN);
                    },
                    _ => {
                        top_row.push_str(NORTH_CLOSED);
                        middle_row.push_str(EAST_CLOSED);
                    },
                }
            }

            top_row.push_str("\n");
            middle_row.push_str("\n");

            output = output + &top_row + &middle_row;
        }
        top_row = String::from(CORNER);
        for _x in 0..self.width {
            top_row.push_str(NORTH_CLOSED);
        }

        output = output + &top_row;
        write!(f, "{}", output)
    }
}

impl From<Grid> for gif::Frame<'static> {
    fn from(grid: Grid) -> Self {
        const BORDER: usize  = 2;
        const CELL: usize    = 10;
        const PADDING: usize = 20;
        const CELL_SIZE: usize = CELL + BORDER;

        let cols = PADDING + BORDER + (CELL_SIZE * grid.width) + PADDING;
        let rows = PADDING + (CELL_SIZE * grid.height) + BORDER + PADDING;

        let mut pixel_map = vec![1; rows * cols];
        let mut frame = gif::Frame::default();

        frame.width = cols as u16;
        frame.height = rows as u16;

        for y in 0..grid.height {
            let pixel_y =
                  (PADDING * cols)
                + (y * CELL_SIZE * cols);

            for i in 0..CELL_SIZE {
                for bs in 0..BORDER {
                    pixel_map[pixel_y + PADDING + (cols * i) + bs] = 0;
                }
            }

            for x in 0..grid.width {
                let top_left_index =
                      pixel_y
                    + PADDING
                    + BORDER
                    + (x * CELL_SIZE);


                if grid.highlights.contains(&((y * grid.width) + x)) {
                    for ix in 0..CELL_SIZE {
                        for iy in 0..CELL_SIZE {
                            pixel_map[top_left_index + ix + (cols * (iy + BORDER))] = 3;
                        }
                    }
                }

                if grid.current_cell_index == ((y * grid.width) + x) {
                    for ix in 0..CELL_SIZE {
                        for iy in 0..CELL_SIZE {
                            pixel_map[top_left_index + ix + (cols * (iy + BORDER))] = 2;
                        }
                    }
                }

                match grid.at(x, y) {
                    Some(GridCell(true, false)) => {
                        for i in 0..CELL_SIZE {
                            for bs in 0..BORDER {
                                pixel_map[top_left_index + CELL + (cols * i) + bs] = 0;
                            }
                        }
                    },
                    Some(GridCell(false, true)) => {
                        for i in 0..CELL_SIZE {
                            for bs in 0..BORDER {
                                pixel_map[top_left_index + i + (cols * bs)] = 0;
                            }
                        }
                    },
                    Some(GridCell(false, false)) => {
                        for i in 0..CELL_SIZE {
                            for bs in 0..BORDER {
                                pixel_map[top_left_index + i + (cols * bs)] = 0;
                                pixel_map[top_left_index + CELL + (cols * i) + bs] = 0;
                            }
                        }
                    },
                    _ => {
                        for bs_x in 0..BORDER {
                            for bs_y in 0..BORDER {
                                pixel_map[top_left_index + CELL + (cols * bs_x)] = 0;
                                pixel_map[top_left_index + CELL + (cols * bs_x) + bs_y] = 0;
                            }
                        }
                    },
                }
            }
        }

        for i in 0..((CELL_SIZE * grid.width) + BORDER) {
            for bs in 0..BORDER {
                let index = (PADDING * cols)
                    + (cols * (grid.height * CELL_SIZE))
                    + PADDING
                    + (cols * bs)
                    + i;

                pixel_map[index] = 0;
            }
        }

        frame.buffer = Cow::Owned(pixel_map);
        frame
    }
}
