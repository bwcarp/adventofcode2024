use super::Day;

pub fn run(folder: &String) -> Day {
    let mut day = Day::new(folder, 2);

    day.pt1 = pt1(&day.input);
    day.pt2 = pt2(&day.input);

    day
}

fn pt1(input: &String) -> i64 {
    let mut total: i64 = 0;
    for line in input.lines() {
        let numbers: Vec<i64> = line.split_whitespace().map(|n| n.parse::<i64>().unwrap()).collect();
        if verify_safe(numbers) {
            total += 1;
        }
    }
    total
}

fn pt2(input: &String) -> i64 {
    let mut total: i64 = 0;
    for line in input.lines() {
        let numbers: Vec<i64> = line.split_whitespace().map(|n| n.parse::<i64>().unwrap()).collect();
        if problem_dampener(numbers) {
            total += 1;
        }
    }
    total
}

fn check_order(n: &Vec<i64>) -> bool {
    let mut l = 0;
    for i in 0..n.len()-1 {
        if n[i] < n[i+1] {
            l += 1;
        }
    }
    if l >= (n.len() / 2) {
        return true;
    }
    false
}

fn verify_safe(n: Vec<i64>) -> bool {
    let forward = check_order(&n);
    for i in 0..n.len()-1 {
        if n[i].abs_diff(n[i+1]) > 3 || (n[i] >= n[i+1] && forward) || (n[i] <= n[i+1] && !forward) {
            return false
        }
    }
    true
}

fn problem_dampener(n: Vec<i64>) -> bool {
    for i in 0..n.len() {
        let ni = n.clone().into_iter()
            .enumerate()
            .filter(|&(u, _)| u != i)
            .map(|(_, v)| v).collect();
        if verify_safe(ni) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use crate::parse_example_inputs;
    use super::*;

    #[test]
    fn day2_pt1() {
        let inputs = parse_example_inputs();
        let input_string = inputs["day2"][0].as_str().unwrap().to_string();
        let total = pt1(&input_string);
        assert_eq!(total, 2);

    }

    #[test]
    fn day2_pt2() {
        let inputs = parse_example_inputs();
        let input_string = inputs["day2"][0].as_str().unwrap().to_string();
        let total = pt2(&input_string);
        assert_eq!(total, 4);
    }
}