use std::collections::HashMap;

use crate::environment::{ActionId, DPEnvironment, ProbabilityT, StateId};

#[derive(Debug)]
pub struct TabularPolicy {
    probs_table: HashMap<StateId, Vec<(ActionId, ProbabilityT)>>,
}

impl TabularPolicy {
    fn find_state_and_action(
        &self,
        state_id: StateId,
        action_id: ActionId,
    ) -> Option<ProbabilityT> {
        Some(
            self.probs_table
                .get(&state_id)?
                .iter()
                .find(|p| p.0 == action_id)?
                .1,
        )
    }

    pub fn probability_for_action(&self, state_id: StateId, action_id: ActionId) -> ProbabilityT {
        self.find_state_and_action(state_id, action_id)
            .unwrap_or(ProbabilityT(0.0))
    }

    pub fn create_uniform_policy(env: &impl DPEnvironment) -> TabularPolicy {
        let mut probs_table = HashMap::new();
        for (state_id, transitions) in env.state_transitions().iter() {
            if transitions.is_empty() {
                continue;
            }
            let policy_prob = ProbabilityT((transitions.len() as f32).recip());
            probs_table.insert(
                *state_id,
                transitions
                    .iter()
                    .map(|t| (t.action_id, policy_prob))
                    .collect(),
            );
        }
        TabularPolicy { probs_table }
    }
}
