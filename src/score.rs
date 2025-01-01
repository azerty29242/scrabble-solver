use crate::board::Board;
use crate::letter::{FromChar, Letter};

const LETTERS_VALUES: [u16; 27] = [
    0, 1, 3, 3, 2, 1, 4, 2, 4, 1, 8, 10, 1, 2, 1, 1, 3, 8, 1, 1, 1, 1, 4, 10, 10, 10, 10,
];

pub const PREMIUM_SQUARES: [[u8; 15]; 15] = [
    [4, 0, 0, 1, 0, 0, 0, 4, 0, 0, 0, 1, 0, 0, 4],
    [0, 3, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 3, 0],
    [0, 0, 3, 0, 0, 0, 1, 0, 1, 0, 0, 0, 3, 0, 0],
    [1, 0, 0, 3, 0, 0, 0, 1, 0, 0, 0, 3, 0, 0, 1],
    [0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0],
    [0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0],
    [0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0],
    [4, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 1, 0, 0, 4],
    [0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0],
    [0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0],
    [0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0],
    [1, 0, 0, 3, 0, 0, 0, 1, 0, 0, 0, 3, 0, 0, 1],
    [0, 0, 3, 0, 0, 0, 1, 0, 1, 0, 0, 0, 3, 0, 0],
    [0, 3, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 3, 0],
    [4, 0, 0, 1, 0, 0, 0, 4, 0, 0, 0, 1, 0, 0, 4],
];

enum SquareType {
    Normal,
    DoubleLetter,
    TripleLetter,
    DoubleWord,
    TripleWord,
}

impl SquareType {
    fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::Normal,
            1 => Self::DoubleLetter,
            2 => Self::TripleLetter,
            3 => Self::DoubleWord,
            4 => Self::TripleWord,
            _ => panic!("Couldn't convert value to SquareType"),
        }
    }
}

pub fn word_value(letters: &str) -> u16 {
    let mut score = 0;
    for letter in letters.chars() {
        score += LETTERS_VALUES[Letter::from_char(letter) as usize];
    }
    score
}

pub fn calculate_score(
    board: &Board,
    value_set: &[[u16; 15]; 15],
    letters: &str,
    row_index: usize,
    mut column_index: usize,
    bingo: bool,
) -> u16 {
    let mut score = 0;
    let mut bonus_score = 0;
    let mut coefficient = 1;
    for letter in letters.chars() {
        let mut letter_value = LETTERS_VALUES[Letter::from_char(letter) as usize];
        let mut bonus_coefficient = 1;
        if board.primary[row_index][column_index] == 0 {
            match SquareType::from_u8(PREMIUM_SQUARES[row_index][column_index]) {
                SquareType::DoubleLetter => {
                    letter_value *= 2;
                }
                SquareType::TripleLetter => {
                    letter_value *= 3;
                }
                SquareType::DoubleWord => {
                    coefficient *= 2;
                    bonus_coefficient = 2;
                }
                SquareType::TripleWord => {
                    coefficient *= 3;
                    bonus_coefficient = 3;
                }
                _ => (),
            }
        } else {
            
        }
        score += letter_value;
        if value_set[row_index][column_index] != 0 {
            bonus_score += value_set[row_index][column_index] * bonus_coefficient;
            bonus_score += letter_value * bonus_coefficient;
        }
        column_index += 1;
    }
    score *= coefficient;
    if bingo {
        score += 50
    }
    score + bonus_score
}
