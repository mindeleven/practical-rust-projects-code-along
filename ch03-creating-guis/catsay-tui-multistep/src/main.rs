/// ncurses (new curses) library overview:
/// https://invisible-island.net/ncurses/announce.html
/// 
/// ncurses 5.101.0 on docs.rs:
/// https://docs.rs/crate/ncurses/5.101.0
/// a very thin wrapper around the ncurses TUI lib
/// NOTE: The ncurses lib is terribly unsafe

use cursive::traits::Nameable;
use cursive::views::{ Checkbox, Dialog, EditView, ListView };
use cursive::Cursive;

// warp all form field values in one struct
// so we can pass them around easily
struct CatsayOptions<'a> {
    message: &'a str,
    dead: bool
}

// setting up form layout and callbacks
fn input_step(siv: &mut Cursive) {
    siv.add_layer(
        Dialog::new()
            .title("Please fill out the form for the cat")
            .content(
                ListView::new()
                .child(
                    "Message:", 
                    EditView::new().with_name("message")
                )
                .child(
                    "Dead?:", 
                    Checkbox::new().with_name("dead")
                )
            )
            .button("OK", |s| {
                let message = s.call_on_name(
                    "message", 
                    |t: &mut EditView| t.get_content()
                ).unwrap();
                let is_dead: bool = s.call_on_name(
                    "dead", 
                    |t: &mut Checkbox| t.is_checked()
                ).unwrap();
                let options = CatsayOptions {
                    message: &message,
                    dead: is_dead,
                };
                result_steps(s, &options)
            })
    );
}

fn result_steps(siv: &mut Cursive, options: &CatsayOptions) {

    let eye = if options.dead { "x" } else { "o" };

    // creating a TextView to hold the ASCII-art
    let cat_text = format!("{msg}
 \\
  \\
     /\\_/\\
    ( {eye} {eye} )
    =( I )=",
        msg = options.message,
        eye = eye
    );  
    
    siv.pop_layer();
    siv.add_layer(
        Dialog::text(cat_text)
            .title("The cat says...")
            .button("OK", |s| s.quit())
    )
}

fn main() {
    // creating a Cursive root object
    // Cursive uses layers to create a stacked view of the components
    let mut siv = cursive::default();
    
    // calling input_step
    input_step(&mut siv); // moving the layout code into input_step

    siv.run(); // starting the event loop

}
