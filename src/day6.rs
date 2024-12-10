use super::Day;
use crate::grid::Grid;

pub fn run(folder: &String) -> Day {
    let mut day = Day::new(folder, 6);

    let grid = Grid {
        x: 0,
        y: 0,
        grid: day.input.lines().map(|l| l.chars().collect()).collect(),
    };

    day.pt1 = pt1(&mut grid.clone());
    day.pt2 = pt2(&mut grid.clone());

    day
}

fn pt1(grid: &mut Grid<char>) -> i64 {
    starting_tile(grid);
    mark_walked_spaces(grid);

    let mut total: i64 = 0;
    for row in &grid.grid {
        total += row.iter().filter(|&x| *x == 'X').count() as i64;
    }

    total
}

fn pt2(grid: &mut Grid<char>) -> i64 {
    let mut total: i64 = 0;

    starting_tile(grid);
    let clean_grid = grid.clone();
    mark_walked_spaces(grid);

    for i in 0..grid.grid.len() {
        for j in 0..grid.grid[0].len() {
            if grid.grid[i][j] == 'X' {
                let mut tmp_grid = clean_grid.clone();
                tmp_grid.grid[i][j] = '#';
                if mark_walked_spaces(&mut tmp_grid) {
                    total += 1;
                }
            }
        }
    }

    total
}

fn starting_tile(grid: &mut Grid<char>) -> (usize, usize) {
    for i in 0..grid.grid.len() {
        let p = grid.grid[i].iter().position(|&x| x == '^');
        if p.is_some() {
            grid.y = i;
            grid.x = p.unwrap();
        }
    }
    (grid.x, grid.y)
}

// true if stuck
fn mark_walked_spaces(grid: &mut Grid<char>) -> bool {
    let mut bumps: Vec<(usize, usize)> = vec![];
    'a: loop {
        // up
        loop {
            grid.grid[grid.y][grid.x] = 'X';
            if grid.y == 0 { break 'a; }
            if grid.grid[grid.y-1][grid.x] == '#' {
                bumps.push((grid.x,grid.y));
                break;
            } else {
                grid.y -= 1;
            }
        }
        // right
        loop {
            grid.grid[grid.y][grid.x] = 'X';
            if grid.x == grid.grid[grid.y].len()-1 { break 'a; }
            if grid.grid[grid.y][grid.x+1] == '#' {
                bumps.push((grid.x,grid.y));
                break;
            } else {
                grid.x += 1;
            }
        }
        // down
        loop {
            grid.grid[grid.y][grid.x] = 'X';
            if grid.y == grid.grid.len()-1 { break 'a; }
            if grid.grid[grid.y+1][grid.x] == '#' {
                bumps.push((grid.x,grid.y));
                break;
            } else {
                grid.y += 1;
            }
        }
        // left
        loop {
            grid.grid[grid.y][grid.x] = 'X';
            if grid.x == 0 { break 'a; }
            if grid.grid[grid.y][grid.x-1] == '#' {
                bumps.push((grid.x,grid.y));
                break;
            } else {
                grid.x -= 1;
            }
        }
        for bump in &bumps {
            if bumps.iter().filter(|&x| *x == *bump).count() > 2 {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use crate::parse_example_inputs;
    use super::*;

    #[test]
    fn day6_pt1() {
        let inputs = parse_example_inputs();
        let input = inputs["day6"][0].as_str().unwrap().to_string();
        let mut grid = Grid {
            x: 0,
            y: 0,
            grid: input.lines().map(|l| l.chars().collect()).collect(),
        };
        let total = pt1(&mut grid);
        assert_eq!(total, 41);
    }

    #[test]
    fn day6_pt2() {
        let inputs = parse_example_inputs();
        let input = inputs["day6"][0].as_str().unwrap().to_string();
        let mut grid = Grid {
            x: 0,
            y: 0,
            grid: input.lines().map(|l| l.chars().collect()).collect(),
        };
        let total = pt2(&mut grid);
        assert_eq!(total, 6);
    }
}