use std::{
    fs::{self, read, File, OpenOptions},
    io::{prelude::*, BufReader, Read, Write},
};

use crate::creation_tools::game_window::game_window_creation;

use super::{game_create::create_dealer, player::Player};

pub fn save_file(name: &String, purse: &Option<i64>) {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(r"C:\Users\Michael Ladderbush\Desktop\Rust\rust_practice_cards\card_game_file.txt")
        .unwrap();

    let mut file_two = OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .open(r"C:\Users\Michael Ladderbush\Desktop\Rust\rust_practice_cards\card_game_file_purse.txt")
    .unwrap();

    let player_name_string = name;
    let player_purse = purse.unwrap();
    let player_purse_string = player_purse.to_string();

    file.write_all(player_name_string.as_bytes())
        .expect("write failed");
    file_two
        .write_all(player_purse_string.as_bytes())
        .expect("write failed");
}

//set up file to be used for continuing games and loading old ones.
pub fn use_file() {
    let string_from_file = fs::read_to_string("card_game_file.txt").expect("Couldn't read");
    let string_from_file_two =
        fs::read_to_string("card_game_file_purse.txt").expect("Couldn't read file");

    let mut new_player = Player {
        hand: vec![],
        hand_total_value: 0,
        wager: None,
        name: string_from_file,
        purse: Some(string_from_file_two.trim().parse().unwrap()),
    };

    let mut new_dealer = create_dealer();

    new_dealer.initialize_hand();
    new_player.initialize_hand();
    game_window_creation(new_player, new_dealer);
}
