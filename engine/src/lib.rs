use serde::{Serialize, Deserialize};

pub type PlayerId = u32;

#[derive(Debug, thiserror::Error)]
pub enum GameError {
    #[error("invalid action: {0}")]
    InvalidAction(String),
}

pub trait Game {
    type State: Clone + Serialize + for<'de> Deserialize<'de>;
    type Action: Clone + Serialize + for<'de> Deserialize<'de>;

    fn new(players: Vec<PlayerId>) -> Self;
    fn state(&self) -> &Self::State;
    fn current_player(&self) -> PlayerId;
    fn apply_action(&mut self, action: Self::Action) -> Result<(), GameError>;
    fn is_finished(&self) -> bool;
}

pub struct GameEngine<G: Game> {
    game: G,
    log: Vec<G::Action>,
}

impl<G: Game> GameEngine<G> {
    pub fn new(game: G) -> Self {
        Self { game, log: Vec::new() }
    }

    pub fn apply_action(&mut self, action: G::Action) -> Result<(), GameError> {
        self.game.apply_action(action.clone())?;
        self.log.push(action);
        Ok(())
    }

    pub fn state_snapshot(&self) -> G::State {
        self.game.state().clone()
    }

    pub fn replay_log(&self) -> &[G::Action] {
        &self.log
    }

    pub fn game(&self) -> &G {
        &self.game
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Serialize, Deserialize)]
    struct CounterState {
        value: u32,
        current: PlayerId,
    }

    #[derive(Clone, Serialize, Deserialize)]
    struct CounterAction;

    #[derive(Clone)]
    struct CounterGame {
        state: CounterState,
        players: Vec<PlayerId>,
    }

    impl Game for CounterGame {
        type State = CounterState;
        type Action = CounterAction;

        fn new(players: Vec<PlayerId>) -> Self {
            Self { state: CounterState { value: 0, current: players[0] }, players }
        }

        fn state(&self) -> &Self::State {
            &self.state
        }

        fn current_player(&self) -> PlayerId {
            self.state.current
        }

        fn apply_action(&mut self, _action: Self::Action) -> Result<(), GameError> {
            self.state.value += 1;
            let idx = self.players.iter().position(|p| *p == self.state.current).unwrap();
            self.state.current = self.players[(idx + 1) % self.players.len()];
            Ok(())
        }

        fn is_finished(&self) -> bool {
            self.state.value >= 5
        }
    }

    #[test]
    fn replay_restores_state() {
        let players = vec![1,2];
        let mut game = CounterGame::new(players.clone());
        let mut engine = GameEngine::new(game.clone());
        for _ in 0..5 {
            engine.apply_action(CounterAction).unwrap();
        }
        let log = engine.replay_log().to_owned();
        let mut game2 = CounterGame::new(players);
        for action in log {
            game2.apply_action(action).unwrap();
        }
        assert_eq!(game2.state.value, 5);
    }
}
