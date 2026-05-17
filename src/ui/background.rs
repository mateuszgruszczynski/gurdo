use std::sync::{Arc, Mutex};

use eframe::egui;
use image::imageops::FilterType;

pub(super) struct BackgroundPainter {
    texture: Option<egui::TextureHandle>,
    last_url: String,
    pending: Arc<Mutex<Option<egui::ColorImage>>>,
}

impl BackgroundPainter {
    pub(super) fn new() -> Self {
        Self {
            texture: None,
            last_url: String::new(),
            pending: Arc::new(Mutex::new(None)),
        }
    }

    pub(super) fn update(
        &mut self,
        ctx: &egui::Context,
        cover_url: Option<&str>,
        cover_bytes: Option<&[u8]>,
    ) {
        match (cover_url, cover_bytes) {
            (Some(url), Some(bytes)) if !bytes.is_empty() => {
                if url != self.last_url {
                    self.last_url = url.to_string();
                    let bytes = bytes.to_vec();
                    let slot = Arc::clone(&self.pending);
                    let ctx = ctx.clone();
                    std::thread::spawn(move || {
                        if let Some(img) = blur_image(&bytes) {
                            *slot.lock().unwrap() = Some(img);
                            ctx.request_repaint();
                        }
                    });
                }
                if let Some(img) = self.pending.lock().unwrap().take() {
                    self.texture = Some(ctx.load_texture("cover_blur", img, Default::default()));
                }
            }
            _ => {
                self.texture = None;
                self.last_url.clear();
            }
        }
    }

    pub(super) fn paint(&self, ctx: &egui::Context, fallback_rgb: [u8; 3]) {
        let mut visuals = ctx.style().visuals.clone();
        if let Some(tex) = &self.texture {
            visuals.panel_fill = egui::Color32::TRANSPARENT;
            ctx.set_visuals(visuals);

            let rect = ctx.screen_rect();
            let painter = ctx.layer_painter(egui::LayerId::background());

            // Blurred cover fills the entire window
            painter.image(
                tex.id(),
                rect,
                egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                egui::Color32::WHITE,
            );

            // Dark vertical gradient overlay for legibility
            let top_color    = egui::Color32::from_rgba_unmultiplied(0, 0, 0, 60);
            let bottom_color = egui::Color32::from_rgba_unmultiplied(0, 0, 0, 200);
            let mesh = gradient_mesh(rect, top_color, bottom_color);
            painter.add(egui::Shape::mesh(mesh));
        } else {
            let [r, g, b] = fallback_rgb;
            visuals.panel_fill = egui::Color32::from_rgb(r, g, b);
            ctx.set_visuals(visuals);
        }
    }
}

fn blur_image(bytes: &[u8]) -> Option<egui::ColorImage> {
    let img = image::load_from_memory(bytes).ok()?;
    let rgba = img
        .resize_exact(256, 256, FilterType::Triangle)
        .into_rgba8();
    let blurred = image::imageops::fast_blur(&rgba, 30.0);
    let pixels = blurred.as_flat_samples();
    Some(egui::ColorImage::from_rgba_unmultiplied(
        [256, 256],
        pixels.as_slice(),
    ))
}

fn gradient_mesh(
    rect: egui::Rect,
    top: egui::Color32,
    bottom: egui::Color32,
) -> egui::Mesh {
    let mut mesh = egui::Mesh::default();
    // 4 vertices: top-left, top-right, bottom-right, bottom-left
    mesh.colored_vertex(rect.left_top(),     top);
    mesh.colored_vertex(rect.right_top(),    top);
    mesh.colored_vertex(rect.right_bottom(), bottom);
    mesh.colored_vertex(rect.left_bottom(),  bottom);
    // Two triangles: (0,1,2) and (0,2,3)
    mesh.add_triangle(0, 1, 2);
    mesh.add_triangle(0, 2, 3);
    mesh
}
