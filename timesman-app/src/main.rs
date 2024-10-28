mod req;

use req::{Requester, Times};
use eframe::{self, egui::ViewportBuilder};
use std::marker::PhantomData;
use std::convert::From;

struct StartState {}
struct TimesState {}

enum AppGUIState {
    Start(AppGUI<StartState>),
    Times(AppGUI<TimesState>),
}

struct AppGUI <State = StartState> {
    _state: PhantomData<State>
}

impl AppGUI<StartState> {
    fn new() -> Self {
        Self {
            _state: PhantomData
        }
    }

    fn update(&self, ctx: &egui::Context, _frame: &mut eframe::Frame) -> bool {
        let mut r = false;
        egui::TopBottomPanel::top("title").show(ctx, |ui| {
            ui.label("Start");
            if ui.button("move next state").clicked() { 
                r = true;
            }
        });
        r
    }
}

impl From<AppGUI<StartState>> for AppGUI<TimesState> {
    fn from(item: AppGUI) -> Self {
        Self {
            _state: PhantomData,
        }
    }
}

impl AppGUI<TimesState> {
    fn update(&self, ctx: &egui::Context, _frame: &mut eframe::Frame) -> bool {
        egui::TopBottomPanel::top("title").show(ctx, |ui| {
            ui.label("Times");
            if ui.button("move next state").clicked() { }
        });
        false
    }
}

struct TimesManApp {
    gui: AppGUIState,
}

impl TimesManApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            gui: AppGUIState::Start(AppGUI::<StartState>::new()),
        }
    }
}

impl eframe::App for TimesManApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match &self.gui {
            AppGUIState::Start(gui) => {
                if gui.update(ctx, _frame) {
                    self.gui = AppGUIState::Times(AppGUI::<TimesState>::from(gui));
                }
            }
            AppGUIState::Times(gui) => {
                gui.update(ctx, _frame);
            }
        };
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 400.0]),
        ..Default::default()
    };

    eframe::run_native(
        "TimesMan",
        options,
        Box::new(|cc| Ok(Box::<TimesManApp>::new(TimesManApp::new(cc)))),
    )
}
