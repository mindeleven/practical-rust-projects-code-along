// crate clap docs @ https://docs.rs/clap/latest/clap/
use clap::Parser;
// crate colored @ https://docs.rs/colored/latest/colored/
// colorized trait is implemented on &str and Str
use colored::Colorize;

#[derive(Parser)]
// struct becomes command line definition
// allows to use `cargo run -- --help` as a default
// description and default value can be added
struct Options {
    #[clap(default_value = "Meow!")]
    /// What does the cat say?
    message: String,
    #[clap(short = 'd', long = "dead")]
    /// Make the cat appear dead
    dead: bool,
}

fn print_msg_from_clap() {
    // print message using clap
    // returns a struct populated with parsed argument values
    let options = Options::parse();

    let message = options.message;

    if message.to_lowercase() == "woof" {
        // Rust's STDERR equivalent of println!
        eprintln!("A cat shouldn't bark like a dog");
    }

    // saving STDOUT and STDERR to textfiles with
    // cargo run "woof" 1> stout.txt 2> stderr.txt
    // cat stout.txt // cat stderr.txt

    let eye = if options.dead { "x" } else { "o" };
    
    // colored trait for String
    println!("{}", message.bright_yellow().underline().on_purple());
    println!(" \\");
    println!("  \\");
    println!("     /\\_/\\");
    println!("    ( {} {} )", eye, eye);
    println!("    =( I )=");
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