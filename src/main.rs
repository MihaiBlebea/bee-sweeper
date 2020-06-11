use std::convert::TryInto;
use rand::Rng;

fn main() {
    
    let mut game = Game::new();

    game.uncover_cell(1, 1);
    game.uncover_cell(2, 2);
    game.uncover_cell(3, 3);
    game.uncover_cell(4, 4);

    for r in game.board.cells {
        for c in r.iter() {
            println!("x: {} and y: {}. Has bee: {}. Number is {}. Visible {}", c.x, c.y, c.bee, c.count, c.shown)
        }
    }

    println!("Turns played {}. Still playing? {}", game.turn, game.playing)
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
            shown: false,
            count: Cell::get_count(bee, count)
        }
    }

    fn get_count(bee: bool, count: u16) -> u16 {
        
        return if bee { 0 } else { count };
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
                // for xoff in 0..3 {
                //     for yoff in 0..3 {
                //         if xoff > 0 && yoff > 0 {
                //             let i = x + (xoff - 1);
                //             let j = y + (yoff - 1);

                //             println!("{} and {}", xoff, yoff);
                //             if matrix[i][j].bee == true {
                //                 total += 1
                //             }
                //         }
                //     }
                // }
                
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
    
        return rand::thread_rng().gen_range(0, 10) < 3;
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
}

struct Game {
    playing: bool,
    turn: u64,
    board: Board,
}

impl Game {
    fn new() -> Game {
        let board = Board::new(5, 5, 3);

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