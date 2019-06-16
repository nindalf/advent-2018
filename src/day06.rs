use std::collections::HashMap;
use std::str::FromStr;

use regex::Regex;

#[derive(Debug)]
struct Grid {
    tiles: Vec<Option<Point>>,
    specials: Vec<Point>,
    length: usize,
    breadth: usize,
}

impl Grid {
    #[allow(dead_code)]
    fn new(input: &str) -> Grid {
        let specials: Vec<Point> = input.lines().map(|l| Point::parse(l)).collect();

        let length = specials.iter().map(|point| point.x).max().unwrap() + 1;
        let breadth = specials.iter().map(|point| point.y).max().unwrap() + 1;

        let tiles: Vec<Option<Point>> = vec![None; length * breadth];
        let mut grid = Grid {
            tiles,
            specials,
            length,
            breadth,
        };
        for i in 0..length {
            for j in 0..breadth {
                let p = Point::new(i, j);
                let nearest = p.nearest_point(&grid.specials);
                grid.set_tile(nearest, i, j);
            }
        }

        grid
    }

    #[allow(dead_code)]
    fn largest_internal_area(&self) -> usize {
        let mut result: HashMap<Point, usize> = HashMap::new();
        self.specials.iter().for_each(|point| {
            result.insert(*point, 0);
        });

        for i in 0..self.length {
            for j in 0..self.breadth {
                let tile = self.get_tile(i, j);
                if tile.is_none() {
                    continue;
                }
                let nearest = tile.unwrap();
                if self.is_edge(i, j) {
                    result.remove(&nearest);
                    continue;
                }
                result.entry(nearest)
                    .and_modify(|times| *times = *times + 1);
            }
        }
        *result.values().max().unwrap()
    }

    #[allow(dead_code)]
    fn largest_safe_area(&self, limit: usize) -> usize {
        let mut result = 0;
        for i in 0..self.length {
            for j in 0..self.breadth {
                let p = Point::new(i, j);
                if p.total_distance(&self.specials) < limit {
                    result += 1;
                }
            }
        }
        result
    }

    fn get_tile(&self, x: usize, y: usize) -> Option<Point> {
        self.tiles[x + y * self.length]
    }

    fn set_tile(&mut self, p: Option<Point>, x: usize, y: usize) {
        self.tiles[x + y * self.length] = p;
    }

    fn is_edge(&self, x: usize, y: usize) -> bool {
        x == 0 || y == 0 || x == self.length - 1 || y == self.breadth - 1
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    #[allow(dead_code)]
    fn parse(s: &str) -> Point {
        lazy_static! {
            static ref RE: Regex = Regex::new("(?P<x>[0-9]*), (?P<y>[0-9]*)").unwrap();
        }

        let caps = RE.captures(s).unwrap();
        let x: usize = usize::from_str(&caps["x"]).unwrap();
        let y: usize = usize::from_str(&caps["y"]).unwrap();
        Point { x, y }
    }

    fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }

    fn nearest_point(&self, points: &[Point]) -> Option<Point> {
        let mut min_distance = usize::max_value();
        let mut closest: Option<Point> = None;
        for point in points.iter() {
            let distance = self.distance(&point);
            if distance == min_distance {
                closest = None;
            }
            if distance < min_distance {
                closest = Some(*point);
                min_distance = distance;
            }
        }
        closest
    }

    fn total_distance(&self, points: &[Point]) -> usize {
        points.iter().map(|point| self.distance(point)).sum()
    }

    #[allow(dead_code)]
    fn distance(&self, other: &Point) -> usize {
        usize::max(self.x, other.x) - usize::min(self.x, other.x) + usize::max(self.y, other.y)
            - usize::min(self.y, other.y)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_grid() {
        let grid = super::Grid::new(TEST_INPUT);
        assert_eq!(6, grid.specials.len());
        assert_eq!(9, grid.length);
        assert_eq!(10, grid.breadth);
    }

    #[test]
    fn test_largest_internal_area() {
        let grid = super::Grid::new(TEST_INPUT);
        assert_eq!(17, grid.largest_internal_area());

        let grid = super::Grid::new(REAL_INPUT);
        assert_eq!(3223, grid.largest_internal_area());
    }

    #[test]
    fn test_largest_safe_area() {
        let grid = super::Grid::new(TEST_INPUT);
        assert_eq!(16, grid.largest_safe_area(32));

        let grid = super::Grid::new(REAL_INPUT);
        assert_eq!(40495, grid.largest_safe_area(10000));
    }

    const TEST_INPUT: &str = "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";
    const REAL_INPUT: &str = "342, 203
79, 64
268, 323
239, 131
246, 87
161, 93
306, 146
43, 146
57, 112
241, 277
304, 303
143, 235
253, 318
97, 103
200, 250
67, 207
345, 149
133, 222
232, 123
156, 359
80, 224
51, 145
138, 312
339, 294
297, 256
163, 311
241, 321
126, 66
145, 171
359, 184
241, 58
108, 312
117, 118
101, 180
58, 290
324, 42
141, 190
270, 149
209, 294
296, 345
68, 266
233, 281
305, 183
245, 230
161, 295
335, 352
93, 66
227, 59
264, 249
116, 173";
}
