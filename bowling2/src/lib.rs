#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    rolls: Vec<u16>, 
    current_frame: usize,   

}   

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            rolls: Vec::new(),
            current_frame: 0,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        // Validate the roll
        if self.current_frame >= 10 {
            return Err(Error::GameComplete);
        }
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }   

        self.rolls.push(pins);
         if self.rolls.len() % 2 == 0 && self.rolls[self.rolls.len() - 2] + pins >= 10 {
            self.current_frame += 1;
        } else if self.rolls.len() % 2 == 1 && pins == 10 {
            self.current_frame += 1;
        } else {
            let x = self.rolls.len() % 2;
            println!("Current frame: {}, rolls: {:?}", self.current_frame, self.rolls);
            println!("X: {}", x);
            self.current_frame  += x;
        }
         

        println!("Rolled {} pins, current frame: {}, rolls: {:?}", pins, self.current_frame, self.rolls);
        Ok(())            
        
    }

    pub fn score(&self) -> Option<u16> {
           println!("Calculating score, rolls: {:?}", self.rolls);
           println!("Current frame: {}", self.current_frame);
        if self.current_frame < 10 {
            println!("Game is not complete, current frame: {}", self.current_frame);
            return None; // Game is not complete
        }
        let mut total_score = 0;
        let mut frame_index = 0;
     

        for _frame in 0..10 {
            println!("Calculating score for frame {}, rolls: {} and {}", _frame + 1, self.rolls[frame_index], self.rolls[frame_index + 1]);
            if self.rolls[frame_index] == 10 { // Strike
                total_score += 10 + self.rolls[frame_index + 1] + self.rolls[frame_index + 2];
                frame_index += 1;
            } else if self.rolls[frame_index] + self.rolls[frame_index + 1] == 10 { // Spare
                println!("Spare detected at frame {}, rolls: {} and {}", _frame + 1, self.rolls[frame_index], self.rolls[frame_index + 1]);
                total_score += 10 + self.rolls[frame_index + 2];
                frame_index += 2;
            } else { // Open frame
                total_score += self.rolls[frame_index] + self.rolls[frame_index + 1];
                frame_index += 2;
            }
        }
        Some(total_score)
    }
}
