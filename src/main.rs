extern crate rand;

use std::fmt;
use std::time::Duration;
use std::thread::sleep;

fn main() {

    #[repr(u8)]
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub enum Cell {
        Dead = 0,
        Alive = 1,
    }

    pub struct Universe {
        width: u32,
        height: u32,
        cells: Vec<Cell>,
    }

    impl Universe {

        fn get_index(&self, row: u32, col: u32) -> usize {
            (row * self.width + col) as usize
        }

        fn live_neighbor_count(&self, row: u32, col: u32) -> u8 {
            let mut count = 0;
            for i in [self.height-1, 0, 1].iter().cloned() {
                for j in [self.width-1, 0, 1].iter().cloned() {
                    if i == 0 && j == 0 {
                        continue;
                    }

                    let neighbor_row = (row + j) % self.height;
                    let neighbor_col = (col + i) % self.width;
                    let idx = self.get_index(neighbor_row, neighbor_col);
                    count += self.cells[idx] as u8;
                }
            }
            count
        }

        pub fn update(&mut self) {
            let mut next = self.cells.clone();

            for row in 0..self.height {
                for col in 0..self.width {
                    let idx = self.get_index(row, col);
                    let cell = self.cells[idx];
                    let live_neighbors = self.live_neighbor_count(row, col);

                    let next_cell = match (cell, live_neighbors) {
                        (Cell::Alive, x) if x < 2           => Cell::Dead,
                        (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                        (Cell::Alive, x) if x > 3           => Cell::Dead,
                        (Cell::Dead, 3)                     => Cell::Alive,
                        (otherwise, _)                      => otherwise,
                    };
                    next[idx] = next_cell;
                }
            }
            self.cells = next;
        }
    }

    fn create_univ() -> Universe {
        let width = 60;
        let height = 30;
        let cells = (0..width*height)
            .map(|_| {
                let a = rand::random::<f32>();
                if a < 0.2 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            }).collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    impl fmt::Display for Universe {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            for line in self.cells.as_slice().chunks(self.width as usize) {
                for &cell in line {
                    let symbol = 
                        if cell == Cell::Dead {"□"} else {"■"};
                        write!(f, "{}", symbol)?;
                }
                write!(f, "\n")?;
            }
            Ok(())
        }
    }

    // Instantiate the universe
    let mut univ = create_univ();
    println!("{}", univ);

    loop {
        univ.update();
        println!("{}", univ);
        sleep(Duration::from_millis(200)); // update the universe every 200 ms
    }
}
