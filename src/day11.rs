use std::collections::HashMap;

struct Grid {
    cells: [[i32; 300]; 300],
    mini_grid_cache: HashMap<(usize, usize, usize), i32>,
}

impl Grid {
    #[allow(dead_code)]
    fn new(serial_number: usize) -> Grid {
        let mut cells: [[i32; 300]; 300] = [[0; 300]; 300];
        for i in 0..cells.len() {
            for j in 0..cells.len() {
                cells[i][j] = Grid::magic_number(i, j, serial_number);
            }
        }
        let mini_grid_cache = HashMap::new();
        Grid {
            cells,
            mini_grid_cache,
        }
    }

    fn magic_number(x: usize, y: usize, serial_number: usize) -> i32 {
        let rack_id = x + 10;
        let mut power_level = rack_id * y;
        power_level += serial_number;
        power_level *= rack_id;

        let hundreds_digit = (power_level / 100) % 10;

        (hundreds_digit as i32) - 5
    }

    #[allow(dead_code)]
    fn best_simple_spot(&mut self) -> (usize, usize) {
        let mut candidate_spot: (usize, usize) = (0, 0);
        let mut candidate_score = self.mini_grid_value(0, 0, 3);
        for i in 0..self.cells.len() - 3 {
            for j in 0..self.cells.len() - 3 {
                let score = self.mini_grid_value(i, j, 3);
                if score > candidate_score {
                    candidate_score = score;
                    candidate_spot = (i, j);
                }
            }
        }
        candidate_spot
    }

    #[allow(dead_code)]
    fn best_complex_spot(&mut self) -> (usize, usize, usize) {
        for i in 0..self.cells.len() {
            for j in 0..self.cells.len() {
                let biggest_possible_square = 300 - usize::max(i, j);
                for k in 1..=biggest_possible_square {
                    self.mini_grid_value(i, j, k);
                }
            }
        }

        let mut max_key = (0, 0, 1);
        let mut max_val = &self.mini_grid_cache[&max_key];
        for (key, value) in &self.mini_grid_cache {
            if value > max_val {
                max_val = value;
                max_key = *key;
            }
        }
        max_key
    }

    fn mini_grid_value(&mut self, x: usize, y: usize, mini_grid_size: usize) -> i32 {
        let cache_key = (x, y, mini_grid_size);
        if mini_grid_size == 1 {
            self.mini_grid_cache.insert(cache_key, self.cells[x][y]);
            return self.cells[x][y];
        }
        if self.mini_grid_cache.contains_key(&cache_key) {
            return self.mini_grid_cache[&cache_key];
        }

        let mut sum = 0;
        sum += self.vector_value(x, y, x + mini_grid_size - 1, y);
        sum += self.vector_value(x, y, x, y + mini_grid_size - 1);
        sum += self.mini_grid_value(x + 1, y + 1, mini_grid_size - 1);
        sum -= self.cells[x][y];

        self.mini_grid_cache.insert(cache_key, sum);
        sum
    }

    fn vector_value(&self, start_x: usize, start_y: usize, end_x: usize, end_y: usize) -> i32 {
        let mut sum = 0;
        for i in start_x..=end_x {
            for j in start_y..=end_y {
                sum += self.cells[i][j]
            }
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::Grid;

    #[test]
    fn test_magic() {
        assert_eq!(4, Grid::magic_number(3, 5, 8));
        assert_eq!(-5, Grid::magic_number(122, 79, 57));
        assert_eq!(0, Grid::magic_number(217, 196, 39));
        assert_eq!(4, Grid::magic_number(101, 153, 71))
    }

    #[test]
    fn test_simple_spot() {
        let mut grid = Grid::new(18);
        assert_eq!((33, 45), grid.best_simple_spot());
        let mut grid = Grid::new(42);
        assert_eq!((21, 61), grid.best_simple_spot());
        let mut grid = Grid::new(1308);
        assert_eq!((21, 41), grid.best_simple_spot());
    }

    #[test]
    #[ignore]
    fn test_complex_spot() {
        let mut grid = Grid::new(1308);
        assert_eq!((227, 199, 19), grid.best_complex_spot());
    }

    #[test]
    fn test_vector_value() {
        let grid = Grid::new(42);
        assert_eq!(14, grid.vector_value(21, 61, 24, 61));
        assert_eq!(13, grid.vector_value(21, 61, 21, 64));
    }
}
