use engine::{Game, GameError, PlayerId};
use serde::{Serialize, Deserialize};
use ui::GameDisplay;
use egui::Ui;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Board {
    cells: [u8; 9],
    current: PlayerId,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Move {
    pub index: usize,
}

#[derive(Clone)]
pub struct TicTacToe {
    board: Board,
    players: Vec<PlayerId>,
}

impl TicTacToe {
    pub fn new(players: Vec<PlayerId>) -> Self {
        Self { board: Board { cells: [0; 9], current: players[0] }, players }
    }
}

impl Game for TicTacToe {
    type State = Board;
    type Action = Move;

    fn new(players: Vec<PlayerId>) -> Self {
        Self::new(players)
    }

    fn state(&self) -> &Self::State {
        &self.board
    }

    fn current_player(&self) -> PlayerId {
        self.board.current
    }

    fn apply_action(&mut self, action: Self::Action) -> Result<(), GameError> {
        if action.index >= 9 { return Err(GameError::InvalidAction("index out of range".into())); }
        if self.board.cells[action.index] != 0 { return Err(GameError::InvalidAction("cell not empty".into())); }
        let mark = self.current_player() + 1;
        self.board.cells[action.index] = mark as u8;
        let idx = self.players.iter().position(|p| *p == self.board.current).unwrap();
        self.board.current = self.players[(idx + 1) % self.players.len()];
        Ok(())
    }

    fn is_finished(&self) -> bool {
        const LINES: [[usize;3];8] = [
            [0,1,2],[3,4,5],[6,7,8], // rows
            [0,3,6],[1,4,7],[2,5,8], // cols
            [0,4,8],[2,4,6],         // diagonals
        ];
        for line in LINES { let [a,b,c] = line; let v = self.board.cells[a]; if v!=0 && v==self.board.cells[b] && v==self.board.cells[c] { return true; } }
        self.board.cells.iter().all(|&v| v != 0)
    }
}

pub struct TicTacToeDisplay;

impl GameDisplay<TicTacToe> for TicTacToeDisplay {
    fn show(&self, ui: &mut Ui, game: &TicTacToe) {
        for y in 0..3 {
            ui.horizontal(|ui| {
                for x in 0..3 {
                    let idx = y*3+x;
                    let label = match game.board.cells[idx] {
                        0 => "-",
                        1 => "X",
                        2 => "O",
                        _ => "?",
                    };
                    ui.label(label);
                }
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine::GameEngine;

    #[test]
    fn play_game() {
        let players = vec![0,1];
        let mut game = TicTacToe::new(players.clone());
        let mut engine = GameEngine::new(game.clone());
        engine.apply_action(Move { index: 0 }).unwrap();
        engine.apply_action(Move { index: 4 }).unwrap();
        engine.apply_action(Move { index: 1 }).unwrap();
        engine.apply_action(Move { index: 5 }).unwrap();
        engine.apply_action(Move { index: 2 }).unwrap();
        assert!(engine.game().is_finished());
        assert_eq!(engine.replay_log().len(), 5);
    }
}
