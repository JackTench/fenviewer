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

}