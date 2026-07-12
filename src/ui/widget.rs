use egui::{Color32, Context, FontId, RichText, TextureHandle};
use std::time::Instant;
use crate::spotify::Track;

pub enum WidgetAction {
    None,
    Play,
    Pause,
    Next,
    Previous,
    SetVolume(u8),
}

const TEXT_COLUMN_WIDTH: f32 = 160.0;
const VOLUME_COLUMN_WIDTH: f32 = 24.0;
const CONTENT_HEIGHT: f32 = 56.0;
const MARQUEE_GAP: f32 = 30.0;
const MARQUEE_SPEED_PX_PER_SEC: f32 = 40.0;

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
    current_volume: Option<u8>,
) -> WidgetAction {
    let mut action = WidgetAction::None;

    let frame = egui::Frame::default()
        .fill(ctx.style().visuals.window_fill())
        .inner_margin(egui::Margin::same(10.0));

    egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
        // Tracks whether the pointer is anywhere over the whole widget,
        // so the volume bar can reveal itself on hover.
        let panel_hovered = ui.rect_contains_pointer(ui.max_rect());

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
                ui.set_max_width(TEXT_COLUMN_WIDTH);
                let text_color = ui.visuals().text_color();
                let weak_color = ui.visuals().weak_text_color();

                match track {
                    Some(t) => {
                        marquee_label(ui, &t.name, TEXT_COLUMN_WIDTH, FontId::proportional(14.0), text_color, marquee_start);
                        marquee_label(ui, &t.artist, TEXT_COLUMN_WIDTH, FontId::proportional(12.0), weak_color, marquee_start);
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

            ui.add_space(6.0);

            // Volume column: always reserves its space (so the fixed-size
            // window never resizes), but only draws the interactive
            // vertical slider when the pointer is over the widget.
            let (rect, _resp) =
                ui.allocate_exact_size(egui::vec2(VOLUME_COLUMN_WIDTH, CONTENT_HEIGHT), egui::Sense::hover());

            if panel_hovered {
                if let Some(volume) = current_volume {
                    let mut slider_ui = ui.child_ui(rect, egui::Layout::top_down(egui::Align::Center), None);
                    // Vertical sliders use `slider_width` for their *height*
                    // (it's the "long axis" setting regardless of
                    // orientation) — without this override it defaults to
                    // ~100px, overflowing past our 56px-tall rect and the
                    // window's actual edge, which is also why drags outside
                    // that overflowed area lost mouse tracking.
                    slider_ui.spacing_mut().slider_width = CONTENT_HEIGHT;

                    let mut vol_f = volume as f32;
                    let slider = egui::Slider::new(&mut vol_f, 0.0..=100.0)
                        .vertical()
                        .show_value(false);
                    if slider_ui.add(slider).changed() {
                        action = WidgetAction::SetVolume(vol_f.round() as u8);
                    }
                }
            }
        });

        if let Some(err) = error {
            ui.add_space(6.0);
            ui.add(egui::Label::new(RichText::new(err).color(Color32::RED)).truncate());
        }
    });

    action
}