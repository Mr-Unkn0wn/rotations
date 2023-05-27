use egui::*;
use macroquad::window::screen_width;

use crate::{court::Court, solutions::Solutions};

#[derive(PartialEq, Debug)]
enum Mode {
    Learn,
    Free,
}

enum LearnStage {
    Dragging,
    Solution,
}

pub(crate) struct Interface {
    mode: Mode,
    width: f32,
    offset: f32,
    learn_stage: LearnStage,
}

impl Interface {
    pub fn new(interface_width: f32, offset: f32) -> Self {
        Interface {
            mode: Mode::Learn,
            width: interface_width,
            offset,
            learn_stage: LearnStage::Dragging,
        }
    }

    pub fn draw_ui(&mut self, court: &mut Court, solutions: &mut Solutions) {
        egui_macroquad::ui(|ctx| {
            self.top_bar(ctx, court, solutions);
            self.right_window(ctx, court, solutions);
        });
        egui_macroquad::draw();
    }

    fn top_bar(&mut self, ctx: &egui::Context, court: &mut Court, solutions: &mut Solutions) {
        egui::TopBottomPanel::top("top bar").exact_height(25.0).show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                use egui::special_emojis::GITHUB;
                ui.label("I hope this small program is useful to you :)");
                ui.separator();
                ui.hyperlink_to(format!("{} Rotation Visualisation on GitHub", GITHUB), "https://github.com/Mr-Unkn0wn/rotations");
                ui.separator();
                self.mode_selector(ui, court, solutions);
            });
        });
    }

    fn right_window(&mut self, ctx: &egui::Context, court: &mut Court, solutions: &mut Solutions) {
        let mut pos = Vec2::new(screen_width(), self.offset);
        pos.x -= self.width + self.offset;
        egui::Window::new(format!("{:?}", self.mode))
            .collapsible(false)
            .fixed_pos((pos.x, pos.y))
            .default_width(self.width)
            .resizable(false)
            .show(ctx, |ui| {
                ui.vertical_centered_justified(|ui| match self.mode {
                    Mode::Learn => self.learn_mode(ui, court, solutions),
                    Mode::Free => self.free_mode(ui, court, solutions),
                });
            });
    }

    fn mode_selector(&mut self, ui: &mut Ui, court: &mut Court, solutions: &mut Solutions) {
        ui.label("Select a mode : ");
        ui.selectable_value(&mut self.mode, Mode::Learn, "Learn");
        ui.selectable_value(&mut self.mode, Mode::Free, "Free");
    }
}

// LEARN
impl Interface {
    fn learn_mode(&mut self, ui: &mut Ui, court: &mut Court, solutions: &mut Solutions) {
        ui.label("In this mode you can practice the 5:1 rotations.");
        ui.hyperlink_to("If you dont know what this means this video should help.", "https://www.youtube.com/watch?v=LkpmYtogPdw");
        ui.separator();
        ui.heading(format!("Rotation {}", court.get_rotation()));

        match self.learn_stage {
            LearnStage::Dragging => self.dragging_around_stage(ui),
            LearnStage::Solution => self.solution_stage(ui, court, solutions),
        }
        ui.separator();
        self.next_button(ui, court, solutions);
        if ui.button("Retry").clicked() {
            court.set_rotation(court.get_rotation());
            solutions.go_to_solution = false;
            solutions.show_solution = false;
            self.learn_stage = LearnStage::Dragging;
        }
    }

    fn dragging_around_stage(&mut self, ui: &mut Ui) {
        ui.label("Drag everyone to their correct position.");
        ui.label("");
        if ui.button("Show solution!").clicked() {
            self.learn_stage = LearnStage::Solution;
        }
    }

    fn solution_stage(&mut self, ui: &mut Ui, court: &mut Court, solutions: &mut Solutions) {
        ui.label("These are the correct positions.");
        ui.label("If you can't see any arrows you did it correctly.");
        solutions.show_solution = true;
        ui.checkbox(&mut solutions.go_to_solution, "Move to solution.");
    }
}

// FREE
impl Interface {
    fn free_mode(&mut self, ui: &mut Ui, court: &mut Court, solutions: &mut Solutions) {
        ui.label("Ignore this for now :3");
        /*
        ui.label("Select Rotation");
        self.rotation_grid(ui, court);
        self.next_prev_button(ui, court, solutions);
        ui.separator();
        ui.checkbox(&mut solutions.show_solution, "Show solution");
        ui.add_enabled(solutions.show_solution, egui::Checkbox::new(&mut solutions.go_to_solution, "Go to solution"));
        ui.separator();
        ui.checkbox(&mut court.serve_played, "Has serve been played");
         */
    }

    fn rotation_grid(&mut self, ui: &mut Ui, court: &mut Court) {
        egui::Grid::new("rotation grid").show(ui, |ui| {
            self.rotation_selecter(ui, 4, court);
            self.rotation_selecter(ui, 3, court);
            self.rotation_selecter(ui, 2, court);

            ui.end_row();

            self.rotation_selecter(ui, 5, court);
            self.rotation_selecter(ui, 6, court);
            self.rotation_selecter(ui, 1, court);
        });
    }

    fn next_prev_button(&mut self, ui: &mut Ui, court: &mut Court, solutions: &mut Solutions) {
        ui.horizontal(|ui| {
            self.prev_button(ui, court, solutions);
            self.next_button(ui, court, solutions)
        });
    }

    fn rotation_selecter(&mut self, ui: &mut Ui, rotation: i32, court: &mut Court) {
        if ui.add(egui::SelectableLabel::new(court.get_rotation() == rotation, rotation.to_string())).clicked() {
            court.set_rotation(rotation);
        }
    }
}

// SHARED
impl Interface {
    fn next_button(&mut self, ui: &mut Ui, court: &mut Court, solutions: &mut Solutions) {
        if ui.add(egui::Button::new("Next rotation")).clicked() {
            let mut rot = court.get_rotation() - 1;
            if rot == 0 {
                rot = 6;
            }
            court.set_rotation(rot);
            solutions.go_to_solution = false;
            solutions.show_solution = false;
            self.learn_stage = LearnStage::Dragging;
        }
    }

    fn prev_button(&mut self, ui: &mut Ui, court: &mut Court, solutions: &mut Solutions) {
        if ui.add(egui::Button::new("Prev")).clicked() {
            let mut rot = court.get_rotation() + 1;
            if rot > 6 {
                rot %= 6;
            }
            court.set_rotation(rot);
            solutions.go_to_solution = false;
            solutions.show_solution = false;
        }
    }
}
