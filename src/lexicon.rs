use crate::letter::{FromChar, Letter};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Node {
    pub is_terminal: bool,
    pub children: HashMap<Letter, Node>,
}

impl Node {
    fn new(is_terminal: bool) -> Node {
        Node {
            is_terminal,
            children: HashMap::new(),
        }
    }

    pub fn get_node(&self, partial_word: &str) -> Option<&Node> {
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

pub struct Lexicon {
    pub root: Node,
}

impl Lexicon {
    pub fn new() -> Lexicon {
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
