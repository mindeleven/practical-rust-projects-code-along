/// ncurses (new curses) library overview:
/// https://invisible-island.net/ncurses/announce.html
/// 
/// ncurses 5.101.0 on docs.rs:
/// https://docs.rs/crate/ncurses/5.101.0
/// a very thin wrapper around the ncurses TUI lib
/// NOTE: The ncurses lib is terribly unsafe

fn main() {
    // creating a Cursive root object
    let mut siv = cursive::default();

    siv.run(); // starting the event loop

}
