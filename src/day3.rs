use super::Day;
use regex::Regex;

pub fn run(folder: &String) -> Day {
    let mut day = Day::new(folder, 3);

    day.pt1 = pt1(&day.input);
    day.pt2 = pt2(&day.input);

    day
}

fn pt1(input: &String) -> i64 {
    let mut total: i64 = 0;

    let re = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();
    let re_num = Regex::new(r"[0-9]+").unwrap();

    for c in re.captures_iter(input) {
        let s = c.get(0).unwrap().as_str();
        let mut numcaps = re_num.captures_iter(s);
        let l = numcaps.next().unwrap().get(0).unwrap().as_str().parse::<i64>().unwrap();
        let r = numcaps.next().unwrap().get(0).unwrap().as_str().parse::<i64>().unwrap();
        total += l * r;
    }

    total
}

fn pt2(input: &String) -> i64 {
    let mut total: i64 = 0;

    let re = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();
    let re_num = Regex::new(r"[0-9]+").unwrap();

    for line in input.split("do()") {
        let substr = line.split("don't()").next().unwrap();
        for c in re.captures_iter(substr) {
            let s = c.get(0).unwrap().as_str();
            let mut numcaps = re_num.captures_iter(s);
            let l = numcaps.next().unwrap().get(0).unwrap().as_str().parse::<i64>().unwrap();
            let r = numcaps.next().unwrap().get(0).unwrap().as_str().parse::<i64>().unwrap();
            total += l * r;
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use crate::parse_example_inputs;
    use super::*;

    #[test]
    fn day3_pt1() {
        let inputs = parse_example_inputs();
        let input = inputs["day3"][0].as_str().unwrap().to_string();
        let total = pt1(&input);
        assert_eq!(total, 161);
    }

    #[test]
    fn day3_pt2() {
        let inputs = parse_example_inputs();
        let input = inputs["day3"][1].as_str().unwrap().to_string();
        let total = pt2(&input);
        assert_eq!(total, 48);
    }
}