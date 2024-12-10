#[derive(Debug, Clone)]
pub struct Grid<T> {
    pub grid: Vec<Vec<T>>,
    pub x: usize, // current horizontal position
    pub y: usize, // current vertical position
}

impl<T: Copy + std::fmt::Debug> Grid<T> {
    pub fn get(&self, mut x: i32, mut y: i32) -> T {
        if x == self.x as i32 && y == self.y as i32 {
            return self.grid[self.y][self.x];
        }

        if y < 0 || y >= self.grid.len() as i32 {
            y = y.rem_euclid(self.grid.len() as i32);
        }
        let y = y as usize;

        if x < 0 || x >= self.grid[y].len() as i32 {
            x = x.rem_euclid(self.grid[y].len() as i32);
        }

        let x = x as usize;

        self.grid[y][x]
    }

    pub fn get_at_cursor(&self) -> T {
        self.grid[self.y][self.x]
    }

    pub fn set_cursor(&mut self, mut x: i32, mut y: i32) {
        if y < 0 || y >= self.grid.len() as i32 {
            y = y.rem_euclid(self.grid.len() as i32);
        }
        self.y = y as usize;

        if x < 0 || x >= self.grid[self.y].len() as i32 {
            x = x.rem_euclid(self.grid[self.y].len() as i32);
        }

        self.x = x as usize;
    }

    pub fn set_and_get(&mut self, x: i32, y: i32) -> T {
        self.set_cursor(x, y);
        self.get(x, y)
    }

    pub fn move_cursor(&mut self, mut x: i32, mut y: i32) -> (usize, usize) {
        x += self.x as i32;
        y += self.y as i32;

        if x > self.grid[self.y].len() as i32 || x < 0 {
            panic!("Cursor moved out of bounds. Attempted location: {}, {}", x, y);
        } else {
            self.x = x as usize;
        }

        if y > self.grid.len() as i32 || y < 0 {
            panic!("Cursor moved out of bounds. Attempted location: {}, {}", x, y);
        } else {
            self.y = y as usize;
        }

        (self.x, self.y)
    }

    pub fn diag_left_corner(&self) -> Vec<Vec<T>> {
        let mut rows = vec![vec![]; self.grid[0].len() + self.grid.len() - 1];

        for x in 0..self.grid[0].len() {
            for y in 0..self.grid.len() {
                rows[x+y].push(self.grid[y][x]);
            }
        }

        rows
    }

    pub fn diag_right_corner(&self) -> Vec<Vec<T>> {
        let mut rows = vec![vec![]; self.grid[0].len() + self.grid.len() - 1];

        let flipped_grid: Vec<Vec<T>> = self.grid.iter()
                                            .map(|v| v.clone().into_iter().rev().collect()).collect();
        for x in 0..flipped_grid[0].len() {
            for y in 0..flipped_grid.len() {
                rows[x+y].push(flipped_grid[y][x]);
            }
        }

        rows
    }

    pub fn rotate_right_new(&self) -> Grid<T> {
        let mut new_grid: Vec<Vec<T>> = vec![];
        for i in 0..self.grid[0].len() {
            let row = self.grid.iter().map(|y| y[i]).rev().collect();
            new_grid.push(row);
        }
        Grid { 
            grid: new_grid,
            x: self.grid.len() - self.y - 1,
            y: self.x,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid() {
        let mut grid_chars = Grid {
            grid: vec![
                vec!["A","B","C","D","E"],
                vec!["F","G","H","I","J"],
                vec!["K","L","M","N","O"],
                vec!["P","Q","R","S","T"],
                vec!["U","V","W","X","Y"],
            ],
            x: 3,
            y: 3,
        };
        assert_eq!(grid_chars.get(1,1), "G");
        assert_eq!(grid_chars.get(-3, -2), "R");

        grid_chars.move_cursor(-2, 1);
        assert_eq!(grid_chars.x, 1);
        assert_eq!(grid_chars.y, 4);

        let new_grid = grid_chars.rotate_right_new();
        assert_eq!(new_grid.get(1,3), "S");
        assert_eq!(new_grid.x, 0);
        assert_eq!(new_grid.y, 1);
        assert_eq!(new_grid.get(new_grid.x as i32, new_grid.y as i32), "V");
    }
}