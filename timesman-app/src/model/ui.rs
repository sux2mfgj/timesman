use egui::{Key, Modifiers};

pub fn consume_escape(ctx: &egui::Context) -> bool {
    ctx.input_mut(|i| i.consume_key(Modifiers::NONE, Key::Escape))
}

pub fn consume_key(ctx: &egui::Context, key: Key) -> bool {
    ctx.input_mut(|i| i.consume_key(Modifiers::NONE, key))
}

pub fn consume_key_with_meta(
    ctx: &egui::Context,
    modi: Modifiers,
    key: Key,
) -> bool {
    ctx.input_mut(|i| i.consume_key(modi, key))
}
