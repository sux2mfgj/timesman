use crate::req::{Requester, Times};
use crate::start::StartPane;
use crate::times::TimesPane;
use eframe;
use egui::{FontData, FontDefinitions, FontFamily};

pub enum Event {
    Nothing,
    ToStart,
    OpenTimes(Times),
}

pub trait Pane {
    fn update(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
        req: &Requester,
    ) -> Event;
}

pub struct App {
    req: Requester,
    pane: Box<dyn Pane>,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::config_font(cc);
        let req = Requester::new(&"http://localhost:8080".to_string());
        Self {
            pane: Box::new(StartPane::new(&req)),
            req,
        }
    }

    fn config_font(cc: &eframe::CreationContext<'_>) {
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "ja".to_owned(),
            FontData::from_static(include_bytes!(
                "../fonts/ja/NotoSansJP-VariableFont_wght.ttf"
            )),
        );

        fonts
            .families
            .entry(FontFamily::Proportional)
            .or_default()
            .insert(0, "ja".to_owned());

        cc.egui_ctx.set_fonts(fonts);
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self.pane.update(ctx, _frame, &self.req) {
            Event::Nothing => {}
            Event::OpenTimes(times) => {
                self.pane = Box::new(TimesPane::new(times, &self.req));
            }
            Event::ToStart => {
                self.pane = Box::new(StartPane::new(&self.req));
            }
        }
    }
}
