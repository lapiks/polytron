use miniquad::EventHandler;

use crate::{game::Game, graphics::Graphics, gui::Gui, renderer::{Renderer, RendererData}};

pub trait System {
    fn init(&mut self);
    fn update(&mut self);
    fn draw(&self, g: Graphics);
}

// The Polytron console
pub struct Console {
    data: RendererData,
    renderer: Renderer,
    game: Game,
    game_init: bool,
    gui: Gui,
}

impl Console {
    pub fn boot() {
        let conf = miniquad::conf::Conf::default();

        miniquad::start(conf, move || {
            Box::new(
                Self {
                    data: RendererData::new(),
                    renderer: Renderer::new(),
                    game: Game::new(),
                    game_init: false,
                    gui: Gui {}
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

        self.game.update();
    }

    fn draw(&mut self) {
        self.data.begin_frame();

        self.game.draw(
            Graphics {
                data: &mut self.data
            }
        );
        self.renderer.draw(&mut self.data);
        //self.renderer.draw_ui(&mut self.gui);
        self.renderer.commit_frame();
    }

    fn mouse_motion_event(&mut self, x: f32, y: f32) {
        self.renderer
        .egui_mq_mut()
        .mouse_motion_event(x, y);
    }

    fn mouse_wheel_event(&mut self, dx: f32, dy: f32) {
        self.renderer
        .egui_mq_mut()
        .mouse_wheel_event(dx, dy);
    }

    fn mouse_button_down_event(&mut self, mb: miniquad::MouseButton, x: f32, y: f32) {
        self.renderer
        .egui_mq_mut()
        .mouse_button_down_event(mb, x, y);
    }

    fn mouse_button_up_event(&mut self, mb: miniquad::MouseButton, x: f32, y: f32) {
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
        self.renderer
        .egui_mq_mut()
        .key_down_event(keycode, keymods);
    }

    fn key_up_event(&mut self, keycode: miniquad::KeyCode, keymods: miniquad::KeyMods) {
        self.renderer
        .egui_mq_mut()
        .key_up_event(keycode, keymods);
    }
}