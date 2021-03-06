extern crate native;
extern crate gl;
extern crate glfw;

pub mod gfx {
    pub mod renderer;
    pub mod opengl;
    pub mod window;
    pub mod shaders;
}
mod logic {
    pub mod entity;
}

#[start]
fn start(argc: int, argv: **u8) -> int {
    return native::start(argc, argv, main);
}

fn main() {
    std::io::println("vodk!");
    gfx::window::main_loop();
}
