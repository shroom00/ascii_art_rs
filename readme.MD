# ASCII-art-rs

A simple Rust commandline program to make ASCII art. It uses Nannou and so requires a desktop.

## Usage

Usage is as follows: `ascii_art_rs.exe -i img.jpg -o img_ascii.png -s 10000 --height 1000 --chars abcdefghijklmnopqrstuvwxyz`.

```
-i <FILE>                 The path to the input image.
-o <FILE>                 The path where the ASCII art will be saved. [default: ascii_art.png]
-s <NUMBER>               How many letters to place. [default: 500]
    --width <NUMBER>      The width of the output image.
    --height <NUMBER>     The height of the output image. [default: 1000]
    --chars <CHARACTERS>  The characters you want to be used in output image. (Defaults to visible ASCII characters)
    --colour              Uses accurate colours when specified, instead of grayscale.
-h, --help                Print help
-V, --version             Print version
```

### Usage Notes

The only required argument is `-i`.

`--colour` takes no values, it's a boolean flag.

The width/height arguments are mutually exclusive. This is because the aspect ratio of the image is preserved, so the height is dependent on the width and vice versa.

## Example Output

![Example Image](/example.png)

This was obtained with the following command, using [this](https://commons.wikimedia.org/wiki/File:Mona_Lisa,_by_Leonardo_da_Vinci,_from_C2RMF_retouched.jpg) image (687x1024 version) as input: `ascii_art_rs.exe -i mona_lisa.jpg -s 15000 --height 1024 -o example.png`

## TODO

- If the output width/height is larger than the desktop width/height, scale the window size so it fits entirely on the desktop.
- ~~Add an argument for accurate colours, as opposed to grayscale.~~
- Add a non-GUI argument for true commandline use.
- Add arguments for minimum and maximum character sizes.
- Add an argument to place characters in grids/cells, rather than random placement.
- Add an argument to output a video by capturing all frames
- Set a more reasonable default value for `steps`, so that it varies depending on the image size and stops output images from being too sparse/dense.
