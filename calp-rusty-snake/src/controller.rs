use std::sync::mpsc::Receiver;
use model::*;
use input::*;

pub fn start (rx1: Receiver<Point>,rx2: Receiver<Point>) -> World{
    let snake1 = Snake::new();
    let snake2 = Snake::new();

    let mut player1: Player = Player::new(rx1, snake1, "Player 1");
    let mut player2: Player = Player::new(rx2, snake2, "Player 2");
    let mut players: Vec<Player> = Vec::new();
    players.push(player1);;
    players.push(player2);;
    let mut wall: Vec<Point> = Vec::new();
    /*
    for i in 0..HEIGHT {
        wall.push(Point(0,i));
        wall.push(Point(i,0));
        wall.push(Point(256,i));
        wall.push(Point(i,256));
    }
    wall.push(Point(256,256));
     */

    for i in 0..HEIGHT {
        wall.push(Point(0, i));
        wall.push(Point(HEIGHT, i));
    }

    for i in 0..WIDTH {
        wall.push(Point(i, 0));
        wall.push(Point(i, WIDTH));
    }

    wall.push(Point(HEIGHT, WIDTH));

    let world = World::new(wall,players);
    return world;
}

/* Mueve las serpientes */
pub fn move_step(pl: &mut Vec<Player>) {
    for p in pl {
        if p.get_alive() {
            p.get_snake_mut().move_snake();
        }
    }
}

fn valid_direction(p1: &Point, p2: &Point) -> bool {
    match (p1,p2) {
        (&UP, &DOWN) | (&DOWN, &UP) => false,
        (&LEFT, &RIGHT) | (&RIGHT, &LEFT) => false,
        _ => true
    }
}

/* Actualiza direcciones de serpientes */
pub fn update_direction(pl: &mut Vec<Player>) {
    for p in pl {
        if p.get_alive() {
            let r = p.get_channel().try_recv();
            let mut sn = p.get_snake_mut();

            match r {
                Ok(p) => if valid_direction(&sn.get_direction(), &p) { sn.set_direction(p) },
                _ => {}
            }
        }
    }
}

pub fn force_direction(dir: Point, pl: &mut Player) {
    let sn = pl.get_snake_mut();
    if valid_direction(&sn.get_direction(), &dir) {
        sn.set_direction(dir);
    }
}
/*
pub fn detect_collision(world: &World) -> Vec<&str> {
    let pl = world.get_players();
    let wl = world.get_wall();
    let mut status: Vec<&str> = Vec::new();
    for p in pl {
        let hd = p.get_snake().get_head();
        for q in pl {
            let sn_body = q.get_snake().get_body();
            match sn_body.iter().find(|&&x| x == hd) {
                Some(_) => status.push(p.get_name()), // DIE!!!
                None => () // YOU SHALL LIVE... FOR NOW!!!
            }
        }
        match wl.iter().find(|&&x| x == hd) {
            Some(x) => status.push(p.get_name()), // YOU'RE JUST A DUMMY!!! A PIECE OF THE PUZZLE.
            None => () // GOOD JOB... YOU AVOIDED A WALL, DUH!
        }
    }
    status
}
 */
/*
pub unsafe fn detect_collision_2(world: &mut World) {
    let len = world.get_players().len();
    let mut pl = world.get_players_mut();
    for i in 0..len {
        let hd = pl[i].get_snake().get_head();
        let (slice_l, slice_i) = pl.split_at_mut(i); //&pl[0..i];
        let (slice_i, slice_r) = pl.split_at_mut(0); //&pl[(i+1)..len];
        for j in 0..len {
            if i > j {
                let sn_body = slice_l[j].get_snake().get_body();
                let mut p = &mut slice_i[0];
                match sn_body.iter().find(|&&x| x == hd) {
                    Some(_) => slice_i[0].kill(),
                    None => ()
                }
            } else if i < j {
                let sn_body = slice_r[j].get_snake().get_body();
                let mut p = &mut slice_i[0];
                match sn_body.iter().find(|&&x| x == hd) {
                    Some(_) => slice_i[0].kill(),
                    None => ()
                }
            }
        }
    }
}
*/
pub fn detect_collision_2(world: &mut World) {
    let len = world.get_players().len();
    for i in 0..len {
        {
            let mut pl = world.get_players_mut();
            let hd = pl[i].get_snake().get_head();
            {
                let (slice_l, slice_i) = pl.split_at_mut(i);
                for j in 0..i {
                    let sn_body = slice_l[j].get_snake().get_body();
                    let mut p = &mut slice_i[0];
                    match sn_body.iter().find(|&&x| x == hd) {
                        Some(_) => p.kill(),
                        None => ()
                    }
                }
            }
            {
                let (slice_i, slice_r) = pl.split_at_mut(i+1);
                for j in 0..(len-i-1) {
                    let sn_body = slice_r[j].get_snake().get_body();
                    let mut p = &mut slice_i[0];
                    match sn_body.iter().find(|&&x| x == hd) {
                        Some(_) => p.kill(),
                        None => ()
                    }
                }
            }
        }
        let mut kill_it = false;
        let mut grow_it = false;
        {
            let pl = world.get_players();
            let hd = pl[i].get_snake().get_head();
            let wall = world.get_wall();
            match wall.iter().find(|&&x| x == hd) {
                Some(_) => kill_it = true,
                None => ()
            }
        }
        {
            let food = world.get_food();
            let pl = world.get_players();
            let hd = pl[i].get_snake().get_head();
            if hd == food {
                grow_it = true;
            }
        }
        if kill_it {
            world.get_players_mut()[i].kill();
        }
        if grow_it {
            world.get_players_mut()[i].get_snake_mut().grow_snake();
            world.make_food();
        }
    }
}
/*
fn get_colliders(world: &World, p: Player) -> Vec<Point> {
    
}
 */

/*
pub fn kill_players(world: &mut World, status: Vec<&str>) {
    
}
 */
