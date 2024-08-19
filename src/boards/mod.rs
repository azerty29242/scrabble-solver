use std::fmt::Display;

pub enum TileType {
    Normal,
    DoubleCharacter,
    TripleCharacter,
    DoubleWord,
    TripleWord,
}

pub struct Tile {
    letter: Option<char>,
    is_joker: bool,
    tile_type: TileType,
}

impl Tile {
    fn new(tile_type: TileType) -> Tile {
        Tile {
            letter: None,
            is_joker: false,
            tile_type,
        }
    }
}

pub struct Board {
    pub rows: Vec<Row>,
}

pub type Row = Vec<Tile>;

impl Board {
    fn new(board: Vec<Row>) -> Board {
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

pub fn load_board() -> Board {
    Board::new(vec![
        vec![
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
        vec![
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
        vec![
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
        vec![
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
        vec![
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
        vec![
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
        vec![
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
        vec![
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
        vec![
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
        vec![
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
        vec![
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
        vec![
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
        vec![
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
        vec![
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
        vec![
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
    ])
}
