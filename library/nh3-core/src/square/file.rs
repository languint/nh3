#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct File(u8);
impl File {
    pub const A: File = File(0);
    pub const B: File = File(1);
    pub const C: File = File(2);
    pub const D: File = File(3);
    pub const E: File = File(4);
    pub const F: File = File(5);
    pub const G: File = File(6);
    pub const H: File = File(7);

    pub const ALL: [File; 8] = [
        File::A,
        File::B,
        File::C,
        File::D,
        File::E,
        File::F,
        File::G,
        File::H,
    ];
}

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
