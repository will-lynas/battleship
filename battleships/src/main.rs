use rand::Rng;
use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Cell {
    Empty,
    Ship,
    // Hit,
    // Miss,
}

const GRID_SIZE: usize = 10;

type Grid = [[Cell; GRID_SIZE]; GRID_SIZE];

fn init_grid() -> Grid {
    [[Cell::Empty; GRID_SIZE]; GRID_SIZE]
}

const SHIP_SIZES: [usize; 5] = [5, 4, 3, 3, 2];

fn place_ships(grid: &mut Grid) {
    let mut rng = rand::thread_rng();

    for &size in &SHIP_SIZES {
        loop {
            let horizontal = rng.gen_bool(0.5);
            let (x, y) = (rng.gen_range(0..GRID_SIZE), rng.gen_range(0..GRID_SIZE));

            let mut can_place = true;

            for i in 0..size {
                let (dx, dy) = if horizontal { (x + i, y) } else { (x, y + i) };

                if dx >= GRID_SIZE || dy >= GRID_SIZE || grid[dx][dy] == Cell::Ship {
                    can_place = false;
                    break;
                }
            }

            if can_place {
                for i in 0..size {
                    let (dx, dy) = if horizontal { (x + i, y) } else { (x, y + i) };
                    grid[dx][dy] = Cell::Ship;
                }
                break;
            }
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cell::Empty => write!(f, "."),
            Cell::Ship => write!(f, "S"),
//            Cell::Hit => write!(f, "H"),
//           Cell::Miss => write!(f, "M"),
        }
    }
}

fn display_grid(grid: &Grid) {
    for row in grid {
        for cell in row {
            print!("{} ", cell);
        }
        println!();
    }
}

fn main() {
    let mut grid = init_grid();
    place_ships(&mut grid);
    display_grid(&grid);
}
