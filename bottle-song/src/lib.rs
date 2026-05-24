use convert_case::{Case, Casing};  // Abusing this crate, as "use capitalize::Capitalize" does a better job, but Exercism doesn't support it

pub fn recite(start_bottles: u32, take_down: u32) -> String {
  
    // There is often a compromise between compact code and code that is easy to maintain
    //  My personal preference is a bias to the latter
    let line1 = " hanging on the wall,\n";
    let line3 = "And if one green bottle should accidentally fall,\n";
    let line4a = "There'll be ";
    let line4b = " hanging on the wall.\n";
    let mut verse = String::new();

    for i in ((start_bottles - take_down)..start_bottles).rev() {
        if !verse.is_empty() {
            verse.push('\n');
        }
        verse.push_str(&number_to_word_green_bottles(i + 1, true));
        verse.push_str(line1);
        verse.push_str(&number_to_word_green_bottles(i + 1, true));
        verse.push_str(line1);
        verse.push_str(line3);
        verse.push_str(line4a);
        verse.push_str(&number_to_word_green_bottles(i, false));
        verse.push_str(line4b);
    }
    verse
}

fn number_to_word_green_bottles(number: u32, first_cap: bool) -> String {
    let words = [
        "no", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
    ];
    
    let mut word = String::from(words[number as usize]);
    if first_cap {
        word = word.to_case(Case::Title);  // Just need to capitalise the first char
    }

    if number == 1 {
        word.push_str(" green bottle");
    } else {
        word.push_str(" green bottles");
    }
    word
}
