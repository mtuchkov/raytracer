// LEARN:
// The module structure is very similar to Node.js.
// It is a DAG of modules where the root is the main.rs file.
// Because compiler enforces the DAG structure, there are no circular dependencies,
// and there is no need for #ifndev/#define like in C++.
//
// What's even more important the IDE can easily navigate through the modules
// and highlight the errors following the language specification.
pub mod vec;
pub mod color;
pub mod rand;
