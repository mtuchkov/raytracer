/**
This module contains the implementation of the PPM image creation.
*/
use std::fs::File;
use std::io::{Error, Write};
use crate::camera::Camera;
use crate::color::Color;
use crate::ffi::drand48_safe;
use crate::material::{Material, Scatterable};
use crate::surfaces::hitable::Hitable;
use crate::surfaces::Surface;
use crate::surfaces::world::World;
use crate::vec::{Ray, Vec3};

///
/// This function creates the background.
///
pub(crate) fn create_image(path: String, w: i32, h: i32) -> () {

    // For simplicity, we assume the aspect ratio is 2:1
    assert_eq!(w / h, 2 / 1, "Aspect ratio must be 2:1.");
    assert!(w >= 100, "Width is too small.");
    assert!(h >= 50, "Height is too small.");

    let mut img_file = create_img_file(&path);

    write_header(&mut img_file, &path, w, h);

    let mut img_file = create_img_file(&path);

    write_header(&mut img_file, &path, w, h);

    render_image(&mut img_file, &path, w, h);

    print_status(&mut img_file, &path);
}

/// LEARN:
/// Here we demonstrate the power of iterators in Rust.
///
/// What method is doing:
/// We iterate over each line from bottom to top and for each line we iterate
/// over each pixel from left to right and calculate the color of the pixel.
/// Then write the pixel's color to the image file.
///
/// LEARN:
/// In contrast to Java's Streams those iterators are Zero Cost Abstractions,
/// meaning the compiler will optimize them away and the cost will be
/// the same as of the `for` loop.
/// Nothing is allocated on the heap, only stack is used, normally all closures are inlined.
/// The space cost of a closure is fn ptr + captured variables but even that
/// can be optimized away if inlined.
///
/// The method also demonstrates the idiomatic Rust way to handle sequential IO operations.
/// Result<Vec<()>, std::io::Error> is a Result type that collects the results
/// of the individual io operations. Result implements the FromIterator trait,
/// so we can use collect() to aggregate the results of the io operations.
/// The iterator will stop at the first error and return it.

/// This function is used to render different images, so it's generic over the color function.
fn render_image(img: &mut File, path: &String, w: i32, h: i32){
    let ns = 100;

    let mut world = World::new();

    world.add(
        Surface::sphere(
            Vec3::new(0.0, 0.0, -1.0),
            0.5,
            Material::lambertian(Vec3::rgb(0.8, 0.3, 0.3))));
    world.add(
        Surface::sphere(
            Vec3::new(0.0, -100.5, -1.0),
            100.0,
            Material::lambertian(Vec3::rgb(0.8, 0.8, 0.0))));
    world.add(
        Surface::sphere(
            Vec3::new(1.0, 0.0, -1.0),
            0.5,
            Material::metal(Vec3::rgb(0.8, 0.6, 0.2), 0.2)));
    world.add(
        Surface::sphere(
            Vec3::new(-1.0, 0.0, -1.0),
            0.5,
            Material::metal(Vec3::rgb(0.8, 0.8, 0.8), 0.8)));

    let camera = Camera::new();

    // LEARN:
    // The closure captures the world and camera values
    // from the outer scope. The captured value refs are copied into the closure by value.
    // This is done automatically by the Rust compiler.
    // Not all closures can run multiple times. E.g. the ones that consume the captured
    // values can run only once. Such closures implement FnOnce trait.
    // Compiler chooses the least restrictive trait that fits the closure.
    // Here the closure implements the Fn trait that can be used multiple times,
    // i.e. for each iteration, which is what we need.
    let render_pixel = |(x, y)| {
        let mut col = Vec3::rgb(0.0, 0.0, 0.0);
        // Antialiasing loop
        for _ in 0..ns {
            let u = (x as f64 + drand48_safe()) as f32 / w as f32;
            let v = (y as f64 + drand48_safe()) as f32 / h as f32;

            let ray = camera.get_ray(u, v);
            col += color(&world, &ray, 0);
        }
        col /= ns as f32;
        col
    };

    // LEARN:
    // No 2D creation is happening here, we're just defining the iterator
    // over the 2D array of points. move |x| (x as f32, y as f32) creates a closure
    // that captures the y value from the outer scope.
    let xy_iter = (0..h).into_iter().rev()
        .flat_map(|y| (0..w).into_iter().map(move |x| (x as f32, y as f32)));

    // LEARN:
    // Note that the last `map` operation returns the `Result<(), Error>` type.
    // The `collect()` is a generic method over the element's type.
    // Compiler uses the impl of the `FromIterator` trait for the `Result` type.
    //
    // `Result`s `FromIterator` impl allows to collect the results of the iterator
    // into a single Result of Vec<results> or stop on the first error.
    //
    // Inspired by the Haskell's `traverse` function for sequences.
    // or in FunctionalJava:
    // <B> Option<Seq<B>> traverseOption(F<A, Option<B>> f){...} in Seq.java
    let result: Result<Vec<()>, Error> = xy_iter
        .map(render_pixel)
        .map(write_color_to_file(img))
        .collect();

    match result {
        Ok(_) => {
            println!("Image rendering has finished.");
        },
        Err(why) => panic!("Couldn't render the image. [Path {}, Reason: {}]", path, why),
    }
}

