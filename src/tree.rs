use std::collections::HashMap;

#[derive(Debug)]
pub struct Node {
    pub edges: HashMap<char, Node>,
    pub is_terminal: bool,
}

impl Node {
    pub fn new(is_terminal: bool) -> Node {
        Node {
            edges: HashMap::new(),
            is_terminal,
        }
    }

    pub fn contains_edge(&self, letter: &char) -> bool {
        self.edges.contains_key(letter)
    }

    pub fn insert_edge(&mut self, letter: char, target: Node) {
        self.edges.insert(letter, target);
    }

    pub fn get_node(&self, letter: &char) -> Option<&Node> {
        self.edges.get(letter)
    }

    pub fn get_node_mut(&mut self, letter: &char) -> Option<&mut Node> {
        self.edges.get_mut(letter)
    }

    // pub fn find_node(&self, word: &String) -> (bool, &Node) {
    //     let mut last_node: &Node = self;
    //     let mut characters = word.chars();
    //     loop {
    //         let next_iteration = characters.next();
    //         if next_iteration == None {
    //             break (true, last_node);
    //         }
    //         let character =
    //             next_iteration.expect("An unexpected error occured while iterating through a word");
    //         if last_node.contains_edge(&character) {
    //             last_node = last_node.get_node(&character).unwrap();
    //         } else {
    //             break (false, last_node);
    //         }
    //     }
    // }

    // pub fn find_node_mut(&mut self, word: &String) -> (bool, &mut Node) {
    //     let mut last_node: &mut Node = self;
    //     let mut characters = word.chars();
    //     loop {
    //         let next_iteration = characters.next();
    //         if next_iteration == None {
    //             break (true, last_node);
    //         }
    //         let character = next_iteration.unwrap();
    //         if last_node.contains_edge(&character) {
    //             last_node = last_node.get_node_mut(&character).unwrap();
    //         } else {
    //             break (false, last_node);
    //         }
    //     }
    // }

    // pub fn list_nodes(
    //     &self,
    //     current_word: &mut String,
    //     available_letters: &mut HashMap<char, usize>,
    //     results: &mut Vec<(String, usize)>,
    //     tiles: &HashMap<char, usize>,
    // ) {
    //     if self.is_terminal {
    //         let mut score: usize = 0;
    //         for character in current_word.chars() {
    //             score += tiles.get(&character).unwrap();
    //         }
    //         results.push((current_word.to_string(), score));
    //     }
    //     for letter in available_letters.clone().keys() {
    //         if self.contains_edge(&letter) {
    //             let node = self.get_node(&letter).unwrap();
    //             current_word.push(letter.clone());
    //             available_letters
    //                 .entry(letter.clone())
    //                 .and_modify(|num_letters| *num_letters -= 1);
    //             if *available_letters.get(letter).unwrap() == 0 as usize {
    //                 available_letters.remove(letter);
    //             }
    //             node.list_nodes(current_word, available_letters, results, tiles);
    //             available_letters
    //                 .entry(letter.clone())
    //                 .and_modify(|num_letters| *num_letters += 1)
    //                 .or_insert(1);
    //             current_word.pop();
    //         }
    //     }
    // }
}
