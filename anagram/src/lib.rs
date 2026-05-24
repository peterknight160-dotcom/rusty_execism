use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
   let mut anagrams = HashSet::new()  ;   
   let word_letters = convert_word_charlist (word) ;
   for i in 0..possible_anagrams.len() {
    let anagram_letters = convert_word_charlist ( possible_anagrams [i]);
    if word_letters == anagram_letters &&  possible_anagrams [i].to_uppercase() !=word.to_uppercase()   { //  Match
        anagrams.insert(possible_anagrams [i] );
      }
    
   }
   anagrams


    
    // todo!("For the '{word}' word find anagrams among the following words: {possible_anagrams:?}");
}

pub fn convert_word_charlist ( word: &str) -> String {
       let mut chars: Vec<char>= word.chars().collect::<Vec<char>>();  
     chars.sort_by(| a,b | a.to_uppercase().cmp(b.to_uppercase()));  //This is getting clunky
          let string:  String = chars.into_iter().collect ();
    string.to_uppercase()

}
