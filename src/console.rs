use glam::uvec2;
use miniquad::EventHandler;

use crate::{game::Game, graphics::Graphics, gui::Gui, inputs::Inputs, renderer::{Renderer, RendererData}};

pub trait System {
    fn init(&mut self);
    fn update(&mut self, inputs: &Inputs);
    fn draw(&self, g: &mut Graphics);
    fn mouse_motion(&mut self, x: f32, y: f32) {}
    fn mouse_wheel(&mut self, dx: f32, dy: f32) {}
    fn mouse_button_down(&mut self, mb: miniquad::MouseButton, x: f32, y: f32) {}
    fn mouse_button_up(&mut self, mb: miniquad::MouseButton, x: f32, y: f32) {}
    fn key_down(&mut self, keycode: miniquad::KeyCode, keymods: miniquad::KeyMods, _repeat: bool) {}
    fn key_up(&mut self, keycode: miniquad::KeyCode, keymods: miniquad::KeyMods) {}
}

// The Polytron console
pub struct Console {
    data: RendererData,
    renderer: Renderer,
    game: Game,
    game_init: bool,
    gui: Gui,
    inputs: Inputs,
}

impl Console {
    pub fn boot() {
        let conf = miniquad::conf::Conf {
            fullscreen: true,
            ..Default::default()
        };

        miniquad::start(conf, move || {
            Box::new(
                Self {
                    data: RendererData::new(),
                    renderer: Renderer::new(),
                    game: Game::new(),
                    game_init: false,
                    gui: Gui {},
                    inputs: Inputs::new(),
                }
            )
        });
    }
}

impl EventHandler for Console {
    fn update(&mut self) {
        if !self.game_init {
            self.game.init();
            self.game_init = true;
        }

        self.game.update(&self.inputs);
        self.inputs.reset();
    }

    fn draw(&mut self) {
        self.data.begin_frame();

        self.game.draw(
            &mut Graphics {
                data: &mut self.data
            }
        );
        self.renderer.draw(&mut self.data);
        //self.renderer.draw_ui(&mut self.gui);
        self.renderer.commit_frame();
    }

    fn resize_event(&mut self, _width: f32, _height: f32) {
        self.renderer.set_screen_resolution(uvec2(_width as u32, _height as u32));
    }

    fn mouse_motion_event(&mut self, x: f32, y: f32) {
        self.inputs.mouse_motion_event(x, y);
        self.game.mouse_motion(x, y);
        self.renderer
        .egui_mq_mut()
        .mouse_motion_event(x, y);
    }

    fn mouse_wheel_event(&mut self, dx: f32, dy: f32) {
        self.game.mouse_wheel(dx, dy);
        self.renderer
        .egui_mq_mut()
        .mouse_wheel_event(dx, dy);
    }

    fn mouse_button_down_event(&mut self, mb: miniquad::MouseButton, x: f32, y: f32) {
        self.game.mouse_button_down(mb, x, y);
        self.renderer
        .egui_mq_mut()
        .mouse_button_down_event(mb, x, y);
    }

    fn mouse_button_up_event(&mut self, mb: miniquad::MouseButton, x: f32, y: f32) {
        self.game.mouse_button_up(mb, x, y);
        self.renderer
        .egui_mq_mut()
        .mouse_button_up_event(mb, x, y);
    }

    fn char_event(&mut self, character: char, _keymods: miniquad::KeyMods, _repeat: bool) {
        self.renderer
        .egui_mq_mut()
        .char_event(character);
    }

    fn key_down_event(&mut self, keycode: miniquad::KeyCode, keymods: miniquad::KeyMods, _repeat: bool) {
        self.inputs.key_down_event(keycode);
        self.game.key_down(keycode, keymods, _repeat);
        self.renderer
        .egui_mq_mut()
        .key_down_event(keycode, keymods);
    }

    fn key_up_event(&mut self, keycode: miniquad::KeyCode, keymods: miniquad::KeyMods) {
        self.inputs.key_up_event(keycode);
        self.game.key_up(keycode, keymods);
        self.renderer
        .egui_mq_mut()
        .key_up_event(keycode, keymods);
    }
}