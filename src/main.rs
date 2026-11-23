use macroquad::prelude::*;
use std::default::Default;

use crate::{
    board::Board,
    view::{CELL_SIZE, MARGIN, clicked_cell, draw_board},
};

mod board;
mod view;

#[macroquad::main(window_conf)]
async fn main() {
    let mut board = Board::new(9, 9, 3);
    board.generate_cells();

    let font = load_ttf_font("assets/NotoColorEmoji-Regular.ttf")
        .await
        .expect("Impossible de charger la police");

    loop {
        clear_background(WHITE);

        if let Some((gx, gy)) = clicked_cell() {
            if is_mouse_button_pressed(MouseButton::Left) {
                board.reveal(gx, gy);
            } else if is_mouse_button_pressed(MouseButton::Right) {
                board.toggle_flag(gx, gy);
            }
        }

        draw_board(&board, &font);

        next_frame().await
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Rustsweeper".to_owned(),
        window_height: CELL_SIZE as i32 * 9 + MARGIN as i32 * 2,
        window_width: CELL_SIZE as i32 * 9 + MARGIN as i32 * 2,
        ..Default::default()
    }
}
