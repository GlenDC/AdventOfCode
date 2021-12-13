use std::collections::VecDeque;
// use colour::{white, dark_yellow, red};

fn main() {
    println!("++++++++++++");
    println!("PART#1");
    println!("++++++++++++");

    part1(TEST_INPUT);

    println!("-----");

    part1(INPUT);

    println!("++++++++++++");
    println!("PART#2");
    println!("++++++++++++");

    part2(TEST_INPUT);

    println!("-----");

    part2(INPUT);
}

fn part1(input: &str) {
    const GRID_SIZE: isize = 10;
    let mut grid: Vec<Vec<DumboOctopus>> = input
        .split("\n")
        .map(|line| line.as_bytes().iter().map(|b| b.into()).collect())
        .collect();
    let mut flash_counter = 0;
    for _step in 0..100 {
        let mut flashed: VecDeque<(isize, isize)> = VecDeque::new();
        for (row_idx, row) in grid.iter_mut().enumerate() {
            for (col_idx, octopus) in row.iter_mut().enumerate() {
                octopus.reset();
                if octopus.charge() {
                    flashed.push_back((row_idx as isize, col_idx as isize));
                    flash_counter += 1;
                }
            }
        }
        let offsets = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        loop {
            match flashed.pop_front() {
                None => break,
                Some((row_idx, col_idx)) => {
                    for (offset_row, offset_col) in offsets {
                        let row_idx = row_idx + offset_row;
                        if row_idx < 0 || row_idx >= GRID_SIZE {
                            continue;
                        }
                        let col_idx = col_idx + offset_col;
                        if col_idx < 0 || col_idx >= GRID_SIZE {
                            continue;
                        }
                        if grid[row_idx as usize][col_idx as usize].charge() {
                            flash_counter += 1;
                            flashed.push_back((row_idx, col_idx));
                        }
                    }
                }
            }
        }
    }
    println!("{}", flash_counter);
}

fn part2(input: &str) {
    const GRID_SIZE: isize = 10;
    let mut grid: Vec<Vec<DumboOctopus>> = input
        .split("\n")
        .map(|line| line.as_bytes().iter().map(|b| b.into()).collect())
        .collect();
    for step in 0..50000 {
        // for row in &grid {
        //     for octopus in row {
        //         match octopus.energy {
        //             10 => {
        //                 dark_yellow!("O");
        //             },
        //             0 => {
        //                 red!("*");
        //             },
        //             energy => {
        //                 white!("{}", energy);
        //             },
        //         };
        //     }
        //     println!("");
        // }
        // println!("");

        let mut all_flushed = true;
        for row in &mut grid {
            for octopus in row {
                all_flushed = octopus.reset() && all_flushed;
            }
        }
        if all_flushed {
            println!("{}", step);
            return;
        }

        let mut flashed: VecDeque<(isize, isize)> = VecDeque::new();
        for (row_idx, row) in grid.iter_mut().enumerate() {
            for (col_idx, octopus) in row.iter_mut().enumerate() {
                if octopus.charge() {
                    flashed.push_back((row_idx as isize, col_idx as isize));
                }
            }
        }
        let offsets = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        loop {
            match flashed.pop_front() {
                None => break,
                Some((row_idx, col_idx)) => {
                    for (offset_row, offset_col) in offsets {
                        let row_idx = row_idx + offset_row;
                        if row_idx < 0 || row_idx >= GRID_SIZE {
                            continue;
                        }
                        let col_idx = col_idx + offset_col;
                        if col_idx < 0 || col_idx >= GRID_SIZE {
                            continue;
                        }
                        if grid[row_idx as usize][col_idx as usize].charge() {
                            flashed.push_back((row_idx, col_idx));
                        }
                    }
                }
            }
        }
    }
    panic!("nothing found!")
}

struct DumboOctopus {
    energy: u8,
    flashed: bool,
}

impl From<&u8> for DumboOctopus {
    fn from(b: &u8) -> DumboOctopus {
        DumboOctopus {
            energy: b - 48,
            flashed: false,
        }
    }
}

impl DumboOctopus {
    pub fn charge(&mut self) -> bool {
        if self.flashed {
            return false;
        }
        self.energy += 1;
        self.flashed = self.energy > 9;
        self.flashed
    }

    pub fn reset(&mut self) -> bool {
        if self.flashed {
            self.flashed = false;
            self.energy = 0;
            return true;
        }
        false
    }
}

const TEST_INPUT: &'static str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

const INPUT: &'static str = "1326253315
3427728113
5751612542
6543868322
4422526221
2234325647
1773174887
7281321674
6562513118
4824541522";
