use eframe::egui;
use futures_util::{SinkExt, StreamExt};
use xcap::{Monitor, Window};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex as StdMutex};
use std::thread;
use std::time::{Duration, Instant};
use tokio::sync::Mutex as TokioMutex;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use xcap::image;
use xcap::image::imageops;
use xcap::image::imageops::FilterType;
use device_query::{DeviceQuery, DeviceState, MouseState};
use base64::{Engine as _, engine::general_purpose::STANDARD};
use once_cell::sync::Lazy;
use tokio::time::{timeout};

// constants
// const WS_SERVER_URL: &str = "wss://rustconnectserver.onrender.com";
const WS_SERVER_URL: &str = "ws://127.0.0.1:8080";
// semi official server. for now.
// else change to "http://127.0.0.1:8080"
const HTTP_SERVER_URL: &str = "http://127.0.0.1:8080";
const RAW_CURSOR: &str = "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";
const RAW_ICON: &str = "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";

// emojis
const STAR_EMOJI: &str = "✨";
const NO_EMOJI: &str = "❌";
const COPY_EMOJI: &str = "📋";
const ROCKET_EMOJI: &str = "🚀";
const STREAMING_EMOJI: &str = "🔴";

// cursor
static CURSOR_IMG: Lazy<image::RgbaImage> = Lazy::new(|| {
    let png_bytes = STANDARD.decode(RAW_CURSOR)
        .expect("Failed to decode base64");

    image::load_from_memory(&png_bytes)
        .expect("Failed to load cursor from decoded bytes")
        .to_rgba8()
});

#[derive(Clone, Debug)]
struct UiError {
    title: String,
    message: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
enum StreamTarget {
    Monitor(u32),
    Window(u32),
}

#[derive(Clone, Serialize, Deserialize)]
struct StreamSettings {
    max_fps: u32,
    resolution_scale: f32,
    color_boost: f32,
    target: StreamTarget,
}

struct SharedAppState {
    incoming_texture_data: Option<egui::ColorImage>,
    room_code: String,
    settings: StreamSettings,
    username: String,
    connected_users: Vec<String>,
    kick_sender: Option<tokio::sync::mpsc::UnboundedSender<ControlPacket>>,
    errors: std::collections::VecDeque<UiError>,
}

struct ScreenClientApp {
    state: Arc<StdMutex<SharedAppState>>,
    texture: Option<egui::TextureHandle>,
    input_room_code: String,
    is_hosting: bool,
    is_watching: bool,
    settings_open: bool,
    username_modal_open: bool,
    
    available_monitors: Vec<(u32, String)>,
    available_windows: Vec<(u32, String)>,
}

static IS_HOSTING_ACTIVE: AtomicBool = AtomicBool::new(false);
static IS_WATCHING_ACTIVE: AtomicBool = AtomicBool::new(false);
static FRAME_DELAY_MS: AtomicU64 = AtomicU64::new(16);

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "payload")]
enum ControlPacket {
    RegisterUser(String),
    UserList(Option<Vec<String>>),
    KickUser(String),
    Kicked,
}

fn load_icon() -> egui::IconData {
    let bytes = base64::decode(RAW_ICON)
        .expect("invalid base64 icon");

    let image = image::load_from_memory(&bytes)
        .expect("failed to decode image")
        .into_rgba8();

    let (width, height) = image.dimensions();

    egui::IconData {
        rgba: image.into_raw(),
        width,
        height,
    }
}

#[tokio::main]
async fn main() -> Result<(), eframe::Error> {
    let _ = register_custom_protocol();
    let args: Vec<String> = std::env::args().collect();
    let mut initial_room = String::new();
    if args.len() > 1 && args[1].starts_with("rustconnect://") {
        initial_room = args[1].replace("rustconnect://", "").replace("/", "");
    }

    let saved_username = std::fs::read_to_string("user_cache.txt").unwrap_or_else(|_| "This is a Username btw".to_string());
    
    let monitors = Monitor::all().unwrap_or_default();
    let initial_target = monitors.first().map(|m| StreamTarget::Monitor(m.id().unwrap())).unwrap_or(StreamTarget::Monitor(0));

    let state = Arc::new(StdMutex::new(SharedAppState {
        incoming_texture_data: None,
        room_code: String::new(),
        settings: StreamSettings {
            max_fps: 60,
            resolution_scale: 1.0,
            color_boost: 1.0,
            target: initial_target,
        },
        username: saved_username,
        connected_users: Vec::new(),
        kick_sender: None,
        errors: std::collections::VecDeque::new(),
    }));

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 700.0])
            .with_icon(load_icon())
            .with_resizable(true),
        run_and_return: false,
        ..Default::default()
    };

    let state_clone = state.clone();
    eframe::run_native(
        "RustConnect",
        options,
        Box::new(move |cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            Ok(Box::new(ScreenClientApp {
                state: state_clone,
                texture: None,
                input_room_code: initial_room,
                is_hosting: false,
                is_watching: false,
                settings_open: false,
                username_modal_open: true,
                available_monitors: Vec::new(),
                available_windows: Vec::new(),
            }))
        }),
    )
}

