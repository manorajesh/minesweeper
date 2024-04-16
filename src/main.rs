use macroquad::prelude::*;

const SQUARE_WIDTH: f32 = 20.0;
const PADDING: f32 = 20.0;
const FONT_SIZE: f32 = 20.0;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
enum CellType {
    Mine,
    Safe(usize), // Number of mines around
}
#[derive(Debug, Clone)]
struct Cell {
    cell_type: CellType,
    visible: bool,
}

impl Cell {
    fn new() -> Self {
        Self {
            cell_type: CellType::Safe(0),
            visible: false,
        }
    }
}

#[derive(Debug, Clone)]
struct MineSweeper {
    field: Vec<Vec<Cell>>,
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

    fn new(width: usize, height: usize, mines: usize) -> Self {
        let mut field = vec![vec![Cell::new(); height]; width];

        // Populate mines
        for _ in 0..mines {
            let x = (rand::rand() as usize) % width;
            let y = (rand::rand() as usize) % height;
            field[x][y].cell_type = CellType::Mine;
        }

        // Count mines around each cell
        for x in 0..width {
            for y in 0..height {
                if let CellType::Safe(_) = field[y][x].cell_type {
                    field[y][x].cell_type = CellType::Safe(MineSweeper::count_mines(&field, x, y));
                }
            }
        }

        Self { field }
    }
}

#[macroquad::main("civilization")]
async fn main() {
    let field = MineSweeper::new(10, 10, 10);

    loop {
        clear_background(BLACK);

        for y in 0..field.field.len() {
            for x in 0..field.field[0].len() {
                match field.field[y][x].cell_type {
                    CellType::Mine => {
                        draw_rectangle(
                            (x as f32) * PADDING,
                            (y as f32) * PADDING,
                            SQUARE_WIDTH,
                            SQUARE_WIDTH,
                            RED
                        );
                    }

                    CellType::Safe(n) => {
                        draw_rectangle(
                            (x as f32) * PADDING,
                            (y as f32) * PADDING,
                            SQUARE_WIDTH,
                            SQUARE_WIDTH,
                            GREEN
                        );

                        draw_text(
                            &format!("{n}"),
                            (x as f32) * PADDING + SQUARE_WIDTH / 2.0,
                            (y as f32) * PADDING + SQUARE_WIDTH / 2.0,
                            FONT_SIZE,
                            WHITE
                        );
                    }
                }
            }
        }

        next_frame().await;
    }
}
