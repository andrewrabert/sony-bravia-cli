use std::sync::{Arc, Mutex};

use axum::{
    Router,
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

use crate::protocol::*;
use crate::transport::Transport;

pub type SharedTransport = Arc<Mutex<Transport>>;

#[derive(Clone)]
pub struct AppState {
    pub transport: SharedTransport,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct StatusResponse {
    pub power: String,
}

#[derive(Serialize, Deserialize)]
pub struct VolumeResponse {
    pub level: u8,
}

#[derive(Serialize, Deserialize)]
pub struct MuteResponse {
    pub muted: bool,
}

#[derive(Serialize, Deserialize)]
pub struct InputResponse {
    pub input_type: u8,
    pub input_num: u8,
}

#[derive(Serialize, Deserialize)]
pub struct ProductInfoResponse {
    pub info1: String,
    pub info2: String,
    pub info3: String,
}

#[derive(Serialize, Deserialize)]
pub struct DeviceIdResponse {
    pub id: String,
}

pub async fn start_http_server(
    device_path: String,
    host: String,
    port: u16,
) -> Result<(), Box<dyn std::error::Error>> {
    let transport = Transport::new(&device_path)?;
    let shared_transport: SharedTransport = Arc::new(Mutex::new(transport));

    let state = AppState {
        transport: shared_transport,
    };

    let app = Router::new()
        // Status routes
        .route("/status", get(get_status))
        .route("/power", get(get_power))
        .route("/volume", get(get_volume))
        .route("/input", get(get_input))
        .route("/mute", get(get_mute))
        .route("/product-info", get(get_product_info))
        .route("/device-id", get(get_device_id))
        // Power routes
        .route("/power/{action}", post(power_control))
        .route("/standby/{action}", post(standby_control))
        // Input routes
        .route("/input/{source}", post(input_control))
        // Volume routes
        .route("/volume/{action}", post(volume_control))
        .route("/volume/set/{value}", post(volume_set))
        // Mute routes
        .route("/mute/toggle", post(mute_toggle))
        .route("/mute/{action}", post(mute_control))
        // Sleep timer
        .route("/sleep/{minutes}", post(sleep_control))
        .route("/sleep/toggle", post(sleep_toggle))
        // Picture mode
        .route("/picture/mode/{mode}", post(picture_mode_control))
        .route("/picture/mode/toggle", post(picture_mode_toggle))
        .route("/picture/brightness/{action}", post(brightness_control))
        .route("/picture/brightness/set/{value}", post(brightness_set))
        .route("/picture/contrast/{action}", post(contrast_control))
        .route("/picture/contrast/set/{value}", post(contrast_set))
        .route("/picture/color/{action}", post(color_control))
        .route("/picture/color/set/{value}", post(color_set))
        .route("/picture/hue/red/{action}", post(hue_red_control))
        .route("/picture/hue/red/set/{value}", post(hue_red_set))
        .route("/picture/hue/green/{action}", post(hue_green_control))
        .route("/picture/hue/green/set/{value}", post(hue_green_set))
        .route("/picture/sharpness/{action}", post(sharpness_control))
        .route("/picture/sharpness/set/{value}", post(sharpness_set))
        .route("/picture/off", post(picture_off))
        .route("/picture/on", post(picture_on))
        .route("/picture/toggle", post(picture_toggle))
        .route("/picture/cine-motion/{action}", post(cine_motion_control))
        // Input toggle
        .route("/input/toggle", post(input_toggle))
        // Display toggle
        .route("/display/toggle", post(display_toggle))
        // Language
        .route("/language/{code}", post(language_control))
        // Screen routes
        .route("/screen/wide/{mode}", post(wide_control))
        .route("/screen/auto-wide/{action}", post(auto_wide_control))
        .route("/screen/4-3-mode/{mode}", post(four_three_mode_control))
        .route("/screen/h-shift/{action}", post(h_shift_control))
        .route("/screen/h-shift/set/{value}", post(h_shift_set))
        .route("/screen/v-size/{action}", post(v_size_control))
        .route("/screen/v-size/set/{value}", post(v_size_set))
        .route("/screen/v-shift/{action}", post(v_shift_control))
        .route("/screen/v-shift/set/{value}", post(v_shift_set))
        // Sound routes
        .route("/sound/mode/{mode}", post(sound_mode_control))
        .route("/sound/speaker/{action}", post(speaker_control))
        // SIRCS routes
        .route("/sircs/{button}", post(sircs_control))
        .with_state(state);

    let addr = format!("{}:{}", host, port);
    println!("HTTP server listening on {}", addr);

    let listener = TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

// === Status Routes ===

async fn get_status(State(app_state): State<AppState>) -> Result<Json<StatusResponse>, StatusCode> {
    let mut transport = app_state.transport.lock().unwrap();
    match transport.query::<Power>() {
        Ok(state) => Ok(Json(StatusResponse {
            power: match state {
                PowerState::On => "on".to_string(),
                PowerState::Off => "off".to_string(),
            },
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_power(State(app_state): State<AppState>) -> Result<Json<StatusResponse>, StatusCode> {
    let mut transport = app_state.transport.lock().unwrap();
    match transport.query::<Power>() {
        Ok(state) => Ok(Json(StatusResponse {
            power: match state {
                PowerState::On => "on".to_string(),
                PowerState::Off => "off".to_string(),
            },
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_volume(State(app_state): State<AppState>) -> Result<Json<VolumeResponse>, StatusCode> {
    let mut transport = app_state.transport.lock().unwrap();
    match transport.query::<Volume>() {
        Ok(level) => Ok(Json(VolumeResponse { level })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_input(State(app_state): State<AppState>) -> Result<Json<InputResponse>, StatusCode> {
    let mut transport = app_state.transport.lock().unwrap();
    match transport.query::<InputSelect>() {
        Ok(state) => Ok(Json(InputResponse {
            input_type: state.input_type,
            input_num: state.input_num,
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_mute(State(app_state): State<AppState>) -> Result<Json<MuteResponse>, StatusCode> {
    let mut transport = app_state.transport.lock().unwrap();
    match transport.query::<Muting>() {
        Ok(state) => Ok(Json(MuteResponse {
            muted: state == MuteState::Muted,
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_product_info(State(app_state): State<AppState>) -> Result<Json<ProductInfoResponse>, StatusCode> {
    let mut transport = app_state.transport.lock().unwrap();
    let info1 = transport.query::<ProductInfo1>().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let info2 = transport.query::<ProductInfo2>().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let info3 = transport.query::<ProductInfo3>().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ProductInfoResponse {
        info1: String::from_utf8_lossy(&info1).to_string(),
        info2: String::from_utf8_lossy(&info2).to_string(),
        info3: String::from_utf8_lossy(&info3).to_string(),
    }))
}

async fn get_device_id(State(app_state): State<AppState>) -> Result<Json<DeviceIdResponse>, StatusCode> {
    let mut transport = app_state.transport.lock().unwrap();
    match transport.query::<IdCommand>() {
        Ok(id) => Ok(Json(DeviceIdResponse {
            id: String::from_utf8_lossy(&id).to_string(),
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// === Power Routes ===

async fn power_control(
    Path(action): Path<String>,
    State(app_state): State<AppState>,
) -> Result<Json<ApiResponse>, StatusCode> {
    let mut transport = app_state.transport.lock().unwrap();
    let action = match action.as_str() {
        "on" => PowerAction::On,
        "off" => PowerAction::Off,
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    match transport.execute::<Power>(&action) {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            message: format!("Power {}", action_str(&action)),
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

fn action_str(action: &PowerAction) -> &str {
    match action {
        PowerAction::On => "on",
        PowerAction::Off => "off",
    }
}

async fn standby_control(
    Path(action): Path<String>,
    State(app_state): State<AppState>,
) -> Result<Json<ApiResponse>, StatusCode> {
    let mut transport = app_state.transport.lock().unwrap();
    let action = match action.as_str() {
        "enable" => StandbyAction::Enable,
        "disable" => StandbyAction::Disable,
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    match transport.execute::<Standby>(&action) {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            message: format!("Standby {}", if matches!(action, StandbyAction::Enable) { "enabled" } else { "disabled" }),
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// === Input Routes ===

async fn input_control(
    Path(source): Path<String>,
    State(app_state): State<AppState>,
) -> Result<Json<ApiResponse>, StatusCode> {
    let mut transport = app_state.transport.lock().unwrap();
    let input = parse_input_source(&source)?;

    match transport.execute::<InputSelect>(&input) {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            message: format!("Input set to {}", source),
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

fn parse_input_source(source: &str) -> Result<InputType, StatusCode> {
    if let Some(stripped) = source.strip_prefix("hdmi") {
        let num = stripped
            .parse::<u8>()
            .map_err(|_| StatusCode::BAD_REQUEST)?;
        Ok(InputType::Hdmi(num))
    } else if let Some(stripped) = source.strip_prefix("component") {
        let num = stripped
            .parse::<u8>()
            .map_err(|_| StatusCode::BAD_REQUEST)?;
        Ok(InputType::Component(num))
    } else if let Some(stripped) = source.strip_prefix("video") {
        let num = stripped
            .parse::<u8>()
            .map_err(|_| StatusCode::BAD_REQUEST)?;
        Ok(InputType::Video(num))
    } else if let Some(stripped) = source.strip_prefix("pc") {
        let num = stripped
            .parse::<u8>()
            .map_err(|_| StatusCode::BAD_REQUEST)?;
        Ok(InputType::Pc(num))
    } else if source == "shared1" {
        Ok(InputType::SharedInput(1))
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}

// === Volume Routes ===

async fn volume_control(
    Path(action): Path<String>,
    State(app_state): State<AppState>,
) -> Result<Json<ApiResponse>, StatusCode> {
    let mut transport = app_state.transport.lock().unwrap();
    let action = match action.as_str() {
        "up" => VolumeAction::Up,
        "down" => VolumeAction::Down,
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    match transport.execute::<Volume>(&action) {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            message: format!(
                "Volume {}",
                if matches!(action, VolumeAction::Up) {
                    "up"
                } else {
                    "down"
                }
            ),
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn volume_set(
    Path(value): Path<u8>,
    State(app_state): State<AppState>,
) -> Result<Json<ApiResponse>, StatusCode> {
    let vol = VolumeValue::new(value).map_err(|_| StatusCode::BAD_REQUEST)?;
    let mut transport = app_state.transport.lock().unwrap();

    match transport.execute::<Volume>(&VolumeAction::Set(vol)) {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            message: format!("Volume set to {}", value),
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// === Mute Routes ===

async fn mute_toggle(State(app_state): State<AppState>) -> Result<Json<ApiResponse>, StatusCode> {
    let mut transport = app_state.transport.lock().unwrap();
    match transport.execute::<Muting>(&MuteAction::Toggle) {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            message: "Mute toggled".to_string(),
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn mute_control(
    Path(action): Path<String>,
    State(app_state): State<AppState>,
) -> Result<Json<ApiResponse>, StatusCode> {
    let mut transport = app_state.transport.lock().unwrap();
    let action = match action.as_str() {
        "on" => MuteAction::Mute,
        "off" => MuteAction::Unmute,
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    match transport.execute::<Muting>(&action) {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            message: format!(
                "Mute {}",
                if matches!(action, MuteAction::Mute) {
                    "on"
                } else {
                    "off"
                }
            ),
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// === Sleep Timer Routes ===

async fn sleep_control(
    Path(minutes): Path<u8>,
    State(app_state): State<AppState>,
) -> Result<Json<ApiResponse>, StatusCode> {
    let sleep_mins = SleepMinutes::new(minutes).map_err(|_| StatusCode::BAD_REQUEST)?;
    let mut transport = app_state.transport.lock().unwrap();

    match transport.execute::<OffTimer>(&SleepAction::Set(sleep_mins)) {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            message: format!("Sleep timer set to {} minutes", minutes),
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn sleep_toggle(State(app_state): State<AppState>) -> Result<Json<ApiResponse>, StatusCode> {
    let mut transport = app_state.transport.lock().unwrap();
    match transport.execute::<OffTimer>(&SleepAction::Toggle) {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            message: "Sleep timer toggled".to_string(),
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// === Picture Mode Routes ===

async fn picture_mode_control(
    Path(mode): Path<String>,
    State(app_state): State<AppState>,
) -> Result<Json<ApiResponse>, StatusCode> {
    let mut transport = app_state.transport.lock().unwrap();
    let mode_action = match mode.as_str() {
        "vivid" => PictureModeAction::Vivid,
        "standard" => PictureModeAction::Standard,
        "cinema" => PictureModeAction::Cinema,
        "custom" => PictureModeAction::Custom,
        "game" => PictureModeAction::Game,
        "graphics" => PictureModeAction::Graphics,
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    match transport.execute::<PictureMode>(&mode_action) {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            message: format!("Picture mode set to {}", mode),
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn picture_mode_toggle(State(app_state): State<AppState>) -> Result<Json<ApiResponse>, StatusCode> {
    let mut transport = app_state.transport.lock().unwrap();
    match transport.execute::<PictureMode>(&PictureModeAction::Toggle) {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            message: "Picture mode toggled".to_string(),
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn brightness_control(
    Path(action): Path<String>,
    State(app_state): State<AppState>,
) -> Result<Json<ApiResponse>, StatusCode> {
    let mut transport = app_state.transport.lock().unwrap();
    let action = match action.as_str() {
        "up" => BrightnessAction::Up,
        "down" => BrightnessAction::Down,
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    match transport.execute::<Brightness>(&action) {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            message: format!(
                "Brightness {}",
                if matches!(action, BrightnessAction::Up) {
                    "up"
                } else {
                    "down"
                }
            ),
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn brightness_set(
    Path(value): Path<u8>,
    State(app_state): State<AppState>,
) -> Result<Json<ApiResponse>, StatusCode> {
    let brightness = BrightnessValue::new(value).map_err(|_| StatusCode::BAD_REQUEST)?;
    let mut transport = app_state.transport.lock().unwrap();

    match transport.execute::<Brightness>(&BrightnessAction::Set(brightness)) {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            message: format!("Brightness set to {}", value),
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn contrast_control(
    Path(action): Path<String>,
    State(app_state): State<AppState>,
) -> Result<Json<ApiResponse>, StatusCode> {
    let mut transport = app_state.transport.lock().unwrap();
    let action = match action.as_str() {
        "up" => ContrastAction::Up,
        "down" => ContrastAction::Down,
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    match transport.execute::<Contrast>(&action) {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            message: format!(
                "Contrast {}",
                if matches!(action, ContrastAction::Up) {
                    "up"
                } else {
                    "down"
                }
            ),
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn contrast_set(
    Path(value): Path<u8>,
    State(app_state): State<AppState>,
) -> Result<Json<ApiResponse>, StatusCode> {
    let contrast = ContrastValue::new(value).map_err(|_| StatusCode::BAD_REQUEST)?;
    let mut transport = app_state.transport.lock().unwrap();

    match transport.execute::<Contrast>(&ContrastAction::Set(contrast)) {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            message: format!("Contrast set to {}", value),
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn color_control(
    Path(action): Path<String>,
    State(app_state): State<AppState>,
) -> Result<Json<ApiResponse>, StatusCode> {
    let mut transport = app_state.transport.lock().unwrap();
    let action = match action.as_str() {
        "up" => ColorAction::Up,
        "down" => ColorAction::Down,
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    match transport.execute::<Color>(&action) {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            message: format!(
                "Color {}",
                if matches!(action, ColorAction::Up) {
                    "up"
                } else {
                    "down"
                }
            ),
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn color_set(
    Path(value): Path<u8>,
    State(app_state): State<AppState>,
) -> Result<Json<ApiResponse>, StatusCode> {
    let color = ColorValue::new(value).map_err(|_| StatusCode::BAD_REQUEST)?;
    let mut transport = app_state.transport.lock().unwrap();

    match transport.execute::<Color>(&ColorAction::Set(color)) {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            message: format!("Color set to {}", value),
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn sharpness_control(
    Path(action): Path<String>,
    State(app_state): State<AppState>,
) -> Result<Json<ApiResponse>, StatusCode> {
    let mut transport = app_state.transport.lock().unwrap();
    let action = match action.as_str() {
        "up" => SharpnessAction::Up,
        "down" => SharpnessAction::Down,
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    match transport.execute::<Sharpness>(&action) {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            message: format!(
                "Sharpness {}",
                if matches!(action, SharpnessAction::Up) {
                    "up"
                } else {
                    "down"
                }
            ),
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn sharpness_set(
    Path(value): Path<u8>,
    State(app_state): State<AppState>,
) -> Result<Json<ApiResponse>, StatusCode> {
    let sharpness = SharpnessValue::new(value).map_err(|_| StatusCode::BAD_REQUEST)?;
    let mut transport = app_state.transport.lock().unwrap();

    match transport.execute::<Sharpness>(&SharpnessAction::Set(sharpness)) {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            message: format!("Sharpness set to {}", value),
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn hue_red_control(
    Path(action): Path<String>,
    State(app_state): State<AppState>,
) -> Result<Json<ApiResponse>, StatusCode> {
    let mut transport = app_state.transport.lock().unwrap();
    let action = match action.as_str() {
        "up" => HueAction::Up(HueChannel::Red),
        "down" => HueAction::Down(HueChannel::Red),
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    match transport.execute::<Hue>(&action) {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            message: format!("Hue red {}", if matches!(action, HueAction::Up(_)) { "up" } else { "down" }),
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn hue_red_set(
    Path(value): Path<u8>,
    State(app_state): State<AppState>,
) -> Result<Json<ApiResponse>, StatusCode> {
    let hue = HueValue::new(value).map_err(|_| StatusCode::BAD_REQUEST)?;
    let mut transport = app_state.transport.lock().unwrap();

    match transport.execute::<Hue>(&HueAction::Set(HueChannel::Red, hue)) {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            message: format!("Hue red set to {}", value),
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn hue_green_control(
    Path(action): Path<String>,
    State(app_state): State<AppState>,
) -> Result<Json<ApiResponse>, StatusCode> {
    let mut transport = app_state.transport.lock().unwrap();
    let action = match action.as_str() {
        "up" => HueAction::Up(HueChannel::Green),
        "down" => HueAction::Down(HueChannel::Green),
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    match transport.execute::<Hue>(&action) {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            message: format!("Hue green {}", if matches!(action, HueAction::Up(_)) { "up" } else { "down" }),
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn hue_green_set(
    Path(value): Path<u8>,
    State(app_state): State<AppState>,
) -> Result<Json<ApiResponse>, StatusCode> {
    let hue = HueValue::new(value).map_err(|_| StatusCode::BAD_REQUEST)?;
    let mut transport = app_state.transport.lock().unwrap();

    match transport.execute::<Hue>(&HueAction::Set(HueChannel::Green, hue)) {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            message: format!("Hue green set to {}", value),
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn picture_off(State(app_state): State<AppState>) -> Result<Json<ApiResponse>, StatusCode> {
    let mut transport = app_state.transport.lock().unwrap();
    match transport.execute::<PictureOff>(&PictureOffAction::Off) {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            message: "Picture off".to_string(),
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn picture_on(State(app_state): State<AppState>) -> Result<Json<ApiResponse>, StatusCode> {
    let mut transport = app_state.transport.lock().unwrap();
    match transport.execute::<PictureOff>(&PictureOffAction::On) {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            message: "Picture on".to_string(),
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn picture_toggle(State(app_state): State<AppState>) -> Result<Json<ApiResponse>, StatusCode> {
    let mut transport = app_state.transport.lock().unwrap();
    match transport.execute::<PictureOff>(&PictureOffAction::Toggle) {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            message: "Picture toggled".to_string(),
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn cine_motion_control(
    Path(action): Path<String>,
    State(app_state): State<AppState>,
) -> Result<Json<ApiResponse>, StatusCode> {
    let mut transport = app_state.transport.lock().unwrap();
    let action = match action.as_str() {
        "off" => CineMotionAction::Off,
        "on" | "auto" => CineMotionAction::Auto,
        "toggle" => {
            return Err(StatusCode::BAD_REQUEST);
        }
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    match transport.execute::<CineMotion>(&action) {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            message: format!("Cine motion {}", if matches!(action, CineMotionAction::Off) { "off" } else { "on" }),
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// === Input Toggle Route ===

async fn input_toggle(State(app_state): State<AppState>) -> Result<Json<ApiResponse>, StatusCode> {
    let mut transport = app_state.transport.lock().unwrap();
    match transport.execute::<InputSelect>(&InputType::Toggle) {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            message: "Input toggled".to_string(),
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// === Display Toggle Route ===

async fn display_toggle(State(app_state): State<AppState>) -> Result<Json<ApiResponse>, StatusCode> {
    let mut transport = app_state.transport.lock().unwrap();
    match transport.execute::<Display>(&()) {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            message: "Display toggled".to_string(),
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// === Language Route ===

async fn language_control(
    Path(code): Path<String>,
    State(app_state): State<AppState>,
) -> Result<Json<ApiResponse>, StatusCode> {
    let language_code = LanguageCode::new(&code).map_err(|_| StatusCode::BAD_REQUEST)?;
    let mut transport = app_state.transport.lock().unwrap();

    match transport.execute::<Language>(&language_code) {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            message: format!("Language set to {}", code),
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// === Screen Routes ===

async fn wide_control(
    Path(mode): Path<String>,
    State(app_state): State<AppState>,
) -> Result<Json<ApiResponse>, StatusCode> {
    let mut transport = app_state.transport.lock().unwrap();
    let action = match mode.as_str() {
        "toggle" => WideAction::Toggle,
        "widezoom" => WideAction::WideZoom,
        "full" => WideAction::Full,
        "zoom" => WideAction::Zoom,
        "normal" => WideAction::Normal,
        "pcnormal" => WideAction::PcNormal,
        "pcfull1" => WideAction::PcFull1,
        "pcfull2" => WideAction::PcFull2,
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    match transport.execute::<Wide>(&action) {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            message: format!("Wide mode set to {}", mode),
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn auto_wide_control(
    Path(action): Path<String>,
    State(app_state): State<AppState>,
) -> Result<Json<ApiResponse>, StatusCode> {
    let mut transport = app_state.transport.lock().unwrap();
    let action = match action.as_str() {
        "toggle" => AutoWideAction::Toggle,
        "on" => AutoWideAction::On,
        "off" => AutoWideAction::Off,
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    match transport.execute::<AutoWide>(&action) {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            message: "Auto wide control executed".to_string(),
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn four_three_mode_control(
    Path(mode): Path<String>,
    State(app_state): State<AppState>,
) -> Result<Json<ApiResponse>, StatusCode> {
    let mut transport = app_state.transport.lock().unwrap();
    let action = match mode.as_str() {
        "toggle" => FourThreeModeAction::Toggle,
        "off" => FourThreeModeAction::Off,
        "widezoom" => FourThreeModeAction::WideZoom,
        "normal" => FourThreeModeAction::Normal,
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    match transport.execute::<FourThreeMode>(&action) {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            message: format!("4:3 mode set to {}", mode),
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn h_shift_control(
    Path(action): Path<String>,
    State(app_state): State<AppState>,
) -> Result<Json<ApiResponse>, StatusCode> {
    let mut transport = app_state.transport.lock().unwrap();
    let action = match action.as_str() {
        "up" => HShiftAction::Up,
        "down" => HShiftAction::Down,
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    match transport.execute::<HShift>(&action) {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            message: format!("H-shift {}", if matches!(action, HShiftAction::Up) { "up" } else { "down" }),
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn h_shift_set(
    Path(value): Path<u8>,
    State(app_state): State<AppState>,
) -> Result<Json<ApiResponse>, StatusCode> {
    if value > 134 {
        return Err(StatusCode::BAD_REQUEST);
    }

    let mut transport = app_state.transport.lock().unwrap();
    let action = if value >= 67 {
        HShiftAction::SetPlus(value - 67)
    } else {
        HShiftAction::SetMinus(67 - value)
    };

    match transport.execute::<HShift>(&action) {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            message: format!("H-shift set to {}", value),
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn v_size_control(
    Path(action): Path<String>,
    State(app_state): State<AppState>,
) -> Result<Json<ApiResponse>, StatusCode> {
    let mut transport = app_state.transport.lock().unwrap();
    let action = match action.as_str() {
        "up" => VSizeAction::Up,
        "down" => VSizeAction::Down,
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    match transport.execute::<VSize>(&action) {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            message: format!("V-size {}", if matches!(action, VSizeAction::Up) { "up" } else { "down" }),
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn v_size_set(
    Path(value): Path<u8>,
    State(app_state): State<AppState>,
) -> Result<Json<ApiResponse>, StatusCode> {
    if value > 99 {
        return Err(StatusCode::BAD_REQUEST);
    }

    let mut transport = app_state.transport.lock().unwrap();
    let action = if value >= 50 {
        VSizeAction::SetPlus(value - 50)
    } else {
        VSizeAction::SetMinus(50 - value)
    };

    match transport.execute::<VSize>(&action) {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            message: format!("V-size set to {}", value),
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn v_shift_control(
    Path(action): Path<String>,
    State(app_state): State<AppState>,
) -> Result<Json<ApiResponse>, StatusCode> {
    let mut transport = app_state.transport.lock().unwrap();
    let action = match action.as_str() {
        "up" => VShiftAction::Up,
        "down" => VShiftAction::Down,
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    match transport.execute::<VShift>(&action) {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            message: format!("V-shift {}", if matches!(action, VShiftAction::Up) { "up" } else { "down" }),
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn v_shift_set(
    Path(value): Path<u8>,
    State(app_state): State<AppState>,
) -> Result<Json<ApiResponse>, StatusCode> {
    if value > 99 {
        return Err(StatusCode::BAD_REQUEST);
    }

    let mut transport = app_state.transport.lock().unwrap();
    let action = if value >= 50 {
        VShiftAction::SetPlus(value - 50)
    } else {
        VShiftAction::SetMinus(50 - value)
    };

    match transport.execute::<VShift>(&action) {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            message: format!("V-shift set to {}", value),
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// === Sound Routes ===

async fn sound_mode_control(
    Path(mode): Path<String>,
    State(app_state): State<AppState>,
) -> Result<Json<ApiResponse>, StatusCode> {
    let mut transport = app_state.transport.lock().unwrap();
    let action = match mode.as_str() {
        "toggle" => SoundModeAction::Toggle,
        "standard" => SoundModeAction::Standard,
        "cinema" => SoundModeAction::Cinema,
        "sports" => SoundModeAction::Sports,
        "music" => SoundModeAction::Music,
        "game" => SoundModeAction::Game,
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    match transport.execute::<SoundMode>(&action) {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            message: format!("Sound mode set to {}", mode),
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn speaker_control(
    Path(action): Path<String>,
    State(app_state): State<AppState>,
) -> Result<Json<ApiResponse>, StatusCode> {
    let mut transport = app_state.transport.lock().unwrap();
    let action = match action.as_str() {
        "toggle" => SpeakerAction::Toggle,
        "on" => SpeakerAction::On,
        "off" => SpeakerAction::Off,
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    match transport.execute::<Speaker>(&action) {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            message: "Speaker control executed".to_string(),
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// === SIRCS Routes ===

async fn sircs_control(
    Path(button): Path<String>,
    State(app_state): State<AppState>,
) -> Result<Json<ApiResponse>, StatusCode> {
    let mut transport = app_state.transport.lock().unwrap();
    let button = match button.as_str() {
        "input" => SircsButton::Input,
        "power" => SircsButton::Power,
        "wide-mode" => SircsButton::WideMode,
        "dot" => SircsButton::Dot,
        "display" => SircsButton::Display,
        "return" => SircsButton::Return,
        "options" => SircsButton::Options,
        "home" => SircsButton::Home,
        "up" => SircsButton::CursorUp,
        "down" => SircsButton::CursorDown,
        "left" => SircsButton::CursorLeft,
        "right" => SircsButton::CursorRight,
        "select" => SircsButton::Select,
        "1" => SircsButton::Num1,
        "2" => SircsButton::Num2,
        "3" => SircsButton::Num3,
        "4" => SircsButton::Num4,
        "5" => SircsButton::Num5,
        "6" => SircsButton::Num6,
        "7" => SircsButton::Num7,
        "8" => SircsButton::Num8,
        "9" => SircsButton::Num9,
        "0" => SircsButton::Num0,
        "cc" => SircsButton::ClosedCaption,
        "volume-up" => SircsButton::VolumeUp,
        "volume-down" => SircsButton::VolumeDown,
        "muting" => SircsButton::Muting,
        "channel-up" => SircsButton::ChannelUp,
        "channel-down" => SircsButton::ChannelDown,
        "jump" => SircsButton::Jump,
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    match transport.execute::<Sircs>(&button) {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            message: "SIRCS button pressed".to_string(),
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
