use fltk::{app, window::Window, group::Pack, button::Button, prelude::{WidgetExt, GroupExt}};

use super::game_creation_tools::game_create::{create_game};

#[derive(Debug, Clone, Copy)]
pub enum Message {

    Start,

}

pub fn window_creation(){

    let app = app::App::default();
    let mut wind = Window::default().with_size(600, 200).with_label("Card Game");
    let mut pack = Pack::default().with_size(120, 140).center_of(&wind);
    pack.set_spacing(10);
    let mut but_start = Button::default().with_size(100, 40).with_label("Start Game");
    pack.end();
    wind.end();
    wind.show();

    let (s, r) = app::channel::<Message>();

    but_start.emit(s, Message::Start);

    while app.wait() {

        if let Some(msg) = r.recv() {

            match msg {

                Message::Start => create_game(),

            }

        }

    }

}