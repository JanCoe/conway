use crossterm::{
    cursor::MoveUp,
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use itertools::Itertools;
use std::cmp::max;
use std::collections::HashMap;
use std::io::stdout;
use std::{thread, time};

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
struct Point {
    row: usize,
    col: usize,
}

impl Point {
    fn count_neighbours(&self, grid: &HashMap<Point, usize>) -> usize {
        let rows = max(1, self.row) - 1..=self.row + 1;
        let cols = max(1, self.col) - 1..=self.col + 1;
        rows.cartesian_product(cols)
            .map(|(row, col)| grid.get(&Point { row, col }).unwrap_or(&0))
            .sum::<usize>()
            - *grid.get(&self).unwrap()
    }

    fn next_state(&self, grid: &HashMap<Point, usize>) -> usize {
        match self.count_neighbours(grid) {
            2 => *grid.get(&self).unwrap(),
            3 => 1,
            _ => 0,
        }
    }
}

fn print_grid(grid: &HashMap<Point, usize>, rows: usize, cols: usize) {
    for row in 0..rows {
        println!();
        for col in 0..cols {
            match grid.get(&Point { row, col }) {
                Some(1) => print!("*"),
                _ => print!("-"),
            };
        }
    }
}

fn update_grid(grid: &mut HashMap<Point, usize>, rows: usize, cols: usize) {
    let old_grid = grid.clone();
    for &ref point in old_grid.keys() {
        grid.insert(point.clone(), point.next_state(&old_grid));
    }
    thread::sleep(time::Duration::from_millis(300));
    print_grid(&grid, rows, cols);
}

fn main() {
    let mut stdout = stdout();
    stdout.execute(Clear(ClearType::All)).unwrap();

    let data = include_str!("../data/grid.csv");
    let mut grid = HashMap::<Point, usize>::new();
    let mut rows: usize = 0;
    for (row, line) in data.lines().enumerate() {
        rows += 1;
        for (col, state) in line.split(',').enumerate() {
            grid.insert(Point { row, col }, state.parse().unwrap());
        }
    }
    let cols = data.lines().next().unwrap().split(',').count();

    for _ in 0..100 {
        _ = stdout.execute(MoveUp(rows as u16));
        update_grid(&mut grid, rows, cols);
    }
}
