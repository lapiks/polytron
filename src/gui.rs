pub struct Gui {}

impl Gui {    
    pub fn draw(&mut self, egui_ctx: &egui::Context, game_texture: egui::TextureId) {
        egui::Window::new("MainWindow")
            .constrain(true)
            .title_bar(false)
            .show(egui_ctx, |ui| {
                ui.image(game_texture, egui_ctx.available_rect().size());
            });
    }
}