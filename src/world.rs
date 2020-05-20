use super::window::Window;
use gl;

pub struct World<TF> {
    window: Window,
    timer_func: TF,
}

impl<TF: FnMut(f64)> World<TF> {
    pub fn new(title: &str, width: i32, height: i32) -> World<Box<dyn FnMut(f64)>> {
        let window = Window::new(title, width, height);

        World {
            window: window,
            timer_func: Box::new(|_| {}),
        }
    }

    pub fn set_timer_func(&mut self, f: TF) {
        self.timer_func = f;
    }

    pub fn main_loop(mut self) {
        let mut timer_func = self.timer_func;
        self.window.render_loop(|delta| unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::CULL_FACE);
            gl::FrontFace(gl::CW);

            (timer_func)(delta);
        });
    }
}
