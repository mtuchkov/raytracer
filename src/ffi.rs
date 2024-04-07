/// LEARN:
/// This is an example of a foreign function interface (FFI) used to call a C function from Rust.
/// We could use a random number generator from the standard library,
/// or use drand48 from the libc crate, but we want to show how to use FFI.
#[cfg(target_family = "unix")]
#[link(name = "m")]
extern {
    fn drand48() -> f64;
}

pub fn drand48_safe() -> f64 {
    unsafe {
        drand48()
    }
}