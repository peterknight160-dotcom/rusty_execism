/* pub struct Allergy {
    score: u32,
    allergy: Allergen,
}
 */
pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

const DECODE_ALLERGENS: [(Allergen, u32); 8] = [
    (Allergen::Eggs, 1),
    (Allergen::Peanuts, 2),
    (Allergen::Shellfish, 4),
    (Allergen::Strawberries, 8),
    (Allergen::Tomatoes, 16),
    (Allergen::Chocolate, 32),
    (Allergen::Pollen, 64),
    (Allergen::Cats, 128),
];

impl Allergies {
    pub fn new(score: u32) -> Self {
        // Put score into self and return the struct
        Self { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let mut score = 0;

        for (current_allergen, current_score) in DECODE_ALLERGENS.iter() {
            if current_allergen == allergen {
                score = *current_score;
                break;
            }
        }

        (self.score & score) != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut allergens: Vec<Allergen> = Vec::new();

        for (allergen, score) in DECODE_ALLERGENS.iter() {
            if self.score & score != 0 {
                allergens.push(*allergen);
            }
        }

        allergens
    }
}
