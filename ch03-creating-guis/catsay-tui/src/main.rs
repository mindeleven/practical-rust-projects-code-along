/// ncurses (new curses) library overview:
/// https://invisible-island.net/ncurses/announce.html
/// 
/// ncurses 5.101.0 on docs.rs:
/// https://docs.rs/crate/ncurses/5.101.0
/// a very thin wrapper around the ncurses TUI lib
/// NOTE: The ncurses lib is terribly unsafe

use cursive::views::{ Dialog, TextView };
use cursive::event::Key;

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

    // wrapping the TextView with a Dialog
    siv.add_layer(Dialog::around(
        TextView::new(cat_text)
        ).button("OK", |s| s.quit() // adding button with callback to Dialog
    ));

    // listen to key events -> press Key::Esc and then quit
    // setting up non-blocking global callback
    // arguments: key event and closure
    // closure takes mutable reference to Cursive as argument 
    siv.add_global_callback(Key::Esc, |s| s.quit());

    siv.run(); // starting the event loop

}
