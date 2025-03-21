use egui::{Key, Modifiers};

pub fn consume_escape(ctx: &egui::Context) -> bool {
    ctx.input_mut(|i| i.consume_key(Modifiers::NONE, Key::Escape))
}
