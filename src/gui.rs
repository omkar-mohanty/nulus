use egui::{Align2, Context};

pub fn nullus_gui(ctx: &Context) {
    egui::Window::new("Nullus")
        .default_open(true)
        .max_width(1000.0)
        .max_height(800.0)
        .default_width(800.0)
        .resizable(true)
        .anchor(Align2::LEFT_TOP, [0.0, 0.0])
        .show(&ctx, |ui| {
            if ui.add(egui::Button::new("Open")).clicked() {}
            ui.end_row();

            // proto_scene.egui(ui);
        });
}
