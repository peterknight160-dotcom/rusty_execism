use std::collections::HashMap;



pub fn brackets_are_balanced(string: &str) -> bool {
    let types = HashMap::from (
    [ ('}', '{'),
   (')', '('),
   (']', '['), ]
);
    
   
    let mut brackets = Vec::new();
  
   
    
   for char in string.chars() {
        
        match char {
            '{' | '[' | '(' => brackets.push(char),
            '}' | ']' | ')'  => {
                if brackets.pop() != types.get(&char).copied()  {
                   return false
                }
            }
            
            _ => (),
        }
        
    }
     brackets.is_empty() 

}
