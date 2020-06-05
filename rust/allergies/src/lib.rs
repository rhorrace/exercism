pub struct Allergies {
    allergens: Vec<Allergen>
}

#[derive(Debug, Clone, Copy, PartialEq)]
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

impl Allergies {
    pub fn new(mut score: u32) -> Self {
        let mut allergens = vec![
            Allergen::Cats,
            Allergen::Pollen,
            Allergen::Chocolate,
            Allergen::Tomatoes,
            Allergen::Strawberries,
            Allergen::Shellfish,
            Allergen::Peanuts,
            Allergen::Eggs,
        ];
        let mut possibles: Vec<Allergen> = Vec::new();

        score %= 256;
        while score != 0 || !allergens.is_empty() {
            let allergen = allergens.pop().unwrap();
            if score % 2 == 1 {
                possibles.push(allergen);
            }
            score /= 2;
        }
        Self {
            allergens: possibles
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergens.iter()
            .any(|a| *a == *allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.allergens.clone()
    }
}
