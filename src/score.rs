use crate::board::Board;
use crate::letter::{FromChar, Letter};

const LETTERS_VALUES: [u16; 27] = [
    0, 1, 3, 3, 2, 1, 4, 2, 4, 1, 8, 10, 1, 2, 1, 1, 3, 8, 1, 1, 1, 1, 4, 10, 10, 10, 10,
];

pub fn calculate_score(board: &Board, letters: &str) -> u16 {
    let mut score = 0;
    for letter in letters.chars() {
        score += LETTERS_VALUES[Letter::from_char(letter) as usize];
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scoring_system() {
        let letters = "ANTICONS ITUTIONNE LEMENT";
        let board = Board::new();
        let score = calculate_score(&board, letters);
        assert_eq!(score, 26);
    }
}
