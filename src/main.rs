use crossterm::{
    cursor::{MoveUp, MoveDown},
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use itertools::Itertools;
use std::cmp::max;
use std::collections::HashMap;
use std::io::stdout;
use std::{thread, time};

type Grid = HashMap<Point, State>;

#[derive(Eq, Hash, PartialEq, Debug, Clone, Default)]
struct Point {
    row: usize,
    col: usize,
}

#[derive(Default)]
struct Area {
    grid: Grid,
    dims: Point,
}

#[derive(Eq, PartialEq, Clone, Copy)]
enum State {
    DEAD,
    ALIVE,
}

fn count_neighbours(point: &Point, grid: &Grid) -> usize {
    let rows = max(1, point.row) - 1..=point.row + 1;
    let cols = max(1, point.col) - 1..=point.col + 1;
    rows.cartesian_product(cols)
        .map(|(row, col)| grid.get(&Point { row, col }).unwrap_or(&State::DEAD))
        .filter(|&&x| x == State::ALIVE)
        .count()
        - match grid.get(&point) {
            Some(&State::ALIVE) => 1,
            _ => 0,
        }
}

fn next_state(point: &Point, grid: &Grid) -> State {
    match count_neighbours(&point, grid) {
        2 => *grid.get(&point).unwrap(),
        3 => State::ALIVE,
        _ => State::DEAD,
    }
}

fn print_grid(area: &Area) {
    for row in 0..area.dims.row {
        println!();
        for col in 0..area.dims.col {
            match area.grid.get(&Point { row, col }) {
                Some(&State::ALIVE) => print!("*"),
                _ => print!("-"),
            };
        }
    }
}

fn update_grid(area: &mut Area) {
    let old_grid = area.grid.clone();
    for point in old_grid.keys() {
        area.grid
            .insert(point.clone(), next_state(&point, &old_grid));
    }
}

fn load_area() -> Area {
    let data = include_str!("../data/grid.csv");
    let mut area = Area::default();
    for (row, line) in data.lines().enumerate() {
        area.dims.row += 1;
        for (col, state) in line.split(',').enumerate() {
            area.grid.insert(
                Point { row, col },
                match state.parse::<u8>().unwrap() {
                    1 => State::ALIVE,
                    _ => State::DEAD,
                },
            );
        }
    }
    area.dims.col = data.lines().next().unwrap().split(',').count();
    area
}

fn main() {
    let mut stdout = stdout();
    stdout.execute(Clear(ClearType::All)).unwrap();

    let mut area = load_area();
    let rows = area.dims.row as u16;
    for _ in 0..10 {
        update_grid(&mut area);
        print_grid(&area);
        thread::sleep(time::Duration::from_millis(300));
        _ = stdout.execute(MoveUp(rows));
    }
    _ = stdout.execute(MoveDown(rows));
}
