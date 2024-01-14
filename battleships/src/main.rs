use rand::{rngs::StdRng, Rng, SeedableRng};
use std::fmt;

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    Ship,
    Hit,
    Miss,
}

const GRID_SIZE: usize = 10;

type Grid = [[Cell; GRID_SIZE]; GRID_SIZE];

fn init_grid() -> Grid {
    [[Cell::Empty; GRID_SIZE]; GRID_SIZE]
}

type ProbabilityGrid = [[i32; GRID_SIZE]; GRID_SIZE];

fn init_probability_grid() -> ProbabilityGrid {
    [[0; GRID_SIZE]; GRID_SIZE]
}

const SHIP_SIZES: [usize; 5] = [5, 4, 3, 3, 2];

fn place_ships(grid: &mut Grid, rng: &mut StdRng) {
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
            Cell::Hit => write!(f, "H"),
            Cell::Miss => write!(f, "M"),
        }
    }
}

// fn display_grid(grid: &Grid) {
//     for row in grid {
//         for cell in row {
//             print!("{} ", cell);
//         }
//         println!();
//     }
// }
//
// fn display_probability_grid(grid: &ProbabilityGrid) {
//     for row in grid {
//         for cell in row {
//             print!("{} ", cell);
//         }
//         println!();
//     }
// }

fn calculate_probability_grid(grid: &Grid) -> ProbabilityGrid {
    // This can be improved.
    // In its current state, this function doesn't pay attention to information about hits,
    // even though shooting next to an existing hit has high probability of success.
    let mut probability_grid = init_probability_grid();
    for ship_size in &SHIP_SIZES {
        for x in 0..GRID_SIZE {
            for y in 0..GRID_SIZE {
                // Horizontal
                let mut can_place = true;
                for i in 0..*ship_size {
                    if x + i >= GRID_SIZE || grid[x + i][y] == Cell::Miss {
                        can_place = false;
                        break;
                    }
                }
                if can_place {
                    for i in 0..*ship_size {
                        probability_grid[x + i][y] += 1;
                    }
                }
                // Vertical
                can_place = true;
                for i in 0..*ship_size {
                    if y + i >= GRID_SIZE || grid[x][y + i] == Cell::Miss {
                        can_place = false;
                        break;
                    }
                }
                if can_place {
                    for i in 0..*ship_size {
                        probability_grid[x][y + i] += 1;
                    }
                }
            }
        }
    }
    probability_grid
}

fn highest_probability_coords(probability_grid: ProbabilityGrid, grid: Grid) -> (i32, i32) {
    let mut best_x: i32 = 0;
    let mut best_y: i32 = 0;
    let mut best = 0;
    for x in 0..GRID_SIZE {
        for y in 0..GRID_SIZE {
            if probability_grid[x][y] > best  && grid[x][y] != Cell::Hit {
                best = probability_grid[x][y];
                best_x = x as i32;
                best_y = y as i32;
            }

        }
    }
    (best_x, best_y)
}

enum ShotResult {
    Hit,
    Miss,
    AlreadyTaken,
}

fn fire_shot(grid: &mut Grid, x: usize, y: usize) -> ShotResult {
    match grid[x][y] {
        Cell::Ship => {
            grid[x][y] = Cell::Hit;
            ShotResult::Hit
        },
        Cell::Empty => {
            grid[x][y] = Cell::Miss;
            ShotResult::Miss
        },
        _ => ShotResult::AlreadyTaken,
    }
}

fn is_game_over(grid: &Grid) -> bool {
    for row in grid {
        for &cell in row {
            if cell == Cell::Ship {
                return false;
            }
        }
    }
    true
}

fn random_shots_game(rng: &mut StdRng) -> i32 {
    let mut grid = init_grid();
    place_ships(&mut grid, rng);
    let mut score = 0;

    while !is_game_over(&grid) {
        let x = rng.gen_range(0..GRID_SIZE);
        let y = rng.gen_range(0..GRID_SIZE);

        match fire_shot(&mut grid, x, y) {
            ShotResult::Hit | ShotResult::Miss => score += 1,
            _ => {}
        }
    }
    score
}

fn random_shots_games(num_runs: i32) -> () {
    let seed = [42; 32];
    let mut rng = StdRng::from_seed(seed);
    let mut total_shots = 0;

    for _ in 0..num_runs {
        let shots = random_shots_game(&mut rng);
        total_shots += shots;
    }

    let average_shots = total_shots as f64 / num_runs as f64;
    println!("Average shots fired over {} games: {:.2}", num_runs, average_shots);
}

fn probability_game(rng: &mut StdRng) -> i32 {
    let mut grid = init_grid();
    place_ships(&mut grid, rng);
    let mut score = 0;

    while !is_game_over(&grid) {
        let probability_grid = calculate_probability_grid(&grid);
        // display_probability_grid(&probability_grid);
        // display_grid(&grid);
        let (x, y) = highest_probability_coords(probability_grid, grid);

        match fire_shot(&mut grid, x as usize, y as usize) {
            ShotResult::Hit | ShotResult::Miss => score += 1,
            _ => {}
        }
    }
    score
}

fn probability_games(num_runs: i32) -> () {
    let seed = [42; 32];
    let mut rng = StdRng::from_seed(seed);
    let mut total_shots = 0;

    for _ in 0..num_runs {
        let shots = probability_game(&mut rng);
        total_shots += shots;
    }

    let average_shots = total_shots as f64 / num_runs as f64;
    println!("Average shots fired over {} games: {:.2}", num_runs, average_shots);
}

fn main() {
    const NUM_RUNS: i32 = 1000;
    println!("Random shots:");
    random_shots_games(NUM_RUNS);
    println!("Probability shots:");
    probability_games(NUM_RUNS);
}
