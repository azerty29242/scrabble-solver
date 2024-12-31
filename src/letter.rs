pub type Letter = u8;

pub trait ToChar {
    fn to_char(&self) -> char;
}

impl ToChar for Letter {
    fn to_char(&self) -> char {
        match *self {
            0 => Ok(' '),
            1..=26 => Ok((self + 0x40) as char),
            _ => Err(format!(
                "Invalid letter value: {self}. Expected a value between 0 and 26. \
                Only 26 letters (A-Z) and the space character are supported."
            )),
        }
        .unwrap()
    }
}

pub trait FromChar {
    fn from_char(letter: char) -> Letter;
}

impl FromChar for Letter {
    fn from_char(letter: char) -> Letter {
        match letter {
            'A'..='Z' => Ok((letter as u8) - 0x40),
            ' ' => Ok(0),
            _ => Err(format!(
                "Invalid character: '{letter}'. Expected an uppercase letter (A-Z) or a space (' ')."
            )),
        }.unwrap()
    }
}
