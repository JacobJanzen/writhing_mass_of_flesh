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

pub struct Image {
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

impl Image {
    pub fn create_from_args(args: &Args) -> Self {
        Image {
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
            cross_distance: Point { x: 0, y: 0 }.distance(&Point {
                x: args.width - 1,
                y: args.height - 1,
            }),
            points: generate_points(args.width, args.height, args.num_cells),
        }
    }

    pub fn fill_canvas(&mut self) {
        self.generate_noise();
    }

    fn generate_noise(&mut self) {
        let mut max_dist = 0.0;

        // Get distance and nearest point for each point on the canvas
        for y in 0..self.height {
            for x in 0..self.width {
                let index = y as usize * self.width as usize + x as usize;
                self.point_data[index] = PointData::get_point_data(self, Point { x, y });
                max_dist = f64::max(max_dist, self.point_data[index].min_dist);
            }
        }

        // normalize distances to [0,1]
        for y in 0..self.height {
            for x in 0..self.width {
                let index = y as usize * self.width as usize + x as usize;
                self.point_data[index].min_dist /= max_dist;
            }
        }

        // write pixels
        for y in 0..self.height {
            for x in 0..self.width {
                let index = y as usize * self.width as usize + x as usize;
                let val = 0xFF - (0xFF as f64 * self.point_data[index].min_dist) as u8;
                self.set_pixel(val, val, val, Point { x, y });
            }
        }
    }

    fn set_pixel(&mut self, r: u8, g: u8, b: u8, p: Point) {
        self.pixels[3 * (self.width as usize * p.y as usize + p.x as usize)] = r;
        self.pixels[3 * (self.width as usize * p.y as usize + p.x as usize) + 1] = g;
        self.pixels[3 * (self.width as usize * p.y as usize + p.x as usize) + 2] = b;
    }
}

impl PointData {
    fn get_point_data(image: &Image, p: Point) -> Self {
        let mut pd = PointData {
            min_dist: image.cross_distance,
            closest_point: Point { x: 0, y: 0 },
        };

        for point in &image.points {
            let d = p.distance(point);
            if d < pd.min_dist {
                pd.min_dist = d;
                pd.closest_point = point.clone();
            }
        }

        pd
    }
}

impl Point {
    fn distance(&self, other: &Point) -> f64 {
        let x_dist: f64 = other.x as f64 - self.x as f64;
        let y_dist: f64 = other.y as f64 - self.y as f64;

        (x_dist * x_dist + y_dist * y_dist).sqrt()
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
