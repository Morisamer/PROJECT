use piston_window::types::Color;
use piston_window::*;
use rand::{thread_rng, Rng};

use crate::snake::{Direction, Snake};
use crate::drawing::{draw_block, draw_rectange};

const FOOD_COLOR: Color = [0.90, 0.49, 0.13, 1.0];
const BORDER_COLOR: Color = [0.741, 0.765, 0.78, 1.0];
const GAMEOVER_COLOR: Color = [0.91, 0.30, 0.24, 0.5];

const MOVING_PERIOD: f64 = 0.2;
const RESTART_TIME: f64 = 1.0; 
impl Game {
    pub fn get_dimensions(&self) -> (i32, i32) {
        (self.width, self.height) // Предположим, что у вас есть width и height как поля структуры
    }
}
pub struct Game {
    snake: Snake,

    food_exist: bool,
    food_x: i32,
    food_y: i32,

    width: i32,
    height: i32,

    is_game_over: bool,
    waiting_time: f64,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            snake: Snake::new(2, 2),
            waiting_time: 0.0,
            food_exist: true,
            food_x: 5,
            food_y: 3,
            width: width,
            height: height,
            is_game_over: false,
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        if self.is_game_over {
            return;
        }

        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => return,
        };

        if dir.unwrap() == self.snake.head_direction().opposite() {
            return;
        }

        self.update_snake(dir);
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        self.snake.draw(con, g);

        if self.food_exist {
            draw_block(FOOD_COLOR, self.food_x, self.food_y, con, g);
        }

        draw_rectange(BORDER_COLOR, 0, 0, self.width, 1, con, g);
        draw_rectange(BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g);
        draw_rectange(BORDER_COLOR, 0, 0, 1, self.height, con, g);
        draw_rectange(BORDER_COLOR, self.width - 1, 0, 1, self.height, con, g);
        if self.is_game_over {
            draw_rectange(GAMEOVER_COLOR, 0, 0, self.width, self.height, con, g);
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;
        if self.is_game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }
        if !self.food_exist {
            self.add_food();
        }
        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None);
        }
    }

    fn check_eating(&mut self) {
        let (head_x, head_y): (i32, i32) = self.snake.head_position();
        if self.food_exist && self.food_x == head_x && self.food_y == head_y {
            self.food_exist = false;
            self.snake.restore_last_removed();
        }
    }

    fn check_if_the_snake_alive(&self, dir: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.next_head_position(dir);
        if self.snake.is_overlap_except_tail(next_x, next_y) {
            return false;
        }
        next_x > 0 && next_y > 0 && next_x < self.width - 1 && next_y < self.height - 1
    }

    fn add_food(&mut self) {
        let mut rng = thread_rng();
        let mut new_x = rng.gen_range(1..(self.width - 1));
        let mut new_y = rng.gen_range(1..(self.height - 1));
        while self.snake.is_overlap_except_tail(new_x, new_y) {
            new_x = rng.gen_range(1..(self.width - 1));
            new_y = rng.gen_range(1..(self.height - 1));
        }
        self.food_x = new_x;
        self.food_y = new_y;
        self.food_exist = true;
    }

    fn update_snake(&mut self, dir: Option<Direction>) {
        if self.check_if_the_snake_alive(dir) {
            self.snake.move_forward(dir);
            self.check_eating();
        } else {
            self.is_game_over = true;
        }
        self.waiting_time = 0.0;
    }

    fn restart(&mut self) {
        self.snake = Snake::new(2, 2);
        self.waiting_time = 0.0;
        self.food_exist = true;
        self.food_x = 5;
        self.food_y = 3;
        self.is_game_over = false;
    }
}