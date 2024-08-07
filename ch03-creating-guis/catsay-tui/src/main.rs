/// ncurses (new curses) library overview:
/// https://invisible-island.net/ncurses/announce.html
/// 
/// ncurses 5.101.0 on docs.rs:
/// https://docs.rs/crate/ncurses/5.101.0
/// a very thin wrapper around the ncurses TUI lib
/// NOTE: The ncurses lib is terribly unsafe

use cursive::views::TextView;

fn main() {
    // creating a Cursive root object
    // Cursive uses layers to create a stacked view of the components
    let mut siv = cursive::default();

    // now back to our meowing cat
    // creating a TextView to hold the ASCII-art
    let cat_text = "Meow!
 \\
  \\
     /\\_/\\
    ( o o )
    =( I )=
    ";  
    
    // declaring the app layout
    // adding the TextView as a layer to the main siv program
    siv.add_layer(TextView::new(cat_text));

    siv.run(); // starting the event loop

}
