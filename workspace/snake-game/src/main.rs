// This a snake game.
//
// To run it:
//      cargo run
//
// It was initially based on "Making a Snake Game in Rust" tutorial:
// https://www.youtube.com/watch?v=HCwMb0KslX8

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, GlyphCache, OpenGL, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use piston::{Button, ButtonEvent, ButtonState, EventLoop, Key};
use rand::{thread_rng, Rng};

use std::collections::LinkedList;
use std::iter::FromIterator;

const BLOCK_SIZE: f64 = 20.0;

const BG_COLOR: [f32; 4] = [0.1568627451, 0.1725490196, 0.1725490196, 1.0];
const SNAKE_COLOR: [f32; 4] = [0.644, 0.776, 0.516, 1.0];
const FOOD_COLOR: [f32; 4] = [0.928, 0.408, 0.452, 1.0];
const TEXT_COLOR: [f32; 4] = [0.996, 0.996, 1.0, 1.0];

#[derive(Debug)]
enum SnakeUpdateError {
    OutOfBounds,
    HitItSelf,
}

#[derive(Clone, PartialEq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

struct Game {
    gl: GlGraphics,
    snake: Snake,
    food: Food,
    running: bool,
    ranking: Option<u32>,
}

impl Game {
    fn render(&mut self, args: &RenderArgs, glyphs: &mut GlyphCache) {
        // TODO: Could we stop drawing if there are no changes, especially for the home page
        // while it is waiting for the user input?
        self.gl.draw(args.viewport(), |_c, gl| {
            graphics::clear(BG_COLOR, gl);
        });

        if !self.running {
            self.render_home(args, glyphs);
            return;
        }

        self.food.render(&mut self.gl, args);

        self.snake.render(&mut self.gl, args);
    }

    fn render_home(&mut self, args: &RenderArgs, glyphs: &mut GlyphCache) {
        self.gl.draw(args.viewport(), |c, gl| {
            match self.ranking {
                Some(ranking) => {
                    graphics::text::Text::new_color(TEXT_COLOR, 20)
                        .draw_pos(
                            "Game over",
                            [20.0, 40.0],
                            glyphs,
                            &c.draw_state,
                            c.transform,
                            gl,
                        )
                        .unwrap();

                    let points = self.snake.body.len() - 2;

                    graphics::text::Text::new_color(TEXT_COLOR, 16)
                        .draw_pos(
                            &format!("Points: {}", points).to_owned(),
                            [20.0, 70.0],
                            glyphs,
                            &c.draw_state,
                            c.transform,
                            gl,
                        )
                        .unwrap();

                    graphics::text::Text::new_color(TEXT_COLOR, 16)
                        .draw_pos(
                            &format!("Ranking: {}", ranking).to_owned(),
                            [20.0, 90.0],
                            glyphs,
                            &c.draw_state,
                            c.transform,
                            gl,
                        )
                        .unwrap();
                }
                None => {
                    graphics::text::Text::new_color(TEXT_COLOR, 20)
                        .draw_pos(
                            "Snake game",
                            [20.0, 50.0],
                            glyphs,
                            &c.draw_state,
                            c.transform,
                            gl,
                        )
                        .unwrap();
                }
            }

            graphics::text::Text::new_color(TEXT_COLOR, 14)
                .draw_pos(
                    "Press:",
                    [20.0, 120.0],
                    glyphs,
                    &c.draw_state,
                    c.transform,
                    gl,
                )
                .unwrap();

            graphics::text::Text::new_color(TEXT_COLOR, 14)
                .draw_pos(
                    "ENTER to start",
                    [20.0, 138.0],
                    glyphs,
                    &c.draw_state,
                    c.transform,
                    gl,
                )
                .unwrap();

            graphics::text::Text::new_color(TEXT_COLOR, 14)
                .draw_pos(
                    "ESC to quit",
                    [20.0, 156.0],
                    glyphs,
                    &c.draw_state,
                    c.transform,
                    gl,
                )
                .unwrap();
        });
    }

