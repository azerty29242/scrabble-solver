mod board;
mod legal_moves;
mod letter;
mod lexicon;
mod score;

use board::Board;
use legal_moves::{add_letter_to_rack, calculate_legal_moves};
use letter::{FromChar, Letter};
use lexicon::Lexicon;
use std::collections::HashMap;

fn main() -> Result<(), String> {
    let mut board: Board = Board::new();
    let lexicon = Lexicon::from_file("src/dictionaries/ods8.txt");
    let mut rack = HashMap::new();
    for letter in "DIAAIAU".chars() {
        add_letter_to_rack(&mut rack, Letter::from_char(letter));
    }
    board.play("TIRANT", 7, 7, true);
    board.play("CABANES", 8, 4, true);
    println!("{}", board);
    let mut legal_moves = calculate_legal_moves(&lexicon, &board, &mut rack);
    board.rotate();
    legal_moves.append(&mut calculate_legal_moves(&lexicon, &board, &mut rack));
    board.rotate();
    legal_moves.sort_by(|a, b| b.score.cmp(&a.score));
    println!(
        "In total, {} words were found by the algorithm. Here is a list of the 30 highest-scoring words:",
        legal_moves.len()
    );
    legal_moves.truncate(30);
    for legal_move in legal_moves {
        println!(
            "Word: {}, row: {}, column: {}, across: {}, score: {}",
            legal_move.letters,
            legal_move.row_index + 1,
            legal_move.column_index + 1,
            legal_move.across,
            legal_move.score
        );
    }
    Ok(())
}
