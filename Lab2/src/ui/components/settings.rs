use crate::context::Context;
use egui::{Color32, DragValue, Grid, RichText, ScrollArea, SidePanel};
use geometry::figures::grid;

#[derive(Debug)]
pub struct SettingsComponent {
    width: f32,
}

impl Default for SettingsComponent {
    fn default() -> Self {
        Self { width: 250.0 }
    }
}

impl SettingsComponent {
    pub fn show(&mut self, ui: &mut egui::Ui, context: &mut Context) {
        SidePanel::left("SETTINGS_PANEL")
            .resizable(false)
            .default_width(self.width)
            .min_width(self.width)
            .max_width(self.width)
            .show_separator_line(true)
            .show_inside(ui, |ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    ui.vertical_centered_justified(|ui| {
                        ui.heading(RichText::new("Settings").color(Color32::WHITE));
                    });

                    ui.add_space(10.0);

                    ui.horizontal(|ui| {
                        ui.label("Pixels on Centimeter:");
                        ui.add(
                            DragValue::new(
                                &mut context.viewport.geometry.pixels_per_centimeter,
                            )
                            .speed(1)
                            .range(geometry::viewport::PX_PER_CM_RANGE),
                        );

                        ui.vertical_centered_justified(|ui| {
                            if ui.button("Reset").clicked() {
                                context.viewport.geometry.reset_pixels_per_centimeter();
                            }
                        });
                    });

                    ui.horizontal(|ui| {
                        ui.label("Unit Length:");
                        ui.add(
                            DragValue::new(&mut context.figures.grid.unit.0)
                                .speed(1)
                                .range(grid::UNIT_RANGE),
                        );

                        ui.vertical_centered_justified(|ui| {
                            if ui.button("Reset").clicked() {
                                context.figures.grid.reset_unit();
                            }
                        });
                    });

                    Grid::new("AUXILIARY_SETTINGS")
                        .num_columns(3)
                        .show(ui, |ui| {
                            ui.checkbox(&mut context.figures.grid.is_enabled, "Grid,");
                            ui.checkbox(&mut context.viewport.config.is_pannable, "Pan,");
                            ui.checkbox(&mut context.viewport.config.is_zoomable, "Zoom");

                            ui.end_row();
                        });

                    ui.vertical_centered_justified(|ui| {
                        if ui.button("Reset Pan").clicked() {
                            context.viewport.geometry.reset_offset();
                        }
                    });

                    ui.vertical_centered_justified(|ui| {
                        if ui.button("Reset all to defaults").clicked() {
                            context.reset();
                        }
                    });

                    ui.add_space(10.0);
                    ui.separator();
                    ui.add_space(10.0);

                    self.epicycloid(ui, context);
                });
            });
    }

    fn epicycloid(&self, ui: &mut egui::Ui, context: &mut Context) {
        ui.vertical_centered_justified(|ui| {
            ui.label(RichText::new("Epicycloid Settings").color(Color32::WHITE));
        });

        Grid::new("EPICYCLOID_SETTINGS")
            .num_columns(2)
            .show(ui, |ui| {
                ui.label("Fixed Radius:");
                ui.add(
                    DragValue::new(&mut context.figures.epicycloid.fixed_radius.0)
                        .speed(1)
                        .range(0.0..=f64::INFINITY),
                );
                ui.end_row();

                ui.label("Rolling Radius:");
                ui.add(
                    DragValue::new(&mut context.figures.epicycloid.rolling_radius.0)
                        .speed(1)
                        .range(0.0..=f64::INFINITY),
                );
                ui.end_row();

                ui.label("Pen Offset:");
                ui.add(
                    DragValue::new(&mut context.figures.epicycloid.pen_offset.0)
                        .speed(1)
                        .range(0.0..=f64::INFINITY),
                );
                ui.end_row();

                ui.label("Rotations:");
                ui.add(
                    DragValue::new(&mut context.figures.epicycloid.rotations)
                        .speed(1)
                        .range(1..=u32::MAX),
                );
                ui.end_row();

                ui.label("Step:");
                ui.add(
                    DragValue::new(&mut context.figures.epicycloid.step)
                        .speed(0.05)
                        .range(0.05..=f64::INFINITY),
                );
                ui.end_row();
            });

        ui.vertical_centered_justified(|ui| {
            if ui.button("Reset").clicked() {
                context.figures.epicycloid.reset();
            }
        });
    }
}
