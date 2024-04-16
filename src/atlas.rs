use macroquad::prelude::*;

use crate::{ Cell, CellType };

const ATLAS_CELL_SIZE: f32 = 16.0;
const ATLAS_WIDTH: f32 = 64.0;
const ATLAS_ROW_COUNT: f32 = 4.0;
pub fn index_alias(index: f32) -> Rect {
    if index > ATLAS_ROW_COUNT * ATLAS_ROW_COUNT {
        return Rect::new(ATLAS_WIDTH, ATLAS_WIDTH, ATLAS_CELL_SIZE, ATLAS_CELL_SIZE);
    }
    Rect::new(
        (index * ATLAS_CELL_SIZE) % ATLAS_WIDTH,
        (index / ATLAS_ROW_COUNT).floor() * ATLAS_CELL_SIZE,
        ATLAS_CELL_SIZE,
        ATLAS_CELL_SIZE
    )
}

pub fn cell_to_image(cell: &Cell) -> Rect {
    if !cell.visible {
        return index_alias(9.0);
    }

    let mut index;

    match cell.cell_type {
        CellType::Mine => {
            index = 12.0;
        }
        CellType::Safe(n) => {
            index = n as f32;
        }
        CellType::Empty => {
            index = 0.0;
        }
    }

    if cell.incorrect {
        index += 1.0;
    }

    if cell.flag {
        index = 10.0;
    }

    index_alias(index)
}
