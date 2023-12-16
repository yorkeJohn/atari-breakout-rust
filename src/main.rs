use macroquad::color::{Color, WHITE};
use macroquad::input::{is_key_pressed, is_mouse_button_pressed, KeyCode, MouseButton, show_mouse};
use macroquad::ui::{root_ui, Skin};
use macroquad::window::{Conf, next_frame};

use crate::breakout::{Breakout, GameState};

mod breakout;

fn window_conf() -> Conf {
    Conf {
        window_title: "Breakout!".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}

const FONT_SIZE: u16 = 56;

#[macroquad::main(window_conf)]
async fn main() {
    let skin = skin(FONT_SIZE);
    root_ui().push_skin(&skin);

    let mut game = Breakout::new(FONT_SIZE);

    loop {
        if game.game_state != GameState::Paused {
            game.update();
        }
        if game.game_state != GameState::Playing {
            game.exit_button();
        }

        handle_mouse_click(&mut game);
        handle_key(&mut game);
        show_mouse(game.game_state != GameState::Playing);

        game.draw();

        next_frame().await
    }

    fn handle_mouse_click(game: &mut Breakout) {
        if is_mouse_button_pressed(MouseButton::Left) && game.game_state != GameState::Playing {
            if game.game_state == GameState::GameOver || game.game_state == GameState::Win {
                *game = Breakout::new(FONT_SIZE);
            }
            game.game_state = GameState::Playing;
        }
    }

    fn handle_key(game: &mut Breakout) {
        if is_key_pressed(KeyCode::Escape) {
            game.game_state = match game.game_state.clone() {
                GameState::Playing => GameState::Paused,
                GameState::Paused => GameState::Playing,
                other => other
            };
        }
    }

    fn skin(font_size: u16) -> Skin {
        let transparent = Color::from_rgba(0, 0, 0, 0);
        let label_style = root_ui().style_builder()
            .font_size(font_size)
            .text_color(WHITE)
            .build();
        let button_style = root_ui().style_builder()
            .font_size(font_size)
            .color(transparent)
            .color_hovered(transparent)
            .text_color(WHITE)
            .text_color_hovered(WHITE)
            .build();
        Skin {
            label_style,
            button_style,
            ..root_ui().default_skin()
        }
    }
}
