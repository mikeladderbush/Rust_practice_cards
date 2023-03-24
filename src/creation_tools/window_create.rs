use fltk::{
    app,
    button::Button,
    group::Pack,
    prelude::{GroupExt, WidgetExt},
    window::Window,
    input::Input,
};

use super::game_creation_tools::game_create::{create_game, use_file};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Start,
    Load,
}
/*pub struct TextInput {
    inner: *mut Fl_Input,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
    is_derived: bool,
}*/

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

    let (s, r) = app::channel::<Message>();

    but_start.emit(s, Message::Start);
    but_load_profile.emit(s, Message::Load);

    while app.wait() {
        if let Some(msg) = r.recv() {
            match msg {
                Message::Start => create_game(),
                Message::Load => use_file(),
            }
        }
    }
}
