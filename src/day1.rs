use super::Day;

pub fn run(folder: &String) -> Day {
    let mut day = Day::new(folder, 1);
    let sorted_lists: Vec<Vec<i64>> = split_and_sort(&day.input);

    day.pt1 = pt1(&sorted_lists);
    day.pt2 = pt2(&sorted_lists);

    day
}

fn pt1(sorted_lists: &Vec<Vec<i64>>) -> i64 {
    let left = &sorted_lists[0];
    let right = &sorted_lists[1];
    let mut total: i64 = 0;

    for n in 0..left.len() {
        total += (left[n] - right[n]).abs();
    }

    total
}

fn pt2(sorted_lists: &Vec<Vec<i64>>) -> i64 {
    let left = &sorted_lists[0];
    let right = &sorted_lists[1];
    let mut total: i64 = 0;

    for n in left {
        let count= right.iter().filter(|&i| *i == *n).count();
        total += n * count as i64;
    }

    total
}

fn split_and_sort(input: &String) -> Vec<Vec<i64>> {
    let mut left_list: Vec<i64> = vec![];
    let mut right_list: Vec<i64> = vec![];
    let mut sorted_lists: Vec<Vec<i64>> = vec![];

    for line in input.lines() {
        let split_str: Vec<&str> = line.split_whitespace().collect();
        left_list.push(split_str[0].parse::<i64>().unwrap());
        right_list.push(split_str[1].parse::<i64>().unwrap());
    }

    left_list.sort();
    right_list.sort();

    sorted_lists.push(left_list);
    sorted_lists.push(right_list);

    sorted_lists
}

#[cfg(test)]
mod tests {
    use crate::parse_example_inputs;
    use super::*;

    #[test]
    fn day1() {
        let inputs = parse_example_inputs();
        let mut input_string = inputs["day1"][0].as_str().unwrap().to_string();

        // The method trims trailing new line, so we will with the test file as well.
        if input_string.ends_with("\n") {
            input_string.pop();
        }

        let sorted_lists = split_and_sort(&input_string);
        let total = pt1(&sorted_lists);
        assert_eq!(total, 11);
        let total = pt2(&sorted_lists);
        assert_eq!(total, 31);
    }
}