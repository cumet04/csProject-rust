mod window;
use window::Window;

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

fn main() {
    let mut window = Window::new("csProject-rust", SCR_WIDTH,SCR_HEIGHT);

    window.render_loop(|| {

    });
}
