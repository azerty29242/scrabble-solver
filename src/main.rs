mod boards;
mod dictionaries;
mod display;
mod tiles;
mod tree;

use boards::load_board;
use dictionaries::load_dictionary;
use display::clear_screen;
use display::get_input_line;
use tiles::load_tiles;
use tree::Node;

fn main() {
    clear_screen();

    let tree = load_dictionary();
    let board = load_board();
    let tiles = load_tiles();

    clear_screen();

    println!("{board}");

    loop {
        let word = get_input_line(format!("What are the letters in your rack: "));
        let mut rack = word.chars().collect::<Vec<char>>();
        let mut partial_word = String::new();
        let mut limit: usize = 7;
        let mut results: Vec<String> = Vec::new();
        left_part(
            &mut partial_word,
            &tree,
            &mut rack,
            &mut limit,
            &mut results,
        );
        println!("{results:?}");
    }
}

fn left_part(
    partial_word: &mut String,
    node: &Node,
    rack: &mut Vec<char>,
    limit: &mut usize,
    results: &mut Vec<String>,
) {
    extend_right(partial_word, node, rack, results);
    if limit.clone() > 0 {
        for letter in node.edges.keys() {
            if rack.contains(letter) {
                rack.remove(
                    rack.iter()
                        .position(|rack_letter| rack_letter == letter)
                        .unwrap(),
                );
                let node = node.get_node(letter).unwrap();
                partial_word.push(letter.clone());
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
            partial_word.push(letter.clone());
            extend_right(partial_word, node, rack, results);
            partial_word.pop();
            rack.push(letter.clone());
        }
    }
}
