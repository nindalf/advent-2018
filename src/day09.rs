use std::collections::HashMap;

struct Board {
    marbles: Vec<i32>,
    scores: HashMap<i32, i32>,
    num_players: i32,
    last_marble: i32,
}

impl Board {
    #[allow(dead_code)]
    fn new(num_players: i32, last_marble: i32) -> Board {
        Board {
            marbles: Vec::with_capacity(last_marble as usize),
            scores: HashMap::with_capacity(num_players as usize),
            num_players,
            last_marble,
        }
    }

    #[allow(dead_code)]
    fn winning_score(&mut self) -> i32 {
        let mut current_index = 1;
        let mut current_player = 2;
        self.marbles.push(0);
        self.marbles.push(1);
        for marble in 2..=self.last_marble {
            current_player = (current_player + 1) % self.num_players;

            if marble % 23 == 0 {
                current_index = (self.marbles.len() + current_index - 7) % self.marbles.len();
                let score = marble + self.marbles.remove(current_index);
                match self.scores.get(&current_player) {
                    Some(n) => self.scores.insert(current_player, score + n),
                    None => self.scores.insert(current_player, score),
                };
                continue;
            }

            current_index = (current_index + 2) % self.marbles.len();
            self.marbles.insert(current_index, marble);
        }
        *self.scores.values().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Board;
    #[test]
    fn test_winning_score() {
        let mut board = Board::new(9, 23);
        assert_eq!(32, board.winning_score());
        let mut board = Board::new(10, 1618);
        assert_eq!(8317, board.winning_score());
        let mut board = Board::new(13, 7999);
        assert_eq!(146373, board.winning_score());
        let mut board = Board::new(17, 1104);
        assert_eq!(2764, board.winning_score());
        let mut board = Board::new(428, 70825);
        assert_eq!(398502, board.winning_score());

        //        Part 2 requires a LinkedList.
        //        let mut board = Board::new(428, 7082500);
        //        assert_eq!(398502, board.winning_score());
    }
}
