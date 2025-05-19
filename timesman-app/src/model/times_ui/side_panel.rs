use timesman_type::Todo;

use super::UIRequest;

#[derive(Copy, Clone)]
pub enum SidePanelType {
    Todo,
    // Tag,
}

pub struct SidePanel {
    ptype: Option<SidePanelType>,
    new_todo: String,
}

impl SidePanel {
    pub fn new() -> Self {
        Self {
            ptype: None,
            new_todo: "".to_string(),
        }
    }

    pub fn update(
        &mut self,
        ctx: &egui::Context,
        todo: &Vec<Todo>,
        ureq: &mut Vec<UIRequest>,
    ) {
        let Some(ptype) = self.ptype else {
            return;
        };

        match ptype {
            SidePanelType::Todo => {
                self.update_todo(ctx, todo, ureq);
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

            ui.text_edit_singleline(&mut self.new_todo);
            if !self.new_todo.is_empty()
                && ui.input(|i| i.key_pressed(egui::Key::Enter))
            {
                ureq.push(UIRequest::Todo(self.new_todo.clone()))
            }
        });
    }

    pub fn clear_todo_text(&mut self) {
        self.new_todo.clear();
    }

    pub fn select_side_panel(&mut self, ptype: Option<SidePanelType>) {
        self.ptype = ptype;
    }
}
