use gtk::prelude::*;
use gtk::{ Application, ApplicationWindow };

fn main() {
    
    let app = Application::new(
        Some("com.shinglyu.catsay-gui"), 
        Default::default()
    );
    
    app.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Catsay");
        window.set_default_size(350, 70);

        window.show_all();
    });
    
    // Image by Dorothe from Pixabay
    // https://pixabay.com/photos/cat-kitten-to-sit-isolated-red-2669554/
    // Free for use under the Pixabay Content License

    app.run();
}
