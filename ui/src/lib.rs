use egui::Ui;
use engine::Game;

pub trait GameDisplay<G: Game> {
    fn show(&self, ui: &mut Ui, game: &G);
}

pub struct GameUi;

impl GameUi {
    pub fn new() -> Self {
        Self
    }

    pub fn render<G: Game, D: GameDisplay<G>>(&self, ui: &mut Ui, game: &G, display: &D) {
        display.show(ui, game);
    }
}
