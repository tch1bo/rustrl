use crate::environment::ActionId;
use crate::tictactoe::cell::CellValue;

#[derive(Debug)]
pub struct TicTacToeAction {
    cell_value: CellValue,
    cell_index: usize,
}

impl TicTacToeAction {
    pub fn id(&self) -> ActionId {
        ActionId(self.cell_index * CellValue::num_values() + self.cell_value.value_id().0)
    }

    pub fn value(&self) -> CellValue {
        self.cell_value
    }

    pub fn index(&self) -> usize {
        self.cell_index
    }

    pub fn new(value: CellValue, index: usize) -> TicTacToeAction {
        TicTacToeAction {
            cell_value: value,
            cell_index: index,
        }
    }
}