impl eframe::App for ScreenClientApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut state_guard = self.state.lock().unwrap();

        if self.username_modal_open {
            egui::Window::new("Profile Setup")
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label("Enter identity nickname handle before joining/hosting streams:");
                    ui.text_edit_singleline(&mut state_guard.username);
                    if ui.button("Save Profile Settings").clicked() {
                        let _ = std::fs::write("user_cache.txt", &state_guard.username);
                        self.username_modal_open = false;
                    }
                });
            return;
        }

        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("⚙ RustConnect");
                ui.label(format!("| Logged in as: {}", state_guard.username));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("⚙ Stream Settings").clicked() {
                        self.settings_open = !self.settings_open;
                        if self.settings_open {
                            self.available_monitors = Monitor::all().unwrap_or_default().into_iter()
                                .map(|m| (m.id().unwrap(), m.name().unwrap())).collect();
                            self.available_windows = Window::all().unwrap_or_default().into_iter()
                                .filter(|w| !w.title().unwrap().trim().is_empty())
                                .map(|w| (w.id().unwrap(), w.title().unwrap())).collect();
                        }
                    }
                });
            });
        });

        if let Some(err) = state_guard.errors.front().cloned() {
            egui::Window::new(format!("⚠️ {}", err.title))
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .collapsible(false)
                .resizable(false)
                .default_width(320.0)
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(8.0);
                        ui.label(&err.message);
                        ui.add_space(12.0);
                        if ui.button("Dismiss").clicked() {
                            state_guard.errors.pop_front();
                        }
                    });
                });
        }

        // LIVE SETTINGS POPOVER WINDOW
        if self.settings_open {
            egui::Window::new("⚙ Stream Controls")
                .resizable(false)
                .collapsible(false)
                .show(ctx, |ui| {
                    ui.label("Target Capture Mode:");
                    
                    egui::ComboBox::from_label("Monitors")
                        .selected_text(match state_guard.settings.target {
                            StreamTarget::Monitor(_) => "Monitor Selected",
                            _ => "Select Monitor..."
                        })
                        .show_ui(ui, |ui| {
                            for (id, name) in &self.available_monitors {
                                ui.selectable_value(&mut state_guard.settings.target, StreamTarget::Monitor(*id), name);
                            }
                        });

                    egui::ComboBox::from_label("Windows")
                        .selected_text(match state_guard.settings.target {
                            StreamTarget::Window(_) => "Window Selected",
                            _ => "Select Window..."
                        })
                        .show_ui(ui, |ui| {
                            for (id, title) in &self.available_windows {
                                ui.selectable_value(&mut state_guard.settings.target, StreamTarget::Window(*id), title);
                            }
                        });

                    ui.separator();
                    ui.label("Performance Limits:");
                    let old_fps = state_guard.settings.max_fps;
                    ui.add(egui::Slider::new(&mut state_guard.settings.max_fps, 15..=60).text("Max Target FPS"));
                    if old_fps != state_guard.settings.max_fps {
                        FRAME_DELAY_MS.store((1000 / state_guard.settings.max_fps) as u64, Ordering::SeqCst);
                    }

                    ui.add(egui::Slider::new(&mut state_guard.settings.resolution_scale, 0.25..=1.0).text("Resolution Downscale"));
                    ui.separator();
                    ui.label("Visual Adjustments:");
                    ui.add(egui::Slider::new(&mut state_guard.settings.color_boost, 1.0..=2.0).text("Color Saturation Boost"));

                    ui.add_space(10.0);
                    if ui.button("Close Panel").clicked() {
                        self.settings_open = false;
                    }
                });
        }

        // SIDE PANEL FOR MODERATION
        // FAILED :(
        // if self.is_hosting || self.is_watching {
        //     egui::SidePanel::right("management_panel").width_range(200.0..=300.0).show(ctx, |ui| {
        //         ui.heading("Connected Spectators");
        //         ui.separator();
        //         for user in &state_guard.connected_users {
        //             ui.horizontal(|ui| {
        //                 ui.label(user.as_str());
        //                 if self.is_hosting {
        //                     ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        //                         if ui.button("Kick").clicked() {
        //                             if let Some(sender) = &state_guard.kick_sender {
        //                                 let _ = sender.send(ControlPacket::KickUser(user.clone()));
        //                             }
        //                         }
        //                     });
        //                 }
        //             });
        //         }
        //     });
        // }

        egui::CentralPanel::default().show(ctx, |ui| {
            if !self.is_hosting && !self.is_watching {
                ui.vertical_centered(|ui| {
                    ui.add_space(40.0);
                    ui.group(|ui| {
                        ui.heading("Host Your Desktop Activities");
                        if ui.button(format!("{} Generate Connection Code", STAR_EMOJI)).clicked() {
                            let state_clone = self.state.clone();
                            thread::spawn(move || {
                                if let Ok(res) = reqwest::blocking::get(&format!("{}/new", HTTP_SERVER_URL)) {
                                    if let Ok(code) = res.text() {
                                        state_clone.lock().unwrap().room_code = code.trim().to_string();
                                    }
                                }
                            });
                        }

                        if !state_guard.room_code.is_empty() {
                            ui.label(format!("Invite Link: rustconnect://{}", state_guard.room_code));
                            if ui.button(format!("{} Copy Invite Link", COPY_EMOJI)).clicked() {
                                ui.output_mut(|o| o.copied_text = format!("rustconnect://{}", state_guard.room_code));
                            }

                            if ui.button(format!("{} Start Live Feed", ROCKET_EMOJI)).clicked() {
                                self.is_hosting = true;
                                IS_HOSTING_ACTIVE.store(true, Ordering::SeqCst);
                                start_hosting_thread(
                                    self.state.clone(),
                                    state_guard.room_code.clone(),
                                    state_guard.username.clone(),
                                    ctx.clone(),
                                );
                            }
                        }
                    });

                    ui.add_space(20.0);

                    ui.group(|ui| {
                        ui.heading("Watch Room Feed");
                        ui.add(egui::TextEdit::singleline(&mut self.input_room_code).hint_text("Enter room ID"));
                        if ui.button("📺 Join Stream").clicked() && !self.input_room_code.is_empty() {
                            self.is_watching = true;
                            IS_WATCHING_ACTIVE.store(true, Ordering::SeqCst);
                            start_watching_thread(self.state.clone(), self.input_room_code.trim().to_string(), state_guard.username.clone(), ctx.clone());
                        }
                    });
                });
            }

            if self.is_hosting {
                ui.vertical_centered(|ui| {
                    ui.colored_label(egui::Color32::LIGHT_RED, format!("{} TRANSMITTING SCREEN DATA", STREAMING_EMOJI));
                    if ui.button(format!("{} Copy Invite Link: {}", COPY_EMOJI, state_guard.room_code)).clicked() {
                        ui.output_mut(|o| o.copied_text = format!("rustconnect://{}", state_guard.room_code));
                    }
                    if ui.button("Stop Feed").clicked() {
                        IS_HOSTING_ACTIVE.store(false, Ordering::SeqCst);
                        self.is_hosting = false;
                    }
                });
            }

            if self.is_watching {
                ui.horizontal(|ui| {
                    if ui.button(format!("{} Leave Feed", NO_EMOJI)).clicked() {
                        IS_WATCHING_ACTIVE.store(false, Ordering::SeqCst);
                        self.is_watching = false;
                        self.texture = None;
                    }
                });

                if let Some(color_image) = state_guard.incoming_texture_data.take() {
                    self.texture = Some(ctx.load_texture("stream_view", color_image, Default::default()));
                }

                if let Some(ref tex) = self.texture {
                    let available_size = ui.available_size();
                    ui.add(egui::Image::from_texture(tex).max_size(available_size));
                } else {
                    ui.centered_and_justified(|ui| { ui.spinner(); });
                }
            }
        });

        ctx.request_repaint();
    }
}

fn start_hosting_thread(
    state: Arc<StdMutex<SharedAppState>>,
    room_id: String,
    username: String,
    ui_context: egui::Context,
) {
    thread::spawn(move || {
        let runtime = tokio::runtime::Runtime::new().unwrap();

        runtime.block_on(async move {
            let base_url = WS_SERVER_URL.trim().trim_end_matches('/');
            let clean_room = room_id.trim().replace(|c: char| c.is_whitespace(), "");
            let clean_user = username.trim().replace(|c: char| c.is_whitespace(), "");
            
            let url = format!(
                "{}/host?room={}&user={}",
                base_url, clean_room, clean_user
            );

            match connect_async(&url).await {
                Ok((ws_stream, _)) => {
                    let (write, mut read) = ws_stream.split();
                    let write = Arc::new(TokioMutex::new(write));
                    let write_clone = write.clone();

                    // channel for control packets
                    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<ControlPacket>();

                    {
                        let mut guard = state.lock().unwrap();
                        guard.kick_sender = Some(tx);
                    }

                    // outgoing control task
                    tokio::spawn(async move {
                        while let Some(packet) = rx.recv().await {
                            if let Ok(json) = serde_json::to_string(&packet) {
                                let mut w = write_clone.lock().await;
                                let _ = w.send(Message::Text(json.into())).await;
                            }
                        }
                    });

                    let state_clone = state.clone();
                    let ui_clone = ui_context.clone();

                    // incoming messages
                    tokio::spawn(async move {
                        while let Some(msg) = read.next().await {
                            match msg {
                                Ok(Message::Text(txt)) => {
                                    if let Ok(ControlPacket::UserList(list)) =
                                        serde_json::from_str::<ControlPacket>(&txt.to_string())
                                    {
                                        if let Ok(mut guard) = state_clone.lock() {
                                            guard.connected_users = list.unwrap_or_default();
                                        }
                                        ui_clone.request_repaint();
                                    }
                                }
                                Ok(Message::Close(_)) => break,
                                Ok(_) => {}
                                Err(_) => break,
                            }
                        }
                    });

                    // capture loop (BLOCKING THREAD, NOT TOKIO)
                    let device_state = DeviceState::new();
                    let cursor_size = 24;
                    let scaled_cursor = image::imageops::resize(
                        &*CURSOR_IMG,
                        cursor_size,
                        cursor_size,
                        FilterType::Triangle,
                    );

                    while IS_HOSTING_ACTIVE.load(Ordering::SeqCst) {
                        let loop_start = Instant::now();
                        let mouse = device_state.get_mouse();
                        let mouse_x = mouse.coords.0;
                        let mouse_y = mouse.coords.1;

                        let (scale, color_boost, target, max_fps) = {
                            let guard = state.lock().unwrap();
                            (
                                guard.settings.resolution_scale,
                                guard.settings.color_boost,
                                guard.settings.target.clone(),
                                guard.settings.max_fps,
                            )
                        };

                        let frame_res = match target {
                            StreamTarget::Monitor(id) => Monitor::all()
                                .unwrap_or_default()
                                .into_iter()
                                .find(|m| m.id().unwrap_or(0) == id)
                                .and_then(|m| m.capture_image().ok()),

                            StreamTarget::Window(id) => Window::all()
                                .unwrap_or_default()
                                .into_iter()
                                .find(|w| w.id().unwrap_or(0) == id)
                                .and_then(|w| w.capture_image().ok()),
                        };

                        if let Some(mut img) = frame_res {
                            if color_boost > 1.05 {
                                for px in img.pixels_mut() {
                                    let r = px[0] as f32;
                                    let g = px[1] as f32;
                                    let b = px[2] as f32;

                                    let gray = 0.299 * r + 0.587 * g + 0.114 * b;
                                    px[0] = (gray + (r - gray) * color_boost).clamp(0.0, 255.0) as u8;
                                    px[1] = (gray + (g - gray) * color_boost).clamp(0.0, 255.0) as u8;
                                    px[2] = (gray + (b - gray) * color_boost).clamp(0.0, 255.0) as u8;
                                }
                            }

                            if mouse_x >= 0
                                && mouse_y >= 0
                                && mouse_x < img.width() as i32
                                && mouse_y < img.height() as i32
                            {
                                imageops::overlay(
                                    &mut img,
                                    &scaled_cursor,
                                    mouse_x as i64,
                                    mouse_y as i64,
                                );
                            }

                            let w = (img.width() as f32 * scale) as u32;
                            let h = (img.height() as f32 * scale) as u32;

                            let resized = image::imageops::resize(&img, w, h, FilterType::Nearest);
                            let mut buf = Vec::new();
                            let mut cursor = std::io::Cursor::new(&mut buf);
                            if image::codecs::jpeg::JpegEncoder::new_with_quality(&mut cursor, 60)
                                .encode_image(&resized)
                                .is_ok()
                            {
                                let mut w = write.lock().await;
                                let _ = w.send(Message::Binary(buf.into())).await;
                            }
                        }

                        let delay = Duration::from_millis(1000 / max_fps.max(1) as u64);
                        let elapsed = loop_start.elapsed();

                        if elapsed < delay {
                            std::thread::sleep(delay - elapsed);
                        }
                    }
                }
                Err(e) => {
                    if let Ok(mut guard) = state.lock() {
                        guard.errors.push_back(UiError {
                            title: "Hosting Connection Failed".to_string(),
                            message: format!("URL Configuration Error: {}. Check formatting strings.", e),
                        });
                    }
                    IS_HOSTING_ACTIVE.store(false, Ordering::SeqCst);
                    ui_context.request_repaint();
                }
            }
        });
    });
}

