#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub enum CellValue {
    None,
    Cross,
    Circle,
}
pub struct CellValueId(pub usize);

impl CellValue {
    pub fn value_id(&self) -> CellValueId {
        match self {
            CellValue::None => CellValueId(0),
            CellValue::Cross => CellValueId(1),
            CellValue::Circle => CellValueId(2),
        }
    }

    pub fn value_with_id(value_id: CellValueId) -> CellValue {
        [CellValue::None, CellValue::Cross, CellValue::Circle][value_id.0]
    }

    pub fn num_values() -> usize {
        3
    }

    pub fn is_set(&self) -> bool {
        match self {
            CellValue::None => false,
            _ => true,
        }
    }
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
