use crate::environment::{DPEnvironment, StateId, StateTransitionTable};
use crate::rl::policy::TabularPolicy;
use itertools::Itertools;
use ndarray::{Array, Array1};

#[derive(Debug)]
pub struct DPTrainerConfig {
    num_epochs: usize,
}

impl DPTrainerConfig {
    pub fn new(num_epochs: usize) -> DPTrainerConfig {
        DPTrainerConfig { num_epochs }
    }
}

pub struct DPTrainer<E: DPEnvironment> {
    env: E,
    config: DPTrainerConfig,
    // A vector of policies, with one policy per `Agent` in the `Environment`.
    policies: Vec<TabularPolicy>,
}

fn evaluate_policy(
    policy: &TabularPolicy,
    state_transitions: &StateTransitionTable,
    value_function: &mut Array1<f32>,
) {
    for i in 0..value_function.len() {
        let state_id = StateId(i);
        let transitions = state_transitions.get(&state_id);
        if let Some(transitions) = transitions {}
    }
}

impl<E: DPEnvironment> DPTrainer<E> {
    pub fn init_with_uniform_policies(env: E, config: DPTrainerConfig) -> DPTrainer<E> {
        let policies = (0..env.num_agents())
            .map(|_| TabularPolicy::create_uniform_policy(&env))
            .collect_vec();
        DPTrainer {
            env,
            policies,
            config,
        }
    }

    pub fn train(&mut self) {
        log::info!("starting training with {0:?}", self.config);
        let state_transitions = self.env.state_transitions();

        let mut value_function: Array1<f32> = Array::zeros(state_transitions.len());

        for epoch in 0..self.config.num_epochs {
            evaluate_policy(policy, &state_transitions, &mut value_function);
        }
    }
}
