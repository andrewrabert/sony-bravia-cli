use std::sync::{Arc, Mutex};
use std::time::Duration;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use clap::{arg, Command};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

const CONTROL_REQUEST: u8 = 0x8c;
const QUERY_REQUEST: u8 = 0x83;
const CATEGORY: u8 = 0x00;
const POWER_FUNCTION: u8 = 0x00;
const INPUT_SELECT_FUNCTION: u8 = 0x02;
const VOLUME_CONTROL_FUNCTION: u8 = 0x05;
const PICTURE_FUNCTION: u8 = 0x0d;
const DISPLAY_FUNCTION: u8 = 0x0f;
const BRIGHTNESS_CONTROL_FUNCTION: u8 = 0x24;
const MUTING_FUNCTION: u8 = 0x06;

const INPUT_TYPE_HDMI: u8 = 0x04;

const RESPONSE_HEADER: u8 = 0x70;
const RESPONSE_ANSWER: u8 = 0x00;

#[derive(Serialize, Deserialize)]
struct ApiResponse {
    success: bool,
    message: String,
}

#[derive(Serialize, Deserialize)]
struct StatusResponse {
    power: String,
}

type SharedPort = Arc<Mutex<Box<dyn serialport::SerialPort + Send>>>;

fn checksum(command: &[u8]) -> u8 {
    let s: u8 = command.iter().fold(0u8, |acc, &b| acc.wrapping_add(b));
    s % 255
}

fn power_on(port: &mut dyn serialport::SerialPort) {
    let args = vec![CONTROL_REQUEST, CATEGORY, POWER_FUNCTION, 0x02, 0x01];
    write_command(port, args);
}

fn power_off(port: &mut dyn serialport::SerialPort) {
    let args = vec![CONTROL_REQUEST, CATEGORY, POWER_FUNCTION, 0x02, 0x00];
    write_command(port, args);
}

fn picture_toggle(port: &mut dyn serialport::SerialPort) {
    let args = vec![CONTROL_REQUEST, CATEGORY, PICTURE_FUNCTION, 0x02, 0x00];
    write_command(port, args);
}

fn picture_on(port: &mut dyn serialport::SerialPort) {
    let args = vec![CONTROL_REQUEST, CATEGORY, PICTURE_FUNCTION, 0x02, 0x01];
    write_command(port, args);
}

fn picture_off(port: &mut dyn serialport::SerialPort) {
    let args = vec![CONTROL_REQUEST, CATEGORY, PICTURE_FUNCTION, 0x02, 0x00];
    write_command(port, args);
}

fn display_toggle(port: &mut dyn serialport::SerialPort) {
    let args = vec![CONTROL_REQUEST, CATEGORY, DISPLAY_FUNCTION, 0x02, 0x00];
    write_command(port, args);
}

fn brightness_up(port: &mut dyn serialport::SerialPort) {
    let args = vec![
        CONTROL_REQUEST,
        CATEGORY,
        BRIGHTNESS_CONTROL_FUNCTION,
        0x03,
        0x00,
        0x00,
    ];
    write_command(port, args);
}

fn brightness_down(port: &mut dyn serialport::SerialPort) {
    let args = vec![
        CONTROL_REQUEST,
        CATEGORY,
        BRIGHTNESS_CONTROL_FUNCTION,
        0x03,
        0x00,
        0x01,
    ];
    write_command(port, args);
}

fn brightness_min(port: &mut dyn serialport::SerialPort) {
    let args = vec![
        CONTROL_REQUEST,
        CATEGORY,
        BRIGHTNESS_CONTROL_FUNCTION,
        0x03,
        0x01,
        0x00,
    ];
    write_command(port, args);
}

fn brightness_max(port: &mut dyn serialport::SerialPort) {
    let args = vec![
        CONTROL_REQUEST,
        CATEGORY,
        BRIGHTNESS_CONTROL_FUNCTION,
        0x03,
        0x01,
        0x32,
    ];
    write_command(port, args);
}

fn volume_up(port: &mut dyn serialport::SerialPort) {
    let args = vec![
        CONTROL_REQUEST,
        CATEGORY,
        VOLUME_CONTROL_FUNCTION,
        0x03,
        0x00,
        0x00,
    ];
    write_command(port, args);
}

fn volume_down(port: &mut dyn serialport::SerialPort) {
    let args = vec![
        CONTROL_REQUEST,
        CATEGORY,
        VOLUME_CONTROL_FUNCTION,
        0x03,
        0x00,
        0x01,
    ];
    write_command(port, args);
}

fn input_select(port: &mut dyn serialport::SerialPort, input_type: u8, input_num: u8) {
    let args = vec![
        CONTROL_REQUEST,
        CATEGORY,
        INPUT_SELECT_FUNCTION,
        0x03,
        input_type,
        input_num,
    ];
    write_command(port, args);
}

