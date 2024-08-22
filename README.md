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

Program able to generate basic matte shadows.

## Features

- [X] Add Lambertian Reflection
- [X] Allow materials to be colored
- [X] Add new materials
  - [X] Add metals
  - [X] Add fuzzy metals
  - [X] Add glass metals
- [ ] Make the camera adjustable
- [ ] Make a threaded computation mode
- [ ] Make a wgpu mode

## Outputs

### Shading by Normal

![Original program image output](./image_archive/shade_by_normal.jpeg)

![Fixed aspect ratio](./image_archive/fixed_aspect_ratio.jpeg)

### With multiple objects

![Four sphere objects being displayed](./image_archive/fixed_faulty_hit_logic.jpeg)

### With Anti-Aliasing

![Anti-aliased output](./image_archive/antialiased.jpeg)

### Basic Working Matt Shadows

![Basic Shadows](./image_archive/basic_shadows.jpeg)

### Other materials

![Metal materials and Lambertian materials in the same scene (with colors)](./image_archive/materials.jpeg)

#### Fuzzy metals

![Now with fuzziness on the metals](./image_archive/fuzzy_metal.jpeg)

#### Hollow glass

![Hollow glass sphere](./image_archive/hollow_glass.jpeg)

### Camera Adjustments

#### High angle with 30 vfov

![High angle down view of the balls](./image_archive/high_angle_down.jpeg)

## Resources

[_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html)
