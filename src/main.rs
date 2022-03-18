use libadwaita::{
    gtk::Orientation,
    prelude::{ApplicationExt, ApplicationExtManual, BoxExt, WidgetExt},
    Application, ApplicationWindow, HeaderBar, WindowTitle,
};

fn build_ui(application: &Application) {
    let content = libadwaita::gtk::Box::new(Orientation::Vertical, 0);
    content.append(&HeaderBar::builder()
        .title_widget(&WindowTitle::new("My GTK4 RSS Reader", ""))
        .build());
    
    let window = ApplicationWindow::builder()
        .application(application)
        .title("My GTK4 RSS Reader")
        .default_height(250)
        .default_width(400)
        .content(&content)
        .build();
    window.show();
}

pub fn main() {
    let application = Application::new(Some("com.example.gtk-rss-reader"), Default::default());
    application.connect_activate(build_ui);
    application.run();
}
