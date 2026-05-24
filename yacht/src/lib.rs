

pub enum Category {
    Ones,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    FullHouse,
    FourOfAKind,
    LittleStraight,
    BigStraight,
    Choice,
    Yacht,
}

fn score_n ( dice: [u8; 5], n: u8) -> u8 {
    dice.into_iter().filter(|d| *d == n).sum()
}

fn score_full_house(dice: [u8; 5]) -> u8 {
    let mut counts = [0; 6];
    for d in dice {
        counts[(d - 1) as usize] += 1;
    }
    if counts.iter().any(|&c| c == 3) && counts.iter().any(|&c| c == 2) {
        dice.into_iter().sum()
    } else {
        0
    }
}
fn score_four_of_a_kind(dice: [u8; 5]) -> u8 {
    let mut counts = [0; 6];
    for d in dice {
        counts[(d - 1) as usize] += 1;
    }
    if let Some((num, &_count)) = counts.iter().enumerate().find(|&(_, &c)| c >= 4) {
        (num as u8 + 1) * 4
    } else {
        0
    }
}
fn score_little_straight(dice: [u8; 5]) -> u8 {
    if dice == [1, 2, 3, 4, 5] {
        30
    } else {
        0
    }
}

fn score_big_straight(dice: [u8; 5]) -> u8 {
    if dice == [2, 3, 4, 5, 6] {
        30
    } else {
        0
    }
}

fn score_choice(dice: [u8; 5]) -> u8 {
    dice.into_iter().sum()
}
fn score_yacht(dice: [u8; 5]) -> u8 {
    if dice.iter().all(|&d| d == dice[0]) {
        50
    } else {
        0
    }
}
type Dice = [u8; 5];
pub fn score(dice: Dice, category: Category) -> u8 {
    use Category::*;
    let mut my_dice = dice ;
    my_dice.sort();
     
    match category {
        Ones => score_n(my_dice, 1),
        Twos => score_n(my_dice, 2),
        Threes => score_n(my_dice, 3),
        Fours => score_n(my_dice, 4),
        Fives => score_n(my_dice, 5),
        Sixes => score_n(my_dice, 6),
        FullHouse => score_full_house(my_dice),
        FourOfAKind => score_four_of_a_kind(my_dice),
        LittleStraight => score_little_straight(my_dice),
        BigStraight => score_big_straight(my_dice),
        Choice => score_choice(my_dice),
        Yacht => score_yacht(my_dice),        
    }
}
