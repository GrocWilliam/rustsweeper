use macroquad::{
    color::{BLACK, GREEN, LIGHTGRAY, RED, SKYBLUE, WHITE},
    input::mouse_position,
    math::Rect,
    shapes::{draw_rectangle, draw_rectangle_lines},
    text::{Font, TextParams, draw_text, draw_text_ex},
};

use crate::board::{Board, Cell, CellState};

pub const CELL_SIZE: f32 = 32.0;
pub const MARGIN: f32 = 8.0;

pub const WINDOW_HEIGHT: f32 = CELL_SIZE * 9.0 + MARGIN * 2.0;
pub const WINDOW_WIDTH: f32 = CELL_SIZE * 9.0 + MARGIN * 2.0;

fn center_text(x: f32, y: f32) -> (f32, f32) {
    (x + 6.0, y + CELL_SIZE - 6.0)
}

fn to_screen(x: usize, y: usize) -> (f32, f32) {
    (MARGIN + x as f32 * CELL_SIZE, MARGIN + y as f32 * CELL_SIZE)
}

fn to_grid(mx: f32, my: f32) -> Option<(usize, usize)> {
    let x = ((mx - MARGIN) / CELL_SIZE) as i32;
    let y = ((my - MARGIN) / CELL_SIZE) as i32;

    Some((x.max(0) as usize, y.max(0) as usize))
}

fn cell_rect(x: usize, y: usize) -> Rect {
    let (screen_x, screen_y) = to_screen(x, y);
    Rect::new(screen_x, screen_y, CELL_SIZE, CELL_SIZE)
}

fn draw_cell(cell: &Cell, x: usize, y: usize, font: &Font) -> () {
    let text_params: TextParams = TextParams {
        font_size: 22,
        color: BLACK,
        font: Some(font),
        ..Default::default()
    };

    let rect = cell_rect(x, y);
    let (text_x, text_y) = center_text(rect.x, rect.y);

    match cell.state {
        CellState::Hidden => {
            if cell.flagged {
                draw_rectangle(rect.x, rect.y, rect.w, rect.h, SKYBLUE);
                draw_text_ex("F", text_x, text_y, text_params);
            } else {
                draw_rectangle(rect.x, rect.y, rect.w, rect.h, LIGHTGRAY);
            }
        }
        CellState::Revealed => {
            draw_rectangle(rect.x, rect.y, rect.w, rect.h, WHITE);

            if cell.mined {
                draw_text_ex("X", text_x, text_y, text_params);
            } else {
                draw_text(
                    cell.mined_neightbors_count.to_string(),
                    rect.x + 6.0,
                    rect.y + CELL_SIZE - 6.0,
                    18.0,
                    BLACK,
                );
            }
        }
        CellState::Detonated => {
            draw_rectangle(rect.x, rect.y, rect.w, rect.h, RED);
            draw_text_ex("X", text_x, text_y, text_params);
        }
    }

    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2.0, WHITE);
}

pub fn draw_board(board: &Board, font: &Font) {
    for x in 0..board.width {
        for y in 0..board.height {
            if let Some(cell) = board.get(x, y) {
                draw_cell(cell, x, y, font);
            }
        }
    }
}

pub fn clicked_cell() -> Option<(usize, usize)> {
    let (mx, my) = mouse_position();
    to_grid(mx, my)
}

pub fn draw_win() {
    draw_text(
        "You win !",
        WINDOW_WIDTH / 2.0,
        WINDOW_HEIGHT / 2.0,
        32.0,
        GREEN,
    );
}

pub fn draw_lose() {
    draw_text(
        "You lose !",
        WINDOW_WIDTH / 2.0,
        WINDOW_HEIGHT / 2.0,
        32.0,
        RED,
    );
}
