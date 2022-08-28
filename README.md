# Writhing Mass of Flesh

This program procedurally generates a GIF that resembles a writhing mass
of flesh. Originally, it was intended to be bubbles, but so it goes.
I recommend doing a 500x500 image with 100 cells and 100 frames to
get the best balance of appearance and run time.

The easiest way to download and compile this is simply
```sh
cargo install writhing_mass_of_flesh
```

Alternatively, you can clone the repository and run
```sh
cargo build --release
```
which will put the compiled executable at `./target/release/writhing_mass_of_flesh`

USAGE:
```sh
writhing_mass_of_flesh --width <WIDTH> --height <HEIGHT> --frames <FRAMES> --num-cells <NUM_CELLS> --out <OUT>
```

OPTIONS:
```
    -f, --frames <FRAMES>          number of gif frames
    -h, --height <HEIGHT>          height of the image
        --help                     Print help information
    -n, --num-cells <NUM_CELLS>    number of cells to generate
    -o, --out <OUT>                output file
    -V, --version                  Print version information
    -w, --width <WIDTH>            width of the image
```
