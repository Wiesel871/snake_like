use crate::{alpha_print, geometry as geo};

use std::collections::VecDeque;

use rand::{Rng, SeedableRng};

mod level;

use minifb::{Key, Window, WindowOptions, Menu, MenuItem};

const FPS: usize = 165;
const SPEED: usize = 2;


const BACK_COL: u32 = geo::color::WHITE;
const BODY_COL: u32 = geo::color::GREY;
const APPL_COL: u32 = geo::color::RED;
const WALL_COL: u32 = geo::color::BLACK;


#[derive(Clone, Debug)]
struct Snake {
    scales: std::collections::VecDeque<geo::Point>,
    dir: geo::Direction,
    last_dir: geo::Direction,
    buf: geo::DrawBuffer,
    score: u32,
    rng: rand::prelude::StdRng,
    alive: bool
}

impl Snake {
    pub fn gen_pickups(self: &mut Self) {
        let mut p = geo::Point::new(self.rng.gen(), self.rng.gen());
        self.buf.normalize(&mut p);
        while self.buf.get(p.x, p.y) != BACK_COL {
            p = geo::Point::new(self.rng.gen(), self.rng.gen());
            self.buf.normalize(&mut p);
        }
        self.buf.set(p.x, p.y, APPL_COL);
    }

    pub fn new(
        lvl: &mut level::Level,
        comp: bool, 
        ) -> Snake {
        let mut res = Snake { 
            scales: VecDeque::new(), 
            buf: geo::DrawBuffer::new(lvl.width, lvl.height, BACK_COL), 
            dir: lvl.start_dir, 
            last_dir: lvl.start_dir,
            score: 0,
            rng: if comp {
                rand::prelude::StdRng::seed_from_u64(0)
            } else {
                rand::prelude::StdRng::seed_from_u64(0)
            },
            alive: true,
        };
        res.buf.set(lvl.snake_start.x, lvl.snake_start.y, res.head_color());
        res.scales.push_back(lvl.snake_start.clone());
        let op = lvl.start_dir.oposite();
        for _ in 1..lvl.snake_len {
            lvl.snake_start.shift(op);
            res.buf.normalize(&mut lvl.snake_start);
            res.buf.set(lvl.snake_start.x, lvl.snake_start.y, BODY_COL);
            res.scales.push_back(lvl.snake_start.clone());
        }
        for wall in lvl.walls.iter() {
            wall.draw(&mut res.buf, WALL_COL);
        }
        assert!(res.scales.len() == lvl.snake_len);
        res.gen_pickups();
        return res;
    }

    pub fn head_color(self: &Self) -> u32 {
        return match self.dir {
            geo::Direction::Left => geo::color::BLUE,
            geo::Direction::Right => geo::color::PURPLE,
            geo::Direction::Up => geo::color::YELLOW,
            geo::Direction::Down => geo::color::ORANGE,
        };
    }

    pub fn shift_draw(self: &mut Self) {
        let last_head = self.scales.front().unwrap().clone();
        let mut new_head = last_head.shifted(self.dir);
        self.last_dir = self.dir;
        self.buf.normalize(&mut new_head);
        let nxt_tile = self.buf.get(new_head.x, new_head.y);

        if nxt_tile == APPL_COL {
            self.score += 1;
            self.gen_pickups();
        } else if nxt_tile == BODY_COL || nxt_tile == WALL_COL {
            self.alive = false;
            return;
        } else {
            let cur_back = self.scales.back().unwrap();
            self.buf.set(cur_back.x, cur_back.y, BACK_COL);
            self.scales.pop_back();
        }
        if !self.scales.is_empty() {
            self.buf.set(last_head.x, last_head.y, BODY_COL);
        }
        let s = ron::to_string(&new_head).unwrap();
        let d = ron::to_string(&self.dir).unwrap();
        println!("head: {}, dir: {}", s, d);

        self.buf.set(new_head.x, new_head.y, self.head_color());
        self.scales.push_front(new_head);
    }

    pub fn buf_as_vec_u32(self: &Self) -> &Vec<u32> {
        return self.buf.as_vec_u32();
    }

    pub fn parse_keys(self: &mut Self, keys: Vec<minifb::Key>) {
        for key in keys {
            let new_dir = match key {
                minifb::Key::Left   | minifb::Key::A => geo::Direction::Left,
                minifb::Key::Right  | minifb::Key::D => geo::Direction::Right,
                minifb::Key::Up     | minifb::Key::W => geo::Direction::Up,
                minifb::Key::Down   | minifb::Key::S => geo::Direction::Down,
                _ => self.dir,
            };
            if new_dir != self.last_dir.oposite() {
                self.dir = new_dir;
            }
        }
        let head = self.scales.front().unwrap().clone();
        self.buf.set(head.x, head.y, self.head_color());
    }

}

pub fn game_loop(pth: &str) {
    let font = alpha_print::font::Font::load("pixel.font");
    let mut lvl = level::Level::load(pth);
    println!("lvl: {}", ron::to_string(&lvl).unwrap());
    let mut snake = Snake::new(
        &mut lvl,
        true
    );

    let mut opts = WindowOptions::default();
    opts.scale = minifb::Scale::FitScreen;
    opts.borderless = true;
    opts.scale_mode = minifb::ScaleMode::AspectRatioStretch;
    opts.resize = true;
    opts.topmost = true;

    let mut window = Window::new(
        "Test - ESC to exit",
        1920,
        1080,
        opts,
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    window.set_background_color(0xff, 0xff, 0xff);

    /*
    let mut help = Menu::new("help").unwrap();
    let wall = MenuItem::new("black - wall", 0);
    help.add_menu_item(&wall);
    let body = MenuItem::new("grey - body", 1);
    help.add_menu_item(&body);
    let head = MenuItem::new("blue - head", 2);
    help.add_menu_item(&head);
    let apll = MenuItem::new("red - apple", 3);
    help.add_menu_item(&apll);
    window.add_menu(&help);
    */

    window.set_target_fps(FPS);
    window
        .update_with_buffer(snake.buf_as_vec_u32(), lvl.width as usize, lvl.height as usize)
        .unwrap();
    let mut cur_frame: usize = 0;
    while snake.alive && window.is_open() && !window.is_key_down(Key::Escape) {
        if cur_frame == FPS / SPEED {
            cur_frame = 0;
            snake.shift_draw();
        } else {
            cur_frame += 1;
        }
        snake.parse_keys(window.get_keys_pressed(minifb::KeyRepeat::Yes));
        window
            .update_with_buffer(snake.buf_as_vec_u32(), lvl.width as usize, lvl.height as usize)
            .unwrap();

    }
    println!("final score: {}", snake.score);

}
