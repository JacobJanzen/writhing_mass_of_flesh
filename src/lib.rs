/*
   This program is free software: you can redistribute it and/or modify it
   under the terms of the GNU General Public License as published by the Free
   Software Foundation; either version 3 of the License, or (at your option)
   any later version.

   This program is distributed in the hope that it will be useful, but WITHOUT
   ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
   FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for
   more details.

   You should have received a copy of the GNU General Public License along
   with this program. If not, see https://www.gnu.org/licenses/.
*/
use rand::Rng;
use std::vec;

use clap::Parser;

pub struct Gif {
    pub height: u16,
    pub width: u16,
    pub frames: u16,
    pub pixels: Vec<u8>,
    point_data: Vec<PointData>,
    cross_distance: f64,
    points: Vec<Point>,
}

#[derive(Clone)]
struct Point {
    pub x: u16,
    pub y: u16,
}

#[derive(Clone)]
struct PointData {
    min_dist: f64,
    closest_point: Point,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// width of the image
    #[clap(short, long, value_parser)]
    pub width: u16,

    /// height of the image
    #[clap(short, long, value_parser)]
    pub height: u16,

    /// number of gif frames
    #[clap(short, long, value_parser)]
    pub frames: u16,

    /// number of cells to generate
    #[clap(short, long, value_parser)]
    pub num_cells: usize,

    /// output file
    #[clap(short, long, value_parser)]
    pub out: String,
}

impl Args {
    pub fn read() -> Self {
        Args::parse()
    }
}

impl Gif {
    pub fn create_from_args(args: &Args) -> Self {
        Gif {
            height: args.height,
            width: args.width,
            frames: args.frames,
            pixels: vec![0; args.height as usize * args.width as usize * 3],
            point_data: vec![
                PointData {
                    min_dist: 0.0,
                    closest_point: Point { x: 0, y: 0 }
                };
                args.height as usize * args.width as usize
            ],
            cross_distance: distance(
                &Point { x: 0, y: 0 },
                &Point {
                    x: args.width - 1,
                    y: args.height - 1,
                },
            ),
            points: generate_points(args.width, args.height, args.num_cells),
        }
    }
}

impl PointData {
    fn get_point_data(gif: &Gif, p: Point) -> Self {
        let mut pd = PointData {
            min_dist: gif.cross_distance,
            closest_point: Point { x: 0, y: 0 },
        };

        for point in &gif.points {
            let d = distance(&p, point);
            if d < pd.min_dist {
                pd.min_dist = d;
                pd.closest_point = point.clone();
            }
        }

        pd
    }
}

pub fn fill_canvas(gif: &mut Gif) {
    generate_noise(gif);
}

fn set_pixel(gif: &mut Gif, r: u8, g: u8, b: u8, x: u16, y: u16) {
    gif.pixels[3 * (gif.width as usize * y as usize + x as usize)] = r;
    gif.pixels[3 * (gif.width as usize * y as usize + x as usize) + 1] = g;
    gif.pixels[3 * (gif.width as usize * y as usize + x as usize) + 2] = b;
}

fn generate_noise(gif: &mut Gif) {
    let mut max_dist = 0.0;

    // Get distance and nearest point for each point on the canvas
    for y in 0..gif.height {
        for x in 0..gif.width {
            let index = y as usize * gif.width as usize + x as usize;
            gif.point_data[index] = PointData::get_point_data(gif, Point { x, y });
            max_dist = f64::max(max_dist, gif.point_data[index].min_dist);
        }
    }

    // normalize distances to [0,1]
    for y in 0..gif.height {
        for x in 0..gif.width {
            let index = y as usize * gif.width as usize + x as usize;
            gif.point_data[index].min_dist /= max_dist;
        }
    }

    for y in 0..gif.height {
        for x in 0..gif.width {
            let index = y as usize * gif.width as usize + x as usize;
            let val = 0xFF - (0xFF as f64 * gif.point_data[index].min_dist) as u8;
            set_pixel(gif, val, val, val, x, y)
        }
    }
}

fn generate_points(width: u16, height: u16, num_cells: usize) -> Vec<Point> {
    let mut points = vec![Point { x: 0, y: 0 }; num_cells];

    for p in &mut points {
        p.x = rand::thread_rng().gen_range(0..width);
        p.y = rand::thread_rng().gen_range(0..height);
    }

    points
}

fn distance(p1: &Point, p2: &Point) -> f64 {
    let x_dist: f64 = p2.x as f64 - p1.x as f64;
    let y_dist: f64 = p2.y as f64 - p1.y as f64;

    (x_dist * x_dist + y_dist * y_dist).sqrt()
}
