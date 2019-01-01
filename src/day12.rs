use std::collections::HashMap;

use regex::Regex;

#[derive(Debug)]
struct Pots<'a> {
    pots: Vec<char>,
    offset: i64,
    replacements: HashMap<&'a str, char>,
}

impl<'a> Pots<'a> {
    #[allow(dead_code)]
    fn new(s: &'a str) -> Pots {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"initial state: (?P<initial_state>[#\.]*)").unwrap();
        }
        let caps = RE.captures(s).unwrap();
        let initial_state: Vec<char> = caps["initial_state"].trim().chars().collect();
        let mut pots = vec!['.'; 5];
        pots.extend(initial_state);
        pots.extend(vec!['.'; 5]);

        let offset = -5;

        let mut replacements = HashMap::new();
        for line in s.lines().skip(2) {
            let key = &line[..5];
            let value = line.chars().nth(9).unwrap();
            replacements.insert(key, value);
        }

        Pots {
            pots,
            offset,
            replacements,
        }
    }

    #[allow(dead_code)]
    fn next_gen(&mut self, generations: u64) -> i64 {
        for _ in 0..generations {
            let current_gen: String = self.pots.iter().collect();
            for i in 2..self.pots.len() - 2 {
                let ancestors = &current_gen[i - 2..=i + 2];

                self.pots[i] = match self.replacements.get(ancestors) {
                    Some(child) => *child,
                    None => '.',
                };
            }
            // left-pad
            let left = self.pots[..5].iter().filter(|x| **x == '#').count();
            for _ in 0..left {
                self.pots.insert(0, '.');
            }
            self.offset -= left as i64;

            let right = self.pots[self.pots.len() - 5..]
                .iter()
                .filter(|x| **x == '#')
                .count();
            self.pots.extend(vec!['.'; right]);
        }
        self.pots
            .iter()
            .enumerate()
            .filter(|(_, x)| **x == '#')
            .map(|(i, _)| i as i64 + self.offset)
            .sum()
    }

    #[allow(dead_code)]
    fn next_gen_fast(&mut self, generations: i64) -> i64 {
        let after_100 = self.next_gen(100);
        let after_101 = self.next_gen(1);
        after_100 + (after_101 - after_100) * (generations - 100)
    }
}

#[cfg(test)]
mod tests {
    use super::Pots;

    #[test]
    fn test_parsing() {
        let pots = Pots::new(TEST_INPUT);
        assert_eq!(35, pots.pots.len());
        assert_eq!(14, pots.replacements.len());
        assert_eq!(-5, pots.offset);
    }

    #[test]
    fn test_next_gen() {
        let mut pots = Pots::new(TEST_INPUT);
        assert_eq!(325, pots.next_gen(20));
        let mut pots = Pots::new(REAL_INPUT);
        assert_eq!(3738, pots.next_gen(20));
    }

    #[test]
    fn test_next_gen_fast() {
        let mut pots = Pots::new(REAL_INPUT);
        assert_eq!(3900000002467, pots.next_gen_fast(50000000000));
    }

    const TEST_INPUT: &str = "initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #";

    const REAL_INPUT: &str = "initial state: .##..#.#..##..##..##...#####.#.....#..#..##.###.#.####......#.......#..###.#.#.##.#.#.###...##.###.#

.##.# => #
##.#. => #
##... => #
#.... => .
.#..# => .
#.##. => .
.##.. => .
.#.## => .
###.. => .
..##. => #
##### => #
#...# => #
.#... => #
###.# => #
#.### => #
##..# => .
.###. => #
...## => .
..#.# => .
##.## => #
....# => .
#.#.# => #
#.#.. => .
.#### => .
...#. => #
..### => .
..#.. => #
..... => .
####. => .
#..## => #
.#.#. => .
#..#. => #";
}
