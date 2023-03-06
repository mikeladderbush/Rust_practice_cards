use std::{io};

use fltk::{button::Button, prelude::WidgetExt, app};
use rand::Rng;

use crate::Message;

#[derive(Debug, Clone)]
pub struct Player {

    pub hand: Vec<(String, String, u32)>,
    pub hand_total_value: u32,
    pub wager: Option<usize>,
    pub purse: usize,
    pub name: String

}

impl Player {

    pub fn add_to_hand(&mut self){
    
        let added_card = create_card();
        self.hand_total_value = self.hand_total_value + &added_card.2;
        self.hand.push(added_card);
        
        println!("{:?}", self);

    }

    pub fn subtract_wager(&mut self){

        self.purse = self.purse - self.wager.unwrap();

    }

    pub fn initialize_hand(&mut self){
       
        let card_1 = create_card();
        let card_2 = create_card();
        self.hand_total_value = self.hand_total_value + card_1.2;
        self.hand_total_value = self.hand_total_value + card_2.2;
        self.hand.push(card_1);
        self.hand.push(card_2);

        println!("{:?}", self);

    }

    pub fn empty_hand(&mut self){

        self.hand = vec![];
        self.hand_total_value = 0;

    }

}

impl Default for Player {
    fn default() -> Self {
        Self { hand: Default::default(), hand_total_value: Default::default(), wager: Default::default(), name: Default::default(), purse: Default::default() }
    }
}


pub fn create_card() -> (String, String, u32) {

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

pub fn create_game() -> (Player, Player){

    let dealer_name = "Dealer".to_string();

    println!("please enter a name for your player: ");
    let mut player_name: String = String::new();
    io::stdin().read_line(&mut player_name).expect("failed to readline");

    println!("please enter your wager: ");
    let mut wager = String::new();
    io::stdin().read_line(&mut wager).expect("failed to readline");

    let mut dealer  = Player {

        hand: vec![],
        hand_total_value: 0,
        wager: None,
        name: dealer_name,
        purse: 0,
    
    };

    let mut player = Player {

        hand: vec![],
        hand_total_value: 0,
        wager: Some(wager.trim().parse().unwrap()),
        name: player_name,
        purse: 25,

    };

    dealer.initialize_hand();
    player.initialize_hand();
    player.subtract_wager();
    return (dealer, player);

}