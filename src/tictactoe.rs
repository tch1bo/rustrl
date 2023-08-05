use crate::environment::{Environment, State};
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Ord, Eq)]
enum CellValue {
    Cross,
    Circle,
    None,
}

impl Into<char> for CellValue {
    fn into(self) -> char {
        match self {
            CellValue::Cross => 'x',
            CellValue::Circle => 'o',
            CellValue::None => ' ',
        }
    }
}

impl Into<String> for CellValue {
    fn into(self) -> String {
        let c: char = self.into();
        String::from(c)
    }
}

impl CellValue {
    fn is_set(&self) -> bool {
        match self {
            CellValue::None => false,
            _ => true,
        }
    }
}

const N: usize = 3;

#[derive(Debug)]
pub struct TicTacToeState {
    // The cells are aranged from left to right, from top to bottom.
    cells: [CellValue; N * N],
}

impl fmt::Display for TicTacToeState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..N {
            let row: Vec<String> = self.cells[i * N..(i + 1) * N]
                .iter()
                .map(|&cell| cell.into())
                .collect();
            f.write_str(&row.join("|"))?;
            f.write_str("\n")?;
            if i < N - 1 {
                f.write_str(&"-".repeat(N * 2 - 1))?;
                f.write_str("\n")?;
            }
        }
        Ok(())
    }
}

impl TicTacToeState {
    fn get_next_cell_value(&self) -> CellValue {
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
    fn has_winning_value(&self) -> CellValue {
        // Check rows.
        for i in 0..N {
            let first_cell = self.cells[i * N];
            if first_cell.is_set()
                && self.cells[i * N..i * N + N]
                    .iter()
                    .all(|c| *c == first_cell)
            {
                return first_cell;
            }
        }

        // Check columns.
        for i in 0..N {
            let first_cell = self.cells[i];
            if first_cell.is_set()
                && self.cells[i..]
                    .iter()
                    .step_by(N)
                    .all(|c| *c == self.cells[i])
            {
                return first_cell;
            }
        }

        // Check the main diagonal.
        let top_left_cell = self.cells[0];
        if top_left_cell.is_set() {
            if (1..N).all(|i| self.cells[i * N + i] == top_left_cell) {
                return top_left_cell;
            }
        }

        // Check the other diagonal.
        let top_right_cell = self.cells[N - 1];
        if top_right_cell.is_set() {
            if (1..N).all(|i| self.cells[i * N + N - i - 1] == top_right_cell) {
                return top_right_cell;
            }
        }
        CellValue::None
    }
    fn apply_action(&self, action: &TicTacToeAction) -> TicTacToeState {
        if self.is_terminal() {
            panic!("tried applying an action to a terminal state");
        }

        if self.cells[action.cell_index].is_set() {
            panic!(
                "action tried setting a cell that was already set. state={:?} action={:?}",
                self, action
            );
        }

        let mut new_cells = self.cells.clone();
        new_cells[action.cell_index] = action.cell_value;
        TicTacToeState { cells: new_cells }
    }
}

impl State for TicTacToeState {
    fn is_terminal(&self) -> bool {
        self.has_winning_value() != CellValue::None || self.all_cells_set()
    }
}

#[derive(Debug)]
pub struct TicTacToeAction {
    cell_value: CellValue,
    cell_index: usize,
}

#[derive(Debug)]
pub struct TicTacToeEnvironment {
    state: TicTacToeState,
}

impl TicTacToeEnvironment {
    pub fn new() -> Self {
        TicTacToeEnvironment {
            state: TicTacToeState {
                cells: [CellValue::None; N * N],
            },
        }
    }
}

impl Environment for TicTacToeEnvironment {
    type Action = TicTacToeAction;
    type State = TicTacToeState;

    fn get_state(&self) -> &TicTacToeState {
        &self.state
    }
    fn get_actions(&self) -> Vec<TicTacToeAction> {
        let mut actions = vec![];
        if self.state.is_terminal() {
            return actions;
        }

        let next_cell_value = self.state.get_next_cell_value();
        for (index, cell_value) in self.state.cells.iter().enumerate() {
            if !cell_value.is_set() {
                actions.push(TicTacToeAction {
                    cell_value: next_cell_value,
                    cell_index: index,
                });
            }
        }
        actions
    }
    fn apply_action(&mut self, action: &TicTacToeAction) -> f32 {
        self.state = self.state.apply_action(action);
        match self.state.has_winning_value() {
            CellValue::Circle => -1.0,
            CellValue::Cross => 1.0,
            CellValue::None => 0.0,
        }
    }
}
