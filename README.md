# ray-tracing-weekend

Ray tracing in one weekend, but it is in Rust. This will not finish in a single weekend.

## Running the program

Currently the program will just spit out ppm format image into the terminal. If
you actually wish to view the image, please pipe the output into a file.

### Example of piping the output into a file

Here is an example of how to run the program, and save it to a file called `image.ppm`.

```bash
cargo run > image.ppm
```

## Program Capabilities

Currently, this program will generate a sphere in the middle of a vertical
gradient that is shaded based on the surface normal vectors.

## Outputs

### Shading by Normal

![Original program image output](./image_archive/shade_by_normal.jpeg)

![Fixed aspect ratio](./image_archive/fixed_aspect_ratio.jpeg)
