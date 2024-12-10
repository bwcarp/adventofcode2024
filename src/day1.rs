use super::Day;

pub fn run(folder: &String) -> Day {
    let mut day = Day::new(folder, 1);
    let (left, right) = split_and_sort(&day.input);

    day.pt1 = pt1(&left, &right);
    day.pt2 = pt2(&left, &right);

    day
}

fn pt1(left: &Vec<i64>, right: &Vec<i64>) -> i64 {
    left.iter().zip(right.iter())
        .fold(0, |sum, (&x, &y) | sum + (x - y).abs())
}

fn pt2(left: &Vec<i64>, right: &Vec<i64>) -> i64 {
    let mut total: i64 = 0;

    for n in left {
        let count = right.iter().filter(|&i| *i == *n).count();
        total += n * count as i64;
    }

    total
}

fn split_and_sort(input: &String) -> (Vec<i64>, Vec<i64>) {
    let mut left: Vec<i64> = vec![];
    let mut right: Vec<i64> = vec![];

    for line in input.lines() {
        let split_str: Vec<&str> = line.split_whitespace().collect();
        left.push(split_str[0].parse::<i64>().unwrap());
        right.push(split_str[1].parse::<i64>().unwrap());
    }

    left.sort();
    right.sort();

    (left, right)
}

#[cfg(test)]
mod tests {
    use crate::parse_example_inputs;
    use super::*;

    #[test]
    fn day1_pt1() {
        let inputs = parse_example_inputs();
        let input_string = inputs["day1"][0].as_str().unwrap().to_string();
        let (left, right) = split_and_sort(&input_string);
        let total = pt1(&left, &right);
        assert_eq!(total, 11);
    }

    #[test]
    fn day1_pt2() {
        let inputs = parse_example_inputs();
        let input_string = inputs["day1"][0].as_str().unwrap().to_string();
        let (left, right) = split_and_sort(&input_string);
        let total = pt2(&left, &right);
        assert_eq!(total, 31);
    }
}