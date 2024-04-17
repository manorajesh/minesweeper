use macroquad::prelude::*;

use crate::{ Cell, CellType };

fn index_alias(index: f32, atlas_cell_size: f32, atlas_width: f32, atlas_row_count: f32) -> Rect {
    if index > atlas_row_count * atlas_row_count {
        return Rect::new(atlas_width, atlas_width, atlas_cell_size, atlas_cell_size);
    }
    Rect::new(
        (index * atlas_cell_size) % atlas_width,
        (index / atlas_row_count).floor() * atlas_cell_size,
        atlas_cell_size,
        atlas_cell_size
    )
}

pub fn cell_to_image(cell: &Cell) -> Rect {
    if !cell.visible && !cell.flag {
        return index_alias(9.0, 16.0, 64.0, 4.0);
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

    if cell.flag {
        index = 10.0;
    }

    if cell.incorrect {
        index += 1.0;
    }

    index_alias(index, 16.0, 64.0, 4.0)
}

#[derive(Debug)]
pub enum Face {
    Smile,
    Pressed,
    Lost,
    Won,
}

pub fn face_to_image(face: &Face) -> Rect {
    let index = match face {
        Face::Smile => 0.0,
        Face::Pressed => 1.0,
        Face::Lost => 4.0,
        Face::Won => 3.0,
    };

    index_alias(index, 50.0, 250.0, 5.0)
}
