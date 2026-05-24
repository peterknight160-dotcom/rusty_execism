pub enum NodeType {
    Root,
    Branch,
    Leaf,
}

pub struct Alphametics{

   pub node_type: NodeType,
   pub char : char,
   pub value: Option<u8>,
   // Reference to the last node in the path, used for backtracking
   pub last : Option<Box<Alphametics>>,
   // Array of references to the next node, 
   pub next: [Option<Box<Alphametics>>; 10],
   pub valid: bool,
 }

impl Alphametics {
    pub fn new(node_type: NodeType, char: char) -> Self {
        Alphametics {
            node_type: NodeType::Leaf,
            char,
            value: None,
            last: None,
            next: [None; 10],
            valid: false,
        }
    }
    pub fn set_value(&mut self, value: u8) {
        self.value = Some(value);
    }
    pub fn set_valid(&mut self, valid: bool) {
        self.valid = valid;
    }
    //
    pub fn build_tree(&mut self, letters: &Vec<char>) {
       if letters.is_empty()  {
            return;
        }
        let new_letters = letters.clone();
        self.node_type = NodeType::Branch;
        let char = letters.pop().unwrap();
        for digit in 0..10 {
            let mut new_node = Alphametics::new(NodeType::Branch, char);
            new_node.set_value(digit);
            self.next[digit as usize] = Some(Box::new(new_node));
            self.next[digit as usize].as_mut().unwrap().build_tree(&new_letters);
        }
    }
}


 // Basic algorithm is to build a tree of possible assignments of digits to letters,
 // and then backtrack through the tree to find a valid solution that satisfies the equation.
 // The root node represents the starting point of the search, and each branch represents a possible assignment of a digit to a letter.
 // The leaf nodes represent complete assignments of digits to letters, and the valid nodes represent assignments that satisfy the equation.


 pub fn build_letters (input: &str) -> Option<Vec<char>> {
    let mut letters = Vec::new();
    for ch in input.chars() {
        if ch.is_alphabetic() && !letters.contains(&ch) {
            letters.push(ch);
        }
    }
    if letters.len() > 10 {
        None
    } else {
        Some(letters)
    }
 }

  use std::collections::HashMap;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
   // Function to solve the alphametics puzzle
   // Input string of the form "WORD1 + WORD2 == WORD3"
   // Returns a mapping of characters to digits if a solution exists
   // or None if no solution exists
   // First parse the input to extract the unique letters  
   let letters = build_letters(input)?;

  // Second build the equations
 
        

   let parts: Vec<&str> = input.split("==").collect();
   if parts.len() != 2 {
       return None;  
   }
 
  
   let left_side = parts[0];
   let right_side = parts[1]; 
   let left_words: Vec<&str> = left_side.split('+').map(|s| s.trim()).collect();
    let right_word = right_side.trim();

    // Build the tree of possible assignments of digits to letters
    let mut root = Alphametics::new(NodeType::Root, ' ');
    root.build_tree(&letters);

    // Do the tree traversal to find a valid solution

    let solutions = walk(&root, &letters, &left_words, right_word);

         


   
   // This is a complex combinatorial problem and requires backtracking
   // Here we would implement the backtracking algorithm to assign digits to letters
   // ensuring no two letters have the same digit and leading letters are not zero
  // We are not allowed to use any external tree libraries, so we will implement a simple backtracking algorithm here
    let mut solution = HashMap::new();
    if backtrack(&letters, &left_words, right_word, &mut solution) {
         Some(solution)
    } else {
         None
    }
}

fn walk (node: &Alphametics, letters: &Vec<char>, left_words: &Vec<&str>, right_word: &str) -> Vec<HashMap<char, u8>> {
    // This function will traverse the tree and collect valid solutions
    let mut solutions = Vec::new();
    if node.valid {
        let mut solution = HashMap::new();
        for i in 0..10 {
            if let Some(ref next_node) = node.next[i] {
                solution.insert(next_node.char, next_node.value.unwrap());
            }
        }
        solutions.push(solution);
    }
    for i in 0..10 {
        if let Some(ref next_node) = node.next[i] {
            let mut child_solutions = walk(next_node, letters, left_words, right_word);
            solutions.append(&mut child_solutions);
        }
    }
    solutions
}


   
   





  

