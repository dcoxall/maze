extern crate clap;

mod grid;
mod bintree;
mod sidewinder;
mod aldousbroder;

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
        .arg(Arg::with_name("algorithm")
            .required(true)
            .long("algo")
            .takes_value(true)
            .possible_value("btree")
            .possible_value("sidewinder")
            .possible_value("aldous-broder"))
        .arg(Arg::with_name("out")
            .long("out")
            .value_name("FILE")
            .takes_value(true))
        .get_matches();

    let width: usize = m.value_of("width").unwrap().parse().unwrap();
    let height: usize = m.value_of("height").unwrap().parse().unwrap();

    let maze: Box<Iterator<Item=Grid>> = match m.value_of("algorithm") {
        Some("sidewinder") => Box::new(sidewinder::Maze::new(width, height)),
        Some("aldous-broder") => Box::new(aldousbroder::Maze::new(width, height)),
        _ => Box::new(bintree::Maze::new(width, height)),
    };

    if let Some(file_name) = m.value_of("out") {
        match generate_gif(file_name, maze) {
            Ok(grid) => println!("{}", grid),
            Err(err) => println!("{:?}", err),
        }
    } else {
        let grid = Grid::from(maze);
        println!("{}", grid);
    }
}

fn generate_gif<I: Iterator<Item=Grid>>(file_name: &str, iter: I) -> io::Result<Grid> {
    let mut image = File::create(file_name)?;

    let grids: Vec<Grid> = iter.collect();
    let grid_count = grids.len();
    let frames: Vec<gif::Frame> = grids.iter()
        .map(|grid| {
            let grid = grid.clone();
            let mut frame: gif::Frame = grid.into();
            // set the delay relative to the number of frames
            frame.delay = (1000 / grid_count) as u16;
            frame
        })
        .collect();
    let first_frame = frames.first().unwrap();

    let mut encoder = gif::Encoder::new(
        &mut image,
        first_frame.width,
        first_frame.height,
        &[
            0xFF, 0xFF, 0xFF,
            0,    0,    0,
            0xFF, 0,    0,
            0   , 0xFF, 0,
        ],
    )?;
    encoder.set(gif::Repeat::Finite(3))?;

    for frame in &frames {
        encoder.write_frame(frame)?;
    }

    Ok(grids.last().unwrap().clone())
}
