//! Rust Clicker game

use std::io::{stdin, stdout, Read, Write};
use std::thread;
use std::time::Duration;
use termion::{async_stdin, event::Key, input::TermRead, raw::IntoRawMode, terminal_size};

/// Stores all the information regarding the player, like to current points, his multiplier and his highscore (not yet implemented)
#[derive(Debug)]
struct Player {
    points: u128,
    multiplier: u128,
    highscore: u128,
}

impl Player {
    /// Update the mutliplier and take into consideration the amount of items. Useful to update the multiplier at launch
    fn initial_multiplier(&mut self, item1: &Item) {
        self.multiplier = item1.amount
    }

    /// Updated the points counter by adding the multiplier (yes I know it's weird, that the multiplier gets added instead of multiplier...)
    fn points_oneit(&mut self) {
        self.points = self.points + self.multiplier;
    }

    /// Updates the multiplier with the given update parameter, can be used for the shop
    fn update_multiplier(&mut self, update: u128) {
        self.multiplier = (self.multiplier + update)
    }
}

/// Stores all the information regarding an item, like name, price, multiplier and amount
#[derive(Debug)]
struct Item {
    name: String,
    price: u128,
    multiplier: u128,
    amount: u128,
}

impl Item {
    /// Handles buying an item. First checks, if player has enough points to buy, updates the multiplier, removes the price from the players points, increases the amount of the item
    fn buy(&mut self, mut playerstats: &mut Player) {
        if playerstats.points >= self.price {
            playerstats.update_multiplier(self.multiplier);
            playerstats.points -= self.price;
            self.amount += 1
        }
    }
}

/// exit function, which prints playerstats and item1 struct data for debug
fn _exitdebug(playerstats: &Player, item1: &Item) {
    println!("{:?} \n {:?}", playerstats, item1);
    std::process::exit(1);
}

/// Function to exit the game, here stuff like saving will be handled (not yet implemented)
fn exit(playerstats: &Player, item1: &Item) {}

fn main() {
    // initial setup
    let mut stdin = async_stdin().keys();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let (termwidth, termheight) = terminal_size().unwrap(); //assigns the touple terinalwidth, terminalhight to the width and height of the terminal

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

    // update multiplier and take in consideration the amount of items
    playerstats.initial_multiplier(&item1);

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
                Key::Char('q') => _exitdebug(&playerstats, &item1),
                Key::Char('1') => item1.buy(&mut playerstats),
                //Key::Char('2') => (multiplier, counter) = shop('2', counter, multiplier),
                _ => {}
            }
        }
        stdout.flush().unwrap();
        thread::sleep(Duration::from_millis(1000)); // sleep for one second
    }
}
