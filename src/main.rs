use std::{collections::HashMap, fmt::Display, fs::File, io::{BufRead, BufReader}};

type Letter = u8;

pub struct Board {
    primary: [[Letter; 15]; 15],
    secondary: [[Letter; 15]; 15]
}

impl Board {
    pub fn new() -> Board {
        Board { primary: [[0; 15]; 15], secondary: [[0; 15]; 15] }
    }
    
    pub fn get(&self, row_index: usize, column_index: usize) -> Letter {
        self.primary[row_index][column_index]
    }

    pub fn rotate(&mut self) {
        let mut primary_board = [[0; 15]; 15];
        let mut secondary_board = [[0; 15]; 15];
        for row_index in 0..15 {
            for column_index in 0..15 {
                secondary_board[column_index][row_index] = self.primary[row_index][column_index];
                primary_board[column_index][row_index] = self.secondary[row_index][column_index];
            }
        }
        self.primary = primary_board;
        self.secondary = secondary_board;
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "┌───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┐")?;
        for (row_index, row) in self.primary.iter().enumerate() {
            write!(f, "│")?;
            for &cell in row.iter() {
                write!(f, " {} │", cell.to_char())?;
            }
            writeln!(f)?;

            if row_index < 14 {
                writeln!(f, "├───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┤")?;
            } else {
                writeln!(f, "└───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┘")?;
            }
        }
        Ok(())
    }
}

trait ToChar {
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
        }.unwrap()
    }
}

trait FromChar {
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

#[derive(Debug)]
struct Node {
    is_terminal: bool,
    children: HashMap<Letter, Node>
}

impl Node {
    fn new(is_terminal: bool) -> Node {
        Node { is_terminal, children: HashMap::new() }
    }

    fn get_node(&self, partial_word: String) -> Option<&Node> {
        let mut current_node = self;
        for letter in partial_word.chars() {
            if current_node.children.contains_key(&Letter::from_char(letter)) {
                current_node = current_node.children.get(&Letter::from_char(letter)).unwrap();
            } else {
                return None;
            }
        };
        Some(current_node)
    }
}

#[derive(Debug)]
struct Lexicon {
    root: Node
}

impl Lexicon {
    fn new() -> Lexicon {
        Lexicon { root: Node::new(false) }
    }

    pub fn from_file(path: &str) -> Self {
        let file = File::open(path).unwrap_or_else(|_| {
            panic!("Failed to open the lexicon file at '{}'. Ensure the file exists and is readable.", path);
        });

        let reader = BufReader::new(file);
        let mut lexicon = Lexicon::new();

        for line_result in reader.lines() {
            let word = line_result.unwrap_or_else(|_| {
                panic!("Failed to read a line from the lexicon file. Ensure it contains valid UTF-8 data.");
            });

            let mut current_node = &mut lexicon.root;
            for letter in word.chars() {
                current_node = current_node
                    .children
                    .entry(Letter::from_char(letter))
                    .or_insert(Node::new(false));
            }
            current_node.is_terminal = true;
        }

        lexicon
    }
}

struct Game {
    board: Board,
    lexicon: Lexicon
}

impl Game {
    fn new() -> Game {
        Game { board: Board::new(), lexicon: Lexicon::from_file("src/dictionaries/ods8.txt") }
    }

    fn letter_set(&self, column_index: usize, row_index: usize) -> u32 {
        let column = self.board.secondary[column_index];
        let mut letter_set = 0;
        let mut current_row_index = row_index;
        let mut before_anchor = String::new();
        let mut after_anchor = String::new();
        loop {
            if current_row_index > 0 && column[current_row_index - 1] != 0 {
                current_row_index -= 1;
                before_anchor.insert(0, (column[current_row_index] as Letter).to_char());
            } else {
                break;
            }
        }
        let mut current_row_index = row_index;
        loop {
            if current_row_index < 14 && column[current_row_index + 1] != 0 {
                current_row_index += 1;
                after_anchor.push((column[current_row_index] as Letter).to_char());
            } else {
                break;
            }
        }
        if let Some(node) = self.lexicon.root.get_node(before_anchor) {
            for (edge, current_node) in node.children.iter() {
                if let Some(final_node) = current_node.get_node(after_anchor.clone()) {
                    if final_node.is_terminal {
                        letter_set |= 1 << edge - 1;
                    }
                }
            }
        }
        letter_set
    }

    fn cross_check_sets(&self) -> [[u32; 15]; 15] {
        let mut cross_check_sets = [[0; 15]; 15];
    
        for (row_index, row) in self.board.primary.iter().enumerate() {
            for (column_index, column) in row.iter().enumerate() {
                if column != &0 {
                    if column_index > 0 && row[column_index - 1] == 0 {
                        cross_check_sets[row_index][column_index - 1] = 67108863
                    }
                    if column_index < 14 && row[column_index + 1] == 0 {
                        cross_check_sets[row_index][column_index + 1] = 67108863;
                    }
                }
            }
        }

        for (column_index, column) in self.board.secondary.iter().enumerate() {
            for (row_index, row) in column.iter().enumerate() {
                if row != &0 {
                    if row_index > 0 && column[row_index - 1] == 0 {
                        cross_check_sets[row_index - 1][column_index] = self.letter_set(column_index, row_index - 1);
                    }
                    if row_index < 14 && self.board.secondary[column_index][row_index + 1] == 0 {
                        cross_check_sets[row_index - 1][column_index] = self.letter_set(column_index, row_index - 1);
                    }
                }
            }
        }

        cross_check_sets
    }

    fn extend_right(
        &self,
        partial_word: &mut String,
        node: &Node,
        cross_check_sets: [u32; 15],
        row_index: usize,
        column_index: usize,
        start_row_index: usize,
        start_column_index: usize,
    ) {
        if column_index < 15 {
            if self.board.get(row_index, column_index) == 0 {
                if node.is_terminal {
                    println!(
                        "Word: {partial_word}, Start: ({}, {})",
                        start_row_index, start_column_index
                    );
                }
                for (edge, current_node) in node.children.iter() {
                    if (cross_check_sets[column_index] & (1 << edge - 1)) != 0 {
                        partial_word.push((*edge as Letter).to_char());
                        self.extend_right(
                            partial_word,
                            current_node,
                            cross_check_sets,
                            row_index,
                            column_index + 1,
                            start_row_index,
                            start_column_index,
                        );
                        partial_word.pop();
                    }
                }
            } else {
                let letter = self.board.get(row_index, column_index);
                if node.children.contains_key(&letter)
                    && (cross_check_sets[column_index] & (1 << letter - 1)) != 0
                {
                    partial_word.push((letter as Letter).to_char());
                    self.extend_right(
                        partial_word,
                        node.children.get(&letter).unwrap(),
                        cross_check_sets,
                        row_index,
                        column_index + 1,
                        start_row_index,
                        start_column_index,
                    );
                    partial_word.pop();
                }
            }
        } else {
            if node.is_terminal {
                println!(
                    "Word: {partial_word}, Start: ({}, {})",
                    start_row_index, start_column_index
                );
            }
        }
    }
    
    fn left_part(
        &self,
        partial_word: &str,
        node: &Node,
        cross_check_sets: [u32; 15],
        limit: u8,
        anchor_row_index: usize,
        anchor_column_index: usize,
        column_index: &mut usize,
        start_row_index: usize,
        start_column_index: usize,
    ) {
        self.extend_right(
            &mut partial_word.to_owned(),
            node,
            cross_check_sets,
            anchor_row_index,
            anchor_column_index,
            start_row_index,
            start_column_index,
        );
        if limit > 0 {
            for (edge, current_node) in node.children.iter() {
                if (cross_check_sets[*column_index] & (1 << edge - 1)) != 0 {
                    let mut partial_word = partial_word.to_owned();
                    partial_word.push((*edge as Letter).to_char());
                    *column_index -= 1;
                    self.left_part(
                        &partial_word,
                        current_node,
                        cross_check_sets,
                        limit - 1,
                        anchor_row_index,
                        anchor_column_index,
                        column_index,
                        start_row_index,
                        start_column_index,
                    );
                }
            }
        }
    }    

    fn solutions(&self) {
        let cross_check_sets = self.cross_check_sets();
        for (row_index, row) in self.board.primary.iter().enumerate() {
            let mut non_anchor_square_count: u8 = 0;
            let mut partial_word = String::new();
            for (column_index, column) in row.iter().enumerate() {
                let letter = (*column as Letter).to_char();
                match cross_check_sets[row_index][column_index] != 0 {
                    false => {
                        if self.board.get(row_index, column_index) != 0 {
                            partial_word.push(letter)
                        }
                        non_anchor_square_count += 1
                    },
                    true => {
                        if !partial_word.is_empty() {
                            let current_node = self.lexicon.root.get_node(partial_word.clone());
                            if let Some(current_node) = &current_node {
                                self.extend_right(
                                    &mut partial_word.clone(),
                                    current_node,
                                    cross_check_sets[row_index],
                                    row_index,
                                    column_index,
                                    row_index,
                                    column_index - partial_word.len(),
                                );
                            }
                            partial_word.clear();
                        } else {
                            self.left_part(
                                "",
                                &self.lexicon.root,
                                cross_check_sets[row_index],
                                non_anchor_square_count,
                                row_index,
                                column_index.clone(),
                                &mut column_index.clone(),
                                row_index,
                                column_index, // Use current column as starting column
                            );
                        }
                        non_anchor_square_count = 0
                    }
                };
            }
        }
    }

    fn play(&mut self, word: &str, row_index: usize, column_index: usize, across: bool) {
        if across {
            for (index, letter) in word.chars().enumerate() {
                self.board.primary[row_index][column_index + index] = Letter::from_char(letter);
                self.board.secondary[column_index + index][row_index] = Letter::from_char(letter);
            }
        } else {
            for (index, letter) in word.chars().enumerate() {
                self.board.primary[row_index + index][column_index] = Letter::from_char(letter);
                self.board.secondary[column_index][row_index + index] = Letter::from_char(letter);
            }
        }
    }
}

fn main() -> Result<(), String> {
    let mut game = Game::new();
    game.play("CAFES", 7, 6, true);
    game.play("KIF", 5, 8, false);
    game.play("HI", 6, 7, true);
    game.play("KAPPA", 5, 8, true);
    game.play("DELAYER", 2, 12, false);
    game.play("YUE", 6, 12, true);
    game.play("LUXER", 3, 14, false);
    game.play("EWE", 3, 10, true);
    game.play("VAR", 2, 8, true);
    game.play("MELIONS", 3, 2, true);
    game.play("EGO", 1, 6, false);
    game.play("DESOLER", 1, 1, true);
    game.play("LEVITERAI", 3, 4, false);
    game.play("ETAIERA", 8, 0, true);
    game.play("TENDUE", 7, 0, false);
    game.play("EUE", 12, 0, true);
    game.play("SAMOANS", 13, 2, true);
    game.play("SOIF", 14, 6, true);
    game.play("LIMITONS", 11, 3, true);
    game.play("ZEN", 9, 9, false);

    game.solutions();

    Ok(())
}