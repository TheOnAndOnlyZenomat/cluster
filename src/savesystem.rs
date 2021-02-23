//! Handles everything related to saving and loading

use crate::player;
use crate::player::Player;

use crate::item;
use crate::item::Item;

use std::collections::HashMap;
use std::fs;

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
        "playerpoints {}\nplayerhighscore {}\nitem1amount {}\nitem2amount {}",
        playerstats.points, highscore, item1.amount, item2.amount
    );

    match fs::write(savefile, savedata) {
        Ok(()) => std::process::exit(0),
        Err(e) => {
            println!("ERROR Savedata-Write: {}", e);
            std::process::exit(1);
        }
    };
}

/// function to load the save data and pass it to the program

pub fn loadsavedata(
    savefile: &String,
    playerstats: &mut Player,
    item1: &mut Item,
    item2: &mut Item,
) {
    let mut savedata = HashMap::new();

    match fs::read_to_string(savefile) {
        Ok(savedataret) => {
            for line in savedataret.lines() {
                let namelength = &line.split_whitespace().next().unwrap().len();
                let value = line[namelength + 1..].parse::<u128>().unwrap();
                savedata.insert(line.split_whitespace().next().unwrap().to_string(), value);
            }

            match savedata.get("playerpoints") {
                Some(v) => playerstats.points = *v,
                None => println!("No savedata found for playerpoints"),
            };

            match savedata.get("playermultiplier") {
                Some(v) => playerstats.multiplier = *v,
                None => println!("No savedata found for playermultiplier"),
            };

            match savedata.get("playerhighscore") {
                Some(v) => playerstats.highscore = *v,
                None => println!("No savedata found for playerhighscore"),
            };

            match savedata.get("item1amount") {
                Some(v) => item1.amount = *v,
                None => println!("No savedata found for item1amount"),
            };

            match savedata.get("item2amount") {
                Some(v) => item2.amount = *v,
                None => println!("No savedata found for item2amount"),
            };
        }
        Err(e) => {
            println!("\nERROR Savedata-Read: {}", e);
            std::process::exit(1);
        }
    };
}
