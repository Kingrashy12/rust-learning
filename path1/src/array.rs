mod array {
    pub fn main() {
        let arr_2 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut loop_idx = 0;

        loop {
            if arr_2[loop_idx] % 2 == 0 {
                loop_idx += 1;
                continue;
            }

            if arr_2[loop_idx] == 9 {
                break;
            }

            println!("Val from while : {}", arr_2[loop_idx]);
            loop_idx += 1;
        }

        while loop_idx < arr_2.len() {
            println!("Arr : {}", arr_2[loop_idx]);
            loop_idx += 1;
        }

        for val in arr_2.iter() {
            println!("Val : {}", val)
        }
    }
}

pub fn array_fn() {
    crate::array::array::main();
}
