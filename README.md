# Rascii
> @TODO: maybe change this title later

Generate ASCII art from image on the command line.

## Building

1. First, you need to have Rust and Cargo installed. You can install Rust by following the instructions at [rustup.rs](https://rustup.rs).
2. Clone the repository:
```sh
$ git clone https://github.com/derivia
```
3. Build rascii using cargo:
```sh
# inside the cloned folder
$ cargo build
```

## Usage

```sh
Usage: rascii [OPTIONS] <IMAGE_PATH>

Arguments:
  <IMAGE_PATH>  Image to convert into ASCII art

Options:
      --width <WIDTH>                Width of the output [default: 100]
      --aspect-ratio <ASPECT_RATIO>  Aspect ratio correction factor for output [default: 0.5]
      --contrast <CONTRAST>          Contrast adjustment (0.5 to 2.0) [default: 1.0]
      --invert                       Invert colors
      --dense                        Use dense character set
  -h, --help                         Print help
```

## License

[MIT](./LICENSE)
