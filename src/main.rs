use macroquad::prelude::*;
mod creation_tools;
use creation_tools::game_window::{self, game_window_creation};

#[macroquad::main("Card Game")]
async fn main() {
    let mut profile_name: String = "Enter profile name here".to_string();
    loop {
        clear_background(BLACK);

        egui_macroquad::ui(|egui_ctx| {
            egui_macroquad::egui::Window::new("Card Game Start Screen").show(egui_ctx, |ui| {
                ui.colored_label(egui_macroquad::egui::Color32::WHITE, "Test");
                ui.add(
                    egui_macroquad::egui::TextEdit::singleline(&mut profile_name)
                        .text_color(egui_macroquad::egui::Color32::RED),
                );
                if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    game_window_creation();
                    //send profile name to new window that will use the name. if the name is in a saved file it will load that profile.
                }
            });
        });
        egui_macroquad::draw();
        next_frame().await
    }
}
