use std::collections::HashMap;

pub fn load_tiles() -> HashMap<char, usize> {
    let mut tiles = HashMap::new();
    tiles.insert('A', 1);
    tiles.insert('B', 3);
    tiles.insert('C', 3);
    tiles.insert('D', 2);
    tiles.insert('E', 1);
    tiles.insert('F', 4);
    tiles.insert('G', 2);
    tiles.insert('H', 4);
    tiles.insert('I', 1);
    tiles.insert('J', 8);
    tiles.insert('K', 10);
    tiles.insert('L', 1);
    tiles.insert('M', 2);
    tiles.insert('N', 1);
    tiles.insert('O', 1);
    tiles.insert('P', 3);
    tiles.insert('Q', 8);
    tiles.insert('R', 1);
    tiles.insert('S', 1);
    tiles.insert('T', 1);
    tiles.insert('U', 1);
    tiles.insert('V', 4);
    tiles.insert('W', 10);
    tiles.insert('X', 10);
    tiles.insert('Y', 10);
    tiles.insert('Z', 10);
    tiles
}
