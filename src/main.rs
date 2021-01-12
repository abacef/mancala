use std::fmt::{Display, Formatter, Result};
use std::fs;
use std::iter::{Peekable, Skip};
use std::str::Lines;

const HOW_MANY_MANCALAS: usize = 6;
const STARTING_BOARD: [u8; 14] = [4,4,4,4,4,4,0,4,4,4,4,4,4,0];
const BOARD_SPOTS: usize = 14;
const OUR_GOAL: u8 = 6;
const THEIR_GOAL: u8 = 13;
const MARBLES: u8 = 48;

struct Board {
    board: [u8; 14],
    our_turn: bool
}
impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for i in 0 .. 7 {
            write!(f, "{number:>width$} ", number=self.board[self.board.len() - i - 1], width=3)?;
        }
        writeln!(f)?;
        write!(f, "    ")?;
        for i in 0 .. 7 {
            write!(f, "{number:>width$} ", number=self.board[i], width=3)?;
        }
        writeln!(f)?;
        let turn_message = match self.our_turn {
            true => "your",
            false => "your opponent's"
        };
        write!(f, "Its {} turn.", turn_message)?;
        Ok(())
    }
}
impl Board {
    fn clear_board_to_sides(&mut self) {
        let our_sum: u8 = self.board.iter().take(6).sum();
        let opponent_sum: u8 = self.board.iter().skip(7).take(6).sum();

        for i in 0 .. BOARD_SPOTS {
            if i != OUR_GOAL as usize && i != THEIR_GOAL as usize {
                self.board[i] = 0;
            }
        }

        self.board[OUR_GOAL as usize] += our_sum;
        self.board[THEIR_GOAL as usize] += opponent_sum;
    }

    fn game_is_done(&self) -> bool {
        let our_sum: u8 = self.board.iter().take(6).sum();
        let opponent_sum: u8 = self.board.iter().skip(7).take(6).sum();
        our_sum == 0 || opponent_sum == 0
    }

    // None means the game is done, true means the player got an extra turn
    fn make_move(&mut self, mut bucket: u8) -> Option<bool> {
        let in_hand = self.board[bucket as usize];
        self.board[bucket as usize] = 0;

        let mut free_turn = false;
        for i in 0 .. in_hand {
            loop {
                bucket += 1;
                bucket %= BOARD_SPOTS as u8;
                if bucket == OUR_GOAL {
                    match self.our_turn {
                        true => {
                            self.board[bucket as usize] += 1;
                            if i == in_hand - 1 {
                                free_turn = true;
                            }
                        },
                        false => continue
                    }
                }
                else if bucket == THEIR_GOAL {
                    match self.our_turn {
                        true => continue,
                        false => {
                            self.board[bucket as usize] += 1;
                            if i == in_hand - 1 {
                                free_turn = true;
                            }
                        }
                    }
                }
                else {
                    self.board[bucket as usize] += 1;
                }
                break;
            }
        }

        if !free_turn {
            // check for capture
            if self.board[bucket as usize] == 1 {
                let op = BOARD_SPOTS - bucket as usize - 2;
                let whose_goal_to_insert = match self.our_turn {
                    true => OUR_GOAL,
                    false => THEIR_GOAL
                } as usize;

                if self.board[op] != 0 &&
                    ((self.our_turn && bucket < 6) ||
                        (!self.our_turn && bucket >= 7 && bucket < 13)) {
                    let in_hand_capture = self.board[op] + self.board[bucket as usize];
                    self.board[op] = 0;
                    self.board[bucket as usize] = 0;
                    self.board[whose_goal_to_insert] += in_hand_capture;
                }
            }

            if self.game_is_done() {
                self.clear_board_to_sides();
                return None
            }

            self.our_turn = !self.our_turn
        }
        Some(free_turn)
    }

    fn move_is_valid(&self, bucket: u8) -> bool {
        if self.board[bucket as usize] == 0 {
            return false
        }

        match self.our_turn {
            true => {
                match bucket {
                    0 ..= 5 => true,
                    _ => false
                }
            },
            false => {
                match bucket {
                    7 ..= 12 => true,
                    _ => false
                }
            }
        }
    }

    fn end_game_message(&self) {
        print!("Game over. ");
        let our_score = self.board[OUR_GOAL as usize];
        let next_communication = match our_score {
            0 ..= 23 => format!("You lost, {} to {}", our_score, MARBLES - our_score),
            24 => format!("You guys tied! {} to {}", 24, 24),
            _ => format!("You won! {} to {}", our_score, MARBLES - our_score)
        };
        println!("{}", next_communication);
    }

    pub fn play_game(&mut self) {
        loop {
            let mut line = String::new();
            let b1 = std::io::stdin().read_line(&mut line).unwrap();
            if b1 < 2 || b1 > 3 {
                println!("Enter only one number, 0 through 5 when your turn, or 7 through 12 for opponent move");
            } else {
                let num = line.chars().take(b1 - 1).collect::<String>().parse::<u8>();
                match num {
                    Ok(num) => {
                        if self.move_is_valid(num) {
                            println!("Playing bucket: {}", num);
                            let free_turn = self.make_move(num);
                            match free_turn {
                                Some(free_turn) => {
                                    println!("{}", self);
                                    if free_turn {
                                        println!("You get a free turn. What will you play?");
                                    }
                                },
                                None => {
                                    self.end_game_message();
                                    return
                                }
                            }
                        } else {
                            println!("Invalid move! Tratate otra vez");
                        }
                    },
                    Err(_) => {
                        println!("Invalid move! Tratate otra vez");
                    }
                }
            }
        }
    }

    // starts from your first mancala on the left and circles around
    pub fn new_intermediate(file: Peekable<Lines>, our_turn: bool) -> Board {
        let items = file
            .map(|x| x.parse::<u8>().unwrap())
            .collect::<Vec<u8>>();

        if items.len() != BOARD_SPOTS {
            panic!("Board file must have {} lines. First the opponent's score, the quantities of your {} mancalas, the quantities of your opponent's {} mancalas and then your score", BOARD_SPOTS, HOW_MANY_MANCALAS, HOW_MANY_MANCALAS);
        }

        let mut board = [0u8; 14];
        items.iter()
            .enumerate()
            .for_each(|(i, x)| board[i] = *x);

        Board {
            board,
            our_turn
        }
    }

    pub fn new_with_turns(turns: Skip<Peekable<Lines>>, i_start: bool) -> Board {
        let mut board = Board::new(i_start);
        turns.for_each(|x| {
            let _ = board.make_move(x.parse().unwrap());
            ()
        });
        board
    }

    pub fn new(our_turn: bool) -> Board {
        Board {
            board: STARTING_BOARD,
            our_turn
        }
    }
}

fn main() {
    let mut board = match std::env::args().len() {
        2 => {
            let filename = std::env::args().skip(1).next().unwrap();
            println!("Loading file: {} as the board", filename);

            let file = fs::read_to_string(filename).unwrap();
            let mut file_iter = file.lines().peekable();
            match *file_iter.peek().unwrap() {
                "m" => Board::new_with_turns(file_iter.skip(1), true),
                " " => Board::new_with_turns(file_iter.skip(1), false),
                _ => Board::new_intermediate(file_iter, true)
            }
        },
        1 => {
            println!("Starting from a new game board");
            Board::new(false)
        },
        _ => panic!("The only argument should be a the board file if loading from a file. You entered more than one argument")
    };

    println!("{}", board);
    board.play_game();
}
