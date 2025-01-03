use std::{cell::RefCell, rc::Rc};

use super::Pane;
use eframe::egui::ScrollArea;
use egui::{Key, Modifiers};

enum UILabelCallback {
    // Wasm, //TODO
    Code(Box<dyn Fn() -> String>),
}

enum UILabel {
    Text(String),
    Callback(UILabelCallback),
}

enum UIButtonCallback {
    // Wasm, //TODO
    Callback(Box<dyn Fn()>),
}

enum UIEdit {
    // Wasm, //TODO
    Callback(Box<dyn Fn(&mut egui::Ui)>),
}

enum UIInput {
    // Wasm,//TODO
    Callback(Modifiers, Key, Box<dyn Fn()>),
}

enum ScrollType {
    Horizontal(Vec<UIComponent>),
    Vertical(Vec<UIComponent>),
}

enum UIComponent {
    Label(UILabel),
    Seperator,
    Button(UILabel, UIButtonCallback),
    Horizontal(Vec<UIComponent>),
    EditSingleLine(UIEdit),
    Input(UIInput),
    ScrollArea(ScrollType),
}

pub struct TestPane {
    ui: Rc<RefCell<TestUI>>,
    top: Vec<UIComponent>,
    // bottom: Vec<UIComponent>,
    // central: Vec<UIComponent>,
}

impl Pane for TestPane {
    fn update(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
        rt: &tokio::runtime::Runtime,
    ) -> Option<crate::app::Event> {
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            for c in &self.top {
                Self::handle_components(ui, c);
            }
        });

        None
    }

    fn reload(&mut self, _rt: &tokio::runtime::Runtime) {}

    fn times_menu(&self, _ui: &mut egui::Ui) -> Option<crate::app::Event> {
        None
    }
    fn show_latest_log(&self, _ui: &mut egui::Ui) {}
    fn times_menu_content(
        &self,
        _ui: &mut egui::Ui,
    ) -> Option<crate::app::Event> {
        None
    }
}

impl TestPane {
    pub fn new() -> Self {
        let mut top = vec![];

        let ui = Rc::new(RefCell::new(TestUI::new()));

        let mut hori = vec![];
        {
            let ui2 = ui.clone();

            let test_label_closure = Box::new(move || {
                let mut ui = ui2.borrow_mut();
                ui.get_test_label()
            });

            let test_label = UIComponent::Label(UILabel::Callback(
                UILabelCallback::Code(test_label_closure),
            ));

            hori.push(test_label);
        }

        {
            let ui2 = ui.clone();
            let label = UILabel::Text("incr".to_string());
            let cb = Box::new(move || {
                let mut ui = ui2.borrow_mut();
                ui.incr();
            });
            let callback = UIButtonCallback::Callback(cb);
            let test_button = UIComponent::Button(label, callback);
            hori.push(test_button);
        }

        {
            let ui2 = ui.clone();

            let label = UILabel::Text("reset".to_string());
            let cb = Box::new(move || {
                let mut ui = ui2.borrow_mut();
                ui.reset();
            });
            let callback = UIButtonCallback::Callback(cb);
            let test_button = UIComponent::Button(label, callback);
            hori.push(test_button);
        }

        top.push(UIComponent::Horizontal(hori));

        {
            let ui = ui.clone();

            let cb = Box::new(move |egui_ui: &mut egui::Ui| {
                let mut ui = ui.borrow_mut();
                egui_ui.text_edit_singleline(&mut ui.edit_text);
            });
            top.push(UIComponent::EditSingleLine(UIEdit::Callback(cb)));
        }

        Self { top, ui }
    }

    fn handle_components(ui: &mut egui::Ui, component: &UIComponent) {
        match component {
            UIComponent::Label(label) => TestPane::handle_uilabel(label, ui),

            UIComponent::Seperator => {
                ui.separator();
            }
            UIComponent::Button(label, callback) => {
                let label = match label {
                    UILabel::Text(label) => label.clone(),
                    UILabel::Callback(cb) => match cb {
                        // UILabelCallback::Wasm => {
                        //     unimplemented!();
                        // }
                        UILabelCallback::Code(cb) => cb(),
                    },
                };

                let cb = match callback {
                    // UIButtonCallback::Wasm => {
                    //     unimplemented!();
                    // }
                    UIButtonCallback::Callback(cb) => cb,
                };

                if ui.button(label).clicked() {
                    cb();
                }
            }
            UIComponent::Horizontal(comps) => {
                ui.horizontal(|ui| {
                    for c in comps {
                        Self::handle_components(ui, c);
                    }
                });
            }
            UIComponent::EditSingleLine(edit) => match edit {
                UIEdit::Callback(cb) => {
                    cb(ui);
                }
            },
            UIComponent::Input(input) => match input {
                UIInput::Callback(modifier, key, cb) => {
                    if ui.input_mut(|i| i.consume_key(*modifier, *key)) {
                        cb();
                    }
                }
            },
            UIComponent::ScrollArea(scroll_type) => {
                let (scroll_area, comps) = match scroll_type {
                    ScrollType::Horizontal(comps) => {
                        let scroll_area = ScrollArea::vertical()
                            .auto_shrink(false)
                            .max_height(ui.available_height());
                        (scroll_area, comps)
                    }
                    ScrollType::Vertical(comps) => {
                        let scroll_area = ScrollArea::vertical()
                            .auto_shrink(false)
                            .max_height(ui.available_height());
                        (scroll_area, comps)
                    }
                };
                scroll_area.show(ui, |ui| {
                    for c in comps {
                        Self::handle_components(ui, c);
                    }
                });
            }
        }
    }

    fn handle_uilabel(label: &UILabel, ui: &mut egui::Ui) {
        match label {
            UILabel::Text(text) => {
                ui.label(text);
            }
            UILabel::Callback(callback) => match callback {
                // UILabelCallback::Wasm => {
                //     unimplemented!();
                // }
                UILabelCallback::Code(cb) => {
                    // debug!("call the callback");
                    ui.label(cb());
                }
            },
        }
    }
}

struct TestUI {
    pub test_val: u64,
    pub text: String,
    pub edit_text: String,
}

impl TestUI {
    fn new() -> Self {
        Self {
            test_val: 0,
            text: "hello".to_string(),
            edit_text: String::new(),
        }
    }

    fn get_test_label(&self) -> String {
        format!("hello world {}", self.test_val)
    }

    fn incr(&mut self) {
        self.test_val += 1;
    }

    fn reset(&mut self) {
        self.test_val = 0;
    }
}
