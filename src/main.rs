mod board;
mod legal_moves;
mod letter;
mod lexicon;
mod score;

use std::collections::HashMap;

use board::Board;
use legal_moves::calculate_legal_moves;
use lexicon::Lexicon;

fn main() -> Result<(), String> {
    let mut board: Board = Board::new();
    let lexicon = Lexicon::from_file("src/dictionaries/ods8.txt");
    let mut rack = HashMap::from([(2, 1), (3, 1), (7, 1), (8, 1), (9, 1), (14, 1), (21, 1)]);
    board.play("CAFES", 7, 6, true);
    board.play("KIF", 5, 8, false);
    board.play("HI", 6, 7, true);
    board.play("KAPPA", 5, 8, true);
    board.play("DELAYER", 2, 12, false);
    board.play("YUE", 6, 12, true);
    board.play("LUXER", 3, 14, false);
    board.play("EWE", 3, 10, true);
    board.play("VAR", 2, 8, true);
    board.play("MELIONS", 3, 2, true);
    board.play("EGO", 1, 6, false);
    board.play("DESOLER", 1, 1, true);
    board.play("LEVITERAI", 3, 4, false);
    board.play("ETAIERA", 8, 0, true);
    board.play("TENDUE", 7, 0, false);
    board.play("EUE", 12, 0, true);
    board.play("SAMOANS", 13, 2, true);
    board.play("SOIF", 14, 6, true);
    board.play("LIMITONS", 11, 3, true);
    board.play("ZEN", 9, 9, false);
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
    //legal_moves.truncate(30);
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
    // Should find 256 solutions in total
    Ok(())
}
