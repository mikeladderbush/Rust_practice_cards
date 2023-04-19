use fltk::{
    app,
    button::Button,
    group::Pack,
    prelude::{GroupExt, WidgetExt},
    window::Window,
};

use super::game_creation_tools::{file_manager::use_file, game_create::create_game};

#[derive(Debug, Clone, Copy)]
pub enum Start_Message {
    Start,
    Load,
}

pub fn window_creation() {
    let app = app::App::default();
    let mut wind = Window::default()
        .with_size(600, 200)
        .with_label("Card Game");
    let mut pack = Pack::default().with_size(120, 140).center_of(&wind);
    pack.set_spacing(10);
    let mut but_start = Button::default()
        .with_size(100, 40)
        .with_label("Start Game");
    let mut but_load_profile = Button::default()
        .with_size(100, 40)
        .with_label("Load Profile");
    pack.end();
    wind.end();
    wind.show();

    let (s, r) = app::channel::<Start_Message>();

    but_start.emit(s, Start_Message::Start);
    but_load_profile.emit(s, Start_Message::Load);

    while app.wait() {
        if let Some(msg) = r.recv() {
            match msg {
                Start_Message::Start => create_game(),
                Start_Message::Load => use_file(),
            }
        }
    }
}
