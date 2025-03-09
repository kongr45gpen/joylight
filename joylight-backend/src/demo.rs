mod colors;
mod fixture;
mod parameter;

use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button, ColorButton, Label, TextView};
use gtk4::gdk::RGBA;
use serde_json;

fn main() -> glib::ExitCode {
    let application = Application::builder()
        .application_id("com.example.FirstGtkApp")
        .build();

    application.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("First GTK Program")
            .default_width(500)
            .default_height(900)
            .build();

        let layout = gtk::Box::new(gtk::Orientation::Vertical, 2);

        let label = gtk::Label::new(Some("Joylight Backend Demo\nSelect a colour"));
        layout.append(&label);

        let chooser = gtk::ColorChooserWidget::builder().use_alpha(false).show_editor(true).build();
        layout.append(&chooser);

        layout.append(&Label::new(Some("\nSelect a result base (RGB):")));

        let text1 = TextView::new();
        let buffer = text1.buffer();
        buffer.set_text("[ [1, 0, 0],\n [0, 1, 0],\n [0, 0, 1] ]");
        layout.append(&text1);

        let button1 = ColorButton::new();
        layout.append(&button1);
        let button1label = Label::new(Some("Result 1"));
        layout.append(&button1label);
        let button2 = ColorButton::new();
        layout.append(&button2);
        let button2label = Label::new(Some("Result 2"));
        layout.append(&button2label);
        let button3 = ColorButton::new();
        layout.append(&button3);
        let button3label = Label::new(Some("Result 3"));
        layout.append(&button3label);

        chooser.connect_rgba_notify(move |chooser| {
            let rgba = chooser.rgba();

            // let text = format!("[ {}, {}, {} ]", rgba.red(), rgba.green(), rgba.blue());
            let text = buffer.text(&buffer.start_iter(), &buffer.end_iter(), false).clone();

            let json: serde_json::Value = serde_json::from_str(&text).unwrap_or(serde_json::Value::Null);

            let _ = std::panic::catch_unwind(|| {
                let base: Vec<colors::RGBTuple> = json.as_array().unwrap().iter().map(|item| {
                    let array = item.as_array().unwrap();
                    colors::RGBTuple([array[0].as_f64().unwrap(), array[1].as_f64().unwrap(), array[2].as_f64().unwrap()])
                }).collect();

                button1.set_rgba(&RGBA::new(
                    base[0].0[0] as f32,
                    base[0].0[1] as f32,
                    base[0].0[2] as f32,
                    1.0
                ));
                button2.set_rgba(&RGBA::new(
                    base[1].0[0] as f32,
                    base[1].0[1] as f32,
                    base[1].0[2] as f32,
                    1.0
                ));
                button3.set_rgba(&RGBA::new(
                    base[2].0[0] as f32,
                    base[2].0[1] as f32,
                    base[2].0[2] as f32,
                    1.0
                ));

                let color = colors::change_base(
                    colors::rgb_base(), 
                    base, 
                    [rgba.red() as f64, rgba.green() as f64, rgba.blue() as f64].to_vec()
                ).inspect_err(|e| eprintln!("failed to change base:")).unwrap();

                button1label.set_text(&format!("Result 1: [{:.03}]", color[0]));
                button2label.set_text(&format!("Result 2: [{:.03}]", color[1]));
                button3label.set_text(&format!("Result 3: [{:.03}]", color[2]));
            });
        });

        window.set_child(Some(&layout));
        window.present();
    });

    application.run()
}