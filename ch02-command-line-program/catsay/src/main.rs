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
    // cat_file is optional and therefore wrapped into an Option
    #[clap(short = 'f', long = "file")]
    /// Load the cat picture from the specified file
    cat_file: Option<std::path::PathBuf>,
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
    // cargo run "woof" 1> assets/stout.txt 2> assets/stderr.txt
    // cat assets/stout.txt // cat assets/stderr.txt

    let eye = if options.dead { "x" } else { "o" };
    
    // colored trait for String
    println!("{}", message.bright_yellow().underline().on_purple());
    println!(" \\");
    println!("  \\");
    println!("     /\\_/\\");
    println!("    ( {} {} )", eye, eye);
    println!("    =( I )=");
}

fn print_cat_from_file() -> Result<(), Box<dyn std::error::Error>> {
    let options = Options::parse();
    let message = options.message;
    let eye = if options.dead { "x" } else { "o" };

    match &options.cat_file {
        Some(path) => {
            // let cat_template = std::fs::read_to_string(path)
            //     .expect(&format!("Could not read file {:?}", path));
            // using the ? operator instead
            let cat_template = std::fs::read_to_string(path)?;
            
            let eye = format!("{}", eye.red().bold());

            let cat_picture = cat_template.replace("{eye}", &eye);

            println!("{}", message.bright_yellow().underline().on_purple());

            println!("{}", &cat_picture);
        },
        None => {
            print_msg_from_clap();
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    // print message from std::env::args()
    // _print_msg_from_std_env();

    // print message using clap
    // print_msg_from_clap();

    // use clap to print message and cat from file
    print_cat_from_file()
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