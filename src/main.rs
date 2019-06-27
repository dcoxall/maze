extern crate clap;

mod grid;
mod bintree;

use std::io;
use std::fs::File;
use grid::Grid;
use clap::{App, Arg};
use gif;
use gif::SetParameter;

fn main() {
    let m = App::new("Maze")
        .version("0.1.0")
        .author("Darren Coxall <darren@darrencoxall.com>")
        .arg(Arg::with_name("width").required(true))
        .arg(Arg::with_name("height").required(true))
        .arg(Arg::with_name("out")
            .long("out")
            .value_name("FILE")
            .takes_value(true))
        .get_matches();

    let width: usize = m.value_of("width").unwrap().parse().unwrap();
    let height: usize = m.value_of("height").unwrap().parse().unwrap();

    if let Some(file_name) = m.value_of("out") {
        let maze = bintree::Maze::new(width, height);
        match generate_gif(file_name, maze.into_iter().map(|grid| gif::Frame::from(grid))) {
            Ok(())   => println!("DONE"),
            Err(err) => println!("{:?}", err),
        }
    } else {
        let grid = Grid::from(bintree::Maze::new(width, height));
        println!("{}", grid);
    }
}

fn generate_gif<I: Iterator<Item=gif::Frame<'static>>>(file_name: &str, mut iter: I) -> io::Result<()> {
    let mut image = File::create(file_name)?;
    let frame = iter.next().unwrap();
    let mut encoder = gif::Encoder::new(&mut image, frame.width, frame.height, &[0xFF, 0xFF, 0xFF, 0, 0, 0, 0xFF, 0, 0])?;
    encoder.set(gif::Repeat::Finite(3))?;
    encoder.write_frame(&frame)?;
    for frame in iter {
        encoder.write_frame(&frame)?;
    }
    Ok(())
}
