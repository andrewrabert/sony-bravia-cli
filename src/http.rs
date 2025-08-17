use std::sync::{Arc, Mutex};
use std::time::Duration;

use axum::{
    extract::{Path, Query, State},
    http::{StatusCode},
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

use crate::{
    brightness_down, brightness_max, brightness_min, brightness_up, display_toggle, input_select,
    is_powered_on, mute_toggle, picture_off, picture_on, picture_toggle, power_off, power_on,
    power_toggle, volume_down, volume_up, INPUT_TYPE_HDMI,
};
use crate::mcp::{SonyBraviaServer, JsonRpcRequest, JsonRpcResponse};

#[derive(Serialize, Deserialize)]
pub struct ApiResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct StatusResponse {
    pub power: String,
}

pub type SharedPort = Arc<Mutex<Box<dyn serialport::SerialPort + Send>>>;

#[derive(Clone)]
pub struct AppState {
    pub port: SharedPort,
    pub mcp_server: Arc<SonyBraviaServer>,
}

pub async fn start_http_server(device_path: String, host: String, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let serial_port = serialport::new(&device_path, 9600)
        .timeout(Duration::from_millis(500))
        .open()
        .expect("Failed to open port.");

    let shared_port: SharedPort = Arc::new(Mutex::new(serial_port));
    let mcp_server = Arc::new(SonyBraviaServer::new(device_path)?);

    let state = AppState {
        port: shared_port,
        mcp_server,
    };

    let app = Router::new()
        .route("/status", get(get_status))
        .route("/power/{action}", post(power_control))
        .route("/picture/{action}", post(picture_control))
        .route("/volume/{action}", post(volume_control))
        .route("/brightness/{action}", post(brightness_control))
        .route("/display/toggle", post(display_control))
        .route("/mute/toggle", post(mute_control))
        .route("/input/hdmi/{port}", post(input_hdmi_control))
        .route("/mcp", get(mcp_endpoint_get).post(mcp_endpoint_post))
        .with_state(state);

    let addr = format!("{}:{}", host, port);
    println!("HTTP server listening on {}", addr);

    let listener = TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn get_status(State(app_state): State<AppState>) -> Result<Json<StatusResponse>, StatusCode> {
    let mut port_guard = app_state.port.lock().unwrap();
    let powered_on = is_powered_on(&mut **port_guard);
    Ok(Json(StatusResponse {
        power: if powered_on { "on".to_string() } else { "off".to_string() },
    }))
}

async fn power_control(Path(action): Path<String>, State(app_state): State<AppState>) -> Result<Json<ApiResponse>, StatusCode> {
    let mut port_guard = app_state.port.lock().unwrap();
    match action.as_str() {
        "on" => {
            power_on(&mut **port_guard);
            Ok(Json(ApiResponse { success: true, message: "Power on".to_string() }))
        }
        "off" => {
            power_off(&mut **port_guard);
            Ok(Json(ApiResponse { success: true, message: "Power off".to_string() }))
        }
        "toggle" => {
            power_toggle(&mut **port_guard);
            Ok(Json(ApiResponse { success: true, message: "Power toggle".to_string() }))
        }
        _ => Err(StatusCode::BAD_REQUEST),
    }
}

async fn picture_control(Path(action): Path<String>, State(app_state): State<AppState>) -> Result<Json<ApiResponse>, StatusCode> {
    let mut port_guard = app_state.port.lock().unwrap();
    match action.as_str() {
        "on" => {
            picture_on(&mut **port_guard);
            Ok(Json(ApiResponse { success: true, message: "Picture on".to_string() }))
        }
        "off" => {
            picture_off(&mut **port_guard);
            Ok(Json(ApiResponse { success: true, message: "Picture off".to_string() }))
        }
        "toggle" => {
            picture_toggle(&mut **port_guard);
            Ok(Json(ApiResponse { success: true, message: "Picture toggle".to_string() }))
        }
        _ => Err(StatusCode::BAD_REQUEST),
    }
}

async fn volume_control(Path(action): Path<String>, State(app_state): State<AppState>) -> Result<Json<ApiResponse>, StatusCode> {
    let mut port_guard = app_state.port.lock().unwrap();
    match action.as_str() {
        "up" => {
            volume_up(&mut **port_guard);
            Ok(Json(ApiResponse { success: true, message: "Volume up".to_string() }))
        }
        "down" => {
            volume_down(&mut **port_guard);
            Ok(Json(ApiResponse { success: true, message: "Volume down".to_string() }))
        }
        _ => Err(StatusCode::BAD_REQUEST),
    }
}

async fn brightness_control(Path(action): Path<String>, State(app_state): State<AppState>) -> Result<Json<ApiResponse>, StatusCode> {
    let mut port_guard = app_state.port.lock().unwrap();
    match action.as_str() {
        "up" => {
            brightness_up(&mut **port_guard);
            Ok(Json(ApiResponse { success: true, message: "Brightness up".to_string() }))
        }
        "down" => {
            brightness_down(&mut **port_guard);
            Ok(Json(ApiResponse { success: true, message: "Brightness down".to_string() }))
        }
        "min" => {
            brightness_min(&mut **port_guard);
            Ok(Json(ApiResponse { success: true, message: "Brightness min".to_string() }))
        }
        "max" => {
            brightness_max(&mut **port_guard);
            Ok(Json(ApiResponse { success: true, message: "Brightness max".to_string() }))
        }
        _ => Err(StatusCode::BAD_REQUEST),
    }
}

async fn display_control(State(app_state): State<AppState>) -> Result<Json<ApiResponse>, StatusCode> {
    let mut port_guard = app_state.port.lock().unwrap();
    display_toggle(&mut **port_guard);
    Ok(Json(ApiResponse { success: true, message: "Display toggle".to_string() }))
}

async fn mute_control(State(app_state): State<AppState>) -> Result<Json<ApiResponse>, StatusCode> {
    let mut port_guard = app_state.port.lock().unwrap();
    mute_toggle(&mut **port_guard);
    Ok(Json(ApiResponse { success: true, message: "Mute toggle".to_string() }))
}

async fn input_hdmi_control(Path(port_num): Path<u8>, State(app_state): State<AppState>) -> Result<Json<ApiResponse>, StatusCode> {
    let mut port_guard = app_state.port.lock().unwrap();
    input_select(&mut **port_guard, INPUT_TYPE_HDMI, port_num);
    Ok(Json(ApiResponse { success: true, message: format!("Input HDMI {}", port_num) }))
}

async fn mcp_endpoint_post(State(app_state): State<AppState>, Json(request): Json<JsonRpcRequest>) -> Json<JsonRpcResponse> {
    let response = app_state.mcp_server.handle_request(request).await;
    Json(response)
}

async fn mcp_endpoint_get(
    State(app_state): State<AppState>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<JsonRpcResponse>, StatusCode> {
    if let Some(json_str) = params.get("request") {
        match serde_json::from_str::<JsonRpcRequest>(json_str) {
            Ok(request) => {
                let response = app_state.mcp_server.handle_request(request).await;
                Ok(Json(response))
            }
            Err(_) => Err(StatusCode::BAD_REQUEST),
        }
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}
