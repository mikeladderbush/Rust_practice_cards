//use creation_tools::window_create;
use eframe::{
    egui::{CentralPanel, Ui},
    epi::App,
    run_native, NativeOptions,
};
//mod creation_tools;

struct CardGame;

impl App for CardGame {
    fn update(&mut self, ctx: &eframe::egui::CtxRef, frame: &mut eframe::epi::Frame) {
        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            ui.label("Card Game");
        });
    }

    fn name(&self) -> &str {
        "Card Game"
    }
}

fn main() {
    let app: CardGame = CardGame;
    let win_options = NativeOptions::default();
    run_native(Box::new(app), win_options);
}
