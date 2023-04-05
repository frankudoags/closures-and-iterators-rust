mod inventory;
mod iterators;
use inventory::{Inventory, ShirtColor};

fn main() {
    let inventory = Inventory {
        shirts: vec![
            ShirtColor::Red,
            ShirtColor::Red,
            ShirtColor::Blue,
            ShirtColor::Blue,
            ShirtColor::Blue,
        ],
    };

    let user_1 = Some(ShirtColor::Red);
    let user_2 = Some(ShirtColor::Blue);
    let user_3: Option<ShirtColor> = None;

    println!("User 1 gets a {:?} shirt", inventory.giveaway(user_1));
    println!("User 2 gets a {:?} shirt", inventory.giveaway(user_2));
    println!("User 3 gets a {:?} shirt", inventory.giveaway(user_3));
}
