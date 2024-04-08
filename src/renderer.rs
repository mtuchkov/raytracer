/**
This module contains the implementation of the PPM image creation.
*/
use std::fs::File;
use std::io::{Error, Write};
use std::path::Path;
use std::time::Instant;

use crate::math::color::Color;
use crate::math::rand::drand32;
use crate::math::vec::{Ray, Vec3};
use crate::scene::camera::{RaySource};
use crate::scene::material::Scatterable;
use crate::scene::Scene;
use crate::scene::surfaces::hitable::Hitable;
use crate::scene::surfaces::world::World;

///
/// This is the main function to render the scene directly to the file.
///
pub(crate) fn render_scene(scene: &Scene, path: &Path) -> Result<(), Error> {

    let now = Instant::now();

    // LEARN:
    // The ? is a shortcut for the match statement that returns the error
    // we could write match File::create(&path) { Ok(file) => file, Err(why) => return Err(why) }
    // but look how much cleaner the code is with the ? operator.
    // The ? operator can be used in functions that return Result type.
    let mut img_file = File::create(&path)?;

    write!(img_file, "P3\n{} {}\n255\n", scene.w, scene.h)?;

    render_to_file(scene, &mut img_file)?;

    img_file.sync_all()?;

    let size = img_file.metadata()?.len();

    // LEARN:
    // The idiomatic way to control how long the file is open is to use a scope { }.

    println!("File size {} bytes. Render time {} secs", size, now.elapsed().as_secs());

    Ok(())
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
fn render_to_file(scene: &Scene, img: &mut File) -> Result<(), Error> {
    let ns = 100;

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
            let u = (x + drand32()) / scene.w as f32;
            let v = (y + drand32()) / scene.h as f32;

            let ray = scene.camera().get_ray(u, v);
            col += color(scene.world(), &ray, 0);
        }
        col /= ns as f32;
        col
    };

    // LEARN:
    // No 2D creation is happening here, we're just defining the iterator
    // over the 2D array of points. move |x| (x as f32, y as f32) creates a closure
    // that captures the y value from the outer scope.
    let xy_iter = (0..scene.h).into_iter().rev()
        .flat_map(|y| (0..scene.w).into_iter().map(move |x| (x as f32, y as f32)));

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

    result.map(|_| ())
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