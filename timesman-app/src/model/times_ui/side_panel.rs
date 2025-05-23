use std::collections::HashMap;

use egui::ComboBox;
use timesman_type::{Tag, TagId, Todo};

use super::UIRequest;

#[derive(Copy, Clone)]
pub enum SidePanelType {
    Todo,
    Tag,
    TagAssigne,
}

pub struct SidePanel {
    ptype: Option<SidePanelType>,
    new: String,
    selected_tagid: u64,
    pub selected_tag: Option<Tag>,
}

impl SidePanel {
    pub fn new() -> Self {
        Self {
            ptype: None,
            new: "".to_string(),
            selected_tagid: 0,
            selected_tag: None,
        }
    }

    pub fn update(
        &mut self,
        ctx: &egui::Context,
        todo: &Vec<Todo>,
        tags: &HashMap<TagId, Tag>,
        ureq: &mut Vec<UIRequest>,
    ) {
        let Some(ptype) = self.ptype else {
            return;
        };

        match ptype {
            SidePanelType::Todo => {
                self.update_todo(ctx, todo, ureq);
            }
            SidePanelType::Tag => {
                self.update_tag(ctx, tags, ureq);
            }
            SidePanelType::TagAssigne => {
                self.update_assign_tag(ctx, tags, ureq);
            }
        }
    }

    fn update_todo(
        &mut self,
        ctx: &egui::Context,
        todos: &Vec<Todo>,
        ureq: &mut Vec<UIRequest>,
    ) {
        egui::SidePanel::right("todo").show(ctx, |ui| {
            ui.label("Todo List");

            for todo in todos {
                let mut done = todo.done_at.is_some();

                let resp = ui.checkbox(&mut done, &todo.content);

                if resp.clicked() {
                    ureq.push(UIRequest::TodoDone(todo.id, done));
                }
            }

            ui.text_edit_singleline(&mut self.new);
            if !self.new.is_empty()
                && ui.input(|i| i.key_pressed(egui::Key::Enter))
            {
                ureq.push(UIRequest::Todo(self.new.clone()))
            }
        });
    }

    fn update_tag(
        &mut self,
        ctx: &egui::Context,
        tags: &HashMap<TagId, Tag>,
        ureq: &mut Vec<UIRequest>,
    ) {
        egui::SidePanel::right("tag").show(ctx, |ui| {
            ui.label("Tag List");

            for (_, tag) in tags {
                ui.label(&tag.name);
            }

            ui.text_edit_singleline(&mut self.new);
            if !self.new.is_empty()
                && ui.input(|i| i.key_pressed(egui::Key::Enter))
            {
                ureq.push(UIRequest::Tag(self.new.clone()));
            }
        });
    }

    fn update_assign_tag(
        &mut self,
        ctx: &egui::Context,
        tags: &HashMap<TagId, Tag>,
        ureq: &mut Vec<UIRequest>,
    ) {
        if tags.is_empty() {
            return;
        }

        egui::SidePanel::right("assign_tag").show(ctx, |ui| {
            ui.label("Assigne Tag");

            let selected_text =
                if let Some(selected_tag) = tags.get(&self.selected_tagid) {
                    self.selected_tag = Some(selected_tag.clone());
                    selected_tag.name.clone()
                } else {
                    "Select a tag".to_string()
                };
            ComboBox::from_label("Assign Tag")
                .selected_text(selected_text)
                .show_ui(ui, |ui| {
                    for (id, tag) in tags {
                        ui.selectable_value(
                            &mut self.selected_tagid,
                            *id,
                            tag.name.clone(),
                        );
                    }
                });
        });
    }

    pub fn clear_text(&mut self) {
        self.new.clear();
    }

    pub fn select_side_panel(&mut self, ptype: Option<SidePanelType>) {
        self.ptype = ptype;
    }
}
