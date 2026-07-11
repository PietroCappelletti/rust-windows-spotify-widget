use std::sync::{mpsc, Arc};
use std::time::{Duration, Instant};

use crate::config::Config;
use crate::hotkey::HotkeyListener;
use crate::spotify::{SpotifyClient, Track};
use crate::tray::{TrayCommand, TrayHandle};
use crate::ui::{draw_widget, WidgetAction};

const OFFSCREEN_POS: egui::Pos2 = egui::pos2(-10000.0, -10000.0);
const TRACK_POLL_INTERVAL: Duration = Duration::from_secs(3);

enum SpotifyEvent {
    TrackUpdated(Option<Track>),
    ActionFailed(String),
    AlbumArtLoaded { url: String, image: egui::ColorImage },
}

pub const WINDOW_WIDTH: f32 = 250.0;
pub const WINDOW_HEIGHT: f32 = 76.0;
pub const SCREEN_MARGIN: f32 = 16.0;

pub struct WidgetApp {
    visible: bool,
    last_interaction: Instant,
    auto_hide_after: Duration,
    hotkey: HotkeyListener,
    tray: TrayHandle,
    runtime: tokio::runtime::Runtime,
    spotify: Arc<SpotifyClient>,
    current_track: Option<Track>,
    last_error: Option<String>,
    event_tx: mpsc::Sender<SpotifyEvent>,
    event_rx: mpsc::Receiver<SpotifyEvent>,
    album_texture: Option<egui::TextureHandle>,
    album_texture_url: Option<String>,
    marquee_start: Instant,
}

impl WidgetApp {
    pub fn new(
        start_visible: bool,
        hotkey: HotkeyListener,
        tray: TrayHandle,
        config: Config,
        runtime: tokio::runtime::Runtime,
        spotify: Arc<SpotifyClient>,
    ) -> Self {
        let auto_hide_after = Duration::from_secs(config.auto_hide_seconds);
        let (event_tx, event_rx) = mpsc::channel();

        let app = Self {
            visible: start_visible,
            last_interaction: Instant::now(),
            auto_hide_after,
            hotkey,
            tray,
            runtime,
            spotify,
            current_track: None,
            last_error: None,
            event_tx,
            event_rx,
            album_texture: None,
            album_texture_url: None,
            marquee_start: Instant::now(),
        };

        app.spawn_track_polling_loop();
        app
    }

    fn spawn_track_polling_loop(&self) {
        let spotify = self.spotify.clone();
        let tx = self.event_tx.clone();

        self.runtime.spawn(async move {
            loop {
                match spotify.get_current_track().await {
                    Ok(track) => {
                        let _ = tx.send(SpotifyEvent::TrackUpdated(track));
                    }
                    Err(e) => {
                        let _ = tx.send(SpotifyEvent::ActionFailed(e));
                    }
                }
                tokio::time::sleep(TRACK_POLL_INTERVAL).await;
            }
        });
    }

    fn spawn_action<F, Fut>(&self, action: F)
    where
        F: FnOnce(Arc<SpotifyClient>) -> Fut + Send + 'static,
        Fut: std::future::Future<Output = Result<(), String>> + Send + 'static,
    {
        let spotify = self.spotify.clone();
        let tx = self.event_tx.clone();

        self.runtime.spawn(async move {
            if let Err(e) = action(spotify.clone()).await {
                let _ = tx.send(SpotifyEvent::ActionFailed(e));
                return;
            }
            tokio::time::sleep(Duration::from_millis(300)).await;
            match spotify.get_current_track().await {
                Ok(track) => {
                    let _ = tx.send(SpotifyEvent::TrackUpdated(track));
                }
                Err(e) => {
                    let _ = tx.send(SpotifyEvent::ActionFailed(e));
                }
            }
        });
    }

    /// Downloads and decodes album art in the background, then reports
    /// the decoded pixels back via the event channel (texture creation
    /// itself has to happen on the UI thread, in `update()`).
    fn spawn_album_art_fetch(&self, url: String) {
        let tx = self.event_tx.clone();

        self.runtime.spawn(async move {
            let result: Result<egui::ColorImage, String> = async {
                let bytes = reqwest::get(&url)
                    .await
                    .map_err(|e| e.to_string())?
                    .bytes()
                    .await
                    .map_err(|e| e.to_string())?;

                let decoded = image::load_from_memory(&bytes).map_err(|e| e.to_string())?;
                let rgba = decoded.to_rgba8();
                let (width, height) = rgba.dimensions();

                Ok(egui::ColorImage::from_rgba_unmultiplied(
                    [width as usize, height as usize],
                    rgba.as_flat_samples().as_slice(),
                ))
            }
            .await;

            match result {
                Ok(image) => {
                    let _ = tx.send(SpotifyEvent::AlbumArtLoaded { url, image });
                }
                Err(e) => {
                    eprintln!("[album_art] Failed to load {url}: {e}");
                }
            }
        });
    }

