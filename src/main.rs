use std::io::{stdin, stdout, Read, Write};
use std::thread; //these two line to sleep
use std::time::Duration;
use termion::{async_stdin, event::Key, input::TermRead, raw::IntoRawMode, terminal_size};
extern crate termion;

#[derive(Debug)]
struct Player {
    points: u128,
    multiplier: u128,
    highscore: u128,
}

impl Player {
    fn points_oneit(&mut self) {
        self.points = self.points + self.multiplier
    }

    fn update_multiplier(&mut self, update: u128) {
        self.multiplier = self.multiplier + update
    }
}

#[derive(Debug)]
struct Item {
    name: String,
    price: u128,
    multiplier: u128,
    amount: u128,
}

impl Item {
    fn buy(&mut self, mut playerstats: &mut Player) {
        if playerstats.points >= self.price {
            playerstats.update_multiplier(self.multiplier);
            playerstats.points -= self.price;
            self.amount += 1
        }
    }
}

fn _exitdebug(playerstats: &Player, item1: &Item) {
    println!("{:?} \n {:?}", playerstats, item1);
    std::process::exit(1);
}

fn exit(playerstats: &Player, item1: &Item) {}

fn main() {
    let mut stdin = async_stdin().keys();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let (termwidth, termheight) = terminal_size().unwrap(); //assigns the touple terinalwidth, terminalhight to the width and height of the terminal
    let mut playerstats = Player {
        points: 0,
        multiplier: 0,
        highscore: 0,
    };

    // shop - setup items
    let mut item1 = Item {
        name: String::from("Simple add"),
        price: 10,
        multiplier: 2,
        amount: 0,
    };

    // Displayloop
    loop {
        playerstats.points_oneit();

        let multiplierlength = playerstats.multiplier.to_string().chars().count() as u16; // converts the multiplier u128 to a string, gets the chars, counts them and stores them as a u16, if multiplierlength is to big for a u16 it gets truncated
        let pointslength = playerstats.points.to_string().chars().count() as u16;

        // this println prints the total number of points and the delta
        writeln!(
            stdout,
            "{}{}Total: {}{}Delta: +{}{}1: {} ({}) - {}{}",
            termion::clear::All,         //clears the terminal screen
            termion::cursor::Goto(1, 1), // positions the cursor at column 1, line 1, prints the points in the top left corner
            playerstats.points,
            termion::cursor::Goto(termwidth - (8 + multiplierlength + pointslength - 1), 1), // postions the cursor at column 80, line 1, prints the delta in the top right
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
                Key::Char('q') => break,
                Key::Char('1') => item1.buy(&mut playerstats),
                //Key::Char('2') => (multiplier, counter) = shop('2', counter, multiplier),
                _ => {}
            }
        }
        stdout.flush().unwrap();
        thread::sleep(Duration::from_millis(1000)); // sleep for one second
    }
}
