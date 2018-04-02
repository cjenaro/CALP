use std::sync::mpsc::Receiver;
use std::clone::Clone;
use rand;
use rand::distributions::{IndependentSample, Range};
use std::collections::VecDeque;

pub const UP: Point = Point(-1, 0);
pub const DOWN: Point = Point(1, 0);
pub const LEFT: Point = Point(0, -1);
pub const RIGHT: Point = Point(0, 1);
const DIRECTIONS: [Point;4] =[UP, DOWN, LEFT, RIGHT];

pub const HEIGHT: i32 = 40;
pub const WIDTH: i32 = 40;

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Point(pub i32, pub i32);

impl Point {
    fn sum(&self, p: &Point) -> Point {
        let new_x = self.0 + p.0;
        let new_y = self.1 + p.1;
        Point(new_x, new_y)
    }
    fn minus(&self, p: &Point) -> Point {
        let new_x = self.0 - p.0;
        let new_y = self.1 - p.1;
        Point(new_x, new_y)
    }
    fn prod(&self, p: i32) -> Point {
        let new_x = self.0 * p;
        let new_y = self.1 * p;
        Point(new_x, new_y)
    }
}

pub struct Snake {
    body: VecDeque<Point>,
    direction : Point,
}

fn sum_points(p: &Point, q: &Point) -> Point {
    Point(p.0 + q.0, p.1 + q.1)
}

impl Snake {
    pub fn new() -> Snake {
        let between = Range::new(5, HEIGHT - 5);
        let mut rng = rand::thread_rng();
        let mut x = between.ind_sample(&mut rng);
        let mut y = between.ind_sample(&mut rng);
        let mut body: VecDeque<Point> = VecDeque::new();
        body.push_front(Point(x, y));

        let between_d = Range::new(0,3);
        let mut dx = between_d.ind_sample(&mut rng);
        let direction: Point = DIRECTIONS[dx];
        let mut sn = Snake { body, direction };
        for i in 1..4{
            //let tail: Point = d.prod(i);
            //b.push_front(head.minus(&tail));
            sn.grow_snake();
        }
        sn
    }

    pub fn move_snake(&mut self) {
        self.grow_snake();
        self.body.pop_back();
    }

    pub fn grow_snake(&mut self) {
        let new_hd = sum_points(&self.get_head(), &self.get_direction());
        self.body.push_front(new_hd);
    }

    pub fn get_body(&self) -> &VecDeque<Point> {
        &self.body
    }

    pub fn get_body_mut(&mut self) -> &mut VecDeque<Point> {
        &mut self.body
    }

    pub fn get_direction(&self) -> Point {
        self.direction
    }

    pub fn set_direction(&mut self, d: Point) {
        self.direction = d;
    }

    pub fn get_head(&self) -> Point {
        let hd = self.body.front().expect("No head!");
        *hd
    }
}

pub struct Player {
    channel: Receiver<Point>,
    snake: Snake,
    name: String,
    alive: bool
}

impl Player {
    pub fn new(c: Receiver<Point>, s: Snake, n: &str ) -> Player {
        Player { channel: c, snake: s, name: String::from(n), alive: true }
    }

    pub fn get_snake(&self) -> &Snake {
        &self.snake
    }

    pub fn get_snake_mut(&mut self) -> &mut Snake {
        &mut self.snake
    }

    pub fn get_channel(&self) -> &Receiver<Point> {
        &self.channel
    }

    pub fn get_name(&self) -> &str {
        &self.name[..]
    }

    pub fn get_alive(&self) -> bool {
        self.alive
    }

    pub fn kill(&mut self) {
        self.alive = false;
    }
}

pub struct World {
    wall: Vec<Point>,
    players: Vec<Player>,
    food: Point
}

impl World {
    pub fn new(w: Vec<Point>, p: Vec<Player>) -> World {
        World { wall: w, players: p, food: Point(HEIGHT/2, WIDTH/2) }
    }

    pub fn get_wall(&self) -> &Vec<Point> {
        &self.wall
    }

    pub fn get_players(&self) -> &Vec<Player> {
        &self.players
    }

    pub fn get_players_mut(&mut self) -> &mut Vec<Player> {
        &mut self.players
    }
    
    pub fn make_food(&mut self) {
        let between = Range::new(1, HEIGHT-1);
        let mut rng = rand::thread_rng();
        let mut x = between.ind_sample(&mut rng);
        let mut y = between.ind_sample(&mut rng);
        self.food = Point(x, y);
    }
    
    pub fn get_food(&self) -> Point{
        self.food
    }
    
    pub fn get_food_mut(&mut self) -> &mut Point{
        &mut self.food
    }
}
