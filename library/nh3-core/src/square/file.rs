#[derive(Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct File(u8);

impl File {
    pub const fn from_index(index: u8) -> File {
        debug_assert!(index < 8, "File::from_index, index out of range");

        File(index)
    }
}

impl TryFrom<char> for File {
    type Error = String;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'a'..='h' => Ok(File::from_index(value as u8 - ('a' as u8))),
            _ => Err(format!("File::try_from::<char>, invalid file char {value}")),
        }
    }
}

impl File {
    #[inline]
    pub const fn index(&self) -> u8 {
        self.0
    }
}
