use rand::Rng;
use std::vec;

pub struct Config {
    pub height: u16,
    pub width: u16,
    pub frames: u16,
    pub out_file: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        args.next();

        let width: u16 = match args.next() {
            Some(arg) => match arg.parse() {
                Ok(num) => num,
                Err(_) => return Err("Width is not a number"),
            },
            None => return Err("Didn't get a width"),
        };

        let height: u16 = match args.next() {
            Some(arg) => match arg.parse() {
                Ok(num) => num,
                Err(_) => return Err("Height is not a number"),
            },
            None => return Err("Didn't get a height"),
        };

        let frames: u16 = match args.next() {
            Some(arg) => match arg.parse() {
                Ok(num) => num,
                Err(_) => return Err("Frames is not a number"),
            },
            None => return Err("Didn't get a frame count"),
        };

        let out_file = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get an output file"),
        };

        Ok(Config {
            height,
            width,
            frames,
            out_file,
        })
    }
}

#[derive(Clone)]
struct PointData {
    min_dist: f64,
    closest_point: Point,
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

pub struct Gif {
    pub height: u16,
    pub width: u16,
    pub frames: u16,
    pub pixels: Vec<u8>,
    point_data: Vec<PointData>,
    cross_distance: f64,
    points: Vec<Point>,
}

impl Gif {
    pub fn create_from_config(config: &Config, num_cells: usize) -> Self {
        Gif {
            height: config.height,
            width: config.width,
            frames: config.frames,
            pixels: vec![0; config.height as usize * config.width as usize * 3],
            point_data: vec![
                PointData {
                    min_dist: 0.0,
                    closest_point: Point { x: 0, y: 0 }
                };
                config.height as usize * config.width as usize
            ],
            cross_distance: distance(
                &Point { x: 0, y: 0 },
                &Point {
                    x: config.width - 1,
                    y: config.height - 1,
                },
            ),
            points: generate_points(config.width, config.height, num_cells),
        }
    }
}

#[derive(Clone)]
struct Point {
    pub x: u16,
    pub y: u16,
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
