
fn print_msg_from_std_env() {
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

fn main() {
    
    print_msg_from_std_env();

}
