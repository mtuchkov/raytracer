// LEARN:
// This is an example of a foreign function interface (FFI) used to call a C function from Rust.
// We could use a random number generator from the standard library,
// or use drand48 from the libc crate, but we want to show how to use FFI.
//
// `link` tells Rust to link the math library libm.so or libm.a (like on my Ubuntu system).
// Obviously, you need to have the math library installed in the system.
//
// If the library does not have a conventional prefix `lib` or it has a unusual location
// then you need to tell Rust where to seacrh the library and how to link it.
// There are multiple ways of doing that: the most common is to use the environment variables or
// in the build.rs script.
#[cfg(target_family = "unix")]
#[link(name = "m")]
extern {
    fn drand48() -> f64;
}

/// LEARN:
/// Even though the function is safe to use, we need to mark it as unsafe
/// because it uses FFI to call the function.
pub fn drand48_safe() -> f64 {
    unsafe {
        drand48()
    }
}