use miniquad::EventHandler;

use crate::{renderer::Renderer, game::Game, graphics::Graphics};

// The Polytron console
pub struct Console {
    renderer: Renderer,
    game: Game,
}

impl Console {
    pub fn boot() {
        let conf = miniquad::conf::Conf::default();

        miniquad::start(conf, move || {
            Box::new(
                Self {
                    renderer: Renderer::new(),
                    game: Game::default(),
                }
            )
        });
    }
}

impl EventHandler for Console {
    fn update(&mut self) {
        self.game.update();
    }

    fn draw(&mut self) {
        self.renderer.draw(
            self.game.draw(
                Graphics::new()
            )
        );
    }
}