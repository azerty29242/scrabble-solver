use crate::board::Board;
use crate::letter::{Letter, ToChar};
use crate::lexicon::{Lexicon, Node};
use crate::score::{calculate_score, word_value};
use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub struct LegalMove {
    pub row_index: usize,
    pub column_index: usize,
    pub across: bool,
    pub letters: String,
    pub score: u16,
}

impl LegalMove {
    fn new(
        board: &Board,
        value_set: &[[u16; 15]; 15],
        mut row_index: usize,
        mut column_index: usize,
        across: bool,
        letters: String,
        bingo: bool,
    ) -> Self {
        let score = calculate_score(board, value_set, &letters, row_index, column_index, bingo);
        if !board.across {
            (row_index, column_index) = (column_index, row_index);
        }
        Self {
            row_index,
            column_index,
            across,
            letters,
            score,
        }
    }
}

fn remove_letter_from_rack(rack: &mut HashMap<Letter, u8>, letter: Letter) {
    if let Entry::Occupied(mut entry) = rack.entry(letter) {
        *entry.get_mut() -= 1;
        if *entry.get() == 0 {
            entry.remove();
        }
    }
}

pub fn add_letter_to_rack(rack: &mut HashMap<Letter, u8>, letter: Letter) {
    match rack.entry(letter) {
        Entry::Occupied(mut entry) => *entry.get_mut() += 1,
        Entry::Vacant(entry) => {
            entry.insert(1);
        }
    }
}

fn calculate_anchors(board: &Board) -> [u16; 15] {
    let mut anchors = [0; 15];
    for (row_index, row) in board.primary.iter().enumerate() {
        for (column_index, column) in row.iter().enumerate() {
            if column != &0 {
                if column_index > 0 && row[column_index - 1] == 0 {
                    anchors[row_index] |= 1 << column_index - 1;
                }
                if column_index < 14 && row[column_index + 1] == 0 {
                    anchors[row_index] |= 1 << column_index + 1;
                }
            }
        }
    }
    for (column_index, column) in board.secondary.iter().enumerate() {
        for (row_index, row) in column.iter().enumerate() {
            if row != &0 {
                if row_index > 0 && column[row_index - 1] == 0 {
                    anchors[row_index - 1] |= 1 << column_index;
                }
                if row_index < 14 && column[row_index + 1] == 0 {
                    anchors[row_index + 1] |= 1 << column_index;
                }
            }
        }
    }
    anchors
}

fn calculate_cross_check_sets_and_value_set(
    lexicon: &Lexicon,
    board: &Board,
) -> ([[u32; 15]; 15], [[u16; 15]; 15]) {
    let mut cross_check_sets = [[67108863; 15]; 15];
    let mut value_set = [[0; 15]; 15];

    for (column_index, column) in board.secondary.iter().enumerate() {
        for (row_index, letter) in column.iter().enumerate() {
            if letter != &0 {
                if row_index > 0 && column[row_index - 1] == 0 {
                    (
                        cross_check_sets[row_index - 1][column_index],
                        value_set[row_index - 1][column_index],
                    ) = calculate_letter_set_and_score(lexicon, column, row_index - 1);
                }
                if row_index < 14 && column[row_index + 1] == 0 {
                    (
                        cross_check_sets[row_index + 1][column_index],
                        value_set[row_index + 1][column_index],
                    ) = calculate_letter_set_and_score(lexicon, column, row_index + 1);
                }
            }
        }
    }

    (cross_check_sets, value_set)
}

fn calculate_letter_set_and_score(
    lexicon: &Lexicon,
    column: &[Letter; 15],
    anchor_row_index: usize,
) -> (u32, u16) {
    let mut letter_set = 0;
    let mut current_row_index = anchor_row_index;
    let mut prefix = String::new();
    let mut suffix = String::new();
    loop {
        if current_row_index > 0 && column[current_row_index - 1] != 0 {
            current_row_index -= 1;
            prefix.insert(0, (column[current_row_index] as Letter).to_char());
        } else {
            break;
        }
    }
    let mut current_row_index = anchor_row_index;
    loop {
        if current_row_index < 14 && column[current_row_index + 1] != 0 {
            current_row_index += 1;
            suffix.push((column[current_row_index] as Letter).to_char());
        } else {
            break;
        }
    }
    if let Some(node) = lexicon.root.get_node(&prefix) {
        for (edge, current_node) in node.children.iter() {
            if let Some(final_node) = current_node.get_node(&suffix) {
                if final_node.is_terminal {
                    letter_set |= 1 << edge - 1;
                }
            }
        }
    }
    let full_word = format!("{} {}", prefix, suffix);
    let score = word_value(&full_word);

    (letter_set, score)
}

fn extend_right(
    board: &Board,
    cross_check_sets: &[[u32; 15]; 15],
    value_set: &[[u16; 15]; 15],
    legal_moves: &mut Vec<LegalMove>,
    row_index: usize,
    column_index: usize,
    node: &Node,
    rack: &mut HashMap<Letter, u8>,
    partial_word: &mut String,
    possible: bool,
) {
    let current_column_index = column_index + partial_word.len();
    if current_column_index >= 15 {
        if node.is_terminal && possible {
            legal_moves.push(LegalMove::new(
                board,
                value_set,
                row_index,
                column_index,
                board.across.clone(),
                partial_word.clone(),
                rack.is_empty(),
            ));
        }
    } else {
        let tile = board.primary[row_index][current_column_index];
        if tile == 0 {
            if node.is_terminal && possible {
                legal_moves.push(LegalMove::new(
                    board,
                    value_set,
                    row_index,
                    column_index,
                    board.across.clone(),
                    partial_word.clone(),
                    rack.is_empty(),
                ));
            }
            for (letter, current_node) in node.children.iter() {
                if (cross_check_sets[row_index][current_column_index] & (1 << *letter - 1)) != 0
                    && rack.contains_key(letter)
                {
                    partial_word.push((*letter as Letter).to_char());
                    remove_letter_from_rack(rack, *letter);
                    extend_right(
                        board,
                        cross_check_sets,
                        value_set,
                        legal_moves,
                        row_index,
                        column_index,
                        current_node,
                        rack,
                        partial_word,
                        true,
                    );
                    add_letter_to_rack(rack, *letter);
                    partial_word.pop();
                }
            }
        } else {
            if let Some(current_node) = node.children.get(&tile) {
                partial_word.push((tile as Letter).to_char());
                extend_right(
                    board,
                    cross_check_sets,
                    value_set,
                    legal_moves,
                    row_index,
                    column_index,
                    current_node,
                    rack,
                    partial_word,
                    true,
                );
                partial_word.pop();
            }
        }
    }
}

fn left_part(
    board: &Board,
    cross_check_sets: &[[u32; 15]; 15],
    value_set: &[[u16; 15]; 15],
    legal_moves: &mut Vec<LegalMove>,
    row_index: usize,
    column_index: usize,
    node: &Node,
    rack: &mut HashMap<Letter, u8>,
    partial_word: &mut String,
    limit: u8,
) {
    extend_right(
        board,
        cross_check_sets,
        value_set,
        legal_moves,
        row_index,
        column_index,
        node,
        rack,
        partial_word,
        false,
    );
    if limit > 0 {
        for (letter, current_node) in node.children.iter() {
            if rack.contains_key(letter) {
                partial_word.push((*letter as Letter).to_char());
                remove_letter_from_rack(rack, *letter);
                left_part(
                    board,
                    cross_check_sets,
                    value_set,
                    legal_moves,
                    row_index,
                    column_index - 1,
                    current_node,
                    rack,
                    partial_word,
                    limit - 1,
                );
                add_letter_to_rack(rack, *letter);
                partial_word.pop();
            }
        }
    }
}

pub fn calculate_legal_moves(
    lexicon: &Lexicon,
    board: &Board,
    rack: &mut HashMap<Letter, u8>,
) -> Vec<LegalMove> {
    let mut legal_moves = Vec::new();
    let mut anchors = calculate_anchors(board);
    if anchors[7] == 0 {
        anchors[7] = 128;
    }
    let (cross_check_sets, value_set) = calculate_cross_check_sets_and_value_set(lexicon, board);
    for (row_index, row) in board.primary.iter().enumerate() {
        let mut non_anchor_square_count: u8 = 0;
        let mut partial_word = String::new();
        for (column_index, tile) in row.iter().enumerate() {
            let letter = (*tile as Letter).to_char();
            match (anchors[row_index] & (1 << column_index)) != 0 {
                true => {
                    if partial_word.is_empty() {
                        left_part(
                            board,
                            &cross_check_sets,
                            &value_set,
                            &mut legal_moves,
                            row_index,
                            column_index,
                            &lexicon.root,
                            rack,
                            &mut partial_word,
                            non_anchor_square_count,
                        );
                    } else {
                        let current_node = lexicon.root.get_node(&partial_word);
                        if let Some(current_node) = current_node {
                            extend_right(
                                board,
                                &cross_check_sets,
                                &value_set,
                                &mut legal_moves,
                                row_index,
                                column_index - partial_word.len(),
                                &current_node,
                                rack,
                                &mut partial_word,
                                false,
                            );
                        }
                        partial_word.clear();
                    }
                    non_anchor_square_count = 0;
                }
                false => {
                    if board.primary[row_index][column_index] != 0 {
                        partial_word.push(letter);
                    }
                    non_anchor_square_count += 1
                }
            }
        }
    }
    legal_moves
}
