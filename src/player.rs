//! Stores all the information regarding the player, like to current points, his multiplier and his highscore (not yet implemented)

use crate::Item;

#[derive(Debug)]
pub struct Player {
    pub points: u128,
    pub multiplier: u128,
    pub highscore: u128,
}

impl Player {
    /// Update the mutliplier and take into consideration the amount of items. Useful to update the multiplier at launch
    pub fn initial_multiplier(&mut self, item1: &Item) {
        self.multiplier = item1.amount
    }

    /// Updated the points counter by adding the multiplier (yes I know it's weird, that the multiplier gets added instead of multiplier...)
    pub fn points_oneit(&mut self) {
        self.points = self.points + self.multiplier;
    }

    /// Updates the multiplier with the given update parameter, can be used for the shop
    pub fn update_multiplier(&mut self, update: u128) {
        self.multiplier = (self.multiplier + update)
    }
}