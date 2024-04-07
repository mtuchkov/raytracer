# Learning Rust with Ray Tracing in a Weekend

This is a Rust implementation of the Ray Tracing in a Weekend 
book series by Peter Shirley. The original book series is written in C++, 
but I wanted to learn Rust, so I decided to implement it in Rust.

The goal of this project is to demonstrate how to write idiomatic Rust code,
Rust's memory management, framework, tools, how to work with 
parallelism in Rust, futures, and async/await.

People with C++ background will find this project useful to learn Rust as 
they can compare the C++ code from the book series with the Rust code.

There are probably a plethora of attempts to implement this book series in Rust
available on the internet, but I wanted to do it myself from scratch to avoid 
being spoiled by other implementations.

In the code you will find comments that explain some of the Rust concepts
that I have found interesting to pay attention to. All of them are marked 
with the `// LEARN` comment.

Some parts of the code are intentionally done in not the most simple way,
as the goal is to demonstrate different ways of doing things in Rust, 
beyond the standard ways of doing the same thing in other languages.

For the demonstration purposes I plan to add the following beyond 
the original book series content:
- parallel execution of the ray tracer,
- implement the ray tracer as a web service.