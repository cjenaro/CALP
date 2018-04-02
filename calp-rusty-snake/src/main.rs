extern crate ncurses;
extern crate rand;
extern crate clap;

use clap::{Arg, App};

use ncurses::*;
use std::{thread, time};

mod screen;
use screen::*;

mod term_view;
use term_view::*;

mod model;
use model::*;

mod controller;
use controller::*;

mod input;
use input::*;

fn main() {
    /* If your locale env is unicode, you should use `setlocale`. */
    // let locale_conf = LcCategory::all;
    // setlocale(locale_conf, "zh_CN.UTF-8"); // if your locale is like mine(zh_CN.UTF-8).

 	// Define command line arguments.

	let matches = App::new("calp-rusty-snake")
					.version("0.1")
					.author("The CALP team 2017")
        			.about("A snake game mixed with slither.io")
        			.arg(Arg::with_name("client")
                 	.short("c")
	                .long("client")
	                .takes_value(true)
	                .help("A port to connect to"))
        			.get_matches();


    // Get value for dir, or default to 3000.
    let dir = matches.value_of("client").unwrap_or("3000");

    /* Start ncurses. */
	  let win = initscr();
	  keypad(win, true);
	  noecho();
      curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);

	  let (rx1, rx2, handle) = input::read_input();
    cbreak();

    /* Draw initial grid*/
    let mut world: World = start(rx1, rx2);
    while true {
        TermView.refresh_screen(&world);
        thread::sleep(time::Duration::from_millis(250));
        {
            let mut players = world.get_players_mut();
            update_direction(players);
            move_step(players);
        }

        detect_collision_2(&mut world);
    }

    /* Wait for a key press. */
    //handle.join();
    /* Terminate ncurses. */
    endwin();
}
