use fltk::{app, window::Window, group::Pack, button::Button, prelude::{WidgetExt, GroupExt}, frame::Frame};
use rand::Rng; // 0.8.5

#[derive(Debug, Clone)]
struct Player {

    hand: Vec<(String, String, u32)>,
    hand_total_value: u32,
    wager: Option<u32>,
    hands_played: Option<u32>

}

#[derive(Debug, Clone, Copy)]
pub enum Message {

    Start,
    Restart,

}

impl Player{

    fn initialize_hand(&mut self){
       
        let card_1 = create_card();
        let card_2 = create_card();
        self.hand_total_value = self.hand_total_value + card_1.2;
        self.hand_total_value = self.hand_total_value + card_2.2;
        self.hand.push(card_1);
        self.hand.push(card_2);

        let mut frame = Frame::default().with_size(0, 40).with_label("0");
        let mut label: i32 = frame.label().parse().unwrap();
        frame.set_label(&(label + 1).to_string());

        println!("{:?}", self);

    }

    fn add_to_hand(&mut self){

        let added_card = create_card();
        self.hand_total_value = self.hand_total_value + &added_card.2;
        self.hand.push(added_card);
       
        if self.hand_total_value > 21 {

            println!("You busted");
            println!("{:?}", self);

        }else {

            println!("hit or stay?");
            println!("{:?}", self);

        }

    }

    fn reset_hand(&mut self){

        self.hand = vec![];
        self.hand_total_value = 0;
        self.wager = Some(0);
        self.hands_played = Some(0 + 1);

    }
}

fn create_card() -> (String, String, u32) {

    let mut single_deck: Vec<(u32, &str)> =
    vec![(1,"Hearts"),(1,"Spades"),(1,"Clubs"),(1,"Diamonds"),(2,"Hearts"),(2,"Spades")
    ,(2,"Clubs"),(2,"Diamonds"),(3,"Hearts"),(3,"Spades"),(3,"Clubs"),(3,"Diamonds")
    ,(4,"Hearts"),(4,"Spades"),(4,"Clubs"),(4,"Diamonds"),(5,"Hearts"),(5,"Spades")
    ,(5,"Clubs"),(5,"Diamonds"),(6,"Hearts"),(6,"Spades"),(6,"Clubs"),(6,"Diamonds")
    ,(7,"Hearts"),(7,"Spades"),(7,"Clubs"),(7,"Diamonds"),(8,"Hearts"),(8,"Spades"),(8,"Clubs")
    ,(8,"Diamonds"),(9,"Hearts"),(9,"Spades"),(9,"Clubs"),(9,"Diamonds"),(10,"Hearts")
    ,(10,"Spades"),(10,"Clubs"),(10,"Diamonds"),(11,"Hearts"),(11,"Spades"),(11,"Clubs")
    ,(11,"Diamonds"),(12,"Hearts"),(12,"Spades"),(12,"Clubs"),(12,"Diamonds"),(13,"Hearts")
    ,(13,"Spades"),(13,"Clubs"),(13,"Diamonds")];

    let mut rng = rand::thread_rng();
    let random_selection = single_deck.remove(rng.gen_range(0..single_deck.len()));
    let mut card_number: u32 = random_selection.0;
    let mut card_number_appearance: String = random_selection.0.to_string();

    match card_number_appearance.parse().unwrap() {

        1 => card_number_appearance = "Ace".to_string(),
        2|3|4|5|6|7|8|9|10 => (),
        11 => card_number_appearance = "Jack".to_string(),
        12 => card_number_appearance = "Queen".to_string(),
        13 => card_number_appearance = "King".to_string(),
        _ => print!("Error creating card!")

    }

    match card_number{

        1|2|3|4|5|6|7|8|9|10 => (),
        11 => card_number = card_number - 1,
        12 => card_number = card_number -2,
        13 => card_number = card_number -3,
        _ => print!("card number error")

    }

    let card_suit: String = random_selection.1.to_owned();

    return (card_number_appearance, card_suit, card_number)

}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let app = app::App::default();
    let mut wind = Window::default().with_size(600, 200).with_label("Card Game");
    let mut pack = Pack::default().with_size(120, 140).center_of(&wind);
    pack.set_spacing(10);
    let mut but_start = Button::default().with_size(100, 40).with_label("Start Game");
    let mut but_restart = Button::default().with_size(100, 40).with_label("Restart Game");
    pack.end();
    wind.end();
    wind.show();

    let (s, r) = app::channel::<Message>();

    but_start.emit(s, Message::Start);
    but_restart.emit(s, Message::Restart);

    let mut player1 = Player {

        hand: vec![],
        hand_total_value: 0,
        wager: Some(0),
        hands_played: Some(0),

    };

    while app.wait() {

        if let Some(msg) = r.recv() {

            match msg {

                Message::Start => player1.initialize_hand(),
                Message::Restart => player1.reset_hand(),

            }
        }
    }
    Ok(())
}