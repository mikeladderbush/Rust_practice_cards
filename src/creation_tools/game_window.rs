use fltk::{app, window::Window, group::Pack, button::Button, prelude::{WidgetExt, GroupExt}};

use crate::creation_tools::window_create::window_creation;

use super::game_creation_tools::game_create::{Player, save_file};

#[derive(Debug, Clone, Copy)]
pub enum Message {

    Stay,
    HitMe,
    Save,

}

pub fn game_window_creation(new_dealer:Player, new_player:Player){

    let app = app::App::default();
    app.quit();
    let mut wind = Window::default().with_size(600, 200).with_label("Card Game");
    let mut pack = Pack::default().with_size(120, 140).center_of(&wind);
    pack.set_spacing(10);
    let mut but_stay = Button::default().with_size(100,40).with_label("I'll stay");
    let mut but_hit_me = Button::default().with_size(100, 40).with_label("Hit me!");
    let mut but_save_profile = Button::default().with_size(100,40).with_label("Save Profile");
    pack.end();
    wind.end();
    wind.show();

    let (s, r) = app::channel::<Message>();

    but_stay.emit(s, Message::Stay);
    but_hit_me.emit(s, Message::HitMe);
    but_save_profile.emit(s,Message::Save);

    let player = new_player;
    let dealer = new_dealer;
    let mut current_players: (Player, Player) = (player, dealer);

    while app.wait() {

        if let Some(msg) = r.recv() {

            match msg {

                Message::Stay => current_players.0.dealer_draw(),
                Message::HitMe => current_players.1.add_to_hand(),
                Message::Save => save_file(&current_players.1),

            }

        }

        if current_players.1.hand_total_value > 21 {
    
            println!("You busted");
            current_players.0.empty_hand();
            current_players.1.empty_hand();
            wind.hide();
            window_creation();
    
        } else if current_players.1.hand_total_value == 21 {

            println!("lucky 21!");
            current_players.0.empty_hand();
            current_players.1.empty_hand();
            current_players.1.add_to_purse();
            wind.hide();

        } else if current_players.0.hand_total_value == 21 {

            println!("21....You lose");
            current_players.0.empty_hand();
            current_players.1.empty_hand();
            wind.hide();

        } else if current_players.0.hand_total_value > 21 {

            println!("Dealer busts. You win!");
            current_players.0.empty_hand();
            current_players.1.empty_hand();
            current_players.1.add_to_purse();
            wind.hide();

        }

    }

}