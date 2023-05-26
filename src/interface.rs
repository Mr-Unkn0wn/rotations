use egui::*;

use crate::{court::Court, solutions::Solutions};

pub fn draw_ui(court: &mut Court, solutions: &mut Solutions, width: f32) {
    egui_macroquad::ui(|ctx| {
        egui::SidePanel::right("Rotations").exact_width(width).show(ctx, |ui| {
            ui.label("Select Rotation");
            rotation_grid(ui, court);
            next_prev_button(ui, court, solutions);
            ui.separator();
        });
    });
    egui_macroquad::draw();
}

fn rotation_grid(ui: &mut Ui, court: &mut Court) {
    egui::Grid::new("rotation grid").show(ui, |ui| {
        ui.horizontal(|ui| {
            rotation_selecter(ui, 4, court);
            rotation_selecter(ui, 3, court);
            rotation_selecter(ui, 2, court);
        });

        ui.end_row();

        ui.horizontal(|ui| {
            rotation_selecter(ui, 5, court);
            rotation_selecter(ui, 6, court);
            rotation_selecter(ui, 1, court);
        });
    });
}

fn next_prev_button(ui: &mut Ui, court: &mut Court, solutions: &mut Solutions) {
    ui.horizontal(|ui| {
        if ui.add(egui::Button::new("Prev")).clicked() {
            let mut rot = court.get_rotation() + 1;
            if rot > 6 {
                rot %= 6;
            }
            court.set_rotation(rot);
        }
        if ui.add(egui::Button::new("Next")).clicked() {
            let mut rot = court.get_rotation() - 1;
            if rot == 0 {
                rot = 6;
            }
            court.set_rotation(rot);
        }
    });
    ui.checkbox(&mut solutions.show_solution, "Show solution");
    ui.add_enabled(solutions.show_solution, egui::Checkbox::new(&mut solutions.go_to_solution, "Go to solution"));
}

fn rotation_selecter(ui: &mut Ui, rotation: i32, court: &mut Court) {
    if ui.add(egui::SelectableLabel::new(court.get_rotation() == rotation, rotation.to_string())).clicked() {
        court.set_rotation(rotation);
    }
}
