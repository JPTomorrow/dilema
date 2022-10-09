use std::io::{self, BufRead};

pub struct UltraWild {
    pub input_text: String,
    pub match_pattern: String,
    pub origional_lines: Vec<String>,
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
            origional_lines: final_lines.clone(),
            modified_lines: final_lines.clone(),
        }
    }

    pub fn print_origional_lines(self) {
        println!(
            "origional lines count: {}",
            self.origional_lines.len().to_string()
        );
        for s in self.origional_lines {
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
        // assert_eq!(_namer.generated_names.len(), 1);
        // assert_eq!(_namer.generated_names[0].contains("test"), true);
    }

    #[test]
    fn ultrawild_check_origional_lines() {
        let input = "test \n test2 \n test3 \n test4";
        let _namer = UltraWild::new(input.to_string(), "*.*".to_string());
        assert_eq!(_namer.origional_lines.len(), 4);

        for l in _namer.origional_lines.iter() {
            assert_eq!(l.contains("test"), true);
        }
    }
}
