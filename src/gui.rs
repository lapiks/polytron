pub struct Gui {}

impl Gui {    
    pub fn draw(&mut self, egui_ctx: &egui::Context, game_texture: egui::TextureId) {
        egui::Window::new("Polytron").show(egui_ctx, |ui| {
            ui.image(game_texture, egui::Vec2::new(1000.0, 1000.0));
        });
    }
}