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
use std::env;
use std::fs::File;
use std::process;

use bubbles::Config;
use bubbles::Gif;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Error printing arguments: {err}");
        process::exit(1);
    });

    // create Gif data
    let mut gif = Gif::create_from_config(&config, 100);

    // Create encoder
    let mut image = File::create(config.out_file).unwrap();
    let mut encoder = gif::Encoder::new(&mut image, config.width, config.height, &[]).unwrap();

    // Repeat infinitely
    if let Err(_) = encoder.set_repeat(gif::Repeat::Infinite) {
        process::exit(1);
    }

    // Create pixel array
    bubbles::fill_canvas(&mut gif);
    let frame = gif::Frame::from_rgb(gif.width, gif.height, &mut gif.pixels);

    // Write frame to file
    encoder.write_frame(&frame).unwrap();
}
