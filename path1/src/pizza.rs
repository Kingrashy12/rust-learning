mod pizza {
    pub struct Pizza {
        pub dough: String,
        pub cheese: String,
        pub topping: String,
    }

    impl Pizza {
        pub fn lunch(topping: &str) -> Pizza {
            Pizza {
                dough: String::from("Regular pizza"),
                cheese: String::from("Cheese pizza"),
                topping: String::from(topping),
            }
        }
    }

    pub mod help_customer {
        fn seat_at_table() {
            println!("Customer is seated")
        }

        pub fn take_order() {
            seat_at_table();
            let cust_pizza: super::Pizza = super::Pizza::lunch("itailan");
            server_customer(cust_pizza);
        }

        fn server_customer(cust_pizza: super::Pizza) {
            println!("The customer is served")
        }
    }
}

pub fn order_food() {
    crate::pizza::pizza::help_customer::take_order();
}
