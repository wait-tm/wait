use crate::game::{Camera, Player, PlayerController};
use crate::input::Input;
use crate::renderer::Renderer;
use std::sync::Arc;
use std::time::Instant;
use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, DeviceId, ElementState, MouseButton, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

#[derive(Default)]
pub struct App {
    window: Option<Arc<Window>>,
    renderer: Option<Renderer>,
    player: Player,
    camera: Option<Camera>,
    input: Input,
    last_frame: Option<Instant>,
    player_controller: PlayerController
}

impl ApplicationHandler for App {
    fn resumed(
        &mut self,
        event_loop: &ActiveEventLoop
    ) {
        if self.window.is_some() {
            return;
        }

        let attributes = Window::default_attributes()
            .with_title("engine");

        let window = Arc::new(event_loop
            .create_window(attributes)
            .expect("failed to create window")
        );

        self.input.lock_mouse(&window);

        let window_size = window.inner_size();
        let aspect = window_size.width as f32 / window_size.height as f32;

        let camera = Camera::new(aspect);

        let renderer = pollster::block_on(Renderer::new(
            window.clone(),
            camera.view_projection_matrix(&self.player)
        ));

        self.window = Some(window.clone());
        self.renderer = Some(renderer);
        self.camera = Some(camera);
        self.last_frame = Some(Instant::now());

        window.request_redraw();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent
    ) {
        let Some(window) = self.window.as_ref() else {
            return
        };

        if window.id() != window_id {
            return;
        }

        match event {
            WindowEvent::Resized(size) => {
                if let Some(renderer) = self.renderer.as_mut() {
                    renderer.resize(size.width, size.height);
                }
                if let Some(camera) = self.camera.as_mut() {
                    camera.resize(size.width as f32, size.height as f32);
                }
            }

            WindowEvent::RedrawRequested => {
                if let Some(renderer) = self.renderer.as_mut() {
                    let Some(camera) = self.camera.as_mut() else { return;};
                    let Some(last_frame) = self.last_frame.as_mut() else { return;};

                    let now = Instant::now();
                    let dt = now.duration_since(*last_frame).as_secs_f32();
                    *last_frame = now;

                    self.player_controller.update(
                        &mut self.player,
                        &mut self.input,
                        dt
                    );

                    renderer.render(
                        camera.view_projection_matrix(&self.player)
                    )
                }

                window.request_redraw();
            }

            WindowEvent::KeyboardInput {event, ..} => {
                let PhysicalKey::Code(key) = event.physical_key else { return;};

                if key == KeyCode::Escape && self.input.is_mouse_locked() {
                    self.input.unlock_mouse(window);
                }

                self.input.set_key(key, event.state.is_pressed())
            }

            WindowEvent::MouseInput {state, button, ..} => {
                let ElementState::Pressed = state else { return;};
                let MouseButton::Left = button else { return; };
                if !self.input.is_mouse_locked() {
                    self.input.lock_mouse(window);
                }
            }

            WindowEvent::Focused(false) => {
                if self.input.is_mouse_locked() {
                    self.input.unlock_mouse(window);
                }
            }

            WindowEvent::CloseRequested => {
               event_loop.exit();
            }

            _ => {}
        }
    }

    fn device_event(&mut self, _event_loop: &ActiveEventLoop, _device_id: DeviceId, event: DeviceEvent) {
        if let DeviceEvent::MouseMotion { delta, .. } = event {
            self.input.add_mouse_motion(delta);
        };
    }
}