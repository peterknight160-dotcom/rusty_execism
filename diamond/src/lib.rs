pub fn get_diamond(c: char) -> Vec<String> {
    // Get the position of the character in the alphabet (A=0, B=1, ..., Z=25)
    let pos: u8 = c as u8 - b'A' ;  // A -> 0, B->1, etc.
    let mut diamond: Vec<String> = Vec::new();
    for i in (0..=pos).chain((0..pos).rev()) {  
        let mut line :Vec<u8> = vec![b' '; (2*pos + 1) as usize];  //full line of spaces, we will replace the right ones with letters
        line[(pos - i) as usize] = b'A' + i;  // Place the first letter
        if i > 0 {
            line[(pos + i) as usize] = b'A' + i;  // Place the second letter
        }   
        // Convert the line from Vec<u8> to String
            
        

        diamond.push(String::from_utf8(line).unwrap()); 
    }
    diamond
 
}
