//! Handles everything related to saving and loading

use crate::player;
use crate::player::Player;

use crate::item;
use crate::item::Item;

use std::fs;

/// function to save the game state
///
/// takes the savefile position, the playerstats struct, and the item structs
///
/// returns a result, which is () on success
pub fn save(savefile: &String, playerstats: &Player, item1: &Item) {
    let savedata: String = format!(
        "playerpoints:{}\nplayermultiplier:{}\nplayerhighscore:{}\nitemoneamount:{}",
        playerstats.points, playerstats.multiplier, playerstats.highscore, item1.amount
    );

    match fs::write(savefile, savedata) {
        Ok(()) => std::process::exit(0),
        Err(e) => {
            println!("ERROR Savedata-Write: {}", e);
            std::process::exit(1);
        }
    }
}

pub fn loadsavedata(savefile: &String, playerstats: &mut Player, item1: &mut Item) {
    let savedata: String;
    match fs::read_to_string(savefile) {
        Ok(savedataret) => {
            savedata = savedataret;
            println!("{}", savedata);
            let (playerpoints, playermultiplier, playerhighscore, item1amount) =
                parsesavedata(savedata);
            playerstats.points = playerpoints.parse::<u128>().unwrap();
            playerstats.multiplier = playermultiplier.parse::<u128>().unwrap();
            playerstats.highscore = playerhighscore.parse::<u128>().unwrap();

            item1.amount = item1amount.parse::<u128>().unwrap();
        }
        Err(e) => {
            println!("\nERROR Savedata-Read: {}", e);
            std::process::exit(1);
        }
    };
}

fn parsesavedata(savedata: String) -> (String, String, String, String) {
    let mut playerpoints = String::new();
    let mut playermultiplier = String::new();
    let mut playerhighscore = String::new();
    let mut item1amount = String::new();
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
        }
    }
    (playerpoints, playermultiplier, playerhighscore, item1amount)
}
