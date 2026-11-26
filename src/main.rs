use macroquad::prelude::*;
use std::default::Default;

use crate::{
    board::Board,
    view::{WINDOW_HEIGHT, WINDOW_WIDTH, clicked_cell, draw_board, draw_lose, draw_win},
};

mod board;
mod view;

#[macroquad::main(window_conf)]
async fn main() {
    let mut board = Board::new(9, 9, 9);
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

        if board.has_win() {
            draw_win();
        } else if board.has_lose() {
            draw_lose();
        }

        next_frame().await
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Rustsweeper".to_owned(),
        window_height: WINDOW_HEIGHT as i32,
        window_width: WINDOW_WIDTH as i32,
        ..Default::default()
    }
}
