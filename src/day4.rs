use super::Day;
use crate::grid::Grid;
use regex::Regex;

pub fn run(folder: &String) -> Day {
    let mut day = Day::new(folder, 4);
    let grid = Grid {
        x: 0,
        y: 0,
        grid: day.input.lines().map(|l| l.chars().collect()).collect(),
    };

    day.pt1 = pt1(&grid);
    day.pt2 = pt2(&grid);

    day
}

fn pt1(grid: &Grid<char>) -> i64 {
    let mut total = 0;

    total += grid.grid.iter()
                .fold(0,|c, v| c + count_words(v));
    total += grid.rotate_right_new().grid.iter()
                .fold(0,|c, v| c + count_words(v));
    total += grid.diag_left_corner().iter()
            .fold(0,|c, v| c + count_words(v));
    total += grid.diag_right_corner().iter()
        .fold(0,|c, v| c + count_words(v));
    total as i64
}

fn pt2(grid: &Grid<char>) -> i64 {
    let mut total: i64 = 0;
    let g = &grid.grid;

    for y in 1..(g.len() - 1) {
        for x in 1..(g[0].len() - 1) {
            if g[y][x] == 'A' {
                let mas: Vec<char> = vec![g[y-1][x-1], g[y-1][x+1],g[y+1][x-1], g[y+1][x+1]]
                    .into_iter().filter(|c| *c == 'M' || *c == 'S').collect();
                if mas.len() == 4 && !(mas[0] == mas[3] || mas[1] == mas[2]) {
                    total += 1;
                }
            }
        }
    }

    total
}

fn count_words(line: &Vec<char>) -> usize {
    let mut total = 0;
    let re_f = Regex::new(r"(XMAS)").unwrap();
    let re_b = Regex::new(r"(SAMX)").unwrap();
    let hay = line.iter().collect::<String>();

    total += re_f.find_iter(hay.as_str()).count();
    total += re_b.find_iter(hay.as_str()).count();

    total
}

#[cfg(test)]
mod tests {
    use crate::parse_example_inputs;
    use super::*;

    #[test]
    fn day4_pt1() {
        let inputs = parse_example_inputs();
        let input = inputs["day4"][0].as_str().unwrap().to_string();
        let grid = Grid {
            x: 0,
            y: 0,
            grid: input.lines().map(|l| l.chars().collect()).collect(),
        };
        let total = pt1(&grid);
        assert_eq!(total, 18);
    }

    #[test]
    fn day4_pt2() {
        let inputs = parse_example_inputs();
        let input = inputs["day4"][0].as_str().unwrap().to_string();
        let grid = Grid {
            x: 0,
            y: 0,
            grid: input.lines().map(|l| l.chars().collect()).collect(),
        };
        let total = pt2(&grid);
        assert_eq!(total, 9);
    }
}