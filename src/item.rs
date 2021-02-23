//! Stores all the information regarding an item, like name, price, multiplier and amount

use crate::player;
use crate::player::Player;

#[derive(Debug)]
pub struct Item {
    pub name: String,
    pub price: u128,
    pub multiplier: u128,
    pub amount: u128,
}

impl Item {
    /// Handles buying an item. First checks, if player has enough points to buy, updates the multiplier, removes the price from the players points, increases the amount of the item
    pub fn buy(&mut self, mut playerstats: &mut Player) {
        if playerstats.points >= self.price {
            playerstats.update_multiplier(self.multiplier);
            playerstats.points -= self.price;
            self.amount += 1;
        }
    }
}
