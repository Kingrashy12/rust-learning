fn main() {
    #[derive(Debug)]
    struct Customer {
        name: String,
        address: String,
        balance: f32,
    }

    let mut bob = Customer {
        name: String::from("Bob Mike"),
        address: String::from("555 main st"),
        balance: 255.50,
    };
    bob.address = String::from("505 main st");
    println!("{:#?}", bob);

    struct Rectangle {
        length: f32,
        width: f32,
    }

    trait Shape {
        fn new(length: f32, width: f32) -> Self;
        fn area(&self) -> f32;
    }

    impl Shape for Rectangle {
        fn new(length: f32, width: f32) -> Self {
            return Rectangle { length, width };
        }

        fn area(&self) -> f32 {
            return self.length * self.width;
        }
    }
}
