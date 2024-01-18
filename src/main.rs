use clap::Parser;

#[derive(Parser)]
struct CliArgs {
    fen_input: String,
}

struct Board {
    board_a: String,
    board_b: String,
    board_c: String,
    board_d: String,
    board_e: String,
    board_f: String,
    board_g: String,
    board_h: String,
}

impl Board {
    pub fn as_array(&self) -> [String; 8] {
        [self.board_a, self.board_b, self.board_c, self.board_d, self.board_e, self.board_f, self.board_g, self.board_h]
    }
}

fn main() {
    
    // Get CLI argument for given FEN.
    let args = CliArgs::parse();
    
    // debug code
    //println!("FEN passed in was {}", args.fen_input);

    // Convert input into board struct.
    let fen_input_split: Vec<&str> = args.fen_input.split(" ").collect();
    let board_in = fen_input_split[0];
    let fen: Vec<&str> = board_in.split("/").collect();
    let my_board = Board {
        board_a: String::from(fen[0]),
        board_b: String::from(fen[1]),
        board_c: String::from(fen[2]),
        board_d: String::from(fen[3]),
        board_e: String::from(fen[4]),
        board_f: String::from(fen[5]),
        board_g: String::from(fen[6]),
        board_h: String::from(fen[7]),
    };

    //debug code
    //println!("{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n", my_board.board_a, my_board.board_b, my_board.board_c, my_board.board_d, my_board.board_e, my_board.board_f, my_board.board_g, my_board.board_h);

    for item in my_board.as_array() {
        println!("{}", item)
    }

}