extern crate wasm_bindgen;
extern crate stdweb;

use std::convert::TryInto;
use rand::Rng;

use wasm_bindgen::prelude::*;

// fn main() {
    
//     let mut game = Game::new();

//     game.uncover_cell(1, 1);
//     game.uncover_cell(2, 2);
//     game.uncover_cell(3, 3);
//     game.uncover_cell(4, 4);

//     for r in game.board.cells {
//         for c in r.iter() {
//             println!("x: {} and y: {}. Has bee: {}. Number is {}. Visible {}", c.x, c.y, c.bee, c.count, c.shown)
//         }
//     }

//     println!("Turns played {}. Still playing? {}", game.turn, game.playing)
// }
#[wasm_bindgen]
extern "C" {

    type HTMLDocument;
    pub type Element;

    static document: HTMLDocument;

    #[wasm_bindgen(method)]
    fn createElement(this: &HTMLDocument, tagName: &str) -> Element;

    #[wasm_bindgen(method, getter)]
    fn body(this: &HTMLDocument) -> Element;

    #[wasm_bindgen(method, js_name = appendChild)]
    fn append(this: &Element, item: Element);

    #[wasm_bindgen(method, setter, js_name = innerHTML)]
    fn set_inner(this: &Element, html: &str);
    
    #[wasm_bindgen(method, getter)]
    fn style(this: &Element) -> Element;

    #[wasm_bindgen(method, setter, js_name = backgroundColor)]
    fn set_bg_color(this: &Element, color: &str);

    #[wasm_bindgen(method, setter, js_name = height)]
    fn set_height(this: &Element, height: &str);

    #[wasm_bindgen(method, setter, js_name = width)]
    fn set_width(this: &Element, width: &str);

    #[wasm_bindgen(method, setter, js_name = display)]
    fn set_display(this: &Element, display: &str);

    #[wasm_bindgen(method, setter, js_name = margin)]
    fn set_margin(this: &Element, margin: &str);

    #[wasm_bindgen(method, setter, js_name = cursor)]
    fn set_cursor(this: &Element, cursor: &str);

    #[wasm_bindgen(method, setter, js_name = addEventListener)]
    fn set_event_listener(this: &Element, event: &str, callback: fn() -> String);
}

#[wasm_bindgen]
pub fn render_hidden_cell() -> Element {
    let div = document.createElement("div");
    div.style().set_bg_color("red");
    div.style().set_height("10px");
    div.style().set_width("10px");
    div.style().set_margin("2px");
    div.style().set_cursor("pointer");

    return div
}

#[wasm_bindgen]
pub fn render_shown_cell(content: &str) -> Element {
    let div = document.createElement("div");
    div.style().set_bg_color("blue");
    div.style().set_height("10px");
    div.style().set_width("10px");
    div.style().set_margin("2px");
    div.set_inner(content);

    return div
}


#[wasm_bindgen]
pub fn start_game() {
    let mut proceed = true;
    let game = Game::new();

    game.board.render();
    // while proceed == true {

    // }
}

struct Cell {
    bee: bool,
    x: u16,
    y: u16,
    shown: bool,
    count: u16
}

impl Cell {
    fn new(bee: bool, x: u16, y: u16, count: u16) -> Cell {
        
        return Cell{
            bee: bee,
            x: x,
            y: y,
            shown: true,
            count: Cell::get_count(bee, count)
        }
    }

    fn get_count(bee: bool, count: u16) -> u16 {
        
        return if bee { 0 } else { count };
    }

    fn render(&self) -> Element {
        let div = document.createElement("div");
        if self.shown == true {
            div.style().set_bg_color("blue");
            if self.bee == true {
                div.set_inner("B");
            } else {
                if self.count > 0 {
                    div.set_inner(&self.count.to_string());
                } else {
                    div.set_inner("");
                }
            }
        } else {
            div.style().set_bg_color("red");
        }
        div.style().set_height("10px");
        div.style().set_width("10px");
        div.style().set_margin("2px");

        return div
    }
}

impl Copy for Cell {}

impl Clone for Cell {
    fn clone(&self) -> Self {
        *self
    }
}

struct Board {
    cells: Vec<Vec<Cell>>,
    h: usize,
    w: usize
}

impl Board {
    fn new(h: usize, w: usize, bee_count: u16) -> Board {

        return Board{
            cells: Board::generate_cells(h, w, bee_count),
            h: h,
            w: w
        }
    }

    fn generate_cells(h: usize, w: usize, bee_count: u16) -> Vec<Vec<Cell>> {
        // Generate the cells without any bee count in them
        let mut bees = bee_count;
        let mut matrix : Vec<Vec<Cell>> = vec!();
        for x in 0..w {
            matrix.push(vec!());
            for y in 0..h {
                let u_x = x.try_into().unwrap();
                let u_y = y.try_into().unwrap();

                let should_have_bee = Board::should_have_bee();
                if should_have_bee == true && bees > 0 {
                    bees -= 1;
                    matrix[x].push(Cell::new(true, u_x, u_y, 0));
                } else {
                    matrix[x].push(Cell::new(false, u_x, u_y, 0));
                }
            }
        }

        // Now that we have all the cells and bees, we can start bee counting
        for x in 0..w {
            for y in 0..h {
                let mut total = 0;
                
                // TODO: This needs to be refactored
                if x > 0 && y > 0 && matrix[x - 1][y - 1].bee == true {
                    total += 1;
                }
                if x > 0 && matrix[x - 1][y].bee == true {
                    total += 1;
                }
                if x > 0 && y < h - 1 && matrix[x - 1][y + 1].bee == true {
                    total += 1;
                }

                if y > 0 && matrix[x][y - 1].bee == true {
                    total += 1;
                }
                if y < h - 1 && matrix[x][y + 1].bee == true {
                    total += 1;
                }

                if x < w - 1 && y > 0 && matrix[x + 1][y - 1].bee == true {
                    total += 1;
                }
                if x < w - 1 && matrix[x + 1][y].bee == true {
                    total += 1;
                }
                if x < w - 1 && y < h - 1 && matrix[x + 1][y + 1].bee == true {
                    total += 1;
                }
                matrix[x][y].count = total
            }
        }

        return matrix
    }

    fn should_have_bee() -> bool {
    
        return rand::thread_rng().gen_range(0, 10) < 2;
    }

    fn has_bee(&mut self, x: usize, y: usize) -> bool {
        if x < self.w && y < self.h {
            // if cell has a bee, then finish the game
            if self.cells[x][y].bee == true {

                return true;
            }

            // if cell doesn't have a bee, then show the cell
            let mut cells = self.cells.to_vec();
            cells[x][y].shown = true;
            self.cells = cells
        }

        return false
    }

    fn render(&self) {
        let board = document.createElement("div");
        board.style().set_display("grid");
    
        for x in 0..self.w {
            let row = document.createElement("div");
            row.style().set_display("inline-flex");
    
            for y in 0..self.h {
                row.append(self.cells[x][y].render())
            }
            board.append(row);
        }

        document.body().append(board);
    }
}

struct Game {
    playing: bool,
    turn: u64,
    board: Board,
}

impl Game {
    fn new() -> Game {
        let board = Board::new(15, 15, 10);

        return Game{
            playing: true,
            turn: 1,
            board: board,
        }
    }

    fn uncover_cell(&mut self, x: usize, y: usize) -> bool {
        if self.board.has_bee(x, y) == true {
            self.playing = false;

            return true;
        }
        self.turn += 1;

        return false;
    }
}