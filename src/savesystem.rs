//! Handles everything related to saving and loading

use crate::player;
use crate::player::Player;

use crate::item;
use crate::item::Item;

use std::fs;
use std::io::prelude::*;
use std::io::{self, BufReader};

/// function to save the game state
///
/// takes the savefile position, the playerstats struct, and the item structs
///
/// returns a result, which is () on success
pub fn save(savefile: &String, playerstats: &Player, item1: &Item, item2: &Item) {
    let highscore;

    // checks if highscore value has to be written
    if playerstats.highscore < playerstats.points {
        highscore = playerstats.points;
    } else {
        highscore = playerstats.highscore;
    };

    let savedata: String = format!(
        "playerpoints:{}\nplayermultiplier:{}\nplayerhighscore:{}\nitemoneamount:{}\nitemtwoamount:{}",
        playerstats.points, playerstats.multiplier, highscore, item1.amount, item2.amount
    );

    match fs::write(savefile, savedata) {
        Ok(()) => std::process::exit(0),
        Err(e) => {
            println!("ERROR Savedata-Write: {}", e);
            std::process::exit(1);
        }
    }
}

/// function to load the save data and pass it to the program
pub fn loadsavedata(
    savefile: &String,
    playerstats: &mut Player,
    item1: &mut Item,
    item2: &mut Item,
) {
    let savedata: String;
    match fs::read_to_string(savefile) {
        Ok(savedataret) => {
            savedata = savedataret;
            println!("{}", savedata);
            let (playerpoints, playermultiplier, playerhighscore, item1amount, item2amount) =
                parsesavedata(savedata);
            playerstats.points = playerpoints.parse::<u128>().unwrap();
            playerstats.multiplier = playermultiplier.parse::<u128>().unwrap();
            playerstats.highscore = playerhighscore.parse::<u128>().unwrap();

            item1.amount = item1amount.parse::<u128>().unwrap();
            item2.amount = item2amount.parse::<u128>().unwrap();

            // update multiplier and take in consideration the amount of items
            playerstats.initial_multiplier(&item1, &item2);
        }
        Err(e) => {
            println!("\nERROR Savedata-Read: {}", e);
            std::process::exit(1);
        }
    };
}

/// local function to parse the save file, returns a tuple with all the data
fn parsesavedata(savedata: String) -> (String, String, String, String, String) {
    let mut playerpoints = String::new();
    let mut playermultiplier = String::new();
    let mut playerhighscore = String::new();
    let mut item1amount = String::new();
    let mut item2amount = String::new();
    let mut whitespacecount: u8 = 0;
    for c in savedata.chars() {
        if c.is_whitespace() {
            whitespacecount += 1;
        } else if c.is_numeric() && whitespacecount == 0 {
            playerpoints.push(c);
        } else if c.is_numeric() && whitespacecount == 1 {
            playermultiplier.push(c);
        } else if c.is_numeric() && whitespacecount == 2 {
            playerhighscore.push(c);
        } else if c.is_numeric() && whitespacecount == 3 {
            item1amount.push(c);
        } else if c.is_numeric() && whitespacecount == 4 {
            item2amount.push(c);
        }
    }
    (
        playerpoints,
        playermultiplier,
        playerhighscore,
        item1amount,
        item2amount,
    )
}

pub fn parsesavedatabyline(
    savefile: &String,
    playerdata: &mut Player,
    item1: &mut Item,
    item2: &mut Item,
) -> io::Result<()> {
    let mut playerpoints = String::new();
    let mut playermultiplier = String::new();
    let mut playerhighscore = String::new();
    let mut item1amount = String::new();
    let mut item2amount = String::new();

    let mut savedata: Vec<&str>;

    let file = fs::File::open(savefile)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        savedata = line.unwrap().split(":").collect();
        println!("hello");
    }

    println!("{:?}", savedata);

    Ok(())
}
