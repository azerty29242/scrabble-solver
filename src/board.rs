use crate::letter::{FromChar, Letter, ToChar};
use std::fmt::Display;

pub struct Board {
    pub primary: [[Letter; 15]; 15],
    pub secondary: [[Letter; 15]; 15],
    pub across: bool,
}

impl Board {
    pub fn new() -> Board {
        Board {
            primary: [[0; 15]; 15],
            secondary: [[0; 15]; 15],
            across: true,
        }
    }

    pub fn play(&mut self, word: &str, row_index: usize, column_index: usize, across: bool) {
        if across {
            for (index, letter) in word.chars().enumerate() {
                self.primary[row_index][column_index + index] = Letter::from_char(letter);
                self.secondary[column_index + index][row_index] = Letter::from_char(letter);
            }
        } else {
            for (index, letter) in word.chars().enumerate() {
                self.primary[row_index + index][column_index] = Letter::from_char(letter);
                self.secondary[column_index][row_index + index] = Letter::from_char(letter);
            }
        }
    }

    pub fn rotate(&mut self) {
        (self.primary, self.secondary) = (self.secondary, self.primary);
        self.across = !self.across;
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const TOP_BORDER: &str = "┌───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┐";
        const MIDDLE_BORDER: &str = "├───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┤";
        const BOTTOM_BORDER: &str = "└───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┘";
        writeln!(f, "{TOP_BORDER}")?;
        for (row_index, row) in self.primary.iter().enumerate() {
            write!(f, "│")?;
            for &cell in row {
                write!(f, " {} │", cell.to_char())?;
            }
            writeln!(f)?;
            if row_index < self.primary.len() - 1 {
                writeln!(f, "{MIDDLE_BORDER}")?;
            } else {
                writeln!(f, "{BOTTOM_BORDER}")?;
            }
        }
        Ok(())
    }
}
