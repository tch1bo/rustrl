use crate::environment::{
    Action, AgentId, DPEnvironment, Environment, ProbabilityT, RewardT, State, StateId,
    StateTransition, StateTransitionTable,
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

    fn reward_for_cross_agent(state: &TicTacToeState) -> RewardT {
        match state.has_winning_value() {
            CellValue::Circle => RewardT(-1.0),
            CellValue::Cross => RewardT(1.0),
            CellValue::None => RewardT(0.0),
        }
    }

    fn reward_for_circle_agent(state: &TicTacToeState) -> RewardT {
        match state.has_winning_value() {
            CellValue::Circle => RewardT(1.0),
            CellValue::Cross => RewardT(-1.0),
            CellValue::None => RewardT(0.0),
        }
    }

    fn reward_for_agent(state: &TicTacToeState, agent_id: AgentId) -> RewardT {
        match agent_id.0 {
            0 => TicTacToeEnvironment::reward_for_cross_agent(state),
            1 => TicTacToeEnvironment::reward_for_circle_agent(state),
            _ => panic!("unexpected agent_id: {:?}", agent_id.0),
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
        TicTacToeEnvironment::reward_for_agent(&self.state, self.agent_id_for_action(action))
    }

    fn agent_id_for_action(&self, action: &Self::Action) -> AgentId {
        match action.value() {
            CellValue::Cross => AgentId(0),
            CellValue::Circle => AgentId(1),
            CellValue::None => panic!("unexpected None action"),
        }
    }

    fn num_agents(&self) -> usize {
        2
    }
}

impl DPEnvironment for TicTacToeEnvironment {
    fn state_transitions(&self) -> StateTransitionTable {
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
                    let agent_id = self.agent_id_for_action(a);
                    StateTransition {
                        action_id: a.id(),
                        prob: ProbabilityT((actions.len() as f32).recip()),
                        new_state_id: new_state.id(),
                        reward: TicTacToeEnvironment::reward_for_agent(&new_state, agent_id),
                        agent_id,
                    }
                })
                .collect();
            transition_table.insert(state_id, transitions);
        }

        transition_table
    }
}
