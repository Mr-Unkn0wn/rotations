use egui::*;
use macroquad::window::screen_width;

use crate::{court::Court, solutions::Solutions};

pub fn draw_ui(court: &mut Court, solutions: &mut Solutions, width: f32, offset: f32) {
    let mut pos = Vec2::new(screen_width(), offset);
    pos.x -= width + offset;

    egui_macroquad::ui(|ctx| {
        egui::Window::new("Rotations")
            .collapsible(false)
            .fixed_pos((pos.x, pos.y))
            .default_width(width)
            .resizable(false)
            .show(ctx, |ui| {
                ui.vertical_centered_justified(|ui| {
                    ui.label("Select Rotation");
                    rotation_grid(ui, court);
                    next_prev_button(ui, court);
                    ui.separator();
                    ui.checkbox(&mut solutions.show_solution, "Show solution");
                    ui.add_enabled(solutions.show_solution, egui::Checkbox::new(&mut solutions.go_to_solution, "Go to solution"));
                });
            });
    });
    egui_macroquad::draw();
}

fn rotation_grid(ui: &mut Ui, court: &mut Court) {
    egui::Grid::new("rotation grid").show(ui, |ui| {
        rotation_selecter(ui, 4, court);
        rotation_selecter(ui, 3, court);
        rotation_selecter(ui, 2, court);

        ui.end_row();

        rotation_selecter(ui, 5, court);
        rotation_selecter(ui, 6, court);
        rotation_selecter(ui, 1, court);
    });
}

fn next_prev_button(ui: &mut Ui, court: &mut Court) {
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
}

fn rotation_selecter(ui: &mut Ui, rotation: i32, court: &mut Court) {
    if ui.add(egui::SelectableLabel::new(court.get_rotation() == rotation, rotation.to_string())).clicked() {
        court.set_rotation(rotation);
    }
}
