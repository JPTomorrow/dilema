use std::io::{self, BufRead};

pub struct UltraWild {
    pub input_text: String,
    pub match_pattern: String,
    pub modified_lines: Vec<String>,
}

impl UltraWild {
    pub fn new(input_text: String, match_pattern: String) -> Self {
        // parse the input as if it were a multi line
        // text file even though the input can be a small string
        let mut final_lines = Vec::new();
        for line in io::BufReader::new(input_text.as_bytes()).lines() {
            if let Ok(text) = line {
                final_lines.push(text);
            }
            // TODO: implement error handling
        }

        Self {
            input_text,
            match_pattern,
            modified_lines: final_lines,
        }
    }

    pub fn print_lines(self) {
        for s in self.modified_lines {
            println!("{}", s);
        }
    }
}

#[cfg(test)]
mod ultrawild_tests {
    use super::*;

    // test new namer
    #[test]
    fn ultrawild_create_new() {
        let _namer = UltraWild::new("test".to_string(), "*.*".to_string());
        //assert_eq!(_namer.generated_names.len(), 1);
        //assert_eq!(_namer.generated_names[0].contains("test"), true);
    }
}
