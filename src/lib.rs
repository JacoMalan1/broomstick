use glium::{
    glutin::{
        dpi::PhysicalSize,
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
        ContextBuilder,
    },
    Display,
};
use log::error;
use scene::Scene;
use std::time::{Duration, Instant};

pub mod builder;
pub mod graphics;
pub mod scene;
#[cfg(test)]
pub mod tests;

pub struct EngineState<'a, S> {
    current_scene: &'a mut Scene,
    user_state: Option<S>,
}

impl<'a, S: Clone> EngineState<'a, S> {
    pub fn new(scene: &'a mut Scene) -> Self {
        Self {
            current_scene: scene,
            user_state: None,
        }
    }

    pub fn switch_scene(&mut self, scene: &'a mut Scene) {
        self.current_scene = scene;
    }

    pub fn set_user_state(&mut self, user_state: S) {
        self.user_state = Some(user_state);
    }

    pub fn user(&'a self) -> Option<&'a S> {
        self.user_state.as_ref()
    }

    pub fn user_mut(&'a mut self) -> Option<&'a mut S> {
        self.user_state.as_mut()
    }
}

#[allow(dead_code)]
pub struct Engine<I, S>
where
    I: Fn(&Display, &mut Scene) -> S,
    S: Clone + 'static,
{
    tick_cb: fn(&mut EngineState<S>),
    init_cb: I,
    state: EngineState<'static, S>,
}

impl<I, S> Engine<I, S>
where
    I: Fn(&Display, &mut Scene) -> S,
    S: Clone + 'static,
{
    pub fn new(scene: &'static mut Scene, init_cb: I, tick_cb: fn(&mut EngineState<S>)) -> Self {
        Self {
            tick_cb,
            init_cb,
            state: EngineState::new(scene),
        }
    }

    pub fn run(mut self) -> anyhow::Result<()> {
        let wb = WindowBuilder::new().with_inner_size(PhysicalSize::new(1920, 1080));
        let cb = ContextBuilder::new()
            .with_depth_buffer(24)
            .with_multisampling(8);
        let event_loop = EventLoop::new();
        let display = Display::new(wb, cb, &event_loop)?;
        let user_data = (self.init_cb)(&display, &mut *self.state.current_scene);
        self.state.set_user_state(user_data);

        event_loop.run(move |event, _, control_flow| {
            (self.tick_cb)(&mut self.state);

            let mut target = display.draw();
            if let Err(err) = self.state.current_scene.draw_all(&mut target) {
                error!("Couldn't draw scene! {err:?}");
            }

            if let Err(err) = target.finish() {
                error!("Couldn't destroy frame! {err:?}");
            }

            if let Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } = event
            {
                *control_flow = ControlFlow::Exit;
            }

            match *control_flow {
                ControlFlow::Exit => (),
                _ => {
                    display.gl_window().window().request_redraw();
                    *control_flow =
                        ControlFlow::WaitUntil(Instant::now() + Duration::from_nanos(16_666_667));
                }
            }
        });
    }
}

pub struct EngineBuilder {}