    fn handle_widget_action(&mut self, action: WidgetAction) {
        match action {
            WidgetAction::Play => self.spawn_action(|c| async move { c.play().await }),
            WidgetAction::Pause => self.spawn_action(|c| async move { c.pause().await }),
            WidgetAction::Next => self.spawn_action(|c| async move { c.next_track().await }),
            WidgetAction::Previous => self.spawn_action(|c| async move { c.previous_track().await }),
            WidgetAction::None => {}
        }
    }

    /// Drains pending Spotify events. Needs `ctx` because loading a
    /// texture from decoded album art pixels must happen here, on the UI thread.
    fn drain_events(&mut self, ctx: &egui::Context) {
        while let Ok(event) = self.event_rx.try_recv() {
            match event {
                SpotifyEvent::TrackUpdated(track) => {
                    let new_art_url = track.as_ref().and_then(|t| t.album_art_url.clone());
                    let new_name = track.as_ref().map(|t| t.name.clone());
                    let old_name = self.current_track.as_ref().map(|t| t.name.clone());

                    if new_name != old_name {
                        self.marquee_start = Instant::now();
                    }

                    if new_art_url != self.album_texture_url {
                        self.album_texture = None;
                        self.album_texture_url = new_art_url.clone();
                        if let Some(url) = new_art_url {
                            self.spawn_album_art_fetch(url);
                        }
                    }

                    self.current_track = track;
                    self.last_error = None;
                }
                SpotifyEvent::ActionFailed(e) => {
                    self.last_error = Some(e);
                }
                SpotifyEvent::AlbumArtLoaded { url, image } => {
                    // Guard against a slow download landing after the track
                    // (and thus the art) has already changed again.
                    if self.album_texture_url.as_deref() == Some(url.as_str()) {
                        self.album_texture =
                            Some(ctx.load_texture("album_art", image, egui::TextureOptions::default()));
                    }
                }
            }
        }
    }

    fn set_shown(&mut self, ctx: &egui::Context, shown: bool) {
        self.visible = shown;
        if shown {
            let target_pos = ctx
                .input(|i| i.viewport().monitor_size)
                .map(|monitor_size| {
                    egui::pos2(
                        monitor_size.x - WINDOW_WIDTH - SCREEN_MARGIN,
                        SCREEN_MARGIN,
                    )
                })
                // Fallback if monitor size isn't reported yet (e.g. very first frame).
                .unwrap_or_else(|| egui::pos2(100.0, 100.0));

            ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(target_pos));
            self.last_interaction = Instant::now();
        } else {
            ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(OFFSCREEN_POS));
        }
    }

    fn toggle_visibility(&mut self, ctx: &egui::Context) {
        let now_shown = !self.visible;
        self.set_shown(ctx, now_shown);
    }

    fn check_auto_hide(&mut self, ctx: &egui::Context) {
        if self.visible && self.last_interaction.elapsed() > self.auto_hide_after {
            self.set_shown(ctx, false);
        }
    }
}

impl eframe::App for WidgetApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.hotkey.was_pressed() {
            self.toggle_visibility(ctx);
        }

        match self.tray.poll() {
            TrayCommand::ToggleVisibility => self.toggle_visibility(ctx),
            TrayCommand::Quit => std::process::exit(0),
            TrayCommand::None => {}
        }

        self.drain_events(ctx);
        self.check_auto_hide(ctx);

        if self.visible {
            let action = draw_widget(
                ctx,
                self.current_track.as_ref(),
                self.last_error.as_deref(),
                self.album_texture.as_ref(),
                self.marquee_start,
            );
            if !matches!(action, WidgetAction::None) {
                self.last_interaction = Instant::now();
                self.handle_widget_action(action);
            }

            if ctx.input(|i| i.pointer.any_down() || i.pointer.is_moving()) {
                self.last_interaction = Instant::now();
            }
        }

        let repaint_interval = if self.visible {
            Duration::from_millis(33) // ~30 FPS while visible, for smooth marquee scrolling
        } else {
            Duration::from_millis(200) // coarser while hidden, just enough to catch hotkey/tray events
        };
        ctx.request_repaint_after(repaint_interval);
    }
}