## A small benchmark program

This repository contains a smalll benchmark program to benchmark FSE and Huffman entropy algorithm speeds over at
[here](https://github.com/etemesi254/zcif)

## Steps to run

Note, you should have installed rust compiler and cargo (preferrably via [rustup](https://rustup.rs/)) and have cargo in your `PATH`

It requires a relatively new rust compiler version, about 1.58.0 and above

Clone this repository and navigate to the directory and then

To test FSE run.


```shell
cargo run --release --  -f [FILE]
```

To test Huffman run.


```shell
cargo run --release --  -z [FILE]
```

Where `[FILE]` is a file to be compressed

The program will load the whole file to memory and compress and decompress it in memory, verify both files match and then print statistics.

It runs compression and decompression 20 times for each file(first results are usually erroneous)
