use egui::*;

use crate::court::Court;

pub fn draw_ui(court: &mut Court, _offset: f32, _clearcourt_size: f32) {
    egui_macroquad::ui(|ctx| {
        egui::SidePanel::right("Rotations").show(ctx, |ui| {
            rotation_grid(ui, court);
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
