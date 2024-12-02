pub mod day0;
pub mod day1;
pub mod day2;

use std::fs;

#[derive(Debug)]
pub struct Day {
    pt1: i64,
    pt2: i64,
    day: i8,
    input: String,
}

impl Day {
    pub fn new(folder: &String, day: i8) -> Day {
        let input_file = format!("{}/day{}.txt", folder, day);
        let mut input = fs::read_to_string(&input_file)
            .unwrap_or_else(|_| panic!("Could not open file {}", &input_file));
        if input.ends_with("\n") {
            input.pop();
        }
        Day {
            pt1: 0,
            pt2: 0,
            day: day,
            input: input,
        }
    }

    pub fn values(&self) {
        println!("\nResults for day {}:", self.day);
        println!("Part 1 - {}", self.pt1);
        println!("Part 2 - {}", self.pt2);
    }
}

// Only used for unit tests
pub fn parse_example_inputs() -> toml::Table {
    let ex_input_text = fs::read_to_string("./inputs/example-inputs.toml")
        .expect("Could not open example inputs file.");
    toml::from_str(&ex_input_text).expect("Could not parse example inputs file.")
}