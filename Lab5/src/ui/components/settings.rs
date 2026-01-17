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

                    self.euclidean(ui, context);
                });
            });
    }

    fn euclidean(&self, ui: &mut egui::Ui, context: &mut Context) {
        ui.vertical_centered_justified(|ui| {
            ui.label(RichText::new("Euclidean Transformations").color(Color32::WHITE));
        });

        ui.add_space(5.0);

        ui.group(|ui| {
            ui.vertical_centered(|ui| {
                ui.label("Offset");
            });

            ui.add_space(5.0);

            Grid::new("EUCLIDEAN_OFFSET").num_columns(4).show(ui, |ui| {
                ui.label("X:");
                ui.add(
                    DragValue::new(&mut context.transformations.offset.x.0)
                        .speed(0.1)
                        .fixed_decimals(2),
                );

                ui.label("Y:");
                ui.add(
                    DragValue::new(&mut context.transformations.offset.y.0)
                        .speed(0.1)
                        .fixed_decimals(2),
                );
            });

            ui.add_space(5.0);

            ui.horizontal(|ui| {
                ui.columns(2, |ui| {
                    ui[0].vertical_centered_justified(|ui| {
                        if ui.button("Apply").clicked() {
                            context.transformations.offset.run();
                        }
                    });
                    ui[1].vertical_centered_justified(|ui| {
                        if ui.button("Reset").clicked() {
                            context.transformations.offset.reset();
                        }
                    });
                });
            });
        });

        ui.add_space(10.0);

        ui.group(|ui| {
            ui.vertical_centered(|ui| {
                ui.label("Rotation");
            });

            ui.add_space(5.0);

            Grid::new("EUCLIDEAN_Rotation")
                .num_columns(2)
                .show(ui, |ui| {
                    ui.label("X:");
                    ui.add(
                        DragValue::new(&mut context.transformations.rotation.x.0)
                            .speed(0.1)
                            .fixed_decimals(2),
                    );
                    ui.end_row();

                    ui.label("Y:");
                    ui.add(
                        DragValue::new(&mut context.transformations.rotation.y.0)
                            .speed(0.1)
                            .fixed_decimals(2),
                    );
                    ui.end_row();

                    ui.label("Angle:");
                    ui.add(
                        DragValue::new(&mut context.transformations.rotation.angle)
                            .speed(1)
                            .suffix(" °"),
                    );
                    ui.end_row();
                });

            ui.add_space(5.0);

            ui.horizontal(|ui| {
                ui.columns(2, |ui| {
                    ui[0].vertical_centered_justified(|ui| {
                        if ui.button("Apply").clicked() {
                            context.transformations.rotation.run();
                        }
                    });
                    ui[1].vertical_centered_justified(|ui| {
                        if ui.button("Reset").clicked() {
                            context.transformations.rotation.reset();
                        }
                    });
                });
            });
        });
    }
}
