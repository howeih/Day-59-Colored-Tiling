extern crate rand;
use rand::{Rng, thread_rng};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Color {
    Uncolored,
    Blue,
    Green,
}

impl Color {
    fn get_rand_color() -> Color {
        let mut rng = thread_rng();
        let rand_color = rng.gen_range(0, 2);
        match rand_color {
            0 => Color::Blue,
            _ => Color::Green,
        }
    }

    fn get_next_color(current_color: &Color) -> Color {
        match current_color {
            Color::Uncolored => Color::Blue,
            Color::Blue => Color::Green,
            Color::Green => Color::Uncolored
        }
    }
}

#[derive(Debug)]
struct Wall {
    tiles: Vec<Vec<Color>>,
    rows: usize,
    cols: usize
}

impl Wall {
    fn init_wall_color(rows: usize, cols: usize, tiles: &mut Vec<Vec<Color>>) {
        for r in (1..rows-1).step_by(2) {
            tiles[r][0] = Color::get_rand_color();
            tiles[r+1][cols - 1] = Color::get_rand_color();
        }

        for c in (1..cols-1).step_by(2) {
            tiles[rows - 1][c] = Color::get_rand_color();
            tiles[0][c+1] = Color::get_rand_color();
        }
    }

    fn new(rows: usize, cols: usize) -> Wall {
        let mut tiles = Vec::<Vec<Color>>::new();
        for _ in 0..rows {
            let mut tile_row = Vec::<Color>::new();
            for _ in 0..cols {
                tile_row.push(Color::Uncolored);
            }
            tiles.push(tile_row);
        }
        Wall::init_wall_color(rows, cols, &mut tiles);
        Wall { tiles, rows, cols }
    }

    fn check_wall(&self, i: usize, j: usize) -> bool {
        let mut result = true;
        if i == 1 && j % 2 != 0 {
            if self.tiles[i][j] != self.tiles[i - 1][j + 1] {
                result = false;
            }
        }

        if j == 1 && i % 2 == 0 {
            if self.tiles[i][j] != self.tiles[i - 1][j - 1] {
                result = false;
            }
        }

        if j == self.cols - 2 && i % 2 == 1 {
            if self.tiles[i][j] != self.tiles[i + 1][j + 1] {
                result = false;
            }
        }

        if i == self.rows - 2 && j % 2 == 0 {
            if self.tiles[i][j] != self.tiles[i + 1][j - 1] {
                result = false;
            }
        }
        result
    }

    fn check_tiles(&self, i: usize, j: usize) -> bool {
        let mut result = true;
        if self.tiles[i][j] == Color::Uncolored {
           return false;
        }
        if i > 0 && i < self.rows - 1 && j > 0 && j < self.cols - 1 {
            if j % 2 == 0 && i % 2 != 0 {
                let tile_color = self.tiles[i + 1][j + 1];
                if tile_color != Color::Uncolored && self.tiles[i][j] != self.tiles[i + 1][j + 1] {
                    result = false;
                }
            }

            if j % 2 == 0 && i % 2 != 0 {
                let tile_color = self.tiles[i + 1][j - 1];
                if tile_color != Color::Uncolored  && self.tiles[i][j] != self.tiles[i + 1][j - 1] {
                    result = false;
                }
            }
        }
        result
    }
    fn check(&self, i: usize, j: usize) -> bool {
        self.check_tiles(i, j) & self.check_wall(i, j)
    }

    fn solve(&mut self, i: usize, j: usize) {
        if i <= self.rows - 2 && i >=1 && j <= self.cols - 2 && j >=1{
            let next_color = Color::get_next_color(&self.tiles[i][j]);
            self.tiles[i][j] = next_color;
            let check_result = self.check(i,j);
            if check_result {
                if j >= self.cols - 2 {
                    self.solve(i + 1, 1);
                } else {
                    self.solve(i, j + 1);
                }
            } else {
                if next_color != Color::Uncolored {
                    self.solve(i,j);
                } else{
                    if j <= 1 {
                        self.solve(i - 1, self.cols - 2);
                    } else {
                        self.solve(i, j - 1);
                    }
                }
            }
        }
    }

    fn print_wall(&self) {
        for r in 0..self.rows {
            for c in 0..self.cols {
                match self.tiles[r][c] {
                    Color::Uncolored => print!("U "),
                    Color::Blue => print!("B "),
                    Color::Green => print!("G "),
                }
            }
            println!();
        }
    }
}

fn main() {
    let mut wall = Wall::new(12, 12);
    wall.solve(1, 1);
    wall.print_wall();

}
