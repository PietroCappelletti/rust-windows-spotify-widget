use egui::{Color32, Context, FontId, RichText, TextureHandle};
use std::time::Instant;
use crate::spotify::Track;

pub enum WidgetAction {
    None,
    Play,
    Pause,
    Next,
    Previous,
}

const TEXT_COLUMN_WIDTH: f32 = 160.0;
const MARQUEE_GAP: f32 = 30.0;
const MARQUEE_SPEED_PX_PER_SEC: f32 = 40.0;

/// Draws `text` in the given width. If it fits, draws it once, static.
/// If it's too wide, clips it and scrolls it left continuously, looping
/// seamlessly, based on time elapsed since `marquee_start`.
fn marquee_label(
    ui: &mut egui::Ui,
    text: &str,
    width: f32,
    font_id: FontId,
    color: Color32,
    marquee_start: Instant,
) {
    let galley = ui.fonts(|f| f.layout_no_wrap(text.to_string(), font_id, color));
    let text_width = galley.size().x;
    let row_height = galley.size().y;

    let (rect, _response) =
        ui.allocate_exact_size(egui::vec2(width, row_height), egui::Sense::hover());

    if text_width <= width {
        ui.painter().galley(rect.left_top(), galley, color);
        return;
    }

    let cycle_width = text_width + MARQUEE_GAP;
    let elapsed = marquee_start.elapsed().as_secs_f32();
    let offset = (elapsed * MARQUEE_SPEED_PX_PER_SEC) % cycle_width;

    let painter = ui.painter_at(rect);

    let first_pos = rect.left_top() - egui::vec2(offset, 0.0);
    let second_pos = first_pos + egui::vec2(cycle_width, 0.0);

    painter.galley(first_pos, galley.clone(), color);
    painter.galley(second_pos, galley, color);
}

pub fn draw_widget(
    ctx: &Context,
    track: Option<&Track>,
    error: Option<&str>,
    album_texture: Option<&TextureHandle>,
    marquee_start: Instant,
) -> WidgetAction {
    let mut action = WidgetAction::None;

    let frame = egui::Frame::default()
        .fill(ctx.style().visuals.window_fill())
        .inner_margin(egui::Margin::same(10.0));

    egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
        ui.horizontal(|ui| {
            if let Some(texture) = album_texture {
                ui.add(
                    egui::Image::new((texture.id(), texture.size_vec2()))
                        .max_width(56.0)
                        .max_height(56.0)
                        .rounding(4.0),
                );
            } else {
                ui.allocate_space(egui::vec2(56.0, 56.0));
            }

            ui.add_space(8.0);

            ui.vertical(|ui| {
                let text_color = ui.visuals().text_color();
                let weak_color = ui.visuals().weak_text_color();

                match track {
                    Some(t) => {
                        marquee_label(
                            ui,
                            &t.name,
                            TEXT_COLUMN_WIDTH,
                            FontId::proportional(14.0),
                            text_color,
                            marquee_start,
                        );
                        marquee_label(
                            ui,
                            &t.artist,
                            TEXT_COLUMN_WIDTH,
                            FontId::proportional(12.0),
                            weak_color,
                            marquee_start,
                        );
                    }
                    None => {
                        ui.add(egui::Label::new("Nothing is currently playing.").truncate());
                    }
                }

                ui.add_space(4.0);

                ui.horizontal(|ui| {
                    if ui.button("⏮").clicked() {
                        action = WidgetAction::Previous;
                    }

                    let play_pause_label = match track {
                        Some(t) if t.is_playing => "⏸",
                        _ => "▶",
                    };
                    if ui.button(play_pause_label).clicked() {
                        action = match track {
                            Some(t) if t.is_playing => WidgetAction::Pause,
                            _ => WidgetAction::Play,
                        };
                    }

                    if ui.button("⏭").clicked() {
                        action = WidgetAction::Next;
                    }
                });
            });
        });

        if let Some(err) = error {
            ui.add_space(6.0);
            ui.add(egui::Label::new(RichText::new(err).color(Color32::RED)).truncate());
        }
    });

    action
}