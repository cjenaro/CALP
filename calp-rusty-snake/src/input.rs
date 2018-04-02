
use model::*;
extern crate ncurses;
use self::ncurses::*;
use std::thread::*;
use std::sync::mpsc::*;

pub fn read_input() -> (Receiver<Point>, Receiver<Point>, JoinHandle<()>) {

    let (tx1, rx1) = channel();
    let (tx2, rx2) = channel();

    let handle = spawn(move || {

        while true {

            let ch = getch();

            let mut p1 : Option<Point> = None;
            let mut p2 : Option<Point> = None;

            match ch {
                119 => p1 = Some(UP),
                115 => p1 = Some(DOWN),
                97 => p1 = Some(LEFT),
                100 => p1 = Some(RIGHT),
                KEY_UP => p2 = Some(UP),
                KEY_DOWN => p2 = Some(DOWN),
                KEY_RIGHT => p2 = Some(RIGHT),
                KEY_LEFT => p2 = Some(LEFT),
                120 => return(),
                _ => {} ,
            }

            match p1  {
                Some(x) => tx1.send(x).unwrap(),
                None => (),
            }

            match p2  {
                Some(x) => tx2.send(x).unwrap(),
                None => (),
            }
        }
    });

    return (rx1, rx2, handle);
}









