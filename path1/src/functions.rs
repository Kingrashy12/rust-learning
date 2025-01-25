pub mod fns {
    use std::ops::Add;

    pub fn get_sum(x: i32, y: i32) -> i32 {
        x + y
    }

    pub fn get_2(x: i32) -> (i32, i32) {
        (x + 4, x + 8)
    }

    pub fn sum_list(list: &[i32]) -> i32 {
        let mut sum = 0;
        for &vec in list.iter() {
            sum += &vec;
        }
        sum
    }

    pub fn get_sum_gen<T: Add<Output = T>>(x: T, y: T) -> T {
        x + y
    }
}
