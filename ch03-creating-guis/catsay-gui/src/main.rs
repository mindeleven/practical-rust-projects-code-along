use gtk::prelude::*;
use gtk::{ 
    Application,
    ApplicationWindow,
    Box as GtkBox,
    Image,
    Label,
    Orientation
};

fn main() {
    
    let app = Application::new(
        Some("com.shinglyu.catsay-gui"), 
        Default::default()
    );
    
    app.connect_activate(|app| {
        let window = ApplicationWindow::new(app);

        // wrapper box to properly control the layout
        // gtk::Box is a container to display elements in row or column
        // here vertical orientation for row
        let layout_box = GtkBox::new(Orientation::Vertical, 0);

        // creating text in a label
        let label = Label::new(
            Some("Meow!\n     \\\n      \\")
        );
        layout_box.add(&label); // adding element to layout box

        // creating a gtk image widget
        let cat_image = Image::from_file(
            "./assets/cat-2669554_640.png"
        );
        layout_box.add(&cat_image); // adding element to layout box
        
        // adding layout box to window
        window.add(&layout_box);
        
        window.set_title("Catsay");
        window.set_default_size(350, 70);

        window.show_all();
    });
    
    // Image by Dorothe from Pixabay
    // https://pixabay.com/photos/cat-kitten-to-sit-isolated-red-2669554/
    // Free for use under the Pixabay Content License

    app.run();
}
