
use gtk::prelude::*;
use std::env::args;
use glib::clone;

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title(Some("Hello World"));
    window.set_default_size(480, 320);

    add_button(&window);

    window.show();
}
fn add_button(window:&gtk::ApplicationWindow){

    let button = gtk::Button::with_label("Click me!");
    button.set_halign(gtk::Align::Center);
    button.set_valign(gtk::Align::Center);

    button.connect_clicked(clone!(@weak window => move |_| {
        show_message_dialog(&window);
    }));

    window.set_child(Some(&button));
}

fn show_message_dialog(window:&gtk::ApplicationWindow){

    let msg_box2 = gtk::MessageDialog::new(Some(window)
        , gtk::DialogFlags::MODAL|gtk::DialogFlags::DESTROY_WITH_PARENT
        , gtk::MessageType::Info
        , gtk::ButtonsType::Ok
        , "Hello World");

    msg_box2.connect_response(move |d: &gtk::MessageDialog, response: gtk::ResponseType| {
        if response == gtk::ResponseType::Ok {
            d.close();
        }
    });

    msg_box2.show();
}

fn main() {

    let appid="com.github.E834159672.gtk4-rs-example.message-dialog";
    let application =
        gtk::Application::new(Some(appid), Default::default())
            .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}