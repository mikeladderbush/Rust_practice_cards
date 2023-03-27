use super::game_creation_tools::{file_manager::save_file, player::Player, game_create::{create_dealer, create_game}};

pub fn game_window_creation() {
    let player: Player = Default::default();
    let dealer: Player = create_dealer();
    let mut current_players: (Player, Player) = (player, dealer);
    create_game();

    if current_players.1.hand_total_value > 21 {
        println!("You busted");
        current_players.0.empty_hand();
        current_players.1.empty_hand();
        current_players.1.subtract_wager();
        save_file(
            &current_players.1.name.to_owned(),
            &current_players.1.purse.to_owned(),
        );
    } else if current_players.1.hand_total_value == 21 {
        println!("lucky 21!");
        current_players.0.empty_hand();
        current_players.1.empty_hand();
        current_players.1.add_to_purse();
        save_file(
            &current_players.1.name.to_owned(),
            &current_players.1.purse.to_owned(),
        );
    } else if current_players.0.hand_total_value == 21 {
        println!("21....You lose");
        current_players.0.empty_hand();
        current_players.1.empty_hand();
        current_players.1.subtract_wager();
        save_file(
            &current_players.1.name.to_owned(),
            &current_players.1.purse.to_owned(),
        );
    } else if current_players.0.hand_total_value > 21 {
        println!("Dealer busts. You win!");
        current_players.0.empty_hand();
        current_players.1.empty_hand();
        current_players.1.add_to_purse();
        save_file(
            &current_players.1.name.to_owned(),
            &current_players.1.purse.to_owned(),
        );
    }
}
