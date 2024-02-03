use miniquad::EventHandler;

use crate::{renderer::Renderer, game::Game, graphics::Graphics};

// The Polytron console
pub struct Console {
    graphics: Graphics,
    renderer: Renderer,
    game: Game,
}

impl Console {
    pub fn boot() {
        let conf = miniquad::conf::Conf::default();

        miniquad::start(conf, move || {
            Box::new(
                Self {
                    graphics: Graphics::new(),
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
        self.game.draw(
            &mut self.graphics
        );

        self.renderer.draw(
            &self.graphics
        );
    }
}