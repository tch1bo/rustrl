use std::char;

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

#[derive(Debug, Clone)]
pub struct CellValueConversionError(char);

impl TryFrom<char> for CellValue {
    type Error = CellValueConversionError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'x' => Ok(CellValue::Cross),
            'o' => Ok(CellValue::Circle),
            ' ' => Ok(CellValue::None),
            _ => Err(CellValueConversionError(c)),
        }
    }
}

impl From<CellValue> for char {
    fn from(value: CellValue) -> char {
        match value {
            CellValue::Cross => 'x',
            CellValue::Circle => 'o',
            CellValue::None => ' ',
        }
    }
}

impl From<CellValue> for String {
    fn from(value: CellValue) -> String {
        char::from(value).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_id() {
        let values: Vec<CellValue> = (0..CellValue::num_values())
            .map(|id| CellValue::value_with_id(CellValueId(id as usize)))
            .collect();

        assert_eq!(
            values,
            [CellValue::None, CellValue::Cross, CellValue::Circle]
        );
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn incorrect_value_id() {
        CellValue::value_with_id(CellValueId(3));
    }

    #[test]
    fn conversion_from_char() {
        let chars = [' ', 'x', 'o'];
        let values: Vec<CellValue> = chars
            .map(|char| CellValue::try_from(char).unwrap())
            .to_vec();

        assert_eq!(
            values,
            [CellValue::None, CellValue::Cross, CellValue::Circle]
        );

        let serialized_chars: Vec<char> = values.iter().map(|v| char::from(*v)).collect();
        assert_eq!(serialized_chars, chars);
    }

    #[test]
    fn incorrect_conversion_from_char() {
        assert!(CellValue::try_from('b').is_err());
    }
}
