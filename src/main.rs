//! Rust Clicker game

#![allow(unused_imports)]
#![allow(unused_variables)]
use std::io::Stdout;
use std::io::{stdin, stdout, Read, Write};
use std::thread;
use std::time::Duration;
use termion::{
    async_stdin,
    event::Key,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
    terminal_size,
};

mod item;
mod player;
mod savesystem;

use crate::item::Item;
use crate::player::Player;

/// exit function, which prints playerstats and item1 struct data for debug
fn _exitdebug(stdout: RawTerminal<Stdout>, playerstats: &Player, item1: &Item) {
    println!("{:?} \n {:?}", playerstats, item1);
    std::process::exit(1);
}

/// Function to exit the game, here stuff like saving will be handled
fn exit(stdout: &mut RawTerminal<Stdout>, savefile: &String, playerstats: &Player, item1: &Item) {
    writeln!(stdout, "{}", termion::clear::All).unwrap();
    stdout
        .suspend_raw_mode()
        .expect("Error suspending raw mode"); // return the terminal from raw mode to it's previous state

    savesystem::save(&savefile, &playerstats, &item1);
}

fn main() {
    // initial setup
    let mut stdin = async_stdin().keys();
    let mut stdout: RawTerminal<Stdout> = stdout().into_raw_mode().unwrap();
    stdout
        .suspend_raw_mode()
        .expect("Error suspending raw mode");
    let (termwidth, termheight) = terminal_size().unwrap(); //assigns the touple terinalwidth, terminalhight to the width and height of the terminal

    let savefile = String::from("./save.txt"); // defines savefile

    // initializes the player
    let mut playerstats = Player {
        points: 100,
        multiplier: 1,
        highscore: 0,
    };

    // shop - setup items
    let mut item1 = Item {
        name: String::from("Simple add"),
        price: 10,
        multiplier: 2,
        amount: 0,
    };

    savesystem::loadsavedata(&savefile, &mut playerstats, &mut item1);

    // update multiplier and take in consideration the amount of items
    playerstats.initial_multiplier(&item1);

    stdout
        .activate_raw_mode()
        .expect("Error activating raw mode");

    // Displayloop
    loop {
        playerstats.points_oneit();

        // chekcs the length of the multiplier and points as a string, so that we can use that to display the interface without cutting anything off
        let multiplierlength = playerstats.multiplier.to_string().chars().count() as u16;
        let pointslength = playerstats.points.to_string().chars().count() as u16;

        // this println prints the total number of points, the delta and the shop
        writeln!(
            stdout,
            "{}{}Total: {}{}Delta: +{}{}1: {} ({}) - {}{}",
            termion::clear::All,         //clears the terminal screen
            termion::cursor::Goto(1, 1), // positions the cursor at column 1, line 1, prints the points in the top left corner
            playerstats.points,
            termion::cursor::Goto(termwidth - (7 + multiplierlength + pointslength), 1), // postions the cursor at column 80, line 1, prints the delta in the top right
            playerstats.multiplier,
            termion::cursor::Goto(1, termheight),
            item1.name,
            item1.price,
            item1.amount,
            termion::cursor::Goto(1, 1)
        )
        .unwrap();

        if let Some(c) = stdin.next() {
            match c.unwrap() {
                Key::Char('a') => playerstats.points += 1,
                Key::Char('q') => exit(&mut stdout, &savefile, &playerstats, &item1),
                Key::Char('1') => item1.buy(&mut playerstats),
                //Key::Char('2') => (multiplier, counter) = shop('2', counter, multiplier),
                _ => {}
            }
        }
        stdout.flush().unwrap();
        thread::sleep(Duration::from_millis(1000)); // sleep for one second
    }
}
