#![allow(unused)]

use threads::threads::{bank, demo_thread};

mod array;
mod closures;
mod enums;
mod file;
mod functions;
mod hashmap;
mod r#match;
mod node;
mod pizza;
mod strings;
mod r#struct;
mod threads;

fn main() {
    // demo_thread();
    bank();
}
