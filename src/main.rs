extern crate glfw;
extern crate gdmath;
extern crate log;
extern crate file_logger;

mod gl {
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

mod backend;
mod mesh;


fn main() {
    println!("Hello, world!");
}
