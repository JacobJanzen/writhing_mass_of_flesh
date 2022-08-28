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
use std::fs::File;
use std::sync::mpsc;
use std::thread;
use std::{f64::consts::PI, vec};

use clap::Parser;

pub struct Image {
    pub height: u16,
    pub width: u16,
    pub frames: u16,
    pub pixels: Vec<u8>,
    point_data: Vec<PointData>,
    cross_distance: f64,
    ellipses: Vec<Ellipse>,
}

#[derive(Clone)]
struct Ellipse {
    centre: Point,
    height: f64,
    width: f64,
    angle: f64,
    curr_point: Point,
    direction: f64,
}

#[derive(Clone)]
struct Point {
    x: i64,
    y: i64,
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
    /// Read the command line arguments
    pub fn read() -> Self {
        Args::parse()
    }
}

impl Image {
    /// Create the basic info for the image from
    /// the command line arguments
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
                x: args.width as i64 - 1,
                y: args.height as i64 - 1,
            }),
            ellipses: generate_points(args.width, args.height, args.num_cells),
        }
    }

    /// Generate the noise for a given frame
    fn generate_noise(&mut self, frame: u16) {
        let mut max_dist = 0.0;

        // set the point on each ellipse to use as the centre of each cell
        let pos = 2.0 * PI * frame as f64 / self.frames as f64;
        for ellipse in &mut self.ellipses {
            let sin_theta = ((ellipse.angle + pos) * ellipse.direction).sin();
            let cos_theta = ((ellipse.angle + pos) * ellipse.direction).cos();
            let a = ellipse.width / 2.0;
            let b = ellipse.height / 2.0;
            let radius =
                (a * b) / (a * a * sin_theta * sin_theta + b * b * cos_theta * cos_theta).sqrt();
            ellipse.curr_point.x =
                (ellipse.centre.x as f64 + radius * (pos * ellipse.direction).sin()) as i64;
            ellipse.curr_point.y =
                (ellipse.centre.y as f64 + radius * (pos * ellipse.direction).cos()) as i64;
        }

        // Get distance and nearest point for each point on the canvas
        for y in 0..self.height {
            for x in 0..self.width {
                let index = y as usize * self.width as usize + x as usize;
                self.point_data[index] = PointData::get_point_data(
                    self,
                    Point {
                        x: x as i64,
                        y: y as i64,
                    },
                );
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
                let red;
                let green;
                let blue;

                if self.point_data[index].min_dist < 0.5 {
                    red = 0xFF - (102.0 * self.point_data[index].min_dist) as u8;
                    blue = 0xFF - (512.0 * self.point_data[index].min_dist) as u8;
                    green = (blue as f64 * 0.8) as u8;
                } else {
                    red = 0xFF - (408.0 * (self.point_data[index].min_dist - 0.5) + 51.0) as u8;
                    green = 0;
                    blue = 0;
                }
                self.set_pixel(
                    red,
                    green,
                    blue,
                    Point {
                        x: x as i64,
                        y: y as i64,
                    },
                );
            }
        }
    }

    /// Set the colour at a given pixel
    fn set_pixel(&mut self, r: u8, g: u8, b: u8, p: Point) {
        self.pixels[3 * (self.width as usize * p.y as usize + p.x as usize)] = r;
        self.pixels[3 * (self.width as usize * p.y as usize + p.x as usize) + 1] = g;
        self.pixels[3 * (self.width as usize * p.y as usize + p.x as usize) + 2] = b;
    }
}

impl PointData {
    /// get closest point and distance to said point
    /// from a given pixel on the frame
    fn get_point_data(image: &Image, p: Point) -> Self {
        let mut pd = PointData {
            min_dist: image.cross_distance,
            closest_point: Point { x: 0, y: 0 },
        };

        // find closest point
        for ellipse in &image.ellipses {
            let d = p.distance(&ellipse.curr_point);
            if d < pd.min_dist {
                pd.min_dist = d;
                pd.closest_point = ellipse.curr_point.clone();
            }
        }

        pd
    }
}

impl Point {
    /// calculates distance between two points
    fn distance(&self, other: &Point) -> f64 {
        let x_dist: f64 = other.x as f64 - self.x as f64;
        let y_dist: f64 = other.y as f64 - self.y as f64;

        (x_dist * x_dist + y_dist * y_dist).sqrt()
    }
}

/// Creates a random ellipse on the frame
fn create_random_ellipse(width: u16, height: u16) -> Ellipse {
    Ellipse {
        // random point on the frame is the centre point
        centre: Point {
            x: rand::thread_rng().gen_range(0..width) as i64,
            y: rand::thread_rng().gen_range(0..height) as i64,
        },
        // height and width of ellipse are capped at 1/5 of the
        // respective dimension of the frame
        height: rand::thread_rng().gen_range(1.0..height as f64 / 5.0),
        width: rand::thread_rng().gen_range(1.0..width as f64 / 5.0),
        // angle determines rotation of the ellipse
        angle: rand::thread_rng().gen_range(0.0..PI),
        // curr point is just set to 0 for now
        curr_point: Point { x: 0, y: 0 },
        // determines clockwise or counter-clockwise direction
        // of points on ellipse
        direction: if rand::thread_rng().gen_range(0..=1) == 1 {
            1.0
        } else {
            -1.0
        },
    }
}

/// Generates a vector of `num_cells` `Ellipse`s that are used to
/// determine the points where cells should be calculated from
fn generate_points(width: u16, height: u16, num_cells: usize) -> Vec<Ellipse> {
    // create vector of 0'd ellipses
    let mut ellipses = vec![
        Ellipse {
            centre: Point { x: 0, y: 0 },
            height: 0.0,
            width: 0.0,
            angle: 0.0,
            curr_point: Point { x: 0, y: 0 },
            direction: 0.0
        };
        num_cells
    ];

    // assign the ellipses values
    for ellipse in &mut ellipses {
        *ellipse = create_random_ellipse(width, height);
    }

    ellipses
}

/// Show a progress bar and percent complete.
/// Should be run in a separate thread from the main
/// to prevent it from blocking the main thread
fn progress_bar(width: u16, max: u16, rx: mpsc::Receiver<u16>) {
    for received in rx {
        let percent_done = received as f64 / max as f64;

        for i in 0..(width as i32 - 6) {
            if i < (percent_done * width as f64) as i32 {
                print!("=");
            } else {
                print!("-");
            }
        }
        print!("[{}%]\r", (percent_done * 100.0) as i32);
    }
}

/// Generate the image itself
pub fn create_image(image: &mut Image, encoder: &mut gif::Encoder<&mut File>) {
    let (tx, rx) = mpsc::channel(); // message passing variables
    let width = termsize::get().unwrap().cols; // the width of the terminal

    // start progress bar thread
    let frames = image.frames;
    let t = thread::spawn(move || progress_bar(width, frames, rx));

    for i in 0..image.frames {
        tx.send(i).unwrap();
        // Create pixel array
        image.generate_noise(i);
        let frame = gif::Frame::from_rgb(image.width, image.height, &mut image.pixels);

        // Write frame to file
        encoder.write_frame(&frame).unwrap();
    }

    // close the progress bar thread
    drop(tx);
    t.join().unwrap();
}
