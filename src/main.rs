#![windows_subsystem = "windows"]

mod color;
mod clipboard;
mod sampler;

use color::PickedColor;
use clipboard::Clipboard;
use sampler::{capture_region, cursor_pos};

use std::time::{Duration, Instant};
use eframe::egui;

struct App {
    history: Vec<PickedColor>,
    clipboard: Clipboard,
    toast_msg: Option<(String, Instant)>,
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Force egui and eframe window to use dark mode
        cc.egui_ctx.set_theme(egui::ThemePreference::Dark);
        
        let mut style = (*cc.egui_ctx.style()).clone();
        let bg = egui::Color32::from_rgb(18, 18, 18);
        style.visuals.window_fill = bg;
        style.visuals.panel_fill = bg;
        style.visuals.window_stroke = egui::Stroke::NONE;
        cc.egui_ctx.set_style(style);

        Self {
            history: Vec::new(),
            clipboard: Clipboard::new().expect("clipboard unavailable"),
            toast_msg: None,
        }
    }

    fn copy_text(&mut self, text: String) {
        if self.clipboard.set_text(&text).is_ok() {
            self.toast_msg = Some((format!("Copied {}", text), Instant::now()));
        }
    }

    fn capture_color(&mut self, picked: PickedColor) {
        self.history.push(picked);
        if self.history.len() > 18 {
            self.history.remove(0);
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if matches!(&self.toast_msg, Some((_, t)) if t.elapsed() > Duration::from_secs(3)) {
            self.toast_msg = None;
        }

        let (cx, cy) = cursor_pos();
        let half = 7i32;
        let capture = capture_region(cx, cy, half);
        let center_rgb = capture.center();
        let picked = PickedColor::from_rgb(center_rgb);

        if ctx.input(|i| i.key_pressed(egui::Key::X) || i.key_pressed(egui::Key::Space) || i.key_pressed(egui::Key::Enter)) {
            self.capture_color(picked.clone());
        }

        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(egui::Color32::from_rgb(18, 18, 18)).inner_margin(24.0))
            .show(ctx, |ui| {
            
            ui.horizontal(|ui| {
                let loupe_px = 160.0;
                let cell = loupe_px / (2.0 * half as f32 + 1.0);
                
                egui::Frame::none()
                    .fill(egui::Color32::from_rgb(12, 12, 12))
                    .rounding(2.0)
                    .inner_margin(4.0)
                    .show(ui, |ui| {
                        let (response, painter) = ui.allocate_painter(egui::vec2(loupe_px, loupe_px), egui::Sense::hover());
                        let tl = response.rect.min;
                        let grid = (2 * half + 1) as usize;

                        for row in 0..grid {
                            for col in 0..grid {
                                let pixel = capture.get(col, row);
                                let rect = egui::Rect::from_min_size(
                                    egui::pos2(tl.x + col as f32 * cell, tl.y + row as f32 * cell),
                                    egui::vec2(cell, cell),
                                );
                                painter.rect_filled(rect, 0.0, pixel.to_egui_color());
                            }
                        }

                        let mid = half as usize;
                        let center_rect = egui::Rect::from_min_size(
                            egui::pos2(tl.x + mid as f32 * cell, tl.y + mid as f32 * cell),
                            egui::vec2(cell, cell),
                        );
                        
                        painter.rect_stroke(center_rect, 0.0, egui::Stroke::new(1.5, egui::Color32::WHITE));
                        painter.rect_stroke(center_rect.expand(1.5), 0.0, egui::Stroke::new(0.5, egui::Color32::BLACK));
                    });

                ui.add_space(24.0);

                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        let (rect, _) = ui.allocate_exact_size(egui::vec2(48.0, 48.0), egui::Sense::hover());
                        ui.painter().rect_filled(rect, 4.0, picked.rgb.to_egui_color());
                        ui.painter().rect_stroke(rect, 4.0, egui::Stroke::new(1.0, egui::Color32::from_gray(60)));
                        
                        ui.add_space(16.0);
                        
                        ui.vertical(|ui| {
                            let hex_resp = ui.add(
                                egui::Label::new(
                                    egui::RichText::new(&picked.hex)
                                        .size(28.0)
                                        .strong()
                                        .color(egui::Color32::from_gray(240))
                                )
                                .sense(egui::Sense::click())
                            );
                            if hex_resp.clicked() {
                                self.copy_text(picked.hex.clone());
                            }
                            hex_resp.on_hover_cursor(egui::CursorIcon::PointingHand);
                        });
                    });

                    ui.add_space(20.0);

                    for (label, value) in [("RGB", picked.rgb_string.clone()), ("HSL", picked.hsl_string.clone())] {
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new(label)
                                    .color(egui::Color32::from_gray(120))
                                    .size(12.0)
                            );
                            ui.add_space(12.0);
                            let vl = ui.add(
                                egui::Label::new(
                                    egui::RichText::new(&value)
                                        .color(egui::Color32::from_gray(200))
                                        .size(14.0)
                                )
                                .sense(egui::Sense::click())
                            );
                            if vl.clicked() {
                                self.copy_text(value.clone());
                            }
                            vl.on_hover_cursor(egui::CursorIcon::PointingHand);
                        });
                        ui.add_space(4.0);
                    }
                    
                    ui.add_space(20.0);
                    ui.label(
                        egui::RichText::new("Press X to capture")
                            .color(egui::Color32::from_gray(100))
                            .size(12.0)
                    );
                });
            });

            ui.add_space(24.0);
            
            let rect = ui.allocate_space(egui::vec2(ui.available_width(), 1.0)).1;
            ui.painter().rect_filled(rect, 0.0, egui::Color32::from_gray(35));
            
            ui.add_space(16.0);

            let mut string_to_copy = None;
            
            ui.horizontal(|ui| {
                if let Some((msg, _)) = &self.toast_msg {
                    ui.label(
                        egui::RichText::new(msg)
                            .color(egui::Color32::from_rgb(100, 250, 150))
                            .strong()
                            .size(13.0)
                    );
                } else {
                    ui.label(
                        egui::RichText::new("History")
                            .color(egui::Color32::from_gray(120))
                            .size(13.0)
                    );
                }
            });

            ui.add_space(12.0);

            ui.scope(|ui| {
                ui.spacing_mut().item_spacing = egui::vec2(8.0, 8.0);
                ui.horizontal_wrapped(|ui| {
                    if self.history.is_empty() {
                        ui.label(
                            egui::RichText::new("No colors captured yet.")
                                .color(egui::Color32::from_gray(80))
                                .size(13.0)
                        );
                    }
                    for h in self.history.iter().rev() {
                        let (rect, resp) = ui.allocate_exact_size(egui::vec2(36.0, 36.0), egui::Sense::click());
                        ui.painter().rect_filled(rect, 4.0, h.rgb.to_egui_color());
                        ui.painter().rect_stroke(rect, 4.0, egui::Stroke::new(1.0, egui::Color32::from_gray(45)));
                        
                        if resp.clicked() {
                            string_to_copy = Some(h.hex.clone());
                        }
                        resp.on_hover_cursor(egui::CursorIcon::PointingHand)
                            .on_hover_text(format!("{}\n{}\n{}", h.hex, h.rgb_string, h.hsl_string));
                    }
                });
            });
            
            if let Some(s) = string_to_copy {
                self.copy_text(s);
            }
        });

        ctx.request_repaint_after(Duration::from_millis(33));
    }
}

fn main() -> eframe::Result<()> {
    let icon_bytes = include_bytes!("../resources/icon.ico");
    let icon = image::load_from_memory(icon_bytes)
        .expect("embedded icon is invalid")
        .into_rgba8();
    let (icon_width, icon_height) = icon.dimensions();

    let opts = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("What color is this.")
            .with_inner_size([460.0, 400.0])
            .with_resizable(false)
            .with_icon(egui::IconData {
                rgba: icon.into_raw(),
                width: icon_width,
                height: icon_height,
            })
            .with_always_on_top(),
        ..Default::default()
    };

    eframe::run_native(
        "whatcolor - What color is this.",
        opts,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
}
