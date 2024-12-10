use super::Day;
use std::cmp::Ordering;

pub fn run(folder: &String) -> Day {
    let mut day = Day::new(folder, 5);

    let (orders, pages) = parse_input(&day.input);

    day.pt1 = pt1(&orders, &pages);
    day.pt2 = pt2(&orders, &pages);

    day
}

fn pt1(orders: &Vec<Vec<i64>>, pages: &Vec<Vec<i64>>) -> i64 {
    let mut total: i64 = 0;

    for pv in pages {
        if validator(&pv, &orders) {
            total += pv[pv.len() / 2];
        }
    }

    total
}

fn pt2(orders: &Vec<Vec<i64>>, pages: &Vec<Vec<i64>>) -> i64 {
    let mut total: i64 = 0;
    for pv in pages {
        if !validator(pv, orders) {
            let mut bad_pages: Vec<i64> = pv.clone();
            bad_pages.sort_by(|a, b| cmp_rules(&orders, *a, *b));
            total += bad_pages[bad_pages.len() / 2];
        }
    }
    total
}

fn cmp_rules(orders: &Vec<Vec<i64>>, a: i64, b: i64) -> Ordering {
    for x in orders {
        if (a, b) == (x[0], x[1]) {
            return Ordering::Less;
        } else if (b, a) == (x[0], x[1]) {
            return Ordering::Greater;
        }
    }
    Ordering::Equal
}

// This ran faster than my pretty solution.
fn validator(pv: &Vec<i64>, orders: &Vec<Vec<i64>>) -> bool {
    let mut valid: bool = false;
    for ov in orders {
        let pos1 = pv.iter().position(|&x| x == ov[0]);
        if pos1.is_some() {
            let pos2 = pv.iter().position(|&x| x == ov[1]);
            if pos2.is_some() {
                valid = true;
                if pos1.unwrap() > pos2.unwrap() {
                    return false
                }
            }
        }
    }
    valid
}

fn parse_input(input: &String) -> (Vec<Vec<i64>>, Vec<Vec<i64>>) {
    let mut orders: Vec<Vec<i64>> = vec![];
    let mut pages: Vec<Vec<i64>> = vec![];
    let mut input_iter = input.split("\n\n");
    for line in input_iter.next().unwrap().lines() {
        let ns: Vec<i64> = line.split("|").into_iter().map(|s| s.parse::<i64>().unwrap()).collect();
        orders.push(ns);
    }

    for line in input_iter.next().unwrap().lines() {
        let ns: Vec<i64> = line.split(",").into_iter().map(|s| s.parse::<i64>().unwrap()).collect();
        pages.push(ns);
    }

    (orders, pages)
}

#[cfg(test)]
mod tests {
    use crate::parse_example_inputs;
    use super::*;

    #[test]
    fn day5_pt1() {
        let inputs = parse_example_inputs();
        let input = inputs["day5"][0].as_str().unwrap().to_string();
        let (orders, pages) = parse_input(&input);
        let total = pt1(&orders, &pages);
        assert_eq!(total, 143);
    }

    #[test]
    fn day5_pt2() {
        let inputs = parse_example_inputs();
        let input = inputs["day5"][0].as_str().unwrap().to_string();
        let (orders, pages) = parse_input(&input);
        let total = pt2(&orders, &pages);
        assert_eq!(total, 123);   
    }
}