use crate::context::Context;
use egui::{Color32, DragValue, Grid, RichText, ScrollArea, SidePanel};
use geometry::figures::detail::{ArcId, CircleId, DetailElementId, SegmentId};
use geometry::figures::grid;
use strum::IntoEnumIterator;

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

                    ui.horizontal(|ui| {
                        ui.label("Pan:");
                        ui.checkbox(&mut context.viewport.config.is_pannable, "");
                    });

                    ui.horizontal(|ui| {
                        ui.label("Zoom:");
                        ui.checkbox(&mut context.viewport.config.is_zoomable, "");
                    });

                    ui.vertical_centered_justified(|ui| {
                        if ui.button("Reset Offset").clicked() {
                            context.viewport.geometry.reset_offset();
                        }
                    });

                    ui.separator();

                    ui.vertical_centered_justified(|ui| {
                        ui.label(RichText::new("Detail").color(Color32::WHITE));
                    });

                    ui.group(|ui| {
                        ui.vertical_centered(|ui| {
                            ui.label("Lengths");
                        });

                        ui.add_space(5.0);

                        Grid::new("Lengths_GRID").num_columns(4).show(ui, |ui| {
                            for (i, segment) in SegmentId::iter().enumerate() {
                                Self::length_radius_drag(
                                    ui,
                                    context,
                                    DetailElementId::Segment(segment),
                                );

                                if (i + 1) % 2 == 0 {
                                    ui.end_row();
                                }
                            }
                        });

                        ui.add_space(10.0);

                        ui.vertical_centered(|ui| {
                            ui.label("Radiuses");
                        });

                        ui.add_space(5.0);

                        Grid::new("Radiuses_GRID").num_columns(4).show(ui, |ui| {
                            Self::length_radius_drag(
                                ui,
                                context,
                                DetailElementId::Arc(ArcId::AL),
                            );
                            Self::length_radius_drag(
                                ui,
                                context,
                                DetailElementId::Circle(CircleId::M),
                            );

                            ui.end_row();
                        });

                        ui.add_space(10.0);

                        ui.vertical_centered_justified(|ui| {
                            if ui.button("Reset All").clicked() {
                                context.figures.detail.reset_all();
                            }
                        });
                    });

                    ui.separator();

                    ui.vertical_centered_justified(|ui| {
                        ui.label(
                            RichText::new("Euclidean Transformations")
                                .color(Color32::WHITE),
                        );
                    });

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
                                    .range(0.0..=f32::INFINITY)
                                    .fixed_decimals(2),
                            );

                            ui.label("Y:");
                            ui.add(
                                DragValue::new(&mut context.transformations.offset.y.0)
                                    .speed(0.1)
                                    .range(0.0..=f32::INFINITY)
                                    .fixed_decimals(2),
                            );
                        });

                        ui.add_space(5.0);

                        ui.horizontal(|ui| {
                            ui.columns(2, |ui| {
                                ui[0].vertical_centered_justified(|ui| {
                                    if ui.button("Apply").clicked() {
                                        context.transformations.offset.enable();
                                    }
                                });
                                ui[1].vertical_centered_justified(|ui| {
                                    if ui.button("Reset").clicked() {
                                        context.transformations.offset.reset();
                                    }
                                });
                            });
                        });
                    })
                });
            });
    }

    fn length_radius_drag(
        ui: &mut egui::Ui, context: &mut Context, element_id: DetailElementId,
    ) {
        let (label, length) = match element_id {
            DetailElementId::Segment(segment_id) => {
                let label = format!("{:#?}:", segment_id);
                let length = segment_id.length(&mut context.figures.detail.sides);
                (label, length)
            },
            DetailElementId::Arc(arc_id) => {
                let label = format!("{:#?}:", arc_id);
                let length = arc_id.radius(&mut context.figures.detail.radiuses);
                (label, length)
            },
            DetailElementId::Circle(circle_id) => {
                let label = format!("{:#?}:", circle_id);
                let length = circle_id.radius(&mut context.figures.detail.radiuses);
                (label, length)
            },
        };

        ui.label(label);

        if ui
            .add(
                DragValue::new(&mut length.0)
                    .speed(0.1)
                    .fixed_decimals(2)
                    .range(0.1..=f32::INFINITY),
            )
            .changed()
        {
            context.figures.detail.update_chain(element_id);
        };
    }
}
