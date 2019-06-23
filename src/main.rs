extern crate clap;

mod grid;
mod bintree;

use grid::Grid;
use clap::{App, Arg};

fn main() {
    let m = App::new("Maze")
        .version("0.1.0")
        .author("Darren Coxall <darren@darrencoxall.com>")
        .arg(Arg::with_name("width").required(true))
        .arg(Arg::with_name("height").required(true))
        .get_matches();

    let width: usize = m.value_of("width").unwrap().parse().unwrap();
    let height: usize = m.value_of("height").unwrap().parse().unwrap();

    let grid = Grid::from(bintree::Maze::new(width, height));
    println!("{}", grid);
}
