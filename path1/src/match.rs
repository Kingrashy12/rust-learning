mod match_age {
    use std::cmp::Ordering;

    pub fn main() {
        let my_age = 18;
        let voting_age = 18;

        match my_age {
            1..=18 => println!("Important birthday"),
            20 | 50 => println!("Important birthday"),
            65..=i32::MAX => println!("Important birthday"),
            _ => println!("Not an important birthday"),
        }

        match my_age.cmp(&voting_age) {
            Ordering::Less => println!("Sorry you can't vote"),
            Ordering::Greater => println!("You can vote"),
            Ordering::Equal => println!("You're welcome to make your first vote"),
        }
    }
}

pub fn match_day() {
    crate::r#match::match_age::main();
}
