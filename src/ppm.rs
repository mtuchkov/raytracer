/**
This module contains the implementation of the PPM image creation.
*/
use std::fs::File;
use std::io::{Error, Write};
use crate::color::Color;
use crate::algebra::{Ray, Vec3};

type Scheme = u8;

struct ColoringScheme;

impl ColoringScheme {
    pub const LERP: u8 = 1;
    pub const SPHERE: u8 = 2;
}

///
/// This function creates the background.
///
pub(crate) fn create_blue_gradient_background(path: String, w: i32, h: i32) -> () {

    // For simplicity, we assume the aspect ratio is 2:1
    assert_eq!(w / h, 2 / 1, "Aspect ratio must be 2:1.");
    assert!(w >= 100, "Width is too small.");
    assert!(h >= 50, "Height is too small.");

    let mut img_file = create_img_file(&path);

    write_header(&mut img_file, &path, w, h);

    let mut img_file = create_img_file(&path);

    write_header(&mut img_file, &path, w, h);

    render_image(&mut img_file, w, h, &path);

    print_status(&mut img_file, &path);
}

/// !!! NOTE !!!
/// Here we demonstrate the power of iterators in Rust.
///
/// What method is doing:
/// We iterate over each line from bottom to top and for each line we iterate
/// over each pixel from left to right and calculate the color of the pixel.
/// Then write the pixel's color to the image file.
///
/// !!! NOTE !!!
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
fn render_image(img: &mut File, w: i32, h: i32, path: &String){

    // This is the lower left corner of the image.
    // Following the way we used to think of 3D coordinate systems in school
    // the point (0, 0, 0) is where the system of coordinates begins.
    // This is the observer's point of view.
    let origin = Vec3::new(0.0, 0.0, 0.0);

    // In 3D system the origin corresponds to the middle point of the screen in front of it.

    // Then screen's lower left corner is at (-2, -1, -1)
    let ll_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);

    // !!! NOTE !!!
    // No 2D creation is happening here, we're just defining the iterator over the 2D array of points.
    // move |x| (x as f32, y as f32) creates a closure that captures the y value from the outer scope.
    let xy_iter = (0..h).into_iter().rev()
        .flat_map(|y| (0..w).into_iter().map(move |x| (x as f32, y as f32)));

    // !!! NOTE !!!
    // The first closure captures the ll_corner, horizontal and vertical values
    // from the outer scope. The captured value refs are copied into the closure by value.
    // This is done automatically by the Rust compiler. The least restrictive trait is used.
    // Hence, the closure implements the Fn trait that can be used multiple times,
    // i.e. for each iteration.

    // `Result` is a monad that implements the `FromIterator` trait.
    // It's `FromIterator` impl allows to collect the results of the iterator into a single Result of Vec<results>
    // or stop on the first error.

    let result: Result<Vec<()>, Error> = xy_iter
        .map(|(x, y)| {
            let u = x / w as f32;
            let v = y / h as f32;
            let direction = &ll_corner + u * &horizontal + v * &vertical;
            let ray = Ray::from(origin.clone(), direction);

            color(ColoringScheme::SPHERE, &ray)
        })
        .map(write_color_to_file(img))
        .collect();

    match result {
        Ok(_) => {
            println!("Image rendering has finished.");
        },
        Err(why) => panic!("Couldn't render the image. [Path {}, Reason: {}]", path, why),
    }
}

fn write_color_to_file(img: &mut File) -> Box<dyn FnMut(Vec3) -> Result<(), Error> + '_> {
    Box::new(|color: Vec3| {
        // normalize the color values to [0, 255] and convert them to integers
        let ir = (255.99 * color.r()) as i32;
        let ig = (255.99 * color.g()) as i32;
        let ib = (255.99 * color.b()) as i32;

        // !!! NOTE !!!
        // Here no heap allocations are happening.
        // No new strings are created. Format is a const string.
        // write! macro writes the format pieces and arguments to the file buffer.
        write!(img, "{} {} {}\n", ir, ig, ib)
    })
}

fn color(scheme: Scheme, r: &Ray) -> Vec3 {
    match scheme {
        ColoringScheme::LERP => background(r),
        ColoringScheme::SPHERE => sphere(r),
        _ => panic!("Unknown coloring scheme {}.", scheme),
    }
}

fn sphere(r: &Ray) -> Vec3 {
    let t = hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let n = (r.point_at(t) - Vec3::new(0.0, 0.0, -1.0)).unit();
        return 0.5 * Vec3::rgb(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
    }
    background(r)
}

fn hit_sphere(center: &Vec3, radius: f32, r: &Ray) -> f32 {
    let oc = r.origin() - center;
    let a = Vec3::dot(r.direction(), r.direction());
    let b = 2.0 * Vec3::dot(&oc, r.direction());
    let c = Vec3::dot(&oc, &oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant > 0.0 {
        (-b - discriminant.sqrt()) / (2.0 * a)
    } else {
        -1.0
    }
}

/// Simple linear interpolation (lerp) of the blue color channel on the Y axis.
/// Remember, the purpose is to learn Rust, never do that in real code.
/// Instead, you could pass the coloring function as a parameter.
fn background(r: &Ray) -> Vec3 {
    let unit_direction = r.direction().unit();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3::rgb(1.0, 1.0, 1.0) + t * Vec3::rgb(0.5, 0.7, 1.0)
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
    // !!! NOTE!!!
    // Again, this is an idiomatic way in Rust to chain IO operations and propagate errors
    // without a need to handle each IO error separately and explicitly.
    let size = img.sync_all()
        .and_then(|_| img.metadata())
        .map(|m| m.len());

    // !!! NOTE !!!
    // Here the size is a `Result` and it is consumed by the match statement.
    // Thus, we can reuse the size variable below in Ok path.
    match size {
        Ok(size) => {
            println!("Image successfully created. [Path {}; File size: {} bytes]", path, size);
        },
        Err(why) => panic!("Couldn't render the image. [Path {}, Reason: {}]", path, why),
    }
}