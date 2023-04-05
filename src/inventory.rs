#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ShirtColor {
    Red,
    Blue,
}

pub struct Inventory {
    pub shirts: Vec<ShirtColor>,
}

impl Inventory {
    /// closure is used in the unwrap_or_else method
    /// to provide a default value if the user_preference is None
    pub fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    pub fn most_stocked(&self) -> ShirtColor {
        let mut red_count = 0;
        let mut blue_count = 0;

        for shirt in &self.shirts {
            match shirt {
                ShirtColor::Red => red_count += 1,
                ShirtColor::Blue => blue_count += 1,
            }
        }

        if red_count > blue_count {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}
