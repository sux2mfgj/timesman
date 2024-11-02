pub mod log;
pub mod start;
pub mod times;

use crate::app::Event;
use crate::req::Requester;
use egui;

pub trait Pane {
    fn update(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
        req: &Requester,
    ) -> Event;
}