fn start_watching_thread(
    state: Arc<StdMutex<SharedAppState>>,
    room_id: String,
    username: String,
    ui_context: egui::Context,
) {
    thread::spawn(move || {
        let runtime = tokio::runtime::Runtime::new().unwrap();

        runtime.block_on(async move {
            let url = format!(
                "{}/join?room={}&user={}",
                WS_SERVER_URL, room_id, username
            );
            
            let Ok((ws_stream, _)) = connect_async(&url).await else {
                if let Ok(mut guard) = state.lock() {
                    guard.errors.push_back(UiError {
                        title: "Join Error".to_string(),
                        message: format!("Connection failed: {}", url),
                    });
                }
                ui_context.request_repaint();
                return;
            };

            // let Ok((ws_stream, _)) = connect_async(url).await else {
            //     return;
            // };

            let (_, mut read) = ws_stream.split();

            while IS_WATCHING_ACTIVE.load(Ordering::SeqCst) {
                let msg = timeout(Duration::from_secs(5), read.next()).await;

                match msg {
                    Ok(Some(Ok(Message::Binary(bytes)))) => {
                        if let Ok(img) = image::load_from_memory(&bytes) {
                            let rgba = img.to_rgba8();

                            let color_image = egui::ColorImage::from_rgba_unmultiplied(
                                [rgba.width() as usize, rgba.height() as usize],
                                rgba.as_raw(),
                            );

                            if let Ok(mut guard) = state.lock() {
                                guard.incoming_texture_data = Some(color_image);
                            }

                            ui_context.request_repaint();
                        }
                    }

                    Ok(Some(Ok(Message::Text(txt)))) => {
                        if txt == "KICKED" || txt == "Room not found" {
                            IS_WATCHING_ACTIVE.store(false, Ordering::SeqCst);
                            break;
                        }
                    }

                    Ok(Some(Ok(Message::Close(_)))) => break,
                    Ok(Some(Err(_))) => break,
                    Ok(None) => break,

                    Err(_) => {
                        // timeout: just continue instead of dying
                        continue;
                    }

                    _ => {}
                }
            }
        });
    });
}

fn register_custom_protocol() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "windows")]
    {
        use winreg::enums::*;
        use winreg::RegKey;
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let path = std::env::current_exe()?;
        let path_str = path.to_str().unwrap();
        let (key, _) = hkcu.create_subkey("Software\\Classes\\rustconnect")?;
        key.set_value("", &"URL:RustConnect Protocol")?;
        key.set_value("URL Protocol", &"")?;
        let (cmd_key, _) = hkcu.create_subkey("Software\\Classes\\rustconnect\\shell\\open\\command")?;
        cmd_key.set_value("", &format!("\"{}\" \"%1\"", path_str))?;
    }
    Ok(())
}