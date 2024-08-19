use crate::display::{get_input_line, hide_cursor, show_cursor, WindowSize};
use crate::tree::Node;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Language<'a> {
    pub name: &'a str,
    pub dictionaries: Vec<Dictionary>,
}

impl<'a> Language<'a> {
    fn new(name: &'a str, dictionaries: Vec<&'a str>) -> Language<'a> {
        Language {
            name,
            dictionaries: dictionaries
                .iter()
                .map(|dictionary| Dictionary::new(name, dictionary))
                .collect::<Vec<Dictionary>>(),
        }
    }
}

impl<'a> Debug for Language<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "name: {}, dictionaries: {:?}",
            self.name, self.dictionaries
        )
    }
}

pub struct Dictionary {
    pub name: String,
    pub path: String,
}

impl<'a> Dictionary {
    fn new(language: &str, dictionary: &str) -> Dictionary {
        Dictionary {
            name: dictionary.to_string(),
            path: "src/dictionaries/".to_string() + language + "/" + dictionary + ".txt",
        }
    }
}

impl Debug for Dictionary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "name: {}, path: {}", self.name, self.path)
    }
}

pub fn load_dictionary() -> Node {
    println!("Here is a list of the available dictionaries :");
    let english = Language::new("English", vec!["Example"]);
    let french = Language::new("French", vec!["ODS 8"]);
    let dictionaries = vec![english, french];
    let num_languages = dictionaries.len();
    dictionaries
        .iter()
        .enumerate()
        .for_each(|(index, language)| {
            let (before_name, is_last): (&str, bool) = match (index, num_languages - index) {
                (0, _) => ("┏━ ", false),
                (_, 1) => ("┗━ ", true),
                _ => ("┣━ ", false),
            };
            println!("{}{}", before_name, language.name);
            let num_dictionaries = language.dictionaries.len();
            language
                .dictionaries
                .iter()
                .enumerate()
                .for_each(|(index, dictionary)| {
                    let before_name = match (num_dictionaries - index, is_last) {
                        (1, false) => "┃  ┗━ ",
                        (1, true) => "   ┗━ ",
                        (_, false) => "┃  ┣━ ",
                        (_, true) => "   ┣━ ",
                    };
                    println!("{}{} ({})", before_name, dictionary.name, dictionary.path)
                });
        });
    let path = get_input_line("Enter the path of your dictionary : ".to_string());
    let file: File = File::open(&path).expect("Couldn't open the file.");
    let reader: BufReader<File> = BufReader::new(file);
    let line_count = reader.lines().count();
    let file: File = File::open(&path).expect("Couldn't open the file.");
    let reader: BufReader<File> = BufReader::new(file);
    let mut tree: Node = Node::new(false);
    let mut index: usize = 0;
    for line in reader.lines() {
        hide_cursor();
        print!("Loading dictionary ({index}/{line_count})\r");
        let line = line.expect("Couldn't read line.");
        let mut last_node: &mut Node = &mut tree;
        let line_length: usize = line.len();
        for (index, character) in line.chars().enumerate() {
            if !last_node.contains_edge(&character) {
                last_node.insert_edge(
                    character,
                    Node::new(match line_length - index {
                        1 => true,
                        _ => false,
                    }),
                )
            }

            last_node = last_node.get_node_mut(&character).unwrap();
        }
        index += 1;
    }
    show_cursor();
    let mut window_size = WindowSize::new();
    window_size.update();
    print!("{}\r", " ".repeat(window_size.columns as usize));
    println!("Loading dictionary ✓");

    tree
}
