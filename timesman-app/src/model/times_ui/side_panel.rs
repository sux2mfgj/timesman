use std::collections::HashMap;

use egui::ComboBox;
use timesman_type::{Tag, TagId, Todo};

use super::UIRequest;

#[derive(Copy, Clone, PartialEq)]
pub enum SidePanelType {
    Todo,
    TodoDetail,
    Tag,
    TagAssigne,
}

pub struct SidePanel {
    ptype: Option<SidePanelType>,
    new: String,
    new_detail: String,
    selected_tagid: u64,
    pub selected_tag: Option<Tag>,
    selected_todo: Option<Todo>,
    editing_detail: bool,
}

impl SidePanel {
    pub fn new() -> Self {
        Self {
            ptype: None,
            new: "".to_string(),
            new_detail: "".to_string(),
            selected_tagid: 0,
            selected_tag: None,
            selected_todo: None,
            editing_detail: false,
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
            SidePanelType::TodoDetail => {
                self.update_todo_detail(ctx, ureq);
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
                ui.horizontal(|ui| {
                    let mut done = todo.done_at.is_some();
                    let resp = ui.checkbox(&mut done, &todo.content);

                    if resp.clicked() {
                        ureq.push(UIRequest::TodoDone(todo.id, done));
                    }

                    // Show detail indicator
                    if todo.detail.is_some() {
                        if ui.small_button("📝").clicked() {
                            self.selected_todo = Some(todo.clone());
                            self.ptype = Some(SidePanelType::TodoDetail);
                        }
                    }
                });
            }

            ui.separator();

            ui.label("New Todo:");
            ui.text_edit_singleline(&mut self.new);
            
            ui.label("Detail (optional):");
            ui.text_edit_multiline(&mut self.new_detail);

            ui.horizontal(|ui| {
                if ui.button("Add Todo").clicked() && !self.new.is_empty() {
                    if self.new_detail.is_empty() {
                        ureq.push(UIRequest::Todo(self.new.clone()));
                    } else {
                        ureq.push(UIRequest::TodoWithDetail(self.new.clone(), self.new_detail.clone()));
                    }
                }

                if ui.button("Clear").clicked() {
                    self.new.clear();
                    self.new_detail.clear();
                }
            });

            // Legacy support for Enter key
            if !self.new.is_empty() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                if self.new_detail.is_empty() {
                    ureq.push(UIRequest::Todo(self.new.clone()));
                } else {
                    ureq.push(UIRequest::TodoWithDetail(self.new.clone(), self.new_detail.clone()));
                }
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

    fn update_todo_detail(
        &mut self,
        ctx: &egui::Context,
        ureq: &mut Vec<UIRequest>,
    ) {
        egui::SidePanel::right("todo_detail").show(ctx, |ui| {
            if let Some(todo) = self.selected_todo.clone() {
                ui.label("Todo Detail");
                ui.separator();

                ui.label(format!("Todo: {}", todo.content));
                
                if let Some(ref detail) = todo.detail {
                    ui.label("Detail:");
                    if self.editing_detail {
                        ui.text_edit_multiline(&mut self.new_detail);
                        ui.horizontal(|ui| {
                            if ui.button("Save").clicked() {
                                ureq.push(UIRequest::UpdateTodoDetail(todo.id, self.new_detail.clone()));
                                self.editing_detail = false;
                            }
                            if ui.button("Cancel").clicked() {
                                self.editing_detail = false;
                                self.new_detail = detail.clone();
                            }
                        });
                    } else {
                        ui.label(detail);
                        ui.horizontal(|ui| {
                            if ui.button("Edit Detail").clicked() {
                                self.editing_detail = true;
                                self.new_detail = detail.clone();
                            }
                            if ui.button("Back").clicked() {
                                self.ptype = Some(SidePanelType::Todo);
                                self.selected_todo = None;
                            }
                        });
                    }
                } else {
                    ui.label("No detail available");
                    ui.text_edit_multiline(&mut self.new_detail);
                    ui.horizontal(|ui| {
                        if ui.button("Add Detail").clicked() && !self.new_detail.is_empty() {
                            ureq.push(UIRequest::UpdateTodoDetail(todo.id, self.new_detail.clone()));
                        }
                        if ui.button("Back").clicked() {
                            self.ptype = Some(SidePanelType::Todo);
                            self.selected_todo = None;
                        }
                    });
                }
            }
        });
    }

    pub fn clear_text(&mut self) {
        self.new.clear();
    }

    pub fn clear_detail_text(&mut self) {
        self.new_detail.clear();
    }

    pub fn select_side_panel(&mut self, ptype: Option<SidePanelType>) {
        self.ptype = ptype;
    }
}
