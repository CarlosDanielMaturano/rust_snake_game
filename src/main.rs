extern crate piston_window;
use piston_window::{types::Color, *};
use rand::Rng;
use std:: collections::LinkedList;
use Button::Keyboard;

const DEFAULT_WINDOW_SIZE: [u32; 2] = [400, 400];
const UPS_COUNT: u64 = 8;
const BACKGROUND_COLOR: Color = color::WHITE;
const SNAKE_SPAWN: [f64; 2] = [40.0, 20.0];
const DEFAULT_BLOCK_SIZE: f64 = 20.0;

#[derive(Copy, Clone)]
struct Block {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
    color: Color,
}

impl Block {
    fn new(x: f64, y: f64, w: f64, h: f64, color: Color) -> Block {
        Block { x, y, w, h, color }
    }
    fn draw<G: Graphics>(&self, c: &Context, g: &mut G) {
        rectangle(self.color, [self.x, self.y, self.w, self.h], c.transform, g);
    }
    fn check_collision(&self, block: &Block) -> bool {
        self.x == block.x && self.y == block.y
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Snake {
    body: LinkedList<Block>,
    direction: [f64; 2],
    food: Block,
}

impl Snake {
    fn new(default_color: Color) -> Snake {
        let mut snake = Snake {
            body: LinkedList::new(),
            direction: [0.0, 1.0],
            food: Snake::spawn_food(),
        };
        let head = Block::new(
            SNAKE_SPAWN[0],
            SNAKE_SPAWN[1],
            DEFAULT_BLOCK_SIZE,
            DEFAULT_BLOCK_SIZE,
            default_color,
        );
        snake.body.push_front(head);
        for _ in 0..=2 {
            snake.grow();
        }
        snake
    }

    fn spawn_food() -> Block {
        let x = rand::thread_rng().gen_range(0..20) as f64
            * (DEFAULT_WINDOW_SIZE[0] as f64 / DEFAULT_BLOCK_SIZE);
        let y = rand::thread_rng().gen_range(0..20) as f64
            * (DEFAULT_WINDOW_SIZE[1] as f64 / DEFAULT_BLOCK_SIZE);


        Block::new(x, y, DEFAULT_BLOCK_SIZE, DEFAULT_BLOCK_SIZE, color::GREEN)
    }

    fn draw<G: Graphics>(&self, c: &Context, g: &mut G) {
        self.body.iter().for_each(|block| block.draw(c, g));
        self.food.draw(c, g);
    }
    fn grow(&mut self) {
        let old_head = self.body.front().expect("Snake has no body");
        let mut new_head = old_head.clone();
        new_head.x += DEFAULT_BLOCK_SIZE * self.direction[0];
        new_head.y += DEFAULT_BLOCK_SIZE * self.direction[1];

        if new_head.x >= DEFAULT_WINDOW_SIZE[0] as f64 {
            new_head.x = 0.0
        }
        if new_head.x < 0.0 as f64 {
            new_head.x = DEFAULT_WINDOW_SIZE[0] as f64
        }
        if new_head.y > DEFAULT_WINDOW_SIZE[1] as f64 {
            new_head.y = 0.0
        }
        if new_head.y < 0.0 as f64 {
            new_head.y = DEFAULT_WINDOW_SIZE[1] as f64
        }
        self.body.push_front(new_head);
    }

    fn check_collision_with_body(&mut self) -> bool {
        let head = self.body.front().expect("Snake has no head");
        self.body.iter().skip(1).any(|piece| piece.check_collision(&head))
    }

    fn update(&mut self) -> Result<bool, bool>{
        self.grow();
        self.body.pop_back().expect("Snake has no body");

        let head = self.body.front().expect("Snake has no head");

        if head.check_collision(&self.food) {
            self.grow();
            self.food = Snake::spawn_food();
        }

        if self.check_collision_with_body() {
            return Err(false)
        }
        
        Ok(true)

    }
    fn set_direction(&mut self, dir: Direction) {
        let moving_x = self.direction[0] != 0.0;
        let moving_y = self.direction[1] != 0.0;

        self.direction = match dir {
            Direction::Up if !moving_y => [0.0, -1.0],
            Direction::Down if !moving_y => [0.0, 1.0],
            Direction::Left if !moving_x => [-1.0, 0.0],
            Direction::Right if !moving_x => [1.0, 0.0],
            _ => self.direction,
        };
    }
}


fn main() {
    let mut window: PistonWindow = WindowSettings::new("Snake_Game", DEFAULT_WINDOW_SIZE)
        .resizable(false)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut window_events = Events::new(EventSettings::new()).ups(UPS_COUNT);

    let mut snake = Snake::new(color::RED);

    while let Some(event) = window_events.next(&mut window) {
        if let Some(_) = event.render_args() {
            window.draw_2d(&event, |context, graphics, _| {
                clear(BACKGROUND_COLOR, graphics);
                snake.draw(&context, graphics);
            });
        }
        if let Some(_) = event.update_args() {
            if let Err(_) = snake.update() {
                snake = Snake::new(color::RED);
            };
        }
        if let Some(key) = event.button_args() {
            if key.state == ButtonState::Press {
                match key.button {
                    Keyboard(Key::Up) | Keyboard(Key::W) => snake.set_direction(Direction::Up),
                    Keyboard(Key::Down) | Keyboard(Key::S) => snake.set_direction(Direction::Down),
                    Keyboard(Key::Left) | Keyboard(Key::A) => snake.set_direction(Direction::Left),
                    Keyboard(Key::Right) | Keyboard(Key::D) => {
                        snake.set_direction(Direction::Right)
                    }
                    _ => {}
                }
                continue;
            }
        }
    }

    println!("Hello, world!");
}
