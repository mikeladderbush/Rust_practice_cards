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

#[derive(Debug, Clone, Copy)]
pub enum Profile_Message {
    Enter,
}

//next steps are to replace normal ftlk with egui so we can use string input.
pub fn create_profile() {
    let profile_app = app::App::default();
    let mut profile_window = Window::default().with_size(600, 200).with_label("Profile");
    let mut pack = Pack::default()
        .with_size(120, 140)
        .center_of(&profile_window);
    let mut but_enter = Button::default().with_size(200, 20).with_label("Enter");
    pack.end();
    profile_window.end();
    profile_window.show();
    let (s, r) = app::channel::<Profile_Message>();

    but_enter.emit(s, Profile_Message::Enter);

    while profile_app.wait() {
        if let Some(msg) = r.recv() {
            match msg {
                Profile_Message::Enter => create_game(),
            }
        }
    }
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
