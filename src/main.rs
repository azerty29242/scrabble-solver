mod boards;
mod display;
mod tree;

use display::clear_screen;
use display::read_line;
use std::env;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};
use tree::Node;

#[derive(Debug)]
struct Tile {
    letter: Option<char>,
    cross_check_set: Vec<char>,
}

impl Tile {
    fn new(letter: Option<char>) -> Tile {
        Tile {
            letter: None,
            cross_check_set: Vec::new(),
        }
    }
}

#[derive(Debug)]
struct Board {
    rows: [[Tile]],
}

impl Board {
    fn new(board: [[Tile]]) -> Board {
        Board { rows: board }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        result += &"━".repeat(61);
        result += "\n";
        for row in self.rows.iter() {
            result += "┃";
            for tile in row.iter() {
                result += match tile.tile_type {
                    TileType::Normal => " ",
                    TileType::DoubleCharacter => "\x1b[46m ",
                    TileType::TripleCharacter => "\x1b[44m ",
                    TileType::DoubleWord => "\x1b[45m ",
                    TileType::TripleWord => "\x1b[41m ",
                };
                result.push(tile.letter.unwrap_or(' '));
                result += " \x1b[0m┃"
            }
            result += "\n";
            result += &"━".repeat(61);
            result += "\n";
        }
        write!(f, "{result}")
    }
}

fn load_dictionary() -> Node {
    let file: File = File::open(&"src/dictionaries/ods8.txt").expect("Couldn't open the file.");
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
    tree
}

fn left_part(
    partial_word: &mut String,
    node: &Node,
    rack: &mut Vec<char>,
    mut limit: isize,
    results: &mut Vec<String>,
) {
    extend_right(partial_word, node, rack, results);
    if limit > 0 {
        for letter in node.edges.keys() {
            if rack.contains(letter) {
                rack.remove(
                    rack.iter()
                        .position(|rack_letter| rack_letter == letter)
                        .unwrap(),
                );
                let node = node.get_node(letter).unwrap();
                partial_word.push(letter.clone());

                limit -= 1;
                left_part(partial_word, node, rack, limit, results);
                partial_word.pop();
                rack.push(letter.clone());
            }
        }
    }
}

fn extend_right(
    partial_word: &mut String,
    node: &Node,
    rack: &mut Vec<char>,
    results: &mut Vec<String>,
) {
    if node.is_terminal {
        results.push(partial_word.clone());
    }
    for letter in node.edges.keys() {
        if rack.contains(letter) {
            rack.remove(
                rack.iter()
                    .position(|rack_letter| rack_letter == letter)
                    .unwrap(),
            );
            let node = node.get_node(letter).unwrap();
            partial_word.push(letter.clone().to_ascii_lowercase());
            extend_right(partial_word, node, rack, results);
            partial_word.pop();
            rack.push(letter.clone());
        }
    }
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    clear_screen();

    let tree = load_dictionary();
    let board = Board::new([
        [
            Tile::new(TileType::TripleWord),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::DoubleCharacter),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::TripleWord),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::DoubleCharacter),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::TripleWord),
        ],
        [
            Tile::new(TileType::Normal),
            Tile::new(TileType::DoubleWord),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::TripleCharacter),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::TripleCharacter),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::DoubleWord),
            Tile::new(TileType::Normal),
        ],
        [
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::DoubleWord),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::DoubleCharacter),
            Tile::new(TileType::Normal),
            Tile::new(TileType::DoubleCharacter),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::DoubleWord),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
        ],
        [
            Tile::new(TileType::DoubleCharacter),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::DoubleWord),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::DoubleCharacter),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::DoubleWord),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::DoubleCharacter),
        ],
        [
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::DoubleWord),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::DoubleWord),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
        ],
        [
            Tile::new(TileType::Normal),
            Tile::new(TileType::TripleCharacter),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::TripleCharacter),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::TripleCharacter),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::TripleCharacter),
            Tile::new(TileType::Normal),
        ],
        [
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::DoubleCharacter),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::DoubleCharacter),
            Tile::new(TileType::Normal),
            Tile::new(TileType::DoubleCharacter),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::DoubleCharacter),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
        ],
        [
            Tile::new(TileType::TripleWord),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::DoubleCharacter),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::DoubleWord),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::DoubleCharacter),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::TripleWord),
        ],
        [
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::DoubleCharacter),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::DoubleCharacter),
            Tile::new(TileType::Normal),
            Tile::new(TileType::DoubleCharacter),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::DoubleCharacter),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
        ],
        [
            Tile::new(TileType::Normal),
            Tile::new(TileType::TripleCharacter),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::TripleCharacter),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::TripleCharacter),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::TripleCharacter),
            Tile::new(TileType::Normal),
        ],
        [
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::DoubleWord),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::DoubleWord),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
        ],
        [
            Tile::new(TileType::DoubleCharacter),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::DoubleWord),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::DoubleCharacter),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::DoubleWord),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::DoubleCharacter),
        ],
        [
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::DoubleWord),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::DoubleCharacter),
            Tile::new(TileType::Normal),
            Tile::new(TileType::DoubleCharacter),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::DoubleWord),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
        ],
        [
            Tile::new(TileType::Normal),
            Tile::new(TileType::DoubleWord),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::TripleCharacter),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::TripleCharacter),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::DoubleWord),
            Tile::new(TileType::Normal),
        ],
        [
            Tile::new(TileType::TripleWord),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::DoubleCharacter),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::TripleWord),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::DoubleCharacter),
            Tile::new(TileType::Normal),
            Tile::new(TileType::Normal),
            Tile::new(TileType::TripleWord),
        ],
    ]);

    clear_screen();

    println!("{:#?}", board.rows[6..9]);

    loop {
        let word = read_line(format!("What are the letters in your rack: "));
        let mut rack = word.chars().collect::<Vec<char>>();
        let mut partial_word = String::new();
        let mut results: Vec<String> = Vec::new();
        left_part(&mut partial_word, &tree, &mut rack, 7, &mut results);
        println!("{results:?}");
    }
}
