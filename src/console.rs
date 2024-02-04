use miniquad::EventHandler;

use crate::{renderer::{Renderer, RendererData}, game::Game, graphics::Graphics};

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
                }
            )
        });
    }
}

impl EventHandler for Console {
    fn update(&mut self) {
        if !self.game_init {
            self.game.init();
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

        self.renderer.draw(
            &mut self.data
        );
    }
}