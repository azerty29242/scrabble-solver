use std::collections::{self, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct LetterSet(u32);

impl LetterSet {
    fn new() -> LetterSet {
        LetterSet(0b11111111111111111111111111)
    }
}

impl LetterSet {
    fn remove_letter(&mut self, letter: char) {
        if self.has_letter(letter) {
            let letter_set = LetterSet::from(letter);
            self.0 &= !letter_set.0;
        }
    }

    fn has_letter(&self, letter: char) -> bool {
        self.0 & LetterSet::from(letter).0 != 0
    }

    fn get_letters(&self) -> Vec<char> {
        let mut letters = Vec::new();
        for i in 0..26 {
            if self.0 & (1 << i) != 0 {
                letters.push((b'A' + i) as char);
            }
        }
        letters
    }
}

impl From<char> for LetterSet {
    fn from(value: char) -> Self {
        let value = value.to_ascii_uppercase();
        if ('A'..='Z').contains(&value) {
            LetterSet(1 << (value as u32 - 'A' as u32))
        } else {
            LetterSet(0)
        }
    }
}

impl Clone for LetterSet {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for LetterSet {}

struct Game {
    dictionary: Node,
    primary_board: [[char; 15]; 15],
    secondary_board: [[char; 15]; 15],
    primary_cross_check_set: [[LetterSet; 15]; 15],
    secondary_cross_check_set: [[LetterSet; 15]; 15],
}

impl Game {
    fn new() -> Game {
        let file: File = match File::open(&"src/dictionaries/ods8.txt") {
            Ok(result) => result,
            Err(_) => panic!("The file couldn't be read")
        };
        let reader: BufReader<File> = BufReader::new(file);
        let mut tree: Node = Node::new(false);
        for line in reader.lines() {
            let line = line.unwrap();
            let mut last_node: &mut Node = &mut tree;
            for (index, character) in line.chars().enumerate() {
                if !last_node.contains_edge(&character) {
                    last_node.insert_edge(
                        character,
                        Node::new(match line.len() - index {
                            1 => true,
                            _ => false,
                        }),
                    )
                }
                last_node = last_node.get_node_mut(&character).unwrap();
            }
        }

        Game { dictionary: tree, primary_board: [[' '; 15]; 15], secondary_board: [[' '; 15]; 15], primary_cross_check_set: [[LetterSet::new(); 15]; 15], secondary_cross_check_set: [[LetterSet::new(); 15]; 15] }
    }
}

impl Game {
    fn play(&mut self, word: String, across: bool, row: usize, column: usize) {
        if across {
            for (index, letter) in word.chars().enumerate() {
                self.primary_board[row][column + index] = letter;
                self.secondary_board[14 - column - index][row] = letter;
            }
        } else {
            for (index, letter) in word.chars().enumerate() {
                self.primary_board[row + index][column] = letter;
                self.secondary_board[14 - column][row + index] = letter;
            }
        }
    
        self.update_cross_check_sets();
    }

    fn update_cross_check_sets(&mut self) {
        self.primary_cross_check_set = [[LetterSet::new(); 15]; 15];
        self.secondary_cross_check_set = [[LetterSet::new(); 15]; 15];
        let mut columns = [[' '; 15]; 15];
        for (row_index, row) in self.primary_board.iter().enumerate() {
            for (column_index, current_letter) in row.iter().enumerate() {
                columns[column_index][row_index] = current_letter.clone();
            }
        }
        for column in columns.iter() {
            let mut start_index = 0;
            while start_index < 15 {
                if column[start_index] == ' ' {
                    while start_index < 14 && column[start_index + 1] == ' ' {
                        start_index += 1;
                    }
                    if start_index >= 14 && column[start_index] == ' ' {
                        break;
                    }
                    let mut end_index = start_index + 1;
                    while end_index < 15 && column[end_index] != ' ' {
                        end_index += 1;
                    }
                    println!("{:?}", column[start_index..end_index].iter().collect::<Vec<&char>>());
                    start_index += 1;
                } else {
                    let mut space_index = start_index;
                    let mut end_index = start_index;
                    while space_index < 15 && column[space_index] != ' ' {
                        space_index += 1;
                        end_index += 1;
                    }
                    end_index += 1;
                    while end_index < 15 && column[end_index] != ' ' {
                        end_index += 1;
                    }
                    if !end_index >= 15 {
                        println!("{:?}", column[start_index..end_index].iter().collect::<Vec<&char>>());
                    }
                    start_index = space_index + 1;
                }
            }
        }
        // columns = [[' '; 15]; 15];
        // for (row_index, row) in self.secondary_board.iter().enumerate() {
        //     for (column_index, current_letter) in row.iter().enumerate() {
        //         columns[column_index][14 - row_index] = current_letter.clone();
        //     }
        // }
        // println!("{:?}", columns);
        // for (column_index, column) in columns.iter().enumerate() {
        //     let mut current_word: Vec<char> = Vec::new();
        //     let mut start_index: usize = 0;
        //     for (row_index, current_letter) in column.iter().enumerate() {
        //         if current_letter.clone() == ' ' || row_index == 14 {
        //             if !current_word.is_empty() {
        //                 if start_index != 0 {
        //                     start_index -= 1;
        //                     println!("{:?}, {}, {}, {}", current_word, column_index, start_index, row_index);
        //                     'tree: for letter in self.secondary_cross_check_set[start_index][column_index].get_letters().iter() {
        //                         let mut node = &self.dictionary.edges[letter];
        //                         for word_letter in current_word.iter() {
        //                             if node.contains_edge(word_letter) {
        //                                 node = node.get_node(word_letter).unwrap();
        //                             } else {
        //                                 self.secondary_cross_check_set[14 - start_index][column_index].remove_letter(letter.clone());
        //                                 continue 'tree;
        //                             }
        //                         }
        //                         if !node.is_terminal {
        //                             self.secondary_cross_check_set[14 -     start_index][column_index].remove_letter(letter.clone());
        //                         }
        //                     }
        //                 }
        //                 if row_index != 14 {
        //                     println!("{:?}, {}, {}, {}", current_word, column_index, start_index, row_index);
        //                     let mut node = &self.dictionary;
        //                     for word_letter in current_word.iter() {
        //                         node = node.get_node(word_letter).unwrap();
        //                     }
        //                     for letter in self.secondary_cross_check_set[14 - row_index][column_index].get_letters().iter() {
        //                         if node.contains_edge(letter) {
        //                             if node.get_node(letter).unwrap().is_terminal {
        //                                 continue;
        //                             }
        //                         }
        //                         self.secondary_cross_check_set[14 - row_index][column_index].remove_letter(letter.clone());
        //                     }
                            
        //                 }
        //                 current_word = Vec::new();
        //             }
        //         } else {
        //             if current_word.is_empty() {
        //                 start_index = row_index;
        //             }
        //             current_word.push(current_letter.clone());
        //         }
        //     }
        // }

    }
}

struct Node {
    edges: HashMap<char, Node>,
    is_terminal: bool,
}

impl Node {
    fn new(is_terminal: bool) -> Node {
        Node {
            edges: HashMap::new(),
            is_terminal,
        }
    }

    fn contains_edge(&self, letter: &char) -> bool {
        self.edges.contains_key(letter)
    }

    fn insert_edge(&mut self, letter: char, target: Node) {
        self.edges.insert(letter, target);
    }

    fn get_node(&self, letter: &char) -> Option<&Node> {
        self.edges.get(letter)
    }

    fn get_node_mut(&mut self, letter: &char) -> Option<&mut Node> {
        self.edges.get_mut(letter)
    }
}

fn main() {
    let mut game = Game::new();

    game.play(String::from("AGIREZ"), true, 7, 6);
    game.play(String::from("BOUGER"), false, 4, 7);
    game.play(String::from("BARRIERE"), true, 4, 7);
    game.play(String::from("CRI"), false, 3, 10);
    game.play(String::from("DIABOLOS"), true, 5, 1);
    game.play(String::from("REQUIN"), true, 9, 7);
    game.play(String::from("QUARTS"), false, 9, 9);





    println!("{:?}", game.primary_board);
    println!("{:?}", game.secondary_board);
    for (row_index, row) in game.primary_cross_check_set.iter().enumerate() {
        for (column_index, column) in row.iter().enumerate() {
            if column.0 != (1 << 26) - 1 {
                println!("---------------------------------------------");
                println!("Column: {}", column_index + 1);
                println!("Row: {}", row_index + 1);
                println!("Letters: {:?}", column.get_letters());
            }
        }
    }
    for (row_index, row) in game.secondary_cross_check_set.iter().enumerate() {
        for (column_index, column) in row.iter().enumerate() {
            if column.0 != (1 << 26) - 1 {
                println!("---------------------------------------------");
                println!("Column: {}",  column_index + 1);
                println!("Row: {}", row_index + 1);
                println!("Letters: {:?}", column.get_letters());
            }
        }
    }
}