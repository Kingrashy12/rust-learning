mod string_fn {
    pub fn main() {
        let mut str1: String = String::new();
        str1.push('A');
        str1.push_str(" word");
        for word in str1.split_whitespace() {
            println!("{}", word);
        }
        let str2 = str1.replace("A", "Another");
        println!("{}", str2);

        let str3 = String::from("k k a g c r r p a");
        let mut v1: Vec<char> = str3.chars().collect();
        v1.sort();
        v1.dedup();
        for char in v1 {
            println!("{}", char)
        }

        let str4 = "Random string";
        let mut str5 = str4.to_string();

        println!("{}", str5);

        let bytes_arr = str5.as_bytes();
        let str6 = &str5[0..6];
        println!("String Length: {}", str6.len());
        str5.clear();
    }
}

pub fn run_str() {
    crate::strings::string_fn::main();
}
