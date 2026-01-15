use crate::context::Context;
use egui::{Color32, DragValue, Grid, RichText, ScrollArea, SidePanel};
use geometry::animations::walker;
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

                    ui.add_space(10.0);
                    ui.separator();
                    ui.add_space(10.0);

                    self.animation(ui, context);

                    ui.add_space(10.0);
                    ui.separator();
                    ui.add_space(10.0);

                    self.curve_walk(ui, context);

                    ui.add_space(10.0);
                    ui.separator();
                    ui.add_space(10.0);

                    self.properties(ui, context);

                    ui.add_space(10.0);
                    ui.separator();
                    ui.add_space(10.0);

                    self.measurements(ui, context);

                    ui.add_space(10.0);
                    ui.separator();
                    ui.add_space(10.0);

                    self.euclidean(ui, context);
                });
            });
    }

    fn epicycloid(&self, ui: &mut egui::Ui, context: &mut Context) {
        ui.vertical_centered_justified(|ui| {
            ui.label(RichText::new("Epicycloid Settings").color(Color32::WHITE));
        });

        let mut changed = false;

        Grid::new("EPICYCLOID_SETTINGS")
            .num_columns(2)
            .show(ui, |ui| {
                ui.label("Fixed Radius:");
                if ui
                    .add(
                        DragValue::new(&mut context.figures.epicycloid.fixed_radius.0)
                            .speed(1)
                            .range(0.0..=f64::INFINITY),
                    )
                    .changed()
                {
                    changed = true;
                };
                ui.end_row();

                ui.label("Rolling Radius:");
                if ui
                    .add(
                        DragValue::new(&mut context.figures.epicycloid.rolling_radius.0)
                            .speed(1)
                            .range(0.0..=f64::INFINITY),
                    )
                    .changed()
                {
                    changed = true;
                };
                ui.end_row();

                ui.label("Pen Offset:");
                if ui
                    .add(
                        DragValue::new(&mut context.figures.epicycloid.pen_offset.0)
                            .speed(1)
                            .range(0.0..=f64::INFINITY),
                    )
                    .changed()
                {
                    changed = true;
                };
                ui.end_row();

                ui.label("Rotations:");
                if ui
                    .add(
                        DragValue::new(&mut context.figures.epicycloid.rotations)
                            .speed(1)
                            .range(1..=u32::MAX),
                    )
                    .changed()
                {
                    changed = true;
                };
                ui.end_row();

                ui.label("Step:");
                if ui
                    .add(
                        DragValue::new(&mut context.figures.epicycloid.step)
                            .speed(0.05)
                            .range(0.05..=f64::INFINITY),
                    )
                    .changed()
                {
                    changed = true;
                };
                ui.end_row();
            });

        if changed {
            context.animations.walker.hide();
            context.figures.epicycloid.calculate_stats();
        }

        ui.vertical_centered_justified(|ui| {
            if ui.button("Reset").clicked() {
                context.figures.epicycloid.reset();
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
                            context.animations.walker.hide();
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
                            context.animations.walker.hide();
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

    fn animation(&self, ui: &mut egui::Ui, context: &mut Context) {
        ui.group(|ui| {
            ui.vertical_centered_justified(|ui| {
                ui.label(RichText::new("Animation").color(Color32::WHITE));
            });

            ui.add_space(5.0);

            ui.horizontal(|ui| {
                ui.label("Status:");
                if context.animations.epicycloid.is_enabled {
                    ui.colored_label(Color32::LIGHT_GREEN, "Running");
                } else {
                    ui.colored_label(Color32::RED, "Stopped");
                }
            });

            ui.vertical_centered_justified(|ui| {
                let text = if context.animations.epicycloid.is_enabled {
                    "Stop"
                } else {
                    "Start"
                };

                if ui.button(text).clicked() {
                    context.animations.epicycloid.toggle();
                }
            });
        });
    }

    fn curve_walk(&self, ui: &mut egui::Ui, context: &mut Context) {
        ui.vertical_centered_justified(|ui| {
            ui.label(RichText::new("Curve Walk").color(Color32::WHITE));
        });

        ui.add_space(5.0);

        ui.group(|ui| {
            Grid::new("CurvePointGrid").num_columns(3).show(ui, |ui| {
                ui.label("Status: ");

                if context.animations.walker.is_visible {
                    ui.label(RichText::new("Visible").color(Color32::LIGHT_GREEN));
                } else {
                    ui.label(RichText::new("Hidden").color(Color32::RED));
                };

                if context.animations.walker.is_enabled {
                    ui.label(RichText::new("Running").color(Color32::LIGHT_GREEN));
                } else {
                    ui.label(RichText::new("Stopped").color(Color32::RED));
                };

                ui.end_row();

                if ui.button("Show / Hide").clicked() {
                    context.animations.walker.show_toggle();
                }

                if ui.button("⏪").clicked() {
                    context.animations.walker.set_decreasing();
                }
                if ui.button("⏩").clicked() {
                    context.animations.walker.set_increasing();
                }

                ui.end_row();

                if context.animations.walker.is_visible {
                    let point = context.animations.walker.point();
                    ui.label("Coordinates: ");
                    ui.label(format!("X: {:.2}", point.x));
                    ui.label(format!("Y: {:.2}", point.y));

                    ui.end_row();
                }

                ui.label("Speed: ");
                ui.add(
                    DragValue::new(&mut context.animations.walker.step)
                        .speed(1)
                        .range(walker::STEP_RANGE),
                );
                ui.end_row();
            });
        });
    }

    fn properties(&self, ui: &mut egui::Ui, context: &mut Context) {
        ui.vertical_centered_justified(|ui| {
            ui.label(RichText::new("Properties").color(Color32::WHITE));
        });

        ui.add_space(5.0);

        Grid::new("CURVE_PROPERTIES").num_columns(2).show(ui, |ui| {
            ui.label("Inflection Points");
            ui.checkbox(
                &mut context.animations.walker.is_inflection_points_enabled,
                "",
            );
            ui.end_row();

            ui.label("Normal");
            ui.checkbox(&mut context.animations.walker.is_normal_enabled, "");
            ui.end_row();

            ui.label("Tangent");
            ui.checkbox(&mut context.animations.walker.is_tangent_enabled, "");
            ui.end_row();

            ui.label("Lines Size");
            ui.add(
                DragValue::new(&mut context.animations.walker.lines_size.0)
                    .speed(1)
                    .range(1..=100),
            );
            ui.end_row();
        });
    }

    fn measurements(&self, ui: &mut egui::Ui, context: &mut Context) {
        ui.label(RichText::new("Measurements").color(Color32::WHITE));
        ui.add_space(5.0);

        let stats = &context.figures.epicycloid.stats;

        Grid::new("CURVE_MEASUREMENTS")
            .num_columns(2)
            .show(ui, |ui| {
                ui.label("Curve Length:");
                ui.label(format!("{:.2} cm", stats.length));
                ui.end_row();

                ui.label("Curve Area:");
                ui.label(format!("{:.2} cm²", stats.area));
                ui.end_row();

                if context.animations.walker.is_visible {
                    let radius = context
                        .animations
                        .walker
                        .current_curvature_radius(&context.figures.epicycloid);
                    ui.label("Curvature Radius:");
                    if radius.is_infinite() {
                        ui.label("Infinite");
                    } else {
                        ui.label(format!("{:.2} cm", radius));
                    }
                    ui.end_row();
                }
            });
    }
}
