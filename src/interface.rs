use egui::*;

use crate::court::Court;

pub fn draw_ui(court: &mut Court, _offset: f32, _clearcourt_size: f32) {
    egui_macroquad::ui(|ctx| {
        egui::Window::new("Rotations").resizable(false).show(ctx, |ui| {
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
}

fn rotation_selecter(ui: &mut Ui, rotation: u8, court: &mut Court) {
    if ui.add(egui::SelectableLabel::new(court.get_rotation() == rotation, rotation.to_string())).clicked() {
        court.set_rotation(rotation);
    }
}
