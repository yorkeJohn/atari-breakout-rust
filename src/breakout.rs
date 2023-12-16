use std::process::exit;

use macroquad::color::{BLACK, Color, GREEN, ORANGE, RED, SKYBLUE, WHITE, YELLOW};
use macroquad::math::{Vec2, vec2};
use macroquad::prelude::{draw_rectangle_lines, mouse_position};
use macroquad::shapes::draw_rectangle;
use macroquad::text::get_text_center;
use macroquad::time::get_frame_time;
use macroquad::ui::root_ui;
use macroquad::window::{clear_background, screen_height, screen_width};

struct Rect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl Rect {
    fn from_vec(pos: Vec2, size: Vec2) -> Self {
        Self {
            x: pos.x,
            y: pos.y,
            width: size.x,
            height: size.y,
        }
    }

    fn intersects(&self, other: &Rect) -> bool {
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
    }
}

// game constants
const BRICK_COUNT: u8 = 14;
const BRICK_SIZE: Vec2 = vec2(50.0, 15.0);
const BRICK_GAP: f32 = 6.0;
const BRICK_ROWS: u8 = 8;
const GAME_WIDTH: f32 = BRICK_COUNT as f32 * (BRICK_SIZE.x + BRICK_GAP) - BRICK_GAP;
const PADDING: f32 = 150.0;
const BASE_SPEED: f32 = 0.5;
const BALL_SIZE: Vec2 = vec2(16.0, 16.0);

#[derive(PartialEq)]
pub struct Brick {
    pos: Vec2,
    row: u8,
}

impl Brick {
    fn new(row: u8, col: u8) -> Self {
        let x = col as f32 * (BRICK_SIZE.x + BRICK_GAP);
        let y = PADDING + (BRICK_ROWS - row - 1) as f32 * (BRICK_SIZE.y + BRICK_GAP);

        Self {
            pos: vec2(x, y),
            row,
        }
    }

    fn color(&self) -> Color {
        match self.row {
            0 | 1 => YELLOW,
            2 | 3 => GREEN,
            4 | 5 => ORANGE,
            6 | 7 => RED,
            _ => Color::default()
        }
    }

    fn point_value(&self) -> u16 {
        ((self.row / 2) as f64).floor() as u16 * 2 + 1
    }
}

#[derive(PartialEq, Clone)]
pub enum GameState {
    NewGame,
    Playing,
    Paused,
    GameOver,
    Win
}

pub struct Breakout {
    pub font_size: u16,
    pub game_state: GameState,
    pub bricks: Vec<Brick>,
    pub ball_pos: Vec2,
    pub ball_vel: Vec2,
    pub paddle_pos: Vec2,
    pub hit_paddle: bool,
    pub last_mouse_x: f32,
    pub score: u16,
    pub balls_rem: u8,
    pub game_count: u8,
}

impl Breakout {
    pub fn new(font_size: u16) -> Self {
        Self {
            font_size,
            game_state: GameState::NewGame,
            bricks: Breakout::bricks(),
            ball_pos: vec2(GAME_WIDTH / 2.0, screen_height() / 2.0),
            ball_vel: vec2(BASE_SPEED, BASE_SPEED),
            paddle_pos: vec2((GAME_WIDTH - BRICK_SIZE.x) / 2.0, screen_height() - PADDING),
            hit_paddle: false,
            last_mouse_x: 0.0,
            score: 0,
            balls_rem: 3,
            game_count: 0,
        }
    }

    fn bricks() -> Vec<Brick> {
        let mut list = Vec::new();
        for row in 0..BRICK_ROWS {
            for col in 0..BRICK_COUNT {
                list.push(Brick::new(row, col))
            }
        }
        list
    }

    pub fn exit_button(&self) {
        let text = "Exit Game";
        if root_ui().button(vec2(screen_width() / 2.0 - self.text_center(text).x, screen_height() - 300.0), text) {
            exit(0)
        }
    }

    pub fn update(&mut self) {
        self.handle_mouse_move();
        self.check_wall_collision();
        self.check_paddle_collision();
        self.check_brick_collision();
        self.update_ball();

        if self.bricks.len() == 0 {
            self.game_state = GameState::Win;
        }
    }

    fn handle_mouse_move(&mut self) {
        let mouse_x = mouse_position().0;
        let delta = mouse_x - self.last_mouse_x;
        self.paddle_pos.x = f32::min(f32::max(self.paddle_pos.x + delta, 0.0), GAME_WIDTH - BRICK_SIZE.x);
        self.last_mouse_x = mouse_x;
    }

    fn check_wall_collision(&mut self) {
        if self.ball_pos.x <= 0.0 || self.ball_pos.x >= GAME_WIDTH - BALL_SIZE.x {
            self.ball_vel.x *= -1.0;
        }
        if self.ball_pos.y <= 0.0 {
            self.ball_vel.y *= -1.0;
        }
    }

    fn check_paddle_collision(&mut self) {
        let ball_rect = Rect::from_vec(self.ball_pos, BALL_SIZE);
        let paddle_rect = if self.game_state == GameState::Playing {
            Rect::from_vec(self.paddle_pos, BRICK_SIZE)
        } else {
            Rect { x: 0.0, y: self.paddle_pos.y, width: GAME_WIDTH, height: BRICK_SIZE.y }
        };
        if ball_rect.intersects(&paddle_rect) {
            if !self.hit_paddle {
                self.hit_paddle = true;
                self.ball_vel.y *= -1.0;
            }
        } else {
            self.hit_paddle = false;
        }
    }

    fn check_brick_collision(&mut self) {
        for (i, brick) in self.bricks.iter().enumerate() {
            let ball_rect = Rect::from_vec(self.ball_pos, BALL_SIZE);
            let brick_rect = Rect::from_vec(brick.pos, BRICK_SIZE);
            if ball_rect.intersects(&brick_rect) {
                self.ball_vel.y *= -1.0;
                if self.game_state == GameState::Playing {
                    self.score += brick.point_value();
                    self.bricks.remove(i);
                }
                break;
            }
        }
    }

    fn update_ball(&mut self) {
        self.ball_pos += self.ball_vel * get_frame_time() * 1000.0;

        if self.ball_pos.y >= self.paddle_pos.y {
            self.ball_pos = vec2(GAME_WIDTH / 2.0, screen_height() / 2.0);
            self.balls_rem -= 1;
            if self.balls_rem == 0 {
                self.game_state = GameState::GameOver;
            }
        }
    }

    pub fn draw(&self) {
        clear_background(BLACK);
        let offset = (screen_width() - GAME_WIDTH) / 2.0;

        // border
        draw_rectangle_lines(offset - 8.0, 0.0, GAME_WIDTH + 16.0, screen_height(), 16.0, WHITE);
        // paddle
        if self.game_state == GameState::Playing {
            draw_rectangle(offset + self.paddle_pos.x, self.paddle_pos.y, BRICK_SIZE.x, BRICK_SIZE.y, SKYBLUE);
        } else {
            draw_rectangle(offset, self.paddle_pos.y, GAME_WIDTH, BRICK_SIZE.y, SKYBLUE);
        }
        // ball
        draw_rectangle(offset + self.ball_pos.x, self.ball_pos.y, BALL_SIZE.x, BALL_SIZE.y, WHITE);

        // bricks
        for brick in &self.bricks {
            draw_rectangle(offset + brick.pos.x, brick.pos.y, BRICK_SIZE.x, BRICK_SIZE.y, brick.color());
        }

        // score and balls rem
        root_ui().label(vec2(offset + 16.0, 32.0), &*format!("{:03}", self.score));
        root_ui().label(vec2(offset + GAME_WIDTH - 100.0, 32.0), &*self.balls_rem.to_string());

        // info text
        match self.game_state {
            GameState::Paused => self.draw_paused_text(),
            GameState::NewGame => self.draw_new_game_text(),
            GameState::GameOver => self.draw_game_over_text(),
            GameState::Win => self.draw_win_text(),
            GameState::Playing => {}
        }
    }

    pub fn draw_new_game_text(&self) {
        let text = "Click anywhere to play";
        root_ui().label(vec2(screen_width() / 2.0 - self.text_center(text).x, screen_height() / 2.0 - 64.0), text);
    }

    pub fn draw_paused_text(&self) {
        let text = "Game paused";
        root_ui().label(vec2(screen_width() / 2.0 - self.text_center(text).x, screen_height() / 2.0 - 64.0), text);
        let text = "Click anywhere to resume";
        root_ui().label(vec2(screen_width() / 2.0 - self.text_center(text).x, screen_height() / 2.0), text);
    }

    pub fn draw_game_over_text(&self) {
        let text = "Game over!";
        root_ui().label(vec2(screen_width() / 2.0 - self.text_center(text).x, screen_height() / 2.0 - 64.0), text);
        let text = "Click anywhere to play again";
        root_ui().label(vec2(screen_width() / 2.0 - self.text_center(text).x, screen_height() / 2.0), text);
    }

    pub fn draw_win_text(&self) {
        let text = "You win!";
        root_ui().label(vec2(screen_width() / 2.0 - self.text_center(text).x, screen_height() / 2.0 - 64.0), text);
        let text = "Click anywhere to play again";
        root_ui().label(vec2(screen_width() / 2.0 - self.text_center(text).x, screen_height() / 2.0), text);
    }

    fn text_center(&self, text: &str) -> Vec2 {
        get_text_center(text, None, self.font_size, 1.0, 0.0)
    }
}