fn mute_toggle(port: &mut dyn serialport::SerialPort) {
    let args = vec![CONTROL_REQUEST, CATEGORY, MUTING_FUNCTION, 0x02, 0x00];
    write_command(port, args);
}

fn is_powered_on(port: &mut dyn serialport::SerialPort) -> bool {
    let args = vec![QUERY_REQUEST, CATEGORY, POWER_FUNCTION, 0xff, 0xff];
    let data = write_command(port, args);
    data[0] == 1
}

fn power_toggle(port: &mut dyn serialport::SerialPort) {
    if is_powered_on(port) {
        println!("is on - turning off!");
        power_off(port);
    } else {
        println!("is off - turning on!");
        power_on(port);
    }
}

fn print_status(port: &mut dyn serialport::SerialPort) {
    if is_powered_on(port) {
        println!("Power: on");
    } else {
        println!("Power: off");
    }
}

fn write_command(port: &mut dyn serialport::SerialPort, contents: Vec<u8>) -> Vec<u8> {
    let mut vec = contents.clone();
    let c = checksum(&vec);
    vec.push(c);
    port.write_all(&vec).unwrap();

    let mut resp_buf = vec![0; 3];
    port.read_exact(resp_buf.as_mut_slice())
        .expect("failure to read response");

    if resp_buf[0] != RESPONSE_HEADER {
        eprintln!("error: unexpected response header");
        std::process::exit(1);
    }
    if resp_buf[1] != RESPONSE_ANSWER {
        eprintln!("error: unexpected response answer");
        std::process::exit(1);
    }
    if vec[0] == QUERY_REQUEST {
        let mut resp_data_buf = vec![0; resp_buf[2] as usize];
        port.read_exact(resp_data_buf.as_mut_slice())
            .expect("failure to read response data");
        let resp_checksum = resp_data_buf.pop().expect("error");
        resp_buf.extend(resp_data_buf.clone());
        if resp_checksum != checksum(&resp_buf) {
            eprintln!("error: invalid response checksum");
            std::process::exit(1);
        }
        resp_data_buf
    } else {
        let resp_checksum = resp_buf.pop().expect("error");
        if resp_checksum != checksum(&resp_buf) {
            eprintln!("error: invalid response checksum");
            std::process::exit(1);
        }
        vec![0; 0]
    }
}

fn cli() -> Command {
    Command::new("sony-bravia-cli")
        .arg(arg!(--dev <DEVICE>).required(true))
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("power")
                .arg(arg!(<ACTION> "The action to perform").value_parser(["on", "off", "toggle"])),
        )
        .subcommand(
            Command::new("picture")
                .arg(arg!(<ACTION> "The action to perform").value_parser(["on", "off", "toggle"])),
        )
        .subcommand(
            Command::new("volume")
                .arg(arg!(<ACTION> "The action to perform").value_parser(["down", "up"])),
        )
        .subcommand(
            Command::new("mute")
                .arg(arg!(<ACTION> "The action to perform").value_parser(["toggle"])),
        )
        .subcommand(
            Command::new("display")
                .arg(arg!(<ACTION> "The action to perform").value_parser(["toggle"])),
        )
        .subcommand(
            Command::new("brightness").arg(
                arg!(<ACTION> "The action to perform").value_parser(["down", "up", "min", "max"]),
            ),
        )
        .subcommand(
            Command::new("input-hdmi")
                .arg(arg!(<PORT> "The action to perform").value_parser(clap::value_parser!(u8))),
        )
        .subcommand(Command::new("status"))
        .subcommand(
            Command::new("http-server")
                .arg(arg!(--port <PORT> "Port to listen on").default_value("8000").value_parser(clap::value_parser!(u16)))
                .arg(arg!(--host <HOST> "Host to bind to").default_value("127.0.0.1"))
        )
}

