use std:: vec::Vec;


#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,

}




pub struct BowlingGame {
   bowls: Vec<u16>,
    second_roll: bool,

}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            bowls: Vec::new(),
            second_roll: false,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }
        if self.second_roll && self.bowls.last().unwrap() + pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }
        if self.score().is_some() {
            return Err(Error::GameComplete);
        }
        self.bowls.push(pins);
        println!("pins is {},  bowls is {} long ", pins, self.bowls.len());
        self.second_roll = !self.second_roll;
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        let mut score: u16 = 0;
        let mut frame = 0;
        let mut i = 0;
       
        for roll in &self.bowls {
            score += roll

       
        Some(score)
    }
}
