use std::fmt;

#[derive(Debug, Clone)]
pub struct GridCell(
    pub bool, // north
    pub bool, // east
);

#[derive(Debug)]
pub struct Grid {
    width:  usize,
    height: usize,
    cells:  Vec<GridCell>,
}

impl Grid {
    pub fn new<T: Into<Vec<GridCell>>>(width: usize, height: usize, cells: T) -> Self {
        Self { width, height, cells: cells.into() }
    }

    pub fn at(&self, x: usize, y: usize) -> Option<&GridCell> {
        return self.cells.get(y * self.width + x);
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
