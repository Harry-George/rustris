use rand::thread_rng;
use rand::seq::SliceRandom;

struct Tetronimo {
    // 4x4 grid
    blocks: [[bool; 4]; 4],
    x: usize,
    y: usize,
}

impl Tetronimo {
    fn new(blocks: [[bool; 4]; 4]) -> Tetronimo {
        let mut tetronimo = Tetronimo {
            blocks,
            x: 0,
            y: 0,
        };
        tetronimo.normalise();

        // I made them all upside down so this is easier
        tetronimo.rotate();
        tetronimo.rotate();

        return tetronimo;
    }

    fn draw(&self, mut string: String) -> String
    {
        for row in self.blocks.iter().rev() {
            for block in row.iter() {
                if *block {
                    string += "x ";
                } else {
                    string += "- ";
                }
            }
            // remove trailing space
            string.pop();

            string += "\n";
        }
        string += "\n";
        return string;
    }

    fn rotate(&mut self) {
        let mut new_blocks = [[false; 4]; 4];
        for x in 0..4 {
            for y in 0..4 {
                new_blocks[x][y] = self.blocks[3 - y][x];
            }
        }
        self.blocks = new_blocks;

        self.normalise();
    }

    fn normalise(&mut self) {
        // shift to top left

        // row
        while !self.blocks[0][0] && !self.blocks[0][1] && !self.blocks[0][2] && !self.blocks[0][3] {
            self.blocks[0] = self.blocks[1];
            self.blocks[1] = self.blocks[2];
            self.blocks[2] = self.blocks[3];
            self.blocks[3] = [false; 4];
        }

        // column
        while !self.blocks[0][0] && !self.blocks[1][0] && !self.blocks[2][0] && !self.blocks[3][0] {
            for x in 0..4 {
                for y in 0..3 {
                    self.blocks[x][y] = self.blocks[x][y + 1];
                }
                self.blocks[x][3] = false;
            }
        }
    }

    fn move_left(&mut self) {
        self.x -= 1;
    }

    fn move_right(&mut self) {
        self.x += 1;
    }

    fn move_down(&mut self) {
        self.y += 1;
    }

    fn move_up(&mut self) {
        self.y -= 1;
    }
}


fn generate_batch() -> Vec<Tetronimo> {
    let mut batch = Vec::new();
    // - x -
    // x x x
    let f = false;
    let t = true;
    batch.push(Tetronimo::new(
        [[f, f, f, f],
            [f, f, f, f],
            [f, t, f, f],
            [t, t, t, f]]));


    // x - -
    // x x x

    batch.push(Tetronimo::new(
        [[f, f, f, f],
            [f, f, f, f],
            [t, f, f, f],
            [t, t, t, f]]));


    // - - x
    // x x x

    batch.push(Tetronimo::new(
        [[f, f, f, f],
            [f, f, f, f],
            [f, f, t, f],
            [t, t, t, f]]));

    // x x
    // x x

    batch.push(Tetronimo::new(
        [[f, f, f, f],
            [f, f, f, f],
            [t, t, f, f],
            [t, t, f, f]]));


    // - x x
    // x x -

    batch.push(Tetronimo::new(
        [[f, f, f, f],
            [f, f, f, f],
            [f, t, t, f],
            [t, t, f, f]]));


    // x x -
    // - x x

    batch.push(Tetronimo::new(
        [[f, f, f, f],
            [f, f, f, f],
            [t, t, f, f],
            [f, t, t, f]]));


    // x x x x

    batch.push(Tetronimo::new(
        [[f, f, f, f],
            [f, f, f, f],
            [f, f, f, f],
            [t, t, t, t]]));


    return batch;
}

// Path: src/main.rs

#[test]
fn test_rotate() {
    let mut batch = generate_batch();

    let mut result: String = String::new();
    for tetronimo in batch.iter_mut() {
        for _ in 0..4 {
            result = tetronimo.draw(result);
            tetronimo.rotate();
        }
        result += "-------\n";
    }

    let expected = "\
- - - -
- - - -
- x - -
x x x -

- - - -
- x - -
x x - -
- x - -

- - - -
- - - -
x x x -
- x - -

- - - -
x - - -
x x - -
x - - -

-------
- - - -
- - - -
- - x -
x x x -

- - - -
x x - -
- x - -
- x - -

- - - -
- - - -
x x x -
x - - -

- - - -
x - - -
x - - -
x x - -

-------
- - - -
- - - -
x - - -
x x x -

- - - -
- x - -
- x - -
x x - -

- - - -
- - - -
x x x -
- - x -

- - - -
x x - -
x - - -
x - - -

-------
- - - -
- - - -
x x - -
x x - -

- - - -
- - - -
x x - -
x x - -

- - - -
- - - -
x x - -
x x - -

- - - -
- - - -
x x - -
x x - -

-------
- - - -
- - - -
x x - -
- x x -

- - - -
- x - -
x x - -
x - - -

- - - -
- - - -
x x - -
- x x -

- - - -
- x - -
x x - -
x - - -

-------
- - - -
- - - -
- x x -
x x - -

- - - -
x - - -
x x - -
- x - -

- - - -
- - - -
- x x -
x x - -

- - - -
x - - -
x x - -
- x - -

-------
- - - -
- - - -
- - - -
x x x x

x - - -
x - - -
x - - -
x - - -

- - - -
- - - -
- - - -
x x x x

x - - -
x - - -
x - - -
x - - -

-------
";

    assert_eq!(result, expected);
}

struct game_board {
    blocks: [[bool; 10]; 20],

    current_batch: Vec<Tetronimo>,
    current_tetronimo: Tetronimo,
}


impl game_board {
    fn new() -> game_board {
        let mut current_batch = generate_batch();
        current_batch.shuffle(&mut thread_rng());
        let mut optional_tetronimo: Option<Tetronimo> = current_batch.pop();
        let mut board = game_board {
            blocks: [[false; 10]; 20],
            current_batch,
            current_tetronimo: optional_tetronimo.unwrap(),
        };
        return board;
    }

    fn draw(&self, mut string: String) -> String
    {
        let mut cur_blocks = self.blocks;

        for x in 0..4 {
            for y in 0..4 {
                if self.current_tetronimo.blocks[x][y] {
                    cur_blocks[self.current_tetronimo.x + x][self.current_tetronimo.y + y] = true;
                }
            }
        }

        for row in cur_blocks.iter().rev() {
            for block in row.iter() {
                if *block {
                    string += "x ";
                } else {
                    string += "- ";
                }
            }
            // remove trailing space
            string.pop();

            string += "\n";
        }
        string += "\n";
        return string;
    }
}

fn main() {
    let mut board = game_board::new();
    println!("{}", board.draw(String::new()));
}

