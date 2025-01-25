pub mod threads {
    use std::{
        sync::{Arc, Mutex},
        thread,
        time::Duration,
    };

    pub fn demo_thread() {
        let thread1 = thread::spawn(|| {
            for i in 1..25 {
                println!("Spawn thread {}", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..20 {
            println!("Main thread {}", i);
            thread::sleep(Duration::from_millis(1));
        }

        thread1.join().unwrap()
    }

    pub fn bank() {
        pub struct Bank {
            balance: f32,
        }

        fn withdraw(bank: &Arc<Mutex<Bank>>, amt: f32) {
            let mut bank_ref = bank.lock().unwrap();

            if bank_ref.balance < 5.00 {
                println!("Current balance is below ${}", amt);
            } else {
                bank_ref.balance -= 5.00;
                println!(
                    "Withdrawal of ${} was successful, Balance left ${}",
                    amt, bank_ref.balance
                )
            }
        }

        fn customer(bank: &Arc<Mutex<Bank>>) {
            withdraw(bank, 5.00);
        }

        let bank = Arc::new(Mutex::new(Bank { balance: 35.00 }));

        let handles = (0..10).map(|_| {
            let bank_ref = bank.clone();
            thread::spawn(move || {
                customer(&bank_ref);
            })
        });
        for handle in handles {
            handle.join().unwrap();
        }
        println!("Total {}", bank.lock().unwrap().balance)
    }
}
