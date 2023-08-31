use crate::environment::{State, StateId};
use crate::tictactoe::action::TicTacToeAction;
use crate::tictactoe::cell::{CellValue, CellValueId};
use std::fmt;

pub const GRID_SIZE: usize = 3;

#[derive(Debug)]
pub struct TicTacToeState {
    // The cells are aranged from left to right, from top to bottom.
    cells: [CellValue; GRID_SIZE * GRID_SIZE],
}

impl fmt::Display for TicTacToeState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let num_dashes = GRID_SIZE * 2 + 1;
        let separator_line = "-".repeat(num_dashes) + "\n";
        f.write_str(&separator_line)?;
        for i in 0..GRID_SIZE {
            f.write_str("|")?;
            let row: Vec<String> = self.cells[i * GRID_SIZE..(i + 1) * GRID_SIZE]
                .iter()
                .map(|&cell| cell.into())
                .collect();
            f.write_str(&row.join("|"))?;
            f.write_str("|\n")?;
            f.write_str(&separator_line)?;
        }
        Ok(())
    }
}

impl TicTacToeState {
    fn next_cell_value(&self) -> CellValue {
        if self.is_terminal() {
            return CellValue::None;
        }
        let num_set_cells = self
            .cells
            .iter()
            .filter(|cell_value| cell_value.is_set())
            .count();
        if num_set_cells % 2 == 0 {
            CellValue::Cross
        } else {
            CellValue::Circle
        }
    }

    fn all_cells_set(&self) -> bool {
        self.cells.iter().all(|c| c.is_set())
    }

    pub fn has_winning_value(&self) -> CellValue {
        // Check rows.
        for i in 0..GRID_SIZE {
            let first_cell = self.cells[i * GRID_SIZE];
            if first_cell.is_set()
                && self.cells[i * GRID_SIZE..i * GRID_SIZE + GRID_SIZE]
                    .iter()
                    .all(|c| *c == first_cell)
            {
                return first_cell;
            }
        }

        // Check columns.
        for i in 0..GRID_SIZE {
            let first_cell = self.cells[i];
            if first_cell.is_set()
                && self.cells[i..]
                    .iter()
                    .step_by(GRID_SIZE)
                    .all(|c| *c == self.cells[i])
            {
                return first_cell;
            }
        }

        // Check the main diagonal.
        let top_left_cell = self.cells[0];
        if top_left_cell.is_set() {
            if (1..GRID_SIZE).all(|i| self.cells[i * GRID_SIZE + i] == top_left_cell) {
                return top_left_cell;
            }
        }

        // Check the other diagonal.
        let top_right_cell = self.cells[GRID_SIZE - 1];
        if top_right_cell.is_set() {
            if (1..GRID_SIZE)
                .all(|i| self.cells[i * GRID_SIZE + GRID_SIZE - i - 1] == top_right_cell)
            {
                return top_right_cell;
            }
        }
        CellValue::None
    }

    pub fn apply_action(&self, action: &TicTacToeAction) -> TicTacToeState {
        if self.is_terminal() {
            panic!("tried applying an action to a terminal state");
        }

        if self.cells[action.index()].is_set() {
            panic!(
                "action tried setting a cell that was already set. state={:?} action={:?}",
                self, action
            );
        }

        let mut new_cells = self.cells.clone();
        new_cells[action.index()] = action.value();
        TicTacToeState { cells: new_cells }
    }

    pub fn create_state_with_id(state_id: StateId) -> Option<TicTacToeState> {
        if state_id.0 > TicTacToeState::max_state_id().0 {
            return None;
        }

        const NUM_CELLS: usize = GRID_SIZE * GRID_SIZE;
        let mut cells = [CellValue::None; NUM_CELLS];
        let mut state_id: usize = state_id.0;
        for cell_id in 0..NUM_CELLS {
            let value_id = state_id % CellValue::num_values();
            cells[cell_id] = CellValue::value_with_id(CellValueId(value_id));
            state_id /= CellValue::num_values();
        }
        Some(TicTacToeState { cells })
    }

