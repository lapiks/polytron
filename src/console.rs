use miniquad::EventHandler;

use crate::{renderer::{Renderer, RendererData}, game::Game, graphics::Graphics};

// The Polytron console
pub struct Console {
    data: RendererData,
    renderer: Renderer,
    game: Game,
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