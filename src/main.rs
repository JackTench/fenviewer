use clap::Parser;

#[derive(Parser)]
struct CliArgs {
    fen_input: String,
}

fn main() {
    
    // Get CLI argument for given FEN.
    let args = CliArgs::parse();
    
    // debug code
    //println!("FEN passed in was {}", args.fen_input);

    // Convert input into board struct.
    let fen_input_split: Vec<&str> = args.fen_input.split(" ").collect();
    let board_in = fen_input_split[0];
    let fen_vec: Vec<&str> = board_in.split("/").collect();
    let fen: [String; 8] = [
        String::from(fen_vec[0]),
        String::from(fen_vec[1]),
        String::from(fen_vec[2]),
        String::from(fen_vec[3]),
        String::from(fen_vec[4]),
        String::from(fen_vec[5]),
        String::from(fen_vec[6]),
        String::from(fen_vec[7])
    ];

    for item in fen {
        println!("{}", item)
    }

}