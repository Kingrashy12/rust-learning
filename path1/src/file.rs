mod file {
    use std::fs::File;
    use std::io::{BufRead, BufReader, Write};

    pub fn write() {
        let path = "text.txt";
        let output = File::create(path);
        let mut output = match output {
            Ok(file) => file,
            Err(error) => {
                panic!("An error occured while creating file: {:?}", error)
            }
        };

        write!(output, "Just some\nRandom worlds").expect("Failed to write file");
        let input = File::open(path).unwrap();
        let buffered = BufReader::new(input);
        for line in buffered.lines() {
            println!("{}", line.unwrap())
        }
    }
}

pub fn write_text_file() {
    crate::file::file::write();
}
