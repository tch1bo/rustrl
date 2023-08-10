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
        for i in 0..GRID_SIZE {
            let row: Vec<String> = self.cells[i * GRID_SIZE..(i + 1) * GRID_SIZE]
                .iter()
                .map(|&cell| cell.into())
                .collect();
            f.write_str(&row.join("|"))?;
            f.write_str("\n")?;
            if i < GRID_SIZE - 1 {
                f.write_str(&"-".repeat(GRID_SIZE * 2 - 1))?;
                f.write_str("\n")?;
            }
        }
        Ok(())
    }
}

impl TicTacToeState {
    fn next_cell_value(&self) -> CellValue {
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

    pub fn create_state_with_id(state_id: StateId) -> TicTacToeState {
        const NUM_CELLS: usize = GRID_SIZE * GRID_SIZE;
        let mut cells = [CellValue::None; NUM_CELLS];
        let mut state_id: usize = state_id.0;
        for cell_id in (0..NUM_CELLS).rev() {
            let value_id = state_id % CellValue::num_values();
            cells[cell_id] = CellValue::value_with_id(CellValueId(value_id));
            state_id /= CellValue::num_values();
        }
        TicTacToeState { cells }
    }
    pub fn id(&self) -> StateId {
        let mut mult: usize = 1;
        let mut state_id: usize = 0;
        for cell_value in self.cells.iter().rev() {
            state_id = cell_value.value_id().0 * mult;
            mult *= CellValue::num_values();
        }
        StateId(state_id)
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
}
