# Bubbles

This program procedurally generates a GIF that resembles a writhing mass
of flesh. Originally, it was intended to be bubbles, but so it goes. 

I strongly recommend compiling in release mode because of the large amount
of processing that has to happen:
```
cargo build --release
```

USAGE:
```
    bubbles --width <WIDTH> --height <HEIGHT> --frames <FRAMES> --num-cells <NUM_CELLS> --out <OUT>
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
