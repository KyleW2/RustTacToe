use std::io::{stdin};

struct Board {
    board: [[i8; 3]; 3],
}

impl Board {
    fn new() -> Self {
        return Self {
            board: [[-1, -1, -1], [-1, -1, -1], [-1, -1, -1]],
        }
    }

    fn make_move(&mut self, turn_number: i8, letter: &String) {
        let (r, c) = Self::get_index_from_letter(&self, &letter);

        if r > self.board.len() {
            println!("Invalid move... dumbass");
            return
        }

        if self.board[r][c] != -1i8 {
            println!("Nice try, that spots taken!");
            return
        }

        if turn_number % 2 == 0 {
            self.board[r][c] = 0;
        } else {
            self.board[r][c] = 1;
        }
    }

    fn get_index_from_letter(&self, letter: &String) -> (usize, usize) {
        match letter.trim() {
            "a" => return (0, 0),
            "b" => return (0, 1),
            "c" => return (0, 2),
            "d" => return (1, 0),
            "e" => return (1, 1),
            "f" => return (1, 2),
            "g" => return (2, 0),
            "h" => return (2, 1),
            "i" => return (2, 2),
            _ => return (self.board.len() + 1, 0)
        }
    }

    fn check_for_wins(&self) -> bool {
        // Check for row wins
        for i in 0..self.board.len() {
            match self.board[i] {
                [0, 0, 0] => return true,
                [1, 1, 1] => return true,
                _ => (),
            }
        }        
        // Check for column wins
        for i in 0..self.board[0].len() {
            let column = [self.board[0][i], self.board[1][i], self.board[2][i]];

            match column {
                [0, 0, 0] => return true,
                [1, 1, 1] => return true,
                _ => (),
            }
        }
        // Check for diagonals
        if self.board[1][1] != -1i8 {
            if self.board[0][0] == self.board[1][1] && self.board[1][1] == self.board[2][2] {
                return true
            }
            if self.board[0][2] == self.board[1][1] && self.board[1][1] == self.board[2][0] {
                return true
            }
        }

        return false
    }

    fn print(&self) {
        for i in 0..self.board.len() {
            for j in 0..self.board[i].len() {
                match self.board[i][j] {
                    -1 => match j {
                        2 => println!("  "),
                        _ => print!("  | "),
                    }
                    _ => match j {
                        2 => println!("{}", self.board[i][j]),
                        _ => print!("{} | ", self.board[i][j]),
                    }
                }
                if j == 2 && i != 2{
                    println!("---------")
                }
            }
        }
    }

    fn print_moves(&self) {
        let move_letters = [["a", "b", "c"], ["d", "e", "f"], ["g", "h", "i"]];

        for i in 0..self.board.len() {
            for j in 0..self.board[i].len() {
                match self.board[i][j] {
                    -1 => match j {
                        2 => println!("{} ", move_letters[i][j]),
                        _ => print!("{} | ", move_letters[i][j]),
                    }
                    _ => match j {
                        2 => println!("{}", " "),
                        _ => print!("{} | ", " "),
                    }
                }
                if j == 2 && i != 2 {
                    println!("---------")
                }
            }
        }
    }
}

fn get_user_input() -> String {
    let mut input = String::new();

    stdin().read_line(&mut input)
    	.ok()
        .expect("Failed to read line");

    return input
}

fn main() {
    let mut game_board = Board::new();
    let mut has_won = false;
    let mut turn = 0;
    let mut player_move;

    while !has_won {
        // Check board
        has_won = game_board.check_for_wins();
        if has_won == true {
            game_board.print();
            if turn % 2 == 0 {
                println!("Player 0 wins!")
            } else {
                println!("Player 1 wins!")
            }
            continue
        }
        if turn == 9 {
            println!("Losers");
            has_won = true;
            continue
        }

        // Clear terminal
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        // Print board
        println!("Current board:");
        game_board.print();
        println!();
        println!("Possible moves:");
        game_board.print_moves();
        println!();

        // State which user is making the move
        if turn % 2 == 0 {
            println!("Player 0's turn")
        } else {
            println!("Player 1's turn")
        }

        // Get user's move
        println!();
        player_move = get_user_input();

        // Make move
        game_board.make_move(turn, &player_move);
        println!();

        // Inc turn
        turn += 1;
    }
}