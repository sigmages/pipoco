
use crate::algorithm::enums::PlayerShape;

pub struct TTTBoard {
    pub matrix: [[i8; 3]; 3],
    pub players: u8,
}

fn match_winner(match_sum: i8) -> Option<PlayerShape> {
    match match_sum {
        -3 => Some(PlayerShape::Cross),
        3 => Some(PlayerShape::Circle),
        _ => None,
    }
}

fn format_line(items: &Vec<String>, spacing: &String) -> String {
    // applying the desired spacing between elements
    let linefmt: String = items.iter().map(|x| {
        format!("|{}{}{}", spacing, x, spacing)
    }).collect();
    // this is necessary to just append the "|" to the last element
    format!("{}|", &linefmt)
}

impl TTTBoard {
    pub fn new() -> Self {
        Self { matrix:  [[0; 3]; 3], players: 0 }
    }

    pub fn insert(&mut self, x: usize, y: usize, player: &PlayerShape) {
        self.matrix[x][y] = player.to_integer();
    }

    pub fn find_winner(&self) -> Option<PlayerShape> {
        let mut winner: Option<PlayerShape>;

        // row to row search
        for (_, row) in self.matrix.iter().enumerate() {
            let match_sum: i8 = row.iter().sum();
            winner = match_winner(match_sum);
            if winner.is_some() {
                return winner;
            }
        };

        // col to col search
        for i in 0..3 {
            let match_sum: i8 = self.matrix[0][i] + self.matrix[1][i] + self.matrix[2][i];
            winner = match_winner(match_sum);
            if winner.is_some() {
                return winner;
            }
        }

        // simple transversal search
        let left_right_sum = self.matrix[0][0] + self.matrix[1][1] + self.matrix[2][2];
        let right_left_sum = self.matrix[0][2] + self.matrix[1][1] + self.matrix[2][0];
        winner = match_winner(left_right_sum);
        if winner.is_some() {
            return winner;
        };
        winner = match_winner(right_left_sum);
        if winner.is_some() {
            return winner;
        };
        winner
    }

    pub fn render_board(&self, spacing: u8) -> String {
        let mut board: Vec<String> = vec![];
        let hspacing = " ".repeat(spacing as usize);
        let mut bar = String::from("");
        for (_, row) in self.matrix.iter().enumerate() {
            // Substituing the matrix numbers for their visual shapes
            let line: Vec<String> = row.iter().map(|x| {
                PlayerShape::from_integer(*x).unwrap().to_string()
            }).collect();
            let linefmt = format_line(&line, &hspacing);
            let vspacing = format_line(&vec![" ".to_string(); 3], &hspacing);

            bar = format!("{}", "-".repeat(linefmt.len()));
            board.push(vspacing.clone());
            board.push(linefmt);
            board.push(vspacing.clone());
            board.push(bar.clone());
        }
        // filling the top of the box with the bar (because now we know the lines sizes)
        board.insert(0, bar.clone());
        board.join("\n")
    }
}

#[cfg(test)]
mod tests {
        use crate::algorithm::{enums::PlayerShape};
        use super::TTTBoard;

    #[test]
    fn assert_render_board() {
        let board =  TTTBoard::new();
        let received = board.render_board(2);
        let expected = String::from(
"-------------------
|     |     |     |
|  -  |  -  |  -  |
|     |     |     |
-------------------
|     |     |     |
|  -  |  -  |  -  |
|     |     |     |
-------------------
|     |     |     |
|  -  |  -  |  -  |
|     |     |     |
-------------------"
        );
        assert_eq!(received, expected);
    }

    #[test]
    fn assert_find_winner_success() {
        let mut board = TTTBoard::new();
        let player_circle = PlayerShape::Circle;
        let player_cross = PlayerShape::Cross;
        board.insert(0, 0, &player_circle);
        board.insert(0, 1, &player_cross);
        board.insert(1, 1, &player_circle);
        board.insert(1, 0, &player_cross);
        board.insert(2, 2, &player_circle);

        let winner = board.find_winner();
        assert!(winner.is_some());
        assert_eq!(winner.unwrap(), PlayerShape::Circle);

    }
    #[test]
    fn assert_find_winner_has_no_winner() {
        let mut board = TTTBoard::new();
        let player_circle = PlayerShape::Circle;
        let player_cross = PlayerShape::Cross;
        board.insert(0, 0, &player_circle);
        board.insert(0, 1, &player_cross);
        board.insert(1, 1, &player_circle);
        board.insert(1, 0, &player_cross);

        let winner = board.find_winner();
        assert!(winner.is_none());

    }
    #[test]
    fn assert_find_winner_success_row() {
        let mut board = TTTBoard::new();
        let player_circle = PlayerShape::Circle;
        let player_cross = PlayerShape::Cross;
        board.insert(0, 0, &player_circle);
        board.insert(1, 1, &player_cross);
        board.insert(0, 1, &player_circle);
        board.insert(1, 0, &player_cross);
        board.insert(0, 2, &player_circle);

        let winner = board.find_winner();
        assert!(winner.is_some());
        assert_eq!(winner.unwrap(), PlayerShape::Circle);

    }
    #[test]
    fn assert_find_winner_success_col() {
        let mut board = TTTBoard::new();
        let player_circle = PlayerShape::Circle;
        let player_cross = PlayerShape::Cross;
        board.insert(0, 0, &player_circle);
        board.insert(1, 1, &player_cross);
        board.insert(1, 0, &player_circle);
        board.insert(1, 2, &player_cross);
        board.insert(2, 0, &player_circle);
        let winner = board.find_winner();
        assert!(winner.is_some());
        assert_eq!(winner.unwrap(), PlayerShape::Circle);

    }
    #[test]
    fn assert_find_winner_success_left_right() {
        let mut board = TTTBoard::new();
        let player_circle = PlayerShape::Circle;
        board.insert(0, 0, &player_circle);
        board.insert(1, 1, &player_circle);
        board.insert(2, 2, &player_circle);
        let winner = board.find_winner();
        assert!(winner.is_some());
        assert_eq!(winner.unwrap(), PlayerShape::Circle);

    }
    #[test]
    fn assert_find_winner_success_right_left() {
        let mut board = TTTBoard::new();
        let player_circle = PlayerShape::Circle;
        board.insert(0, 2, &player_circle);
        board.insert(1, 1, &player_circle);
        board.insert(2, 0, &player_circle);
        let winner = board.find_winner();
        assert!(winner.is_some());
        assert_eq!(winner.unwrap(), PlayerShape::Circle);

    }
}
