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
