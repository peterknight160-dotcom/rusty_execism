use convert_case::{Case,Casing};

pub fn abbreviate(phrase: &str) -> String {
    let mut acronym  =String::new();
    // This statement uses the libarary to convert camelCase words to words seperated by a space (Title format)
    // Result is temporary, so cannot be passed directly to split, so is stored
    let my_str= phrase.from_case(Case::Camel).to_case(Case::Title); 
    for x in my_str.split (&[' ','-']) {
       
     //   let mut flag=false ;
        for y in x.chars() {
           
            if ! y.is_alphabetic () {
                continue;
            } 
             acronym.push(y.to_ascii_uppercase());
             break;               

        }
    }
    acronym
    
}