async fn start_http_server(device_path: String, host: String, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let serial_port = serialport::new(&device_path, 9600)
        .timeout(Duration::from_millis(500))
        .open()
        .expect("Failed to open port.");

    let shared_port: SharedPort = Arc::new(Mutex::new(serial_port));

    let app = Router::new()
        .route("/status", get(get_status))
        .route("/power/{action}", post(power_control))
        .route("/picture/{action}", post(picture_control))
        .route("/volume/{action}", post(volume_control))
        .route("/brightness/{action}", post(brightness_control))
        .route("/display/toggle", post(display_control))
        .route("/mute/toggle", post(mute_control))
        .route("/input/hdmi/{port}", post(input_hdmi_control))
        .with_state(shared_port);

    let addr = format!("{}:{}", host, port);
    println!("HTTP server listening on {}", addr);

    let listener = TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn get_status(State(port): State<SharedPort>) -> Result<Json<StatusResponse>, StatusCode> {
    let mut port_guard = port.lock().unwrap();
    let powered_on = is_powered_on(&mut **port_guard);
    Ok(Json(StatusResponse {
        power: if powered_on { "on".to_string() } else { "off".to_string() },
    }))
}

async fn power_control(Path(action): Path<String>, State(port): State<SharedPort>) -> Result<Json<ApiResponse>, StatusCode> {
    let mut port_guard = port.lock().unwrap();
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

async fn picture_control(Path(action): Path<String>, State(port): State<SharedPort>) -> Result<Json<ApiResponse>, StatusCode> {
    let mut port_guard = port.lock().unwrap();
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

async fn volume_control(Path(action): Path<String>, State(port): State<SharedPort>) -> Result<Json<ApiResponse>, StatusCode> {
    let mut port_guard = port.lock().unwrap();
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

async fn brightness_control(Path(action): Path<String>, State(port): State<SharedPort>) -> Result<Json<ApiResponse>, StatusCode> {
    let mut port_guard = port.lock().unwrap();
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

async fn display_control(State(port): State<SharedPort>) -> Result<Json<ApiResponse>, StatusCode> {
    let mut port_guard = port.lock().unwrap();
    display_toggle(&mut **port_guard);
    Ok(Json(ApiResponse { success: true, message: "Display toggle".to_string() }))
}

async fn mute_control(State(port): State<SharedPort>) -> Result<Json<ApiResponse>, StatusCode> {
    let mut port_guard = port.lock().unwrap();
    mute_toggle(&mut **port_guard);
    Ok(Json(ApiResponse { success: true, message: "Mute toggle".to_string() }))
}

async fn input_hdmi_control(Path(port_num): Path<u8>, State(port): State<SharedPort>) -> Result<Json<ApiResponse>, StatusCode> {
    let mut port_guard = port.lock().unwrap();
    input_select(&mut **port_guard, INPUT_TYPE_HDMI, port_num);
    Ok(Json(ApiResponse { success: true, message: format!("Input HDMI {}", port_num) }))
}

#[tokio::main]
async fn main() {
    let matches = cli().get_matches();
    let device_path = matches.get_one::<String>("dev").expect("required").clone();

    if let Some(("http-server", sub_matches)) = matches.subcommand() {
        let host = sub_matches.get_one::<String>("host").expect("required").clone();
        let port = *sub_matches.get_one::<u16>("port").expect("required");

        if let Err(e) = start_http_server(device_path, host, port).await {
            eprintln!("Server error: {}", e);
            std::process::exit(1);
        }
        return;
    }

    let mut port = serialport::new(&device_path, 9600)
        .timeout(Duration::from_millis(500))
        .open()
        .expect("Failed to open port.");
    match matches.subcommand() {
        Some(("power", sub_matches)) => {
            match sub_matches
                .get_one::<String>("ACTION")
                .expect("required")
                .as_str()
            {
                "on" => power_on(&mut *port),
                "off" => power_off(&mut *port),
                "toggle" => power_toggle(&mut *port),
                _ => unreachable!(),
            };
        }
        Some(("input-hdmi", sub_matches)) => {
            input_select(
                &mut *port,
                INPUT_TYPE_HDMI,
                *sub_matches.get_one::<u8>("PORT").expect("required"),
            );
        }
        Some(("picture", sub_matches)) => {
            match sub_matches
                .get_one::<String>("ACTION")
                .expect("required")
                .as_str()
            {
                "on" => picture_on(&mut *port),
                "off" => picture_off(&mut *port),
                "toggle" => picture_toggle(&mut *port),
                _ => unreachable!(),
            };
        }
        Some(("brightness", sub_matches)) => {
            match sub_matches
                .get_one::<String>("ACTION")
                .expect("required")
                .as_str()
            {
                "down" => brightness_down(&mut *port),
                "up" => brightness_up(&mut *port),
                "min" => brightness_min(&mut *port),
                "max" => brightness_max(&mut *port),
                _ => unreachable!(),
            };
        }
        Some(("volume", sub_matches)) => {
            match sub_matches
                .get_one::<String>("ACTION")
                .expect("required")
                .as_str()
            {
                "down" => volume_down(&mut *port),
                "up" => volume_up(&mut *port),
                _ => unreachable!(),
            };
        }
        Some(("display", sub_matches)) => {
            match sub_matches
                .get_one::<String>("ACTION")
                .expect("required")
                .as_str()
            {
                "toggle" => display_toggle(&mut *port),
                _ => unreachable!(),
            };
        }
        Some(("mute", sub_matches)) => {
            match sub_matches
                .get_one::<String>("ACTION")
                .expect("required")
                .as_str()
            {
                "toggle" => mute_toggle(&mut *port),
                _ => unreachable!(),
            };
        }
        Some(("status", _sub_matches)) => {
            print_status(&mut *port);
        }
        _ => unreachable!(),
    }
}
