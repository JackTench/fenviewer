use clap::Parser;

#[derive(Parser)]
#[clap(
    name = "fenparser",
    version = "1.0",
    author = "Jack Tench",
    about = "A simple utility for visualising chess positons."
)]
struct CliArgs {
    fen_input: String,
}

fn main() {
    
    // Get CLI argument for given FEN.
    let args = CliArgs::parse();

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

    // Print formatted board.
    let mut n = 8;
    for item in fen {
        print!("{}", n);
        n = n - 1;
        print!("|");
        for c in item.chars() {
            if c.is_numeric() {
                let spaces = (c.to_string()).parse::<i32>().unwrap();
                for _i in 0..spaces {
                    print!(" |");
                }
            } else {
                print!("{}|", c);
            }
        }
        println!("");
    }
    println!("  a b c d e f g h ");

}
