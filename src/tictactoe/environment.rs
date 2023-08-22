use crate::environment::{
    DPEnvironment, Environment, ProbabilityT, RewardT, StateId, StateTransition,
};
use crate::tictactoe::action::TicTacToeAction;
use crate::tictactoe::cell::CellValue;
use crate::tictactoe::state::TicTacToeState;
use std::collections::HashMap;

#[derive(Debug)]
pub struct TicTacToeEnvironment {
    state: TicTacToeState,
}

impl TicTacToeEnvironment {
    pub fn new() -> Self {
        TicTacToeEnvironment {
            state: TicTacToeState::create_state_with_id(StateId(0)).unwrap(),
        }
    }

    fn reward_for_state(state: &TicTacToeState) -> RewardT {
        match state.has_winning_value() {
            CellValue::Circle => RewardT(-1.0),
            CellValue::Cross => RewardT(1.0),
            CellValue::None => RewardT(0.0),
        }
    }
}

impl Environment for TicTacToeEnvironment {
    type Action = TicTacToeAction;
    type State = TicTacToeState;

    fn state(&self) -> &TicTacToeState {
        &self.state
    }
    fn actions(&self) -> Vec<TicTacToeAction> {
        self.state.actions()
    }
    fn apply_action(&mut self, action: &TicTacToeAction) -> RewardT {
        self.state = self.state.apply_action(action);
        TicTacToeEnvironment::reward_for_state(&self.state)
    }
}

impl DPEnvironment for TicTacToeEnvironment {
    fn state_transitions(&self) -> HashMap<StateId, Vec<StateTransition>> {
        let mut transition_table = HashMap::new();
        let num_states = TicTacToeState::max_state_id();
        for i in 0..num_states.0 {
            let state_id = StateId(i);
            let state = TicTacToeState::create_state_with_id(state_id).unwrap();
            let actions = state.actions();
            let transitions = actions
                .iter()
                .map(|a| {
                    let new_state = state.apply_action(a);
                    StateTransition {
                        action_id: a.id(),
                        prob: ProbabilityT((actions.len() as f64).recip()),
                        new_state_id: new_state.id(),
                        reward: TicTacToeEnvironment::reward_for_state(&new_state),
                    }
                })
                .collect();
            transition_table.insert(state_id, transitions);
        }

        transition_table
    }
}
