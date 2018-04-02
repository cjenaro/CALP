extern crate ncurses;

use ncurses::*;

use model::*;
use screen::*;

pub struct TermView;

impl TermView {
    fn draw_wall(&self, w: &Vec<Point>) {
        for p in w {
            mvprintw(p.0, 2*(p.1), "#");
        }
    }
    
    fn draw_food(&self, f: Point){
        mvprintw(f.0, 2*(f.1), "C");
    }

    fn draw_snake1(&self, s: &Snake) {
        for p in s.get_body() {
            mvprintw(p.0, 2*(p.1), "O");
        }
        let hd = s.get_head();
        mvprintw(hd.0, 2*(hd.1), "@");
    }
    
    fn draw_snake2(&self, s: &Snake) {
        for p in s.get_body() {
            mvprintw(p.0, 2*(p.1), "0");
        }
        let hd = s.get_head();
        mvprintw(hd.0, 2*(hd.1), "#");
    }
}

impl Screen for TermView {
    fn refresh_screen(&self, world: &World) {
        clear();
        &self.draw_wall(world.get_wall());
        let mut b = true;
        for p in world.get_players() {
            if p.get_alive(){
                if b {
                    let s = p.get_snake();
                    &self.draw_snake1(s);
                    b = false;
                } else {
                    let s = p.get_snake();
                    &self.draw_snake2(s);
                }
            }
        }
        let food = world.get_food();
        &self.draw_food(food);
        refresh();
    }
}

