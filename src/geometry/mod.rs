use std::ops::{Add, Sub};

mod math;
use math::n_mod_m;

pub mod color;


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub const fn new(x: isize, y: isize) -> Point {
        return Point {x, y};
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
    
}

impl Point {
    pub fn shift(&mut self, dir: Direction) {
        match dir {
            Direction::Left     => self.x -= 1,
            Direction::Right    => self.x += 1,
            Direction::Up       => self.y -= 1,
            Direction::Down     => self.y += 1,
        };
    }

    pub fn shifted(&self, dir: Direction) -> Point {
        return match dir {
            Direction::Left     => Point { x: self.x - 1, y: self.y},
            Direction::Right    => Point { x: self.x + 1, y: self.y},
            Direction::Up       => Point { x: self.x, y: self.y - 1},
            Direction::Down     => Point { x: self.x, y: self.y + 1},
        }
    }
}

#[derive(Clone, Debug)]
pub struct DrawBuffer {
    buf: Vec<u32>,
    bounds: Point,
}

impl DrawBuffer {
    pub fn new(x: isize, y: isize, col: u32) -> DrawBuffer {
        return DrawBuffer { buf: vec![col; (x * y) as usize], bounds: Point::new(x, y)};
    }

    pub fn get(&self, x: isize, y: isize) -> u32 {
        return self.buf[(y * self.bounds.x + x) as usize];
    }

    pub fn set(&mut self, x: isize, y: isize, col: u32) {
        self.buf[(y * self.bounds.x + x) as usize] = col;
    }

    pub fn set_with_sc(&mut self, x: isize, y: isize, sc: isize, col: u32) {
        for x in x..x + sc {
            for y in y..y + sc {
                self.set(x, y, col);
            }
        }
    }

    pub fn as_vec_u32(&self) -> &Vec<u32> {
        return &self.buf;
    }

    pub fn normalize(&mut self, p: &mut Point) {
        p.x = self.normalized_x(p.x);
        p.y = self.normalized_y(p.y);
    }

    fn normalized_x(&self, x: isize) -> isize {
        return n_mod_m(x, self.bounds.x);
    }

    fn normalized_y(&self, y: isize) -> isize {
        return n_mod_m(y, self.bounds.y);
    }

    pub fn draw_point(&mut self, st: Point, sc: isize, col: u32) {
        self.set_with_sc(st.x, st.y, sc, col);
    }

    pub fn draw_line(&mut self, st: Point, ls: Point, sc: isize, col: u32) {
        if st.x != ls.x && st.y != ls.y {
            todo!();
        }
        if st.x != ls.x {
            let mut x = st.x;
            while x != ls.x {
                self.set_with_sc(x, st.y, sc, col);
                x = self.normalized_x(x + 1);
            }
            self.set_with_sc(x, st.y, sc, col);
        } else {
            let mut y = st.y;
            while y != ls.y {
                self.set_with_sc(st.x, y, sc, col);
                y = self.normalized_y(y + 1);
            }
            self.set_with_sc(st.x, y, sc, col);
        }
    }

    pub fn draw_rect(&mut self, min: Point, max: Point, fil: bool, sc: isize, col: u32) {
        if fil {
            todo!();
        }
        let mut x = min.y;
        while x != max.x {
            self.set_with_sc(x, min.y, sc, col);
            self.set_with_sc(x, max.y, sc, col);
            x = self.normalized_x(x + 1);
        }
        self.set_with_sc(x, min.y, sc, col);
        self.set_with_sc(x, max.y, sc, col);

        let mut y = min.y;
        while y != max.y {
            self.set_with_sc(min.x, y, sc, col);
            self.set_with_sc(max.x, y, sc, col);
            y = self.normalized_y(y + 1);
        }
        self.set_with_sc(min.x, y, sc, col);
        self.set_with_sc(max.x, y, sc, col);
    }

}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down
}

impl Direction {
    pub fn oposite(&self) -> Direction {
        return match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        };
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Shape {
    Point(Point),
    Line(Point, Point),
    Rectangle(Point, Point, bool),
}

impl Shape {
    pub fn new_point(x: isize, y: isize) -> Shape {
        return Shape::Point(Point::new(x, y));
    }

    pub fn new_line(stx: isize, sty: isize, lsx: isize, lsy: isize) -> Shape {
        return Shape::Line(Point::new(stx, sty), Point::new(lsx, lsy));
    }

    pub fn new_rect(minx: isize, miny: isize, maxx: isize, maxy: isize, filled: bool) -> Shape {
        return Shape::Rectangle(Point::new(minx, miny), Point::new(maxx, maxy), filled);
    }

    pub fn draw(&self, buf: &mut DrawBuffer, col: u32) {
        match self {
            Shape::Point(p) => buf.draw_point(p.clone(), 1, col),
            Shape::Line(st, ls) => buf.draw_line(st.clone(), ls.clone(), 1, col),
            Shape::Rectangle(st, ls, fil) => buf.draw_rect(st.clone(), ls.clone(), fil.clone(), 1, col),
        };
    }

}