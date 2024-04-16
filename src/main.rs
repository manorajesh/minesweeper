use macroquad::prelude::*;
use minesweeper::*;
use atlas::*;

mod atlas;
mod minesweeper;

const SQUARE_WIDTH: f32 = 30.0;
const NUM_ROWS: usize = 10;
const NUM_COLS: usize = 10;
const NUM_MINES: usize = 10;

fn window_conf() -> Conf {
    Conf {
        window_title: "Mine Sweeper".to_owned(),
        window_width: (NUM_COLS as i32) * (SQUARE_WIDTH as i32),
        window_height: (NUM_ROWS as i32) * (SQUARE_WIDTH as i32),
        ..Default::default()
    }
}

fn mouse_position_to_cell(mouse_position: (f32, f32)) -> (usize, usize) {
    let x = (mouse_position.0 / SQUARE_WIDTH) as usize;
    let y = (mouse_position.1 / SQUARE_WIDTH) as usize;

    (x, y)
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut field = MineSweeper::new(NUM_COLS, NUM_ROWS, NUM_MINES);
    let atlas = load_texture("atlas.png").await.unwrap();

    loop {
        clear_background(BLACK);

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position_to_cell(mouse_position());
            if field.reveal(mouse_x, mouse_y) {
                field.reveal_all();
            }
        }

        if is_mouse_button_pressed(MouseButton::Right) {
            let (mouse_x, mouse_y) = mouse_position_to_cell(mouse_position());
            field.flag(mouse_x, mouse_y);
        }

        for y in 0..field.field.len() {
            for x in 0..field.field[0].len() {
                draw_texture_ex(
                    &atlas,
                    (x as f32) * SQUARE_WIDTH,
                    (y as f32) * SQUARE_WIDTH,
                    WHITE,
                    DrawTextureParams {
                        source: Some(cell_to_image(&field.field[y][x])),
                        dest_size: Some(Vec2::new(SQUARE_WIDTH, SQUARE_WIDTH)),
                        ..Default::default()
                    }
                );
            }
        }

        next_frame().await;
    }
}
