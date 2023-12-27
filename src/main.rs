use leptos::ev::Event;
use rand::thread_rng;
use rand::seq::SliceRandom;
use std::{thread, time};
use std::rc::Rc;

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
        for row in self.blocks.iter() {
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
        if self.y > 0 {
            self.y -= 1;
        }
    }

    fn move_right(&mut self) {
        self.y += 1;
    }

    fn move_down(&mut self) {
        self.x += 1;
    }

    fn move_up(&mut self) {
        self.x -= 1;
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
x x x -
- x - -
- - - -
- - - -

- x - -
x x - -
- x - -
- - - -

- x - -
x x x -
- - - -
- - - -

x - - -
x x - -
x - - -
- - - -

-------
x x x -
- - x -
- - - -
- - - -

- x - -
- x - -
x x - -
- - - -

x - - -
x x x -
- - - -
- - - -

x x - -
x - - -
x - - -
- - - -

-------
x x x -
x - - -
- - - -
- - - -

x x - -
- x - -
- x - -
- - - -

- - x -
x x x -
- - - -
- - - -

x - - -
x - - -
x x - -
- - - -

-------
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
- - - -
- - - -

-------
- x x -
x x - -
- - - -
- - - -

x - - -
x x - -
- x - -
- - - -

- x x -
x x - -
- - - -
- - - -

x - - -
x x - -
- x - -
- - - -

-------
x x - -
- x x -
- - - -
- - - -

- x - -
x x - -
x - - -
- - - -

x x - -
- x x -
- - - -
- - - -

- x - -
x x - -
x - - -
- - - -

-------
x x x x
- - - -
- - - -
- - - -

x - - -
x - - -
x - - -
x - - -

x x x x
- - - -
- - - -
- - - -

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

    fn block(&self, x: usize, y: usize) -> bool {
        if x < 0 || y < 0 {
            return true;
        }

        if x >= 20 || y >= 10 {
            return true;
        }

        return self.blocks[x][y];
    }

    fn draw(&self, mut string: String) -> String
    {
        let mut cur_blocks = self.blocks;

        string += "<table>";

        for x in 0..4 {
            for y in 0..4 {
                if self.current_tetronimo.blocks[x][y] {
                    cur_blocks[self.current_tetronimo.x + x][self.current_tetronimo.y + y] = true;
                }
            }
        }

        for row in cur_blocks.iter() {
            string += "<tr>";

            for block in row.iter() {
                if *block {
                    string += "<td bgcolor=\"red\" \\>";
                } else {
                    string += "<td bgcolor=\"white\" \\>";
                }
            }
            string += "</tr>";
            // remove trailing space
            string.pop();

            string += "<br />";
        }
        string += "\n";
        return string;
    }

    fn is_overlapping(&self, tetronimo: &Tetronimo) -> bool {
        for x in 0..4 {
            for y in 0..4 {
                if tetronimo.blocks[x][y] && self.block(tetronimo.x + x, tetronimo.y + y) {
                    return true;
                }
            }
        }
        return false;
    }

    fn lower(&mut self) {
        self.current_tetronimo.move_down();
        if self.is_overlapping(&self.current_tetronimo) {
            self.current_tetronimo.move_up();
            for x in 0..4 {
                for y in 0..4 {
                    if self.current_tetronimo.blocks[x][y] {
                        self.blocks[self.current_tetronimo.x + x][self.current_tetronimo.y + y] = true;
                    }
                }
            }

            // Delete any full lines
            for x in 0..20 {
                let mut full = true;
                for y in 0..10 {
                    if !self.blocks[x][y] {
                        full = false;
                        break;
                    }
                }
                if full {
                    for y in 0..10 {
                        self.blocks[x][y] = false;
                    }
                    for x2 in (1..x).rev() {
                        for y in 0..10 {
                            self.blocks[x2 + 1][y] = self.blocks[x2][y];
                        }
                    }
                }
            }

            let mut optional_tetronimo: Option<Tetronimo> = self.current_batch.pop();
            if optional_tetronimo.is_none() {
                self.current_batch = generate_batch();
                self.current_batch.shuffle(&mut thread_rng());
                optional_tetronimo = self.current_batch.pop();
            }
            self.current_tetronimo = optional_tetronimo.unwrap();

            if self.is_overlapping(&self.current_tetronimo) {
                panic!("Game over");
            }
        }
    }
}

use leptos::*;

use leptos::prelude::*;
use leptos::logging::*;


use wasm_bindgen::prelude::*;
use std::cell::RefCell;

struct AppState {
    board: game_board,
    // read_board_str: Signal<String>,
    write_board_str: WriteSignal<String>,
}

#[wasm_bindgen]
pub fn start_app() {
    // Initialize your app state
    std::panic::set_hook(Box::new(|panic_info| {
        error!("Panic: {}", panic_info);
    }));
    let (read_board_str, write_board_str) = create_signal(String::new());
    let app_state = Rc::new(RefCell::new(AppState {
        board: game_board::new(),
        // read_board_str: Signal::from(read_board_str),
        write_board_str: WriteSignal::from(write_board_str),
    }));

    let app_state_clone_for_window = app_state.clone();
    let app_state_clone_for_closure = app_state.clone();
    let app_state_clone_for_event_handler = app_state.clone();


    // Create a closure to be executed after the delay
    let closure = Closure::wrap(Box::new(move || {
        log! { "called" }
        let _app_state_inner = Rc::clone(&app_state_clone_for_closure);
        let mut app_state_inner = _app_state_inner.borrow_mut();
        app_state_inner.board.lower();
        app_state_inner.write_board_str.set(app_state_inner.board.draw(String::new()));
    }) as Box<dyn FnMut()>);

    // Schedule the closure for execution after the delay
    window().set_interval_with_callback_and_timeout_and_arguments_0(
        closure.as_ref().unchecked_ref(),
        300,
    ).expect("Problem scheduling interval");


    // Mount the initial view
    mount_to_body(move || {
        view! { <div inner_html={move || read_board_str.get()}></div> }
    });


    let handle = leptos::leptos_dom::helpers::window_event_listener(ev::keypress, move |ev| {
        let mut app_state_inner = app_state_clone_for_event_handler.borrow_mut();

        match ev.key().as_str() {
            "a" => {
                log!("left");
                app_state_inner.board.current_tetronimo.move_left();
                if app_state_inner.board.is_overlapping(&app_state_inner.board.current_tetronimo) {
                    app_state_inner.board.current_tetronimo.move_right();
                }
            }
            "d" => {
                log!("right");
                app_state_inner.board.current_tetronimo.move_right();
                if app_state_inner.board.is_overlapping(&app_state_inner.board.current_tetronimo) {
                    app_state_inner.board.current_tetronimo.move_left();
                }
            }
            "w" => {
                log!("rotate");
                app_state_inner.board.current_tetronimo.rotate();
            }
            "s" => {
                log!("down");
                app_state_inner.board.lower();
            }
            x => {
                log!("unknown key {}", x);
            }
        }
        app_state_inner.write_board_str.set(app_state_inner.board.draw(String::new()));
    });


    closure.forget();
}


fn main() {
    start_app();
}