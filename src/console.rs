use miniquad::EventHandler;

use crate::{renderer::Renderer, game::Game, graphics::Graphics};

// The Polytron console
pub struct Console {
    renderer: Renderer,
}

impl Console {
    pub fn boot() {
        let conf = miniquad::conf::Conf::default();

        miniquad::start(conf, move || {
            Box::new(
                Self {
                    renderer: Renderer::new()
                }
            )
        });
    }

    pub fn load_game(self) -> Self {
        todo!();
    }
}

impl EventHandler for Console {
    fn update(&mut self) {

    }

    fn draw(&mut self) {
        let game = Game {};
        self.renderer.draw(game.draw(Graphics::new()));
    }
}