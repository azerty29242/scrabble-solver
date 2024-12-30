use std::{
    collections::HashMap,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
};

type Letter = u8;

pub struct Board {
    primary: [[Letter; 15]; 15],
    secondary: [[Letter; 15]; 15],
}

impl Board {
    pub fn new() -> Board {
        Board {
            primary: [[0; 15]; 15],
            secondary: [[0; 15]; 15],
        }
    }

    pub fn get(&self, row_index: usize, column_index: usize) -> Letter {
        self.primary[row_index][column_index]
    }

    pub fn rotate(&mut self) {
        (self.primary, self.secondary) = (self.secondary, self.primary);
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "┌───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┐"
        )?;
        for (row_index, row) in self.primary.iter().enumerate() {
            write!(f, "│")?;
            for &cell in row.iter() {
                write!(f, " {} │", cell.to_char())?;
            }
            writeln!(f)?;

            if row_index < 14 {
                writeln!(
                    f,
                    "├───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┤"
                )?;
            } else {
                writeln!(
                    f,
                    "└───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┘"
                )?;
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
        }
        .unwrap()
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
    children: HashMap<Letter, Node>,
}

impl Node {
    fn new(is_terminal: bool) -> Node {
        Node {
            is_terminal,
            children: HashMap::new(),
        }
    }

    fn get_node(&self, partial_word: &str) -> Option<&Node> {
        let mut current_node = self;
        for letter in partial_word.chars() {
            if current_node
                .children
                .contains_key(&Letter::from_char(letter))
            {
                current_node = current_node
                    .children
                    .get(&Letter::from_char(letter))
                    .unwrap();
            } else {
                return None;
            }
        }
        Some(current_node)
    }
}

#[derive(Debug)]
struct Lexicon {
    root: Node,
}

impl Lexicon {
    fn new() -> Lexicon {
        Lexicon {
            root: Node::new(false),
        }
    }

