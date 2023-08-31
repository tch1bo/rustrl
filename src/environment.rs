use std::collections::HashMap;

#[derive(Debug)]
pub struct RewardT(pub f32);

#[derive(Debug, Clone, Copy)]
pub struct ProbabilityT(pub f32);

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub struct StateId(pub usize);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ActionId(pub usize);

// TODO: make a template for these structs.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct AgentId(pub usize);

pub trait State {
    fn is_terminal(&self) -> bool;
    fn id(&self) -> StateId;
}

pub trait Action {
    fn id(&self) -> ActionId;
}

pub trait Environment {
    type State: State;
    type Action: Action;

    fn state(&self) -> &Self::State;
    fn actions(&self) -> Vec<Self::Action>;
    fn apply_action(&mut self, action: &Self::Action) -> RewardT;
    fn agent_id_for_action(&self, action: &Self::Action) -> AgentId;
    fn num_agents(&self) -> usize;
}

#[derive(Debug)]
pub struct StateTransition {
    pub action_id: ActionId,
    pub new_state_id: StateId,
    pub reward: RewardT,
    pub prob: ProbabilityT,
    pub agent_id: AgentId,
}

pub type StateTransitionTable = HashMap<StateId, Vec<StateTransition>>;

pub trait DPEnvironment: Environment {
    fn state_transitions(&self) -> StateTransitionTable;
}
