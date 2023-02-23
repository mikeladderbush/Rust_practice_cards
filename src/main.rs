use rand::Rng; // 0.8.5

#[derive(Debug)]
struct Player {

    hand: Vec<(String, String, u32)>,
    wager: Option<u32>,
    hands_played: Option<u32>

}

impl Player{

    fn initialize_hand(&mut self){

        self.hand.push(create_card());
        self.hand.push(create_card());

    }

    fn add_to_hand(&mut self){

        self.hand.push(create_card());

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


fn main() {

    let first_hand = vec![];
    let second_hand = vec![];

        let mut player1 = Player {

            hand: first_hand,
            hands_played: Some(1),
            wager: Some(50)

        };

        let mut dealer = Player {

            hand: second_hand,
            hands_played: None,
            wager: None

        };

        player1.initialize_hand();
        dealer.initialize_hand();

        println!("{:?}", player1);
        println!("{:?}", dealer);

}