    fn update(&mut self) -> Result<(), SnakeUpdateError> {
        if !self.running {
            return Ok(());
        }

        match self.snake.update(self.food.at) {
            Ok(eat) => {
                if eat {
                    self.food.update(&self.snake)
                }
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    fn pressed(&mut self, btn: &Button) {
        if !self.running && btn == &Button::Keyboard(Key::Return) {
            // restart game
            self.running = true;
            self.snake = Snake {
                body: LinkedList::from_iter((vec![(0, 0), (0, 1)]).into_iter()),
                dir: Direction::Right,
            };
            return;
        }

        let last_direction = self.snake.dir.clone();

        self.snake.dir = match btn {
            &Button::Keyboard(Key::Up) if last_direction != Direction::Down => Direction::Up,
            &Button::Keyboard(Key::Down) if last_direction != Direction::Up => Direction::Down,
            &Button::Keyboard(Key::Left) if last_direction != Direction::Right => Direction::Left,
            &Button::Keyboard(Key::Right) if last_direction != Direction::Left => Direction::Right,
            _ => last_direction,
        }
    }
}

struct Food {
    at: (i32, i32),
}

impl Food {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        let square = graphics::rectangle::square(
            self.at.0 as f64 * BLOCK_SIZE,
            self.at.1 as f64 * BLOCK_SIZE,
            BLOCK_SIZE,
        );

        gl.draw(args.viewport(), |c, gl| {
            graphics::rectangle(FOOD_COLOR, square, c.transform, gl);
        });
    }

    fn update(&mut self, snake: &Snake) {
        // Find new position left in the board
        loop {
            let new_position = (thread_rng().gen_range(0..10), thread_rng().gen_range(0..10));

            let in_use = snake.is_at(new_position);

            if !in_use {
                self.at = new_position;
                break;
            }
        }
    }
}

struct Snake {
    body: LinkedList<(i32, i32)>,
    dir: Direction,
}

impl Snake {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        let squares: Vec<graphics::types::Rectangle> = self
            .body
            .iter()
            .map(|&(x, y)| {
                graphics::rectangle::square(
                    x as f64 * BLOCK_SIZE,
                    y as f64 * BLOCK_SIZE,
                    BLOCK_SIZE,
                )
            })
            .collect();

        gl.draw(args.viewport(), |c, gl| {
            squares
                .into_iter()
                .for_each(|square| graphics::rectangle(SNAKE_COLOR, square, c.transform, gl));
        })
    }

    fn update(&mut self, food_at: (i32, i32)) -> Result<bool, SnakeUpdateError> {
        let mut new_head = (*self.body.front().expect("Snake has no body")).clone();

        match self.dir {
            Direction::Left => new_head.0 -= 1,
            Direction::Right => new_head.0 += 1,
            Direction::Up => new_head.1 -= 1,
            Direction::Down => new_head.1 += 1,
        }

        let x = new_head.0;
        let y = new_head.1;
        if x < 0 || y < 0 || x >= 10 || y >= 10 {
            return Err(SnakeUpdateError::OutOfBounds);
        }

        if self.is_at(new_head) {
            return Err(SnakeUpdateError::HitItSelf);
        }

        self.body.push_front(new_head);

        // Remove the snake last part if it didn't pass over the food position.
        if x != food_at.0 || y != food_at.1 {
            self.body.pop_back().unwrap();
            Ok(false)
        } else {
            Ok(true)
        }
    }

    fn is_at(&self, position: (i32, i32)) -> bool {
        self.body
            .iter()
            .any(|&(x, y)| x == position.0 && y == position.1)
    }
}

fn main() {
    println!("Starting snake game...");

    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("Snake Game", [200.0, 200.0])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();

    let mut game = Game {
        ranking: None,
        running: false,
        gl: GlGraphics::new(opengl),
        food: Food { at: (5, 5) },
        snake: Snake {
            body: LinkedList::from_iter((vec![(0, 0), (0, 1)]).into_iter()),
            dir: Direction::Right,
        },
    };

    let mut glyphs = GlyphCache::new(
        "/System/Library/Fonts/Avenir.ttc",
        (),
        TextureSettings::new(),
    )
    .unwrap();

    let mut events = Events::new(EventSettings::new()).ups(8);

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            game.render(&args, &mut glyphs);
        }

        if let Some(_args) = e.update_args() {
            if let Err(_e) = game.update() {
                game.running = false;

                // Record the last game result if it is higher than the
                // current ranking.
                let last_game_points = (game.snake.body.len() - 2) as u32;
                game.ranking = match game.ranking {
                    Some(ranking) => {
                        if last_game_points > ranking {
                            Some(last_game_points)
                        } else {
                            Some(ranking)
                        }
                    }
                    None => Some(last_game_points),
                }
            }
        }

        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                game.pressed(&k.button);
            }
        }
    }
}
