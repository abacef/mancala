use std::slice::Iter;

const HOW_MANY_MANCALAS: usize = 6;
const STARTING_SPACES: [u8; 6] = [4; 6];
const INPUT_FILE_LINES: u8 = 14;

struct Board {
    opponent_points: u8,
    opponent_spaces: [u8; 6],
    our_points: u8,
    our_spaces: [u8; 6],
    our_turn: bool
}
impl Board {

    fn distribute(&mut self, space: u8) {
        match self.our_turn {
            true => {
                let in_hand = self.our_spaces[space];
                self.our_spaces[space] = 0;
                let acc = space;
                for i in 1 ..= in_hand {
                    let curr = acc + i;
                    let x = curr / 6;
                    let y = curr % 6;
                    if x != 0 {
                        if y == 0
                    }
                }
            }
        }
    }

    pub fn win_game(&mut self) {
        match our_turn {
            true => {
                for i in 0 .. HOW_MANY_MANCALAS {
                    if self.our_spaces[i] != 0 {
                        distribute
                    }
                }
            }
        }
    }

    fn assemble_owned_array(stream: &mut Iter<u8>) -> [u8; 6] {
        let mut x = [0 as u8; 6];
        for i in 0 .. 6 {
            x[i] = stream.next().unwrap().clone()
        }
        x
    }

    // starts from your first mancala on the left and circles around
    pub fn new_intermediate(filename: String, our_turn: bool) -> Board {
        let items = filename.lines().map(|x| x.parse::<u8>().unwrap()).collect::<Vec<u8>>();
        if items.len() != 14 {
            panic!("Board file must have {} lines. First the opponent's score, the quantities of your {} mancalas, the quantities of your opponent's {} mancalas and then your score", INPUT_FILE_LINES, HOW_MANY_MANCALAS, HOW_MANY_MANCALAS);
        }
        let mut shared_stream = items.iter();

        let opponent_points = &shared_stream.next().unwrap().clone();
        let our_spaces = Board::assemble_owned_array(&mut shared_stream);
        let opponent_spaces = Board::assemble_owned_array(&mut shared_stream);
        let our_points = &shared_stream.next().unwrap().clone();

        Board {
            opponent_points: *opponent_points,
            opponent_spaces,
            our_points: *our_points,
            our_spaces,
            our_turn
        }
    }

    pub fn new(our_turn: bool) -> Board {
        Board {
            opponent_points: 0,
            opponent_spaces: STARTING_SPACES,
            our_points: 0,
            our_spaces: STARTING_SPACES,
            our_turn
        }
    }
}

fn main() {
    let mut board = match std::env::args().len() {
        2 => {
            let board_filename = std::env::args().skip(1).take(1).collect();
            Board::new_intermediate(board_filename, true)
        },
        1 => {
            println!("Starting from a new game board");
            Board::new(true)
        },
        _ => panic!("The only argument should be a the board file if loading from a file. You entered more than one argument")
    };

    board.win_game();
}
