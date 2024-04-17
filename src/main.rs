use macroquad::prelude::*;
use minesweeper::*;
use atlas::*;

mod atlas;
mod minesweeper;

const SQUARE_WIDTH: f32 = 30.0;
const FACE_WIDTH: f32 = 60.0;
const NUM_ROWS: usize = 10;
const NUM_COLS: usize = 10;
const NUM_MINES: usize = 10;

fn window_conf() -> Conf {
    Conf {
        window_title: "Mine Sweeper".to_owned(),
        window_width: (NUM_COLS as i32) * (SQUARE_WIDTH as i32),
        window_height: (NUM_ROWS as i32) * (SQUARE_WIDTH as i32) + (FACE_WIDTH as i32),
        ..Default::default()
    }
}

enum ClickType {
    Cell(usize, usize),
    Face,
}

fn mouse_position_to_cell(mouse_position: (f32, f32)) -> Option<ClickType> {
    if
        mouse_position.1 < FACE_WIDTH &&
        mouse_position.0 > ((NUM_COLS as f32) * SQUARE_WIDTH) / 2.0 - FACE_WIDTH / 2.0 &&
        mouse_position.0 < ((NUM_COLS as f32) * SQUARE_WIDTH) / 2.0 + FACE_WIDTH / 2.0
    {
        return Some(ClickType::Face);
    }

    if mouse_position.1 < FACE_WIDTH {
        return None;
    }

    let x = (mouse_position.0 / SQUARE_WIDTH) as usize;
    let y = (mouse_position.1 / SQUARE_WIDTH - FACE_WIDTH / SQUARE_WIDTH) as usize;

    Some(ClickType::Cell(x, y))
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut field = MineSweeper::new(NUM_COLS, NUM_ROWS, NUM_MINES);
    let atlas = load_texture("atlas.png").await.unwrap();
    let face = load_texture("face.png").await.unwrap();
    let mut face_state = Face::Smile;

    loop {
        clear_background(Color::from_hex(0xc0c0c0));

        if is_mouse_button_down(MouseButton::Left) {
            match mouse_position_to_cell(mouse_position()) {
                Some(ClickType::Face) => {
                    face_state = Face::Pressed;
                }

                _ => {}
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            match mouse_position_to_cell(mouse_position()) {
                Some(ClickType::Face) => {
                    println!("Resetting game");
                    field = MineSweeper::new(NUM_COLS, NUM_ROWS, NUM_MINES);
                    face_state = Face::Smile;
                }

                _ => {}
            }
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            match mouse_position_to_cell(mouse_position()) {
                Some(ClickType::Cell(x, y)) => {
                    println!("Revealing cell at ({}, {})", x, y);
                    if field.reveal(x, y, 0) {
                        field.reveal_all();
                        face_state = Face::Lost;
                    }
                }

                _ => {}
            }
        }

        if is_mouse_button_pressed(MouseButton::Right) {
            if
                let Some(ClickType::Cell(mouse_x, mouse_y)) = mouse_position_to_cell(
                    mouse_position()
                )
            {
                println!("Flagging cell at ({}, {})", mouse_x, mouse_y);
                field.flag(mouse_x, mouse_y);
            }
        }

        if field.is_win(NUM_MINES) {
            println!("Game over!");
            field.reveal_all();
            face_state = Face::Won;
        }

        // Draw field
        for y in 0..field.field.len() {
            for x in 0..field.field[0].len() {
                draw_texture_ex(
                    &atlas,
                    (x as f32) * SQUARE_WIDTH,
                    (y as f32) * SQUARE_WIDTH + FACE_WIDTH,
                    WHITE,
                    DrawTextureParams {
                        source: Some(cell_to_image(&field.field[y][x])),
                        dest_size: Some(Vec2::new(SQUARE_WIDTH, SQUARE_WIDTH)),
                        ..Default::default()
                    }
                );
            }
        }

        // Draw face
        draw_texture_ex(
            &face,
            ((NUM_COLS as f32) * SQUARE_WIDTH) / 2.0 - (FACE_WIDTH as f32) / 2.0,
            0.0,
            WHITE,
            DrawTextureParams {
                source: Some(face_to_image(&face_state)),
                dest_size: Some(Vec2::new(FACE_WIDTH, FACE_WIDTH)),
                ..Default::default()
            }
        );

        next_frame().await;
    }
}
