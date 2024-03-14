use egui::{Align2, Context};

use crate::integration::Controller;

#[derive(Default)]
struct ToolController {
    is_open: bool,
}

impl ToolController {
    fn process_event(&mut self, ctx: &Context) {
        ctx.input(|i| self.is_open = i.key_pressed(egui::Key::C));
    }
    fn show(&self, ctx: &Context) {
        if self.is_open {}
    }
}

pub fn nullus_gui(ctx: &Context, controller: &dyn Controller) {
    egui::Window::new("Control Plane")
        .default_open(true)
        .resizable(true)
        .show(ctx, |ui| {
            let mut input = String::new();
            if ui.add(egui::RadioButton::new(false, "Console")).clicked() {}
        });
    controller.process_events(ctx);
}
