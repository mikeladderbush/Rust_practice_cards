use fltk::{app, window::Window, group::Pack, button::Button, prelude::{WidgetExt, GroupExt}};
use game_create::{create_game, Player};

mod game_create;

#[derive(Debug, Clone, Copy)]
pub enum Message {

    Start,
    Stay,
    HitMe,

}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let app = app::App::default();
    let mut wind = Window::default().with_size(600, 200).with_label("Card Game");
    let mut pack = Pack::default().with_size(120, 140).center_of(&wind);
    pack.set_spacing(10);
    let mut but_start = Button::default().with_size(100, 40).with_label("Start Game");
    let mut but_stay = Button::default().with_size(100,40).with_label("I'll stay");
    let mut but_hit_me = Button::default().with_size(100, 40).with_label("Hit me!");
    pack.end();
    wind.end();
    wind.show();

    let (s, r) = app::channel::<Message>();

    but_start.emit(s, Message::Start);
    but_stay.emit(s, Message::Stay);
    but_hit_me.emit(s, Message::HitMe);

    but_stay.hide();
    but_hit_me.hide();

    let player = Player::default();
    let dealer = Player::default();
    let mut starting_players: (Player, Player) = (player, dealer);

    while app.wait() {

        if let Some(msg) = r.recv() {

            match msg {

                Message::Start => starting_players = create_game(),
                Message::Stay => starting_players.0.add_to_hand(),
                Message::HitMe => starting_players.1.add_to_hand(),

            }

        }

        but_hit_me.show();
        but_stay.show();

        if starting_players.1.hand_total_value > 21 {
    
            println!("You busted");
            starting_players.0.empty_hand();
            starting_players.1.empty_hand();
    
        }
    }
    Ok(())
}