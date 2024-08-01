// crate clap docs @ https://docs.rs/clap/latest/clap/
use clap::Parser;

#[derive(Parser)]
// struct becomes command line definition
// allows to use `cargo run -- --help` as a default
// description and default value can be added
struct Options {
    #[clap(default_value = "Meow!")]
    /// What does the cat say?
    message: String,
}

fn print_msg_from_clap() {
    // print message using clap
    // returns a struct populated with parsed argument values
    let args = Options::parse();

    println!("{}", args.message);
}

fn main() {
    
    // print message from std::env::args()
    // _print_msg_from_std_env();

    // print message using clap
    print_msg_from_clap();
}

fn _print_msg_from_std_env() {
    // getting an iterator of command line arguments
    // zeroth name (nth(0)) is name of binary itself
    // so lets read arguments beginning with nth(1)
    let message = std::env::args().nth(1)
        .expect("Missing the message: usage catsay <message>");

    println!("{}", message);
    println!(" \\");
    println!("  \\");
    println!("     /\\_/\\");
    println!("    ( o o )");
    println!("    =( I )=");
}