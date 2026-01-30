#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

impl TryFrom<char> for Color {
    type Error = String;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value.to_ascii_lowercase() {
            'w' => Ok(Color::White),
            'b' => Ok(Color::Black),
            _ => Err(format!("Color::from_str, Invalid color: {value}!")),
        }
    }
}
