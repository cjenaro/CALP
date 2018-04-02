extern crate piston_window;
extern crate rand;


mod model;
use model::*;

use std::*;
use piston_window::*;
use piston_window::types::Color;

const BACK_COLOR: Color = [0.204, 0.286, 0.369, 1.0];
const WHITE_COLOR: Color = [1.0;4];
const BLACK_COLOR: Color = [0.0, 0.0, 0.0, 0.1];
const PLAYER1_COLOR: Color = [1.0, 0.0, 1.0, 1.0];
const PLAYER1_COLOR_HEAD: Color = [0.5, 0.0, 1.0, 1.0];
const PLAYER2_COLOR: Color = [1.0, 0.5, 1.0, 1.0];
const PLAYER2_COLOR_HEAD: Color = [0.5, 0.5, 1.0, 1.0];
const BLOCK_SIZE: f64 = 25.0;

fn to_gui_coord(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}

fn to_gui_coord_u32(game_coord: i32) -> u32 {
    to_gui_coord(game_coord) as u32
}

fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let gui_x = to_gui_coord(x);
    let gui_y = to_gui_coord(y);

    rectangle(color, [gui_x, gui_y,
            BLOCK_SIZE, BLOCK_SIZE], con.transform, g);
}

fn draw_vector(color: Color, v: &Vec<Point>, c: &Context, g: &mut G2d){
    for p in v {
        draw_block(color, p.1, p.0, &c, g);
    }
}

fn find_window_values(w: &Vec<Point>) -> (i32,i32){
    let mut height = 0;
    let mut width = 0;
    for p in w {
        height = std::cmp::max(width,p.0);
        width = std::cmp::max(height,p.1);
    } 
    (width/4,height/4)
}

fn draw_snake(color: Color, color_head: Color, s: &Snake, c: &Context, g: &mut G2d) {
    draw_vector(color, s.get_body(), c, g);
    let body = s.get_body();
    let last = &body[body.len() - 1];
    draw_block(color_head, last.1, last.0, &c, g);
}

fn draw_world(w: &World, c: &Context, g: &mut G2d){
    clear(BLACK_COLOR,g);
    draw_vector(WHITE_COLOR, &w.wall, c, g);
    let mut i=0;
    for x in w.players {
        if i == 0 {
            draw_snake(PLAYER1_COLOR, PLAYER1_COLOR_HEAD, &w.players.x, c, g);
            i = i+1;
        } else {
            draw_snake(PLAYER2_COLOR, PLAYER2_COLOR_HEAD, &w.players.x, c, g);
        }
    }
}
