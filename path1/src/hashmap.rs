mod hero {
    use std::collections::HashMap;

    pub fn main() {
        let mut heros = HashMap::new();
        heros.insert("Batman", "Wayne");
        heros.insert("Superman", "clerk");
        heros.insert("Flash", "Berry");

        for (k, v) in heros.iter() {
            println!("{} = {}", k, v)
        }

        if heros.contains_key(&"Batman") {
            let the_batman = heros.get(&"Batman");

            match the_batman {
                Some(x) => println!("Batman is a hero"),
                None => println!("Batman is not a hero"),
            }
        }
    }
}

pub fn heros() {
    crate::hashmap::hero::main();
}
