use super::Day;

pub fn run(folder: &String) -> Day {
    Day::new(folder, 0)
}

// Tests are dependent on example-inputs.toml.
// Make sure to fill out this file before using.
// It does not come prepopulated due to copyright.
#[cfg(test)]
mod tests {
    use crate::parse_example_inputs;
    use super::*;

    #[test]
    fn day0_hello_world() {
        let inputs = parse_example_inputs();
        let mut input_string = inputs["day0"][0].as_str().unwrap().to_string();
        let day = run(&"./inputs".to_string());

        // The method trims trailing new line, so we will with the test file as well.
        if input_string.ends_with("\n") {
            input_string.pop();
        }

        assert_eq!(input_string, day.input);
    }
}