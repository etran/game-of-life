use rand::Rng;
use std::{thread, time};

struct GameOfLife {
    grid: Vec<Vec<i32>>,
    w: usize,
    h: usize,
}

impl GameOfLife {
    fn new(width: usize, height: usize) -> GameOfLife {
        let mut new_game = GameOfLife {
            grid: vec![vec![0; height]; width],
            w: width,
            h: height,
        };
        new_game.randomize_grid();
        new_game
    }

    fn randomize_grid(&mut self) {
        let mut rng = rand::thread_rng();
        for x in 0..self.w {
            for y in 0..self.h {
                self.grid[x][y] = rng.gen_range(0, 2);
            }
        }
    }

    fn update_game(&mut self) {
        let mut new_grid = vec![vec![0; self.h]; self.w];

        for x in 0..self.w {
            for y in 0..self.h {
                let num_alive = self.check_neighbours(x as i32, y as i32);
                // Any live cell with two or three live neighbors survives.
                // Any dead cell with three live neighbors becomes a live cell.
                // All other live cells die in the next generation. Similarly, all other dead cells stay dead.
                if self.grid[x][y] != 0 {
                    match num_alive {
                        2 | 3 => new_grid[x][y] = 1,
                        _ => new_grid[x][y] = 0,
                    }
                } else {
                    match num_alive {
                        3 => new_grid[x][y] = 1,
                        _ => new_grid[x][y] = 0,
                    }
                }
            }
        }
        self.grid = new_grid;
    }

    fn check_neighbours(&self, x: i32, y: i32) -> i32 {
        let w = self.w as i32;
        let h = self.h as i32;

        let neighbours: [(i32, i32); 8] = [
            (x - 1, y - 1),
            (x, y - 1),
            (x + 1, y - 1),
            (x - 1, y),
            (x + 1, y),
            (x - 1, y + 1),
            (x, y + 1),
            (x + 1, y + 1),
        ];
        let mut alive_neighbours = 0;

        for (i, j) in neighbours.iter() {
            let real_x = ((*i + w) % w) as usize;
            let real_y = ((*j + h) % h) as usize;
            alive_neighbours += self.grid[real_x][real_y];
        }
        alive_neighbours
    }

    fn print_grid(&self) {
        for y in 0..self.h {
            for x in 0..self.w {
                match self.grid[x][y] {
                    1 => print!("*"),
                    0 => print!("-"),
                    _ => {}
                }
            }
            print!("\n");
        }
        for _ in 0..self.h {
            print!("\x1b[1A");
        }
    }
}

fn main() {
    const HEIGHT: usize = 25;
    const WIDTH: usize = 150;

    let mut game = GameOfLife::new(WIDTH, HEIGHT);

    for _ in 0..100000 {
        game.update_game();
        thread::sleep(time::Duration::from_millis(200));
        game.print_grid();
    }
}
