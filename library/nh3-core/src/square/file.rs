#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct File(u8);

impl File {
    #[must_use]
    pub const fn from_index(index: u8) -> File {
        debug_assert!(index < 8, "File::from_index, index out of range");

        File(index)
    }
}

impl TryFrom<char> for File {
    type Error = String;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'a'..='h' => Ok(File::from_index(value as u8 - b'a')),
            _ => Err(format!("File::try_from::<char>, invalid file char {value}")),
        }
    }
}

impl File {
    #[must_use]
    #[inline]
    pub const fn index(&self) -> u8 {
        self.0
    }
}
