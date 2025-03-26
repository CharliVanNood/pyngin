use winit::{
    event::*, event_loop::{ControlFlow, EventLoop}, window::{Fullscreen, Icon, Window}
};

#[path="../src/transforms.rs"]
mod transforms;

struct State {
    init: transforms::InitWgpu
}
impl State {
    async fn new(window: &Window) -> Self {
        println!("Creating wgpu binding");
        let init = transforms::InitWgpu::init_wgpu(window).await;

        Self {
            init
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.init.instance.poll_all(true);
            self.init.size = new_size;
            self.init.config.width = new_size.width;
            self.init.config.height = new_size.height;
            self.init.surface.configure(&self.init.device, &self.init.config);
        }
    }

    #[allow(unused_variables)]
    fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        Ok(())
    }
}

pub fn create_window(title: &str) {
    let event_loop = EventLoop::new();
    let window = winit::window::WindowBuilder::new().build(&event_loop).unwrap();
    window.set_title(title);

    let mut window_state = pollster::block_on(State::new(&window));    

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                if !window_state.input(event) {
                    match event {
                        WindowEvent::CloseRequested {} => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            window_state.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            window_state.resize(**new_inner_size);
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    });
}