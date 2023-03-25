use crate::creation_tools::game_creation_tools::game_create::create_card;

#[derive(Debug, Clone, Default)]
pub struct Player {
    pub hand: Vec<(String, String, u32)>,
    pub hand_total_value: u32,
    pub wager: Option<i64>,
    pub purse: Option<i64>,
    pub name: String,
}

impl Player {
    pub fn add_to_purse(&mut self) -> i64 {
        if self.wager.is_some() {
            let wager = self.wager.unwrap();

            if self.purse.is_some() {
                let mut purse = self.purse.unwrap();
                purse += wager;
                purse
            } else {
                self.purse.unwrap()
            }
        } else {
            self.purse.unwrap()
        }
    }

    pub fn add_to_hand(&mut self) {
        let added_card = create_card();
        self.hand_total_value += added_card.2;
        self.hand.push(added_card);
        println!("{:?}", self);
    }

    pub fn subtract_wager(&mut self) -> i64 {
        if self.wager.is_some() {
            let wager = self.wager.unwrap();

            if self.purse.is_some() {
                let mut purse = self.purse.unwrap();
                purse -= wager;
                purse
            } else {
                self.purse.unwrap()
            }
        } else {
            self.purse.unwrap()
        }
    }

    pub fn initialize_hand(&mut self) {
        let card_1 = create_card();
        let card_2 = create_card();
        self.hand_total_value += card_1.2;
        self.hand_total_value += card_2.2;
        self.hand.push(card_1);
        self.hand.push(card_2);

        println!("{:?}", self);
    }

    pub fn empty_hand(&mut self) {
        self.hand = vec![];
        self.hand_total_value = 0;
    }

    pub fn dealer_draw(&mut self) {
        while self.hand_total_value < 21 {
            self.add_to_hand();
        }
    }
}
