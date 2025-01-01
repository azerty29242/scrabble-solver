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
use std::io::{stdin, stdout, Write};

fn get_user_input() -> String {
    let mut result = String::new();
    let _ = stdout().flush();
    stdin()
        .read_line(&mut result)
        .expect("Did not enter a correct string");
    if let Some('\n') = result.chars().next_back() {
        result.pop();
    }
    if let Some('\r') = result.chars().next_back() {
        result.pop();
    }
    return result;
}

fn clear_screen() {
    print!("{}[2J", 27 as char);
}

fn main() -> Result<(), String> {
    let mut board: Board = Board::new();
    let lexicon = Lexicon::from_file("src/dictionaries/ods8.txt");
    let mut rack = HashMap::new();
    loop {
        clear_screen();
        rack.clear();
        println!("{}", board);
        println!("What do you want to do?");
        println!("[1] Place a word on the board");
        println!("[2] Calculate the best moves");
        println!("[3] Quit");
        print!(": ");
        match get_user_input().parse::<u8>() {
            Ok(1) => {
                print!("Word: ");
                let word = get_user_input();
                print!("Row number: ");
                let row = get_user_input().parse::<usize>().unwrap();
                print!("Column number: ");
                let column = get_user_input().parse::<usize>().unwrap();
                println!("Choose the orientation of the word");
                println!("[1] Horizontal");
                println!("[2] Vertical");
                print!(": ");
                let across;
                match get_user_input().parse::<u8>() {
                    Ok(1) => {
                        across = true;
                    }
                    Ok(2) => {
                        across = false;
                    }
                    _ => {
                        println!("Please enter a valid number");
                        continue;
                    }
                }
                board.play(&word, row - 1, column - 1, across);
            }
            Ok(2) => {
                print!("Rack: ");
                for letter in get_user_input().chars() {
                    add_letter_to_rack(&mut rack, Letter::from_char(letter));
                }
                let mut legal_moves = calculate_legal_moves(&lexicon, &board, &mut rack);
                board.rotate();
                legal_moves.append(&mut calculate_legal_moves(&lexicon, &board, &mut rack));
                board.rotate();
                legal_moves.sort_by(|a, b| b.score.cmp(&a.score));
                println!(
                    "In total, {} words were found by the algorithm.",
                    legal_moves.len()
                );
                println!("Here is a list of the 30 highest-scoring words: ");
                legal_moves.truncate(30);
                for (index, legal_move) in legal_moves.iter().enumerate() {
                    println!("[{}] {} horizontal: {} row: {} column: {} ({} pts)", index + 1, legal_move.letters, legal_move.across, legal_move.row_index + 1, legal_move.column_index + 1, legal_move.score);
                }
                print!("Number corresponding to the word to play: ");
                let word_index = get_user_input().parse::<usize>().unwrap() - 1;
                let word = &legal_moves[word_index];
                board.play(&word.letters, word.row_index, word.column_index, word.across);
            }
            Ok(3) => break Ok(()),
            _ => println!("Please enter a valid number"),
        }
        for letter in "".chars() {
            add_letter_to_rack(&mut rack, Letter::from_char(letter));
        }
    }
}
