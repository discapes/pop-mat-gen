use std::fmt::Display;

enum Tile {
    Empty,
    City(usize),
    Mountain,
    Swamp,
    River,
    Road,
}

impl Tile {
    pub fn from_string(str: &str) -> Tile {
        match str {
            "" => Tile::Empty,
            "U" => Tile::Mountain,
            "N" => Tile::Swamp,
            "J" => Tile::River,
            "T" => Tile::Road,
            _ if str.contains("X") => Tile::City(str.chars().filter(|c| *c == 'X').count()),
            _ => panic!("Invalid tile: {str}"),
        }
    }
    pub fn habitability(&self) -> f32 {
        match self {
            Tile::Empty => 1.0,
            Tile::Mountain => 0.0,
            Tile::Swamp => 0.3,
            Tile::River => 0.0,
            Tile::Road => 1.3,
            Tile::City(n) => *n as f32 * 2.0,
        }
    }
}

fn normalize_to_sum_one(vec: &mut Vec<f32>) {
    let total: f32 = vec.iter().sum();
    for e in vec {
        *e /= total;
    }
}

fn mat_col_for_city(pos: (usize, usize), map: &Vec<Vec<Tile>>) -> Vec<f32> {
    let this = &map[pos.0][pos.1];
    let mut out: Vec<f32> = Vec::with_capacity(map[0].len() * map.len());

    for (n_row, row) in map.iter().enumerate() {
        for (n_col, other) in row.iter().enumerate() {
            if (pos.0, pos.1) == (n_row, n_col) {
                out.push(this.habitability() * 10.0);
            } else {
                let dist = f32::sqrt(
                    ((pos.0 as i32 - n_row as i32).pow(2) + (pos.1 as i32 - n_col as i32).pow(2))
                        as f32,
                );
                out.push((other.habitability() - 0.1 * dist.powf(2.0)).max(0.0));
            }
        }
    }
    normalize_to_sum_one(&mut out);
    out
}

fn print_cols<T: Display>(cols: Vec<Vec<T>>) {
    for n_row in 0..cols[0].len() {
        for n_col in 0..cols.len() {
            print!("{}\t", cols[n_col][n_row])
        }
        println!()
    }
}

fn main() -> () {
    let map_bytes = include_bytes!("map.txt");
    let map_str = String::from_utf8_lossy(map_bytes);
    let map: Vec<Vec<Tile>> = map_str
        .split("\n")
        .map(|row| row.split("\t").map(Tile::from_string).collect())
        .collect();

    // check
    if !map.iter().all(|row| row.len() == map[0].len()) {
        panic!("Map is not a rectangle!");
    }

    let mat_cols: Vec<Vec<f32>> = map
        .iter()
        .enumerate()
        .flat_map(|(n_row, row)| {
            let map = &map;
            // in order to use n_row, we need to capture it by moving
            // we don't want to capture map by moving though, so we make it a reference
            // see https://stackoverflow.com/questions/67230394/can-i-capture-some-things-by-reference-and-others-by-value-in-a-closure
            row.iter()
                .enumerate()
                .map(move |(n_col, _)| mat_col_for_city((n_row, n_col), map))
        })
        .collect();

    print_cols(mat_cols);
}