fn write_color_to_file(img: &mut File) -> impl FnMut(Vec3) -> Result<(), Error> + '_ {
    |color: Vec3| {
        // There is a bug in the book, probably.
        // According to the book the color should be divided by ns.
        // But the image turns to be very dark.
        // let mut col = color / ns;
        // Gamma correction (gamma 2) is applied to the color to make the objects lighter.
        let col = Vec3::new(color.r().sqrt(), color.g().sqrt(), color.b().sqrt());
        // normalize the color values to [0, 255] and convert them to integers
        let ir = (255.99 * col.r()) as i32;
        let ig = (255.99 * col.g()) as i32;
        let ib = (255.99 * col.b()) as i32;

        // LEARN:
        // Here no heap allocations are happening.
        // No new strings are created. Format is a const string.
        // write! macro splits the format, and writes the pieces and arguments
        // to the file buffer.
        // Compare to C++ std::cout << ir << " " << ig << " " << ib << std::endl;
        write!(img, "{} {} {}\n", ir, ig, ib)
    }
}

fn color(w: &World, r: &Ray, recurs_dep: i32) -> Vec3 {
    // 0.001 as a min value is chosen to avoid the
    // shadow acne problem (too white or too dark spots).
    match w.hit(r, 0.001, f32::MAX) {
        Some(hit) => {
            if recurs_dep < 50 {
                match hit.material.scatter(r, hit) {
                    Some((s, a)) => {
                        a * color(w, &s, recurs_dep + 1)
                    },
                    None => Vec3::zero(),
                }
            } else {
                Vec3::zero()
            }
        },
        None => background(r),
    }
}

/// Simple linear interpolation of the blue color channel on the Y axis.
fn background(r: &Ray) -> Vec3 {
    let unit_direction = r.direction().unit();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3::basis() + t * Vec3::rgb(0.5, 0.7, 1.0)
}

fn write_header(img_file: &mut File, path: &String, width: i32, height: i32) {
    match write!(img_file, "P3\n{} {}\n255\n", width, height) {
        Ok(_) => println!("Image header written successfully."),
        Err(why) => panic!("Couldn't write header to image. [Path {}, Reason: {}]", path, why),
    }
}

fn create_img_file(path: &String) -> File {
    match File::create(&path) {
        Ok(file) => file,
        Err(why) => panic!("Couldn't create image. [Path {}, Reason: {}]", path, why),
    }
}

fn print_status(img: &mut File, path: &String) {
    // LEARN:
    // Again, this is an idiomatic way in Rust to chain IO operations and propagate errors
    // without a need to handle each IO error separately and explicitly.
    let size = img.sync_all()
        .and_then(|_| img.metadata())
        .map(|m| m.len());

    // LEARN:
    // Here the size is a `Result` and it is consumed by the match statement.
    // Thus, we can reuse the size variable below in Ok path.
    match size {
        Ok(size) => {
            println!("Image successfully created. [Path {}; File size: {} bytes]", path, size);
        },
        Err(why) => panic!("Couldn't render the image. [Path {}, Reason: {}]", path, why),
    }
}