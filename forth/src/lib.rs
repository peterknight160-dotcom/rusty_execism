pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

pub struct Forth {
    stack: Vec<Value>,
    definitions: std::collections::HashMap<String, Vec<String>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Forth { stack: Vec::new(), definitions: std::collections::HashMap::new() }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }
    pub fn eval(&mut self, input: &str) -> Result {
        let input = input.to_ascii_lowercase();
        println!("eval:  Evaluating input: {}", input);
        let mut expanded_input = String::new();
        let mut last_word_was_colon = false; 
        for word in input.split_whitespace() {
            if let Some(definition) = self.definitions.get(word) && !last_word_was_colon {
                expanded_input.push_str(&definition.join(" "));
                expanded_input.push(' '); // Add a space after the definition
            } else {
                expanded_input.push_str(word);
                expanded_input.push(' '); // Add a space after the word
                last_word_was_colon = word == ":";
            }
        }
        println!("Expanded input: {}", expanded_input);
        self.eval_expanded(&expanded_input)
    }

     fn eval_expanded(&mut self, input: &str) -> Result {
        // Split the input into words and process each word
        for word in input.split_whitespace() {
          println!("eval_expanded:  Processing word: {}", word);
            match &word.to_ascii_lowercase()[..] {
                "+" => self.add()?,
                "-" => self.sub()?,
                "*" => self.mul()?,
                "/" => self.div()?,
                "dup" => self.dup()?,
                "drop" => self.drop()?,
                "swap" => self.swap()?,
                "over" => self.over()?,
                ":" => { self.define_word(input)? ; 
                    println!("eval_expanded:  Defined new word at 56: {}", input);
                    break; // Stop processing after defining a new word
                },

                _ => {if let Ok(num) = word.parse::<Value>() {
                self.stack.push(num);
            } else {

                return Err(Error::UnknownWord);
            }},
               
            }
           
        } // End for

        Ok(())
    }
    fn add(&mut self) -> Result {
        if self.stack.len() < 2 {
            return Err(Error::StackUnderflow);
        }
        let b = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();
        self.stack.push(a + b);
        Ok(())
    }
    fn sub(&mut self) -> Result {
        if self.stack.len() < 2 {
            return Err(Error::StackUnderflow);
        }
        let b = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();
        self.stack.push(a - b);
        Ok(())
    }
    fn mul(&mut self) -> Result {
        if self.stack.len() < 2 {
            return Err(Error::StackUnderflow);
        }
        let b = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();
        self.stack.push(a * b);
        Ok(())
    }
    fn div(&mut self) -> Result {
        if self.stack.len() < 2 {
            return Err(Error::StackUnderflow);
        }
        let b = self.stack.pop().unwrap();
        if b == 0 {
            return Err(Error::DivisionByZero);
        }
        let a = self.stack.pop().unwrap();
        self.stack.push(a / b);
        Ok(())
    }   
    fn dup(&mut self) -> Result {
        if self.stack.is_empty() {
            return Err(Error::StackUnderflow);
        }
        let top = *self.stack.last().unwrap();
        self.stack.push(top);
        Ok(())
    }
    fn drop(&mut self) -> Result {
        if self.stack.is_empty() {
            return Err(Error::StackUnderflow);
        }
        self.stack.pop();
        Ok(())
    }
    fn swap(&mut self) -> Result {
        if self.stack.len() < 2 {
            return Err(Error::StackUnderflow);  
        }
        let len = self.stack.len();
        self.stack.swap(len - 1, len - 2);
        Ok(())
    }
    fn over(&mut self) -> Result {
        if self.stack.len() < 2 {
            return Err(Error::StackUnderflow);
        }
        let len = self.stack.len();
        let second = self.stack[len - 2];
        self.stack.push(second);
        Ok(())  
        }
    fn define_word(&mut self, input: &str) -> Result {
        let input = input.to_ascii_lowercase();
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() < 4 || parts[0] != ":" || parts[parts.len() - 1] != ";" {
            //println!("define_word:  Invalid word definition: {}", input);
            return Err(Error::InvalidWord);
        }
      
        let word_name = parts[1].to_string();
        // Check if the word name is a valid number
        if word_name.parse::<Value>().is_ok() {
            println!("define_word:  Invalid word name (cannot be a number): {}", word_name);
            return Err(Error::InvalidWord);
        }
        let definition = parts[2..parts.len() - 1].iter().map(|s| s.to_string()).collect();
        //println!("define_word:  Defining word: {} as {:?}", word_name, definition);
        self.definitions.insert(word_name, definition);
        //println!("define_word:  Current definitions: {:?}", self.definitions);
        Ok(())
    }
}
