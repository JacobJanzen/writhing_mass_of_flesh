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
use std::fs::File;
use std::process;
use std::sync::mpsc;
use std::thread;

use bubbles::Args;
use bubbles::Image;

fn main() {
    let args = Args::read();
    let (tx, rx) = mpsc::channel();
    let width = termsize::get().unwrap().cols;

    // create Gif data
    let mut data = Image::create_from_args(&args);

    // Create encoder
    let mut image = File::create(args.out).unwrap();
    let mut encoder = gif::Encoder::new(&mut image, args.width, args.height, &[]).unwrap();

    // Repeat infinitely
    if let Err(_) = encoder.set_repeat(gif::Repeat::Infinite) {
        process::exit(1);
    }

    thread::spawn(move || {
        for received in rx {
            let percent_done = received as f64 / data.frames as f64;

            for i in 0..(width as i32 - 5) as i32 {
                if i < (percent_done * width as f64) as i32 {
                    print!("=");
                } else {
                    print!("-");
                }
            }
            print!("[{}%]\r", (percent_done * 100.0) as i32);
        }
    });

    for i in 0..data.frames {
        tx.send(i).unwrap();
        // Create pixel array
        data.fill_canvas(i);
        let frame = gif::Frame::from_rgb(data.width, data.height, &mut data.pixels);

        // Write frame to file
        encoder.write_frame(&frame).unwrap();
    }
}
