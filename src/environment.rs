use std::collections::HashMap;

pub trait State {
    fn is_terminal(&self) -> bool;
}

#[derive(Debug)]
pub struct RewardT(pub f64);

#[derive(Debug)]
pub struct ProbabilityT(pub f64);

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub struct StateId(pub usize);

#[derive(Debug)]
pub struct ActionId(pub usize);

pub trait Environment {
    type State: State;
    type Action;

    fn state(&self) -> &Self::State;
    fn actions(&self) -> Vec<Self::Action>;
    fn apply_action(&mut self, action: &Self::Action) -> RewardT;
}

#[derive(Debug)]
pub struct StateTransition {
    pub action_id: ActionId,
    pub new_state_id: StateId,
    pub reward: RewardT,
    pub prob: ProbabilityT,
}

pub trait DPEnvironment: Environment {
    fn state_transitions(&self) -> HashMap<StateId, Vec<StateTransition>>;
}
