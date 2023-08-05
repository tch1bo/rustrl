pub trait State {
    fn is_terminal(&self) -> bool;
}

pub trait Environment {
    type State: State;
    type Action;

    fn get_state(&self) -> &Self::State;
    fn get_actions(&self) -> Vec<Self::Action>;
    fn apply_action(&mut self, action: &Self::Action) -> f32;
}
