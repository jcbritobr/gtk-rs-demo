use std::{fmt::Display, slice::Iter};

#[derive(Debug, Clone, Copy)]
pub enum ExampleType {
    Button = 0,
    CustomCellRenderer,
    ComboBox,
    DrawingArea,
    Entry,
}

impl Display for ExampleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ExampleType {
    pub fn iterator() -> Iter<'static, ExampleType> {
        static COUNTER: [ExampleType; 5] = [
            ExampleType::Button,
            ExampleType::CustomCellRenderer,
            ExampleType::ComboBox,
            ExampleType::DrawingArea,
            ExampleType::Entry,
        ];
        COUNTER.iter()
    }

    pub fn from_number(number: usize) -> ExampleType {
        match number {
            0 => ExampleType::Button,
            1 => ExampleType::CustomCellRenderer,
            2 => ExampleType::ComboBox,
            3 => ExampleType::DrawingArea,
            4 => ExampleType::Entry,
            _ => ExampleType::Button,
        }
    }

    // pub fn as_number(self) -> usize {
    //     self as usize
    // }
}