    pub fn actions(&self) -> Vec<TicTacToeAction> {
        let mut actions = vec![];
        if self.is_terminal() {
            return actions;
        }

        let next_cell_value = self.next_cell_value();
        for (index, cell_value) in self.cells.iter().enumerate() {
            if !cell_value.is_set() {
                actions.push(TicTacToeAction::new(next_cell_value, index))
            }
        }
        actions
    }

    pub fn max_state_id() -> StateId {
        let n = usize::pow(3, (GRID_SIZE * GRID_SIZE).try_into().unwrap());
        StateId(n)
    }
}

impl State for TicTacToeState {
    fn is_terminal(&self) -> bool {
        self.has_winning_value() != CellValue::None || self.all_cells_set()
    }

    fn id(&self) -> StateId {
        let mut mult: usize = 1;
        let mut state_id: usize = 0;
        for cell_value in self.cells.iter() {
            state_id += cell_value.value_id().0 * mult;
            mult *= CellValue::num_values();
        }
        StateId(state_id)
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;
    use crate::tictactoe::action::TicTacToeAction;
    use itertools::sorted;
    use pretty_assertions::assert_eq;

    fn make_state(cell_chars: [[char; GRID_SIZE]; GRID_SIZE]) -> TicTacToeState {
        let cells: Vec<CellValue> = cell_chars
            .iter()
            .flat_map(|row| row.iter())
            .map(|c| CellValue::try_from(*c).unwrap())
            .collect();
        let cells: [CellValue; GRID_SIZE * GRID_SIZE] = cells.try_into().unwrap();
        TicTacToeState { cells }
    }

    #[test]
    fn fmt_1() {
        let state = make_state([[' ', 'x', 'o'], ['x', 'o', 'x'], [' ', ' ', 'o']]);
        assert_eq!(
            state.to_string(),
            "\
-------
| |x|o|
-------
|x|o|x|
-------
| | |o|
-------
"
        );
    }

    #[test]
    fn all_cells_set_1() {
        let state = make_state([[' ', 'x', 'o'], ['x', 'o', 'x'], [' ', ' ', 'o']]);
        assert!(!state.all_cells_set());
        assert!(!state.is_terminal());
    }

    #[test]
    fn all_cells_set_2() {
        let state = make_state([['x', 'x', 'o'], ['x', 'o', 'x'], ['o', 'x', 'o']]);
        assert!(state.all_cells_set());
        assert!(state.is_terminal());
    }

    #[test]
    fn all_cells_set_3() {
        let state = make_state([[' ', ' ', ' '], [' ', ' ', ' '], [' ', ' ', ' ']]);
        assert!(!state.all_cells_set());
        assert!(!state.is_terminal());
    }

    #[test]
    fn has_winning_value_none() {
        let state = make_state([['x', 'o', 'x'], ['o', 'x', 'o'], ['o', 'x', ' ']]);
        assert_eq!(state.has_winning_value(), CellValue::None);
        assert!(!state.is_terminal());
    }

    #[test]
    fn has_winning_value_cols() {
        let state = make_state([['x', ' ', ' '], ['x', ' ', ' '], ['x', ' ', ' ']]);
        assert_eq!(state.has_winning_value(), CellValue::Cross);
        assert!(state.is_terminal());

        let state = make_state([[' ', 'x', ' '], [' ', 'x', ' '], [' ', 'x', ' ']]);
        assert_eq!(state.has_winning_value(), CellValue::Cross);
        assert!(state.is_terminal());

        let state = make_state([[' ', ' ', 'x'], [' ', ' ', 'x'], [' ', ' ', 'x']]);
        assert_eq!(state.has_winning_value(), CellValue::Cross);
        assert!(state.is_terminal());
    }

    #[test]
    fn has_winning_value_rows() {
        let state = make_state([['o', 'o', 'o'], [' ', ' ', ' '], [' ', ' ', ' ']]);
        assert_eq!(state.has_winning_value(), CellValue::Circle);
        assert!(state.is_terminal());

        let state = make_state([[' ', ' ', ' '], ['o', 'o', 'o'], [' ', ' ', ' ']]);
        assert_eq!(state.has_winning_value(), CellValue::Circle);
        assert!(state.is_terminal());

        let state = make_state([[' ', ' ', ' '], [' ', ' ', ' '], ['o', 'o', 'o']]);
        assert_eq!(state.has_winning_value(), CellValue::Circle);
        assert!(state.is_terminal());
    }

    #[test]
    fn has_winning_value_diagonals() {
        let state = make_state([['x', ' ', ' '], [' ', 'x', ' '], [' ', ' ', 'x']]);
        assert_eq!(state.has_winning_value(), CellValue::Cross);
        assert!(state.is_terminal());

        let state = make_state([[' ', ' ', 'x'], [' ', 'x', ' '], ['x', ' ', ' ']]);
        assert_eq!(state.has_winning_value(), CellValue::Cross);
        assert!(state.is_terminal());
    }

    #[test]
    fn next_cell_value_1() {
        let state = make_state([[' ', ' ', ' '], [' ', ' ', ' '], [' ', ' ', ' ']]);
        assert_eq!(state.next_cell_value(), CellValue::Cross);
    }

    #[test]
    fn next_cell_value_2() {
        let state = make_state([['x', ' ', ' '], [' ', ' ', ' '], [' ', ' ', ' ']]);
        assert_eq!(state.next_cell_value(), CellValue::Circle);
    }

    #[test]
    fn next_cell_value_3() {
        let state = make_state([['x', ' ', ' '], [' ', 'o', ' '], [' ', ' ', ' ']]);
        assert_eq!(state.next_cell_value(), CellValue::Cross);
    }

    #[test]
    fn next_cell_value_4() {
        let state = make_state([['x', ' ', ' '], [' ', 'o', ' '], ['x', ' ', ' ']]);
        assert_eq!(state.next_cell_value(), CellValue::Circle);
    }

    #[test]
    fn next_cell_value_5() {
        let state = make_state([['x', ' ', ' '], ['o', 'o', ' '], ['x', ' ', ' ']]);
        assert_eq!(state.next_cell_value(), CellValue::Cross);
    }

    #[test]
    fn next_cell_value_6() {
        let state = make_state([['x', ' ', ' '], ['o', 'o', 'x'], ['x', ' ', ' ']]);
        assert_eq!(state.next_cell_value(), CellValue::Circle);
    }

    #[test]
    fn next_cell_value_7() {
        let state = make_state([['x', 'o', ' '], ['o', 'o', 'x'], ['x', ' ', ' ']]);
        assert_eq!(state.next_cell_value(), CellValue::Cross);
    }

    #[test]
    fn next_cell_value_8() {
        let state = make_state([['x', 'o', 'x'], ['o', 'o', 'x'], ['x', ' ', ' ']]);
        assert_eq!(state.next_cell_value(), CellValue::Circle);
    }

    #[test]
    fn next_cell_value_9() {
        let state = make_state([['x', 'o', 'x'], ['o', 'o', 'x'], ['x', 'o', ' ']]);
        // Circles won, so the next value is None.
        assert_eq!(state.has_winning_value(), CellValue::Circle);
        assert_eq!(state.next_cell_value(), CellValue::None);
    }

    #[test]
    fn next_cell_value_terminal_1() {
        let state = make_state([['x', 'o', 'x'], ['o', 'o', 'x'], ['x', 'x', 'o']]);
        assert!(state.is_terminal());
        assert_eq!(state.next_cell_value(), CellValue::None);
    }

    #[test]
    fn apply_actions_0() {
        let state = make_state([[' ', ' ', ' '], [' ', ' ', ' '], [' ', ' ', ' ']]);
        let actions = sorted(state.actions()).collect_vec();
        assert_eq!(
            actions,
            vec![
                TicTacToeAction::new(CellValue::Cross, 0),
                TicTacToeAction::new(CellValue::Cross, 1),
                TicTacToeAction::new(CellValue::Cross, 2),
                TicTacToeAction::new(CellValue::Cross, 3),
                TicTacToeAction::new(CellValue::Cross, 4),
                TicTacToeAction::new(CellValue::Cross, 5),
                TicTacToeAction::new(CellValue::Cross, 6),
                TicTacToeAction::new(CellValue::Cross, 7),
                TicTacToeAction::new(CellValue::Cross, 8)
            ]
        );

        let new_states_as_strings = actions
            .iter()
            .map(|action| state.apply_action(action).to_string())
            .collect_vec();
        assert_eq!(
            state.to_string(),
            "\
-------
| | | |
-------
| | | |
-------
| | | |
-------
"
        );

        assert_eq!(
            new_states_as_strings,
            vec![
                "\
-------
|x| | |
-------
| | | |
-------
| | | |
-------
",
                "\
-------
| |x| |
-------
| | | |
-------
| | | |
-------
",
                "\
-------
| | |x|
-------
| | | |
-------
| | | |
-------
",
                "\
-------
| | | |
-------
|x| | |
-------
| | | |
-------
",
                "\
-------
| | | |
-------
| |x| |
-------
| | | |
-------
",
                "\
-------
| | | |
-------
| | |x|
-------
| | | |
-------
",
                "\
-------
| | | |
-------
| | | |
-------
|x| | |
-------
",
                "\
-------
| | | |
-------
| | | |
-------
| |x| |
-------
",
                "\
-------
| | | |
-------
| | | |
-------
| | |x|
-------
",
            ]
        );
    }

    #[test]
    fn apply_actions_1() {
        let state = make_state([['x', ' ', ' '], [' ', 'o', ' '], [' ', ' ', ' ']]);
        let actions = sorted(state.actions()).collect_vec();
        assert_eq!(
            actions,
            vec![
                TicTacToeAction::new(CellValue::Cross, 1),
                TicTacToeAction::new(CellValue::Cross, 2),
                TicTacToeAction::new(CellValue::Cross, 3),
                TicTacToeAction::new(CellValue::Cross, 5),
                TicTacToeAction::new(CellValue::Cross, 6),
                TicTacToeAction::new(CellValue::Cross, 7),
                TicTacToeAction::new(CellValue::Cross, 8)
            ]
        );

        let new_states_as_strings = actions
            .iter()
            .map(|action| state.apply_action(action).to_string())
            .collect_vec();
        assert_eq!(
            state.to_string(),
            "\
-------
|x| | |
-------
| |o| |
-------
| | | |
-------
"
        );

        assert_eq!(
            new_states_as_strings,
            vec![
                "\
-------
|x|x| |
-------
| |o| |
-------
| | | |
-------
",
                "\
-------
|x| |x|
-------
| |o| |
-------
| | | |
-------
",
                "\
-------
|x| | |
-------
|x|o| |
-------
| | | |
-------
",
                "\
-------
|x| | |
-------
| |o|x|
-------
| | | |
-------
",
                "\
-------
|x| | |
-------
| |o| |
-------
|x| | |
-------
",
                "\
-------
|x| | |
-------
| |o| |
-------
| |x| |
-------
",
                "\
-------
|x| | |
-------
| |o| |
-------
| | |x|
-------
",
            ]
        );
    }

    #[test]
    fn apply_actions_on_terminal_state_1() {
        let state = make_state([['x', 'x', 'x'], ['o', 'o', ' '], [' ', ' ', ' ']]);
        assert!(state.is_terminal());
        let actions = sorted(state.actions()).collect_vec();
        assert_eq!(actions, vec![]);
    }

    #[test]
    fn apply_actions_on_terminal_state_2() {
        let state = make_state([['x', 'o', 'x'], ['o', 'o', 'x'], ['x', 'x', 'o']]);
        assert!(state.is_terminal());
        let actions = sorted(state.actions()).collect_vec();
        assert_eq!(actions, vec![]);
    }

    #[test]
    fn ids_are_bijective() {
        for id in 0..TicTacToeState::max_state_id().0 {
            let id = StateId(id);
            let state = TicTacToeState::create_state_with_id(id).unwrap();
            assert_eq!(id, state.id(), "state={:#}", state);
        }
    }
}
