mod closures {
    pub fn closure() {
        let can_vote = |age: i32| age >= 18;

        println!("Can vote: {}", can_vote(10));

        let mut smap1 = 5;

        let print_var = || println!("Samp1 is = {}", smap1);

        print_var();

        smap1 = 10;

        let mut change_var = || smap1 += 5;
        change_var();

        println!("Samp1 is = {}", smap1);

        smap1 = 15;

        println!("Samp1 is = {}", smap1);
    }
}

pub fn run() {
    crate::closures::closures::closure();
}
