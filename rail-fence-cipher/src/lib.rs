pub struct RailFence {
    rails: u32,  // The number of rails in the fence - everything else can be derived from this
}

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        RailFence { rails }
    }

/*
W . . . E . . . C . . . R . . . L . . . T . . . E
. E . R . D . S . O . E . E . F . E . A . O . C .
. . A . . . I . . . V . . . D . . . E . . . N . .
    
    So the top row is 1st, 2*(rails-1)+1, 2*2*(rails-1)+1, ...
    The next row is 2, 2*(rails-1)+2, 2*2*(rails-1)+2, ...
    The next row is 3, 2*(rails-1)+3, 2
    
     */

    pub fn encode(&self, text: &str) -> String {
        let mut result:Vec<String> = Vec::new();
        for _ in 0..self.rails {
            result.push(String::new());
        }
        let mut current_rail = 0;
        let mut rail_direction = 1; // 1 for down, -1 for up
        for c in text.chars() {
            println!("c  is {} ", c);
            println!("current_rail is {} ", current_rail);
            println!("rail_direction is {} ", rail_direction);

            if result[current_rail].is_empty() {
              
                result[current_rail] = c.to_string();
            } else {
                result[current_rail].push(c);
             
               
              
            }
            current_rail = (current_rail as i32 + rail_direction) as usize;
            if current_rail == 0 || current_rail == (self.rails - 1) as usize {
                rail_direction *= -1; // Change direction at the top and bottom rails
            }
        }
        result.join("") 

        
    }

    pub fn decode(&self, cipher: &str) -> String {
       // Convert cipher to a vector of characters for easier manipulation
        let cipher_chars: Vec<char> = cipher.chars().collect();
        let mut rail_lengths:Vec<u32> = vec![0; self.rails as usize];
        
        // Total number of characters in the cipher
             let mut current_rail = 0;
        let mut rail_direction = 1; // 1 for down, -1 for up
        for _ in 0..cipher.len(){
            rail_lengths[current_rail as usize] += 1; // Increment the count for the current rail
            current_rail = (current_rail as i32 + rail_direction) as usize;
            if current_rail == 0 || current_rail == (self.rails - 1) as usize {
                rail_direction *= -1; // Change direction at the top and bottom rails
            }
        }
        for i in 1..self.rails {
            rail_lengths[i as usize] += rail_lengths[(i-1) as usize]; // Cumulative sum to determine the starting index for each rail
        }
        let mut result:Vec<char> = Vec::with_capacity(cipher.len()) ; // Initialize
        current_rail = 0;
        rail_direction = 1; // Reset for decoding
         let mut rail_indices:Vec<u32> = vec![0; self.rails as usize]; // To track the current index for each rail
         for _ in 0..cipher.len() {
            let rail_index = current_rail as usize;
            let char_index = if rail_index == 0 {
                rail_indices[0]
            } else {
                rail_lengths[rail_index - 1] + rail_indices[rail_index]
            } as usize;
            result.push(cipher_chars[char_index]); // Append the character from the cipher to the result
            rail_indices[rail_index] += 1; // Move to the next character in the current rail
            current_rail = (current_rail as i32 + rail_direction) as usize;
            if current_rail == 0 || current_rail == (self.rails - 1) as usize {
                rail_direction *= -1; // Change direction at the top and bottom rails
            }
        }   
       
       

        result.iter().collect() // Convert the vector of characters back to a string
      
    }
}
