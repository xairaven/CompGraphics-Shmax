use crate::context::Context;
use egui::{Color32, DragValue, Grid, RichText, ScrollArea, SidePanel};

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
                        ui.label("Axes Length:");
                        ui.add(
                            DragValue::new(&mut context.figures.grid.length.0)
                                .speed(1)
                                .range(1.0..=f64::INFINITY),
                        );
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

                    self.matrix(ui, context);

                    ui.add_space(10.0);
                    ui.separator();
                    ui.add_space(10.0);

                    self.surface(ui, context);

                    ui.add_space(10.0);
                    ui.separator();
                    ui.add_space(10.0);

                    self.euclidean(ui, context);
                });
            });
    }

    fn surface(&self, ui: &mut egui::Ui, context: &mut Context) {
        ui.label(RichText::new("Surface Settings").color(Color32::WHITE));

        ui.add_space(5.0);

        Grid::new("Surface Settings").num_columns(2).show(ui, |ui| {
            ui.label("Height:");
            ui.add(
                DragValue::new(&mut context.figures.surface.height.0)
                    .speed(0.1)
                    .range(1.0..=f64::INFINITY)
                    .fixed_decimals(2),
            );
            ui.end_row();

            ui.label("Radius:");
            ui.add(
                DragValue::new(&mut context.figures.surface.radius_base.0)
                    .speed(0.1)
                    .range(1.0..=f64::INFINITY)
                    .fixed_decimals(2),
            );
            ui.end_row();

            ui.label("Mesh Density:");
            ui.add(
                DragValue::new(&mut context.figures.surface.mesh)
                    .speed(1)
                    .range(10.0..=f64::INFINITY)
                    .fixed_decimals(0),
            );
            ui.end_row();
        });
    }

    fn matrix(&self, ui: &mut egui::Ui, context: &mut Context) {
        ui.label(RichText::new("Perspective Coefficients").color(Color32::WHITE));

        ui.add_space(5.0);

        ui.horizontal(|ui| {
            ui.label("Q (X vanish):");
            ui.add(
                DragValue::new(&mut context.projections.twopoint.q)
                    .speed(0.0001)
                    .fixed_decimals(4),
            );
        });

        ui.horizontal(|ui| {
            ui.label("R (Z vanish):");
            ui.add(
                DragValue::new(&mut context.projections.twopoint.r)
                    .speed(0.0001)
                    .fixed_decimals(4),
            );
        });

        ui.vertical_centered_justified(|ui| {
            if ui.button("1-Point (Z)").clicked() {
                context.projections.twopoint.q = 0.0;
                context.projections.twopoint.r = 0.002;
            }
        });
        ui.vertical_centered_justified(|ui| {
            if ui.button("2-Point").clicked() {
                context.projections.twopoint.q = 0.002;
                context.projections.twopoint.r = 0.002;
            }
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
                ui.end_row();

                ui.label("Y:");
                ui.add(
                    DragValue::new(&mut context.transformations.offset.y.0)
                        .speed(0.1)
                        .fixed_decimals(2),
                );
                ui.end_row();

                ui.label("Z:");
                ui.add(
                    DragValue::new(&mut context.transformations.offset.z.0)
                        .speed(0.1)
                        .fixed_decimals(2),
                );
                ui.end_row();
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
                    ui.label("Angle X:");
                    ui.add(
                        DragValue::new(&mut context.transformations.rotation.angle_x)
                            .speed(0.1)
                            .fixed_decimals(2),
                    );
                    ui.end_row();

                    ui.label("Angle Y:");
                    ui.add(
                        DragValue::new(&mut context.transformations.rotation.angle_y)
                            .speed(0.1)
                            .fixed_decimals(2),
                    );
                    ui.end_row();

                    ui.label("Angle Z:");
                    ui.add(
                        DragValue::new(&mut context.transformations.rotation.angle_z)
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
