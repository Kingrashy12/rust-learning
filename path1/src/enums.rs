mod day {
    pub fn main() {
        enum Day {
            Monday,
            TuesDay,
            WednesDay,
            Thursday,
            Friday,
            Saturday,
            Sunday,
        }

        impl Day {
            fn is_weenkend(&self) -> bool {
                match self {
                    Day::Saturday | Day::Sunday => true,
                    _ => false,
                }
            }
        }

        let today: Day = Day::Monday;

        match today {
            Day::Monday => println!("Everyone hates monday"),
            Day::TuesDay => println!("Get up and go to work"),
            Day::WednesDay => println!("Idk"),
            Day::Thursday => println!("It's almost friday"),
            Day::Friday => println!("Everyone loves friday"),
            _ => println!("It weekend 👌"),
        }

        println!("Is it weekend? {}", today.is_weenkend())
    }
}

pub fn day_enum() {
    crate::enums::day::main();
}
