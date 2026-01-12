use crate::context::Context;
use egui::{Color32, DragValue, Grid, RichText, ScrollArea, SidePanel};
use geometry::figures::detail::SegmentId;
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
                        ui.heading("Settings");
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

                        Grid::new("Lengths_GRID").num_columns(4).show(ui, |ui| {
                            Self::length_drag(ui, context, SegmentId::AB);
                            Self::length_drag(ui, context, SegmentId::BC);
                            ui.end_row();

                            Self::length_drag(ui, context, SegmentId::CD);
                            Self::length_drag(ui, context, SegmentId::DE);
                            ui.end_row();

                            Self::length_drag(ui, context, SegmentId::EF);
                            Self::length_drag(ui, context, SegmentId::FG);
                            ui.end_row();

                            Self::length_drag(ui, context, SegmentId::GH);
                            Self::length_drag(ui, context, SegmentId::HI);
                            ui.end_row();

                            Self::length_drag(ui, context, SegmentId::IJ);
                            Self::length_drag(ui, context, SegmentId::JK);
                            ui.end_row();

                            Self::length_drag(ui, context, SegmentId::KL);
                            ui.end_row();
                        });
                    });
                });
            });
    }

    fn length_drag(ui: &mut egui::Ui, context: &mut Context, segment: SegmentId) {
        let length = segment.length(&mut context.figures.detail.lengths);

        ui.label(format!("{:#?}:", segment));
        if ui
            .add(
                DragValue::new(&mut length.0)
                    .speed(0.1)
                    .fixed_decimals(2)
                    .range(0.1..=f32::INFINITY),
            )
            .changed()
        {
            context.figures.detail.update_chain(segment);
        };
    }
}
