use core::fmt;
use egui::Vec2;
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::runtime;
use tokio::sync::Mutex;

use crate::config::Config;
use crate::log::LogRecord;
use crate::pane::config::ConfigPane;
use crate::pane::log::LogPane;
use crate::pane::select::SelectPane;
use crate::pane::start::StartPane;
use crate::pane::times::TimesPane;
use crate::pane::Pane;

use eframe;
use timesman_bstore::Store;
use timesman_type::Times;

pub enum UIOperation {
    ChangeScale(f32),
    ChangeWindowSize(f32, f32),
}

pub enum Event {
    Connect(Arc<Mutex<Box<dyn Store + Send + Sync + 'static>>>),
    Select(Arc<Mutex<Box<dyn Store + Send + Sync + 'static>>>, Times),
    Pop,
    Logs,
    Config,
    UpdateConfig(Config),
    ChangeUI(UIOperation),
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Event::Connect(_) => {
                write!(f, "Connect")
            }
            Event::Select(_, _) => {
                write!(f, "Disconnect")
            }
            Event::Pop => {
                write!(f, "Pop")
            }
            Event::Logs => {
                write!(f, "Logs")
            }
            Event::Config => {
                write!(f, "Config")
            }
            Event::UpdateConfig(_) => {
                write!(f, "Update config")
            }
            Event::ChangeUI(_op) => {
                write!(f, "change ui: WIP")
            }
        }
    }
}

pub struct App {
    pane_stack: VecDeque<Box<dyn Pane>>,
    logs: Arc<std::sync::Mutex<Vec<LogRecord>>>,
    config: Config,
    rt: runtime::Runtime,
    event_queue: VecDeque<Event>,
}

impl App {
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        config: Config,
        logs: Arc<std::sync::Mutex<Vec<LogRecord>>>,
    ) -> Result<Self, String> {
        let mut stack: VecDeque<Box<dyn Pane>> = VecDeque::new();
        // stack.push_front(Box::new(StartPane::new(config.clone())));
        {
            use crate::pane::test::TestPane;
            stack.push_front(Box::new(TestPane::new()));
        }

        config.fonts.load_fonts(cc);

        let mut event_queue = VecDeque::new();
        config.append_init_events(&mut event_queue);

        Ok(Self {
            pane_stack: stack,
            logs,
            config,
            rt: runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap(),
            event_queue,
        })
    }

    fn handle_events(&mut self, event: Event, ctx: &egui::Context) {
        match event {
            Event::Connect(store) => {
                self.pane_stack
                    .push_front(Box::new(SelectPane::new(store, &self.rt)));
            }
            Event::Select(store, times) => self
                .pane_stack
                .push_front(Box::new(TimesPane::new(store, times, &self.rt))),
            Event::Pop => {
                self.pane_stack.pop_front();
                let p: &mut Box<dyn Pane> = match self.pane_stack.front_mut() {
                    Some(p) => p,
                    None => {
                        return;
                    }
                };

                p.reload(&self.rt);
            }
            Event::Logs => {
                self.pane_stack
                    .push_front(Box::new(LogPane::new(self.logs.clone())));
            }
            Event::Config => {
                self.pane_stack
                    .push_front(Box::new(ConfigPane::new(self.config.clone())));
            }
            Event::UpdateConfig(config) => {
                self.config = config;
                self.config
                    .store_config()
                    .map_err(|e| {
                        error!(format!("{}", e));
                        format!("{e}")
                    })
                    .unwrap();
            }
            Event::ChangeUI(op) => {
                self.change_ui(ctx, op);
            }
        }
    }

    fn change_ui(&self, ctx: &egui::Context, op: UIOperation) {
        match op {
            UIOperation::ChangeScale(scale) => {
                ctx.set_zoom_factor(scale);
            }

            UIOperation::ChangeWindowSize(h, w) => {
                let size = Vec2::new(h, w);
                ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(size));
            }
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let pane: &mut Box<dyn Pane> = match self.pane_stack.front_mut() {
            Some(pane) => pane,
            None => {
                unimplemented!("shoud close app");
            }
        };

        match pane.update(ctx, _frame, &self.rt) {
            Some(event) => {
                self.event_queue.push_back(event);
            }
            None => {}
        };

        loop {
            if let Some(event) = self.event_queue.pop_front() {
                debug!("Handle Event: {}", event);
                self.handle_events(event, ctx);
            } else {
                break;
            }
        }
    }
}
