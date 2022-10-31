use std::fmt;
use std::io::{self, BufRead};

/// pattern match parsing error handler
type PatternMatchResult<T> = std::result::Result<T, PatternMatchError>;

#[derive(Debug, Clone)]
struct PatternMatchError;
impl fmt::Display for PatternMatchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Pattern match error!")
    }
}

type InstructionParseResult<T> = std::result::Result<T, InstructionParseError>;
#[derive(Debug, Clone)]
struct InstructionParseError;
impl fmt::Display for InstructionParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error parsing instruction!")
    }
}

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

        let cmds = Self::build_match_pattern_commands(&match_pattern);

        // apply match pattern commands to the origional lines and store them in a new vector
        let mut modified_lines: Vec<String> = Vec::new();

        Self {
            input_text,
            match_pattern,
            origional_lines: final_lines.clone(),
            modified_lines: final_lines.clone(),
        }
    }

    fn build_match_pattern_commands(match_pattern: &str) -> Vec<MatchPatternCommand> {
        let mut cmds: Vec<MatchPatternCommand> = Vec::new();

        for cmd in match_pattern.split('|') {
            match MatchPatternCommand::new(cmd.to_string()) {
                Ok(cmd) => cmds.push(cmd),
                Err(_) => {
                    panic!("invalid match pattern command");
                }
            };
        }

        cmds
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

enum ShiftDirection {
    Left,
    Right,
}
#[derive(Debug, Clone)]
struct PatternInstruction {
    delimeter: String,
    shift_count: i32,
    shift_direction: ShiftDirection,
    textTransform: TextTransform,
}

// /// a single instruction in a match pattern command
// #[derive(Debug, Clone)]
// enum PatternInstruction {
//     Delimeter(char),
//     ShiftDirectionAndCount(String),
//     TextTransform(String),
//     // CharTrimDirectionAndCount(String),
// }

// /// alias for long name
// type PI = PatternInstruction;

// impl PartialEq for PatternInstruction {
//     fn eq(&self, other: &Self) -> bool {
//         match (self, other) {
//             (PI::Delimeter(a), PI::Delimeter(b)) => a == b,
//             (PI::ShiftDirectionAndCount(a), PI::ShiftDirectionAndCount(b)) => a == b,
//             (PI::TextTransform(a), PI::TextTransform(b)) => a == b,
//             _ => false,
//         }
//     }
// }

// impl PatternInstruction {
//     /// resolve a single instruction of a match pattern command and apply it to the input string
//     fn resolve(&self, input: &str) -> PatternMatchResult<String> {
//         match *self {
//             PI::Delimeter(c) => {
//                 let valid_delims = ['.', ',', ';', '/', '\\'];
//                 if valid_delims.contains(&c) {
//                     return Ok(c.to_string());
//                 }

//                 Err(PatternMatchError)
//             }
//             PI::ShiftDirectionAndCount(ref s) => {
//                 panic!("shift direction and count: {}", s);
//             }
//             PI::TextTransform(ref s) => {
//                 panic!("text transform: {}", s);
//             }
//             _ => Err(PatternMatchError),
//         }
//     }
// }

/// a single command in a match pattern
struct MatchPatternCommand {
    pub instructions: Vec<PatternInstruction>,
}

impl MatchPatternCommand {
    fn new(command: String) -> InstructionParseResult<Self> {
        let raw_instructions: Vec<&str> = command.split('%').collect();
        if raw_instructions.len() == 0 {
            return Err(InstructionParseError);
        }

        let mut instructions: Vec<PatternInstruction> = Vec::new();

        // delimeter
        let delimeter = raw_instructions[0].chars().next().unwrap();
        instructions.push(PI::Delimeter(delimeter));

        // shift direction and count
        let sdc = raw_instructions[1].to_string();
        instructions.push(PI::ShiftDirectionAndCount(sdc));

        // text transform
        let tt = raw_instructions[2].to_string();
        instructions.push(PI::TextTransform(tt));

        Ok(Self { instructions })
    }

    // apply the command to the input text
    pub fn apply(&self, input: &str) -> String {
        let mut output = input.to_string();

        // apply the delimeter
        let valid_delims = ['.', ',', ';', '/', '\\'];
        match self.instructions[0] {
            PI::Delimeter(c) => {
                if valid_delims.contains(&c) {
                    output.push(c);
                }
            }
            _ => {}
        }

        // apply the shift direction and count
        let sdc = self.instructions[1];

        // apply the text transform
        let tt = self.instructions[2];

        output
    }
}

#[cfg(test)]
mod ultrawild_tests {
    use super::*;

    #[test]
    fn ultrawild_check_origional_lines() {
        let input = "test \n test2 \n test3 \n test4";
        let _namer = UltraWild::new(input.to_string(), ".%<0%does *".to_string());
        assert_eq!(_namer.origional_lines.len(), 4);

        for l in _namer.origional_lines.iter() {
            assert_eq!(l.contains("test"), true);
        }
    }

    #[test]
    fn match_pattern_first_split() {
        let input = ".%<0%does *";
        match MatchPatternCommand::new(input.to_string()) {
            Ok(cmd) => {
                assert_eq!(cmd.instructions.len(), 3);
                assert_eq!(cmd.instructions[0], PI::Delimeter('.'));
            }
            Err(_) => {
                assert!(false);
            }
        }
    }

    #[test]
    fn match_pattern_instruction_check() {
        let arr = vec![".", "<0", "does *"];
        let input = arr.join("%");
        match MatchPatternCommand::new(input.to_string()) {
            Ok(cmd) => {
                assert_eq!(
                    cmd.instructions[0],
                    PI::Delimeter(arr[0].chars().next().unwrap())
                );
                assert_eq!(
                    cmd.instructions[1],
                    PI::ShiftDirectionAndCount(arr[1].to_string())
                );
                assert_eq!(cmd.instructions[2], PI::TextTransform(arr[2].to_string()));
            }
            _ => {
                assert!(false);
            }
        }
    }

    #[test]
    fn match_pattern_delimeter_check() {
        let arr = vec![".", "<0", "does *"];
        let input = arr.join("%");
        match MatchPatternCommand::new(input.to_string()) {
            Ok(cmd) => {
                assert_eq!(
                    cmd.apply_instruction(0, &input.to_string()),
                    ".".to_string()
                );
            }
            _ => {
                assert!(false);
            }
        }
    }
}