    pub fn from_file(path: &str) -> Self {
        let file = File::open(path).unwrap_or_else(|_| {
            panic!(
                "Failed to open the lexicon file at '{}'. Ensure the file exists and is readable.",
                path
            );
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

struct Coordinates {
    row_index: usize,
    column_index: usize,
}

impl Coordinates {
    fn new() -> Coordinates {
        Coordinates {
            row_index: 0,
            column_index: 0,
        }
    }
}

struct PartialWord {
    start: Coordinates,
    string: String,
}

impl PartialWord {
    fn new() -> PartialWord {
        PartialWord {
            string: String::new(),
            start: Coordinates::new(),
        }
    }
}

struct Game {
    board: Board,
    lexicon: Lexicon,
    cross_check_sets: [[u32; 15]; 15],
    anchors: [u16; 15],
    rack: HashMap<Letter, usize>,
}

impl Game {
    fn new() -> Game {
        Game {
            board: Board::new(),
            lexicon: Lexicon::from_file("src/dictionaries/ods8.txt"),
            cross_check_sets: [[67108863; 15]; 15],
            anchors: [0; 15],
            rack: HashMap::new(),
        }
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
        if let Some(node) = self.lexicon.root.get_node(&before_anchor) {
            for (edge, current_node) in node.children.iter() {
                if let Some(final_node) = current_node.get_node(&after_anchor) {
                    if final_node.is_terminal {
                        letter_set |= 1 << edge - 1;
                    }
                }
            }
        }

        letter_set
    }

    fn calculate_anchors(&mut self) {
        self.anchors = [0; 15];

        for (row_index, row) in self.board.primary.iter().enumerate() {
            for (column_index, column) in row.iter().enumerate() {
                if column != &0 {
                    if column_index > 0 && row[column_index - 1] == 0 {
                        self.anchors[row_index] |= 1 << column_index - 1;
                    }
                    if column_index < 14 && row[column_index + 1] == 0 {
                        self.anchors[row_index] |= 1 << column_index + 1;
                    }
                }
            }
        }

        for (column_index, column) in self.board.secondary.iter().enumerate() {
            for (row_index, row) in column.iter().enumerate() {
                if row != &0 {
                    if row_index > 0 && column[row_index - 1] == 0 {
                        self.anchors[row_index - 1] |= 1 << column_index;
                    }
                    if row_index < 14 && column[row_index + 1] == 0 {
                        self.anchors[row_index + 1] |= 1 << column_index;
                    }
                }
            }
        }
    }

    fn calculate_cross_check_sets(&mut self) {
        self.cross_check_sets = [[67108863; 15]; 15];

        for (column_index, column) in self.board.secondary.iter().enumerate() {
            for (row_index, row) in column.iter().enumerate() {
                if row != &0 {
                    if row_index > 0 && column[row_index - 1] == 0 {
                        self.cross_check_sets[row_index - 1][column_index] =
                            self.letter_set(column_index, row_index - 1);
                    }
                    if row_index < 14 && column[row_index + 1] == 0 {
                        self.cross_check_sets[row_index + 1][column_index] =
                            self.letter_set(column_index, row_index + 1);
                    }
                }
            }
        }
    }

    fn extend_right(&self, partial_word: &mut PartialWord, node: &Node, possible: bool) {
        let current_row_index = partial_word.start.row_index;
        let current_column_index = partial_word.start.column_index + partial_word.string.len();
        if current_column_index >= 15 {
            if node.is_terminal {
                println!(
                    "Word: {}, row: {}, column: {}",
                    partial_word.string,
                    partial_word.start.row_index,
                    partial_word.start.column_index
                );
                return;
            }
        } else {
            let letter = self.board.get(current_row_index, current_column_index);
            if self.board.get(current_row_index, current_column_index) == 0 {
                if node.is_terminal && possible {
                    println!(
                        "Word: {}, row: {}, column: {}",
                        partial_word.string,
                        partial_word.start.row_index,
                        partial_word.start.column_index
                    );
                }
                for (letter, current_node) in node.children.iter() {
                    if (self.cross_check_sets[current_row_index][current_column_index]
                        & (1 << *letter - 1))
                        != 0
                    {
                        partial_word.string.push((*letter as Letter).to_char());
                        self.extend_right(partial_word, current_node, true);
                        partial_word.string.pop();
                    }
                }
            } else {
                if let Some(node) = node.children.get(&letter) {
                    partial_word.string.push((letter as Letter).to_char());
                    self.extend_right(partial_word, node, true);
                    partial_word.string.pop();
                }
            }
        }
    }

    fn left_part(&self, partial_word: &mut PartialWord, node: &Node, limit: u8) {
        self.extend_right(partial_word, node, false);
        if limit > 0 {
            partial_word.start.column_index -= 1;
            for (letter, current_node) in node.children.iter() {
                partial_word.string.push((*letter as Letter).to_char());
                self.left_part(partial_word, current_node, limit - 1);
                partial_word.string.pop();
            }
            partial_word.start.column_index += 1;
        }
    }

    fn solutions(&mut self) {
        self.calculate_anchors();
        self.calculate_cross_check_sets();
        for (row_index, row) in self.board.primary.iter().enumerate() {
            let mut non_anchor_square_count: u8 = 0;
            let mut partial_word = PartialWord::new();
            for (column_index, column) in row.iter().enumerate() {
                let letter = (*column as Letter).to_char();
                match (self.anchors[row_index] & (1 << column_index)) != 0 {
                    false => {
                        if self.board.get(row_index, column_index) != 0 {
                            partial_word.string.push(letter);
                        }
                        non_anchor_square_count += 1
                    }
                    true => {
                        partial_word.start.row_index = row_index;
                        partial_word.start.column_index = column_index - partial_word.string.len();
                        if !partial_word.string.is_empty() {
                            let current_node = self.lexicon.root.get_node(&partial_word.string);
                            if let Some(current_node) = current_node {
                                self.extend_right(&mut partial_word, current_node, false);
                            }
                            partial_word.string.clear();
                        } else {
                            self.left_part(
                                &mut partial_word,
                                &self.lexicon.root,
                                non_anchor_square_count,
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
    let mut game: Game = Game::new();
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
    /*
        game.rack.insert(2, 1);
        game.rack.insert(3, 1);
        game.rack.insert(7, 1);
        game.rack.insert(8, 1);
        game.rack.insert(9, 1);
        game.rack.insert(14, 1);
        game.rack.insert(21, 1);
    */
    game.solutions();

    Ok(())
}
