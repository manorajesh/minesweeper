use rand::Rng;

const MAX_SEARCH_DEPTH: usize = 5;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum CellType {
    Mine,
    Safe(usize), // Number of mines around
    Empty,
}
#[derive(Debug, Clone)]
pub struct Cell {
    pub cell_type: CellType,
    pub visible: bool,
    pub flag: bool,
    pub incorrect: bool,
}

impl Cell {
    fn new() -> Self {
        Self {
            cell_type: CellType::Empty,
            visible: false,
            flag: false,
            incorrect: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MineSweeper {
    pub field: Vec<Vec<Cell>>,
    correct_flags: usize,
}

impl MineSweeper {
    fn count_mines(field: &Vec<Vec<Cell>>, x: usize, y: usize) -> usize {
        let mut mines = 0;
        for i in -1..=1 {
            let y_shift = (y as i32) + i;
            if y_shift < 0 {
                continue;
            }
            for j in -1..=1 {
                let x_shift = (x as i32) + j;
                if x_shift < 0 {
                    continue;
                }
                if let Some(row) = field.get(y_shift as usize) {
                    if let Some(cell) = row.get(x_shift as usize) {
                        if cell.cell_type == CellType::Mine {
                            mines += 1;
                        }
                    }
                }
            }
        }

        mines
    }

    pub fn new(width: usize, height: usize, mines: usize) -> Self {
        let mut field = vec![vec![Cell::new(); height]; width];

        // Populate mines
        for _ in 0..mines {
            let x = rand::thread_rng().gen_range(0..width);
            let y = rand::thread_rng().gen_range(0..height);
            field[x][y].cell_type = CellType::Mine;
        }

        // Count mines around each cell
        for x in 0..width {
            for y in 0..height {
                if let CellType::Empty = field[y][x].cell_type {
                    let mines = MineSweeper::count_mines(&field, x, y);
                    match mines {
                        0 => {
                            field[y][x].cell_type = CellType::Empty;
                        }
                        _ => {
                            field[y][x].cell_type = CellType::Safe(mines);
                        }
                    }
                }
            }
        }

        Self { field, correct_flags: 0 }
    }

    // Returns true if the cell is a mine
    pub fn reveal(&mut self, x: usize, y: usize, depth: usize) -> bool {
        if let Some(row) = self.field.get_mut(y) {
            if let Some(cell) = row.get_mut(x) {
                if cell.flag {
                    return false;
                }

                if cell.visible {
                    return false;
                }

                if let CellType::Mine = cell.cell_type {
                    cell.visible = true;
                    cell.incorrect = true;
                    return true;
                }

                cell.visible = true;

                if depth > MAX_SEARCH_DEPTH {
                    return false;
                }

                if let CellType::Empty = cell.cell_type {
                    for i in -1..=1 {
                        let y_shift = (y as i32) + i;
                        if y_shift < 0 {
                            continue;
                        }
                        for j in -1..=1 {
                            let x_shift = (x as i32) + j;
                            if x_shift < 0 {
                                continue;
                            }
                            self.reveal(x_shift as usize, y_shift as usize, depth + 1);
                        }
                    }
                }

                return false;
            }
        }

        false
    }

    pub fn flag(&mut self, x: usize, y: usize) {
        if let Some(row) = self.field.get_mut(y) {
            if let Some(cell) = row.get_mut(x) {
                if cell.visible {
                    return;
                }

                cell.flag = !cell.flag;

                if cell.flag {
                    if let CellType::Mine = cell.cell_type {
                        self.correct_flags += 1;
                    }
                } else {
                    if let CellType::Mine = cell.cell_type {
                        self.correct_flags -= 1;
                    }
                }
            }
        }
    }

    pub fn reveal_all(&mut self) {
        for row in self.field.iter_mut() {
            for cell in row.iter_mut() {
                cell.visible = true;

                if cell.flag && cell.cell_type != CellType::Mine {
                    cell.incorrect = true;
                }
            }
        }
    }

    pub fn is_win(&self, total_mines: usize) -> bool {
        if self.correct_flags == total_mines {
            return true;
        }

        false
    }
}
