use std::sync::{Arc, Mutex};
use std::time::Duration;

use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::io::{stdin, stdout};
use tokio_util::codec::{FramedRead, FramedWrite, LinesCodec};

use crate::{
    brightness_down, brightness_max, brightness_min, brightness_up, display_toggle, input_select,
    is_powered_on, mute_toggle, picture_off, picture_on, picture_toggle, power_off, power_on,
    power_toggle, volume_down, volume_up, INPUT_TYPE_HDMI,
};

pub type SharedPort = Arc<Mutex<Box<dyn serialport::SerialPort + Send>>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id: Option<Value>,
    pub method: String,
    pub params: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcError {
    code: i32,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct InitializeParams {
    #[serde(rename = "protocolVersion")]
    protocol_version: String,
    capabilities: ClientCapabilities,
    #[serde(rename = "clientInfo")]
    client_info: ClientInfo,
}

#[derive(Debug, Serialize, Deserialize)]
struct ClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    experimental: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sampling: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ClientInfo {
    name: String,
    version: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct InitializeResult {
    #[serde(rename = "protocolVersion")]
    protocol_version: String,
    capabilities: ServerCapabilities,
    #[serde(rename = "serverInfo")]
    server_info: ServerInfo,
}

#[derive(Debug, Serialize, Deserialize)]
struct ServerCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<ToolsCapability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logging: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prompts: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resources: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ToolsCapability {
    #[serde(rename = "listChanged")]
    list_changed: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ServerInfo {
    name: String,
    version: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Tool {
    name: String,
    description: String,
    #[serde(rename = "inputSchema")]
    input_schema: Value,
}

#[derive(Debug, Serialize, Deserialize)]
struct CallToolParams {
    name: String,
    arguments: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CallToolResult {
    content: Vec<ToolContent>,
    #[serde(rename = "isError", skip_serializing_if = "Option::is_none")]
    is_error: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
enum ToolContent {
    #[serde(rename = "text")]
    Text { text: String },
}

pub struct SonyBraviaServer {
    port: SharedPort,
}

impl SonyBraviaServer {
    pub fn new(device_path: String) -> Result<Self, Box<dyn std::error::Error>> {
        let serial_port = serialport::new(&device_path, 9600)
            .timeout(Duration::from_millis(500))
            .open()
            .map_err(|e| format!("Failed to open port: {}", e))?;

        let shared_port: SharedPort = Arc::new(Mutex::new(serial_port));

        Ok(Self {
            port: shared_port,
        })
    }

    fn execute_power_action(&self, action: &str) -> Result<String, String> {
        let mut port_guard = self.port.lock().unwrap();
        match action {
            "on" => {
                power_on(&mut **port_guard);
                Ok("Power on".to_string())
            }
            "off" => {
                power_off(&mut **port_guard);
                Ok("Power off".to_string())
            }
            "toggle" => {
                power_toggle(&mut **port_guard);
                Ok("Power toggle".to_string())
            }
            _ => Err("Invalid power action".to_string()),
        }
    }

    fn execute_picture_action(&self, action: &str) -> Result<String, String> {
        let mut port_guard = self.port.lock().unwrap();
        match action {
            "on" => {
                picture_on(&mut **port_guard);
                Ok("Picture on".to_string())
            }
            "off" => {
                picture_off(&mut **port_guard);
                Ok("Picture off".to_string())
            }
            "toggle" => {
                picture_toggle(&mut **port_guard);
                Ok("Picture toggle".to_string())
            }
            _ => Err("Invalid picture action".to_string()),
        }
    }

    fn execute_volume_action(&self, action: &str) -> Result<String, String> {
        let mut port_guard = self.port.lock().unwrap();
        match action {
            "up" => {
                volume_up(&mut **port_guard);
                Ok("Volume up".to_string())
            }
            "down" => {
                volume_down(&mut **port_guard);
                Ok("Volume down".to_string())
            }
            _ => Err("Invalid volume action".to_string()),
        }
    }

    fn execute_brightness_action(&self, action: &str) -> Result<String, String> {
        let mut port_guard = self.port.lock().unwrap();
        match action {
            "up" => {
                brightness_up(&mut **port_guard);
                Ok("Brightness up".to_string())
            }
            "down" => {
                brightness_down(&mut **port_guard);
                Ok("Brightness down".to_string())
            }
            "min" => {
                brightness_min(&mut **port_guard);
                Ok("Brightness min".to_string())
            }
            "max" => {
                brightness_max(&mut **port_guard);
                Ok("Brightness max".to_string())
            }
            _ => Err("Invalid brightness action".to_string()),
        }
    }

    fn execute_display_toggle(&self) -> Result<String, String> {
        let mut port_guard = self.port.lock().unwrap();
        display_toggle(&mut **port_guard);
        Ok("Display toggle".to_string())
    }

    fn execute_mute_toggle(&self) -> Result<String, String> {
        let mut port_guard = self.port.lock().unwrap();
        mute_toggle(&mut **port_guard);
        Ok("Mute toggle".to_string())
    }

    fn execute_input_hdmi(&self, port_num: u8) -> Result<String, String> {
        let mut port_guard = self.port.lock().unwrap();
        input_select(&mut **port_guard, INPUT_TYPE_HDMI, port_num);
        Ok(format!("Input HDMI {}", port_num))
    }

    fn get_status(&self) -> Result<String, String> {
        let mut port_guard = self.port.lock().unwrap();
        let powered_on = is_powered_on(&mut **port_guard);
        Ok(if powered_on { "on".to_string() } else { "off".to_string() })
    }

    fn get_tools(&self) -> Vec<Tool> {
        vec![
            Tool {
                name: "power".to_string(),
                description: "Control TV power state".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "action": {
                            "type": "string",
                            "enum": ["on", "off", "toggle"],
                            "description": "Power action to perform"
                        }
                    },
                    "required": ["action"]
                }),
            },
            Tool {
                name: "picture".to_string(),
                description: "Control TV picture state".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "action": {
                            "type": "string",
                            "enum": ["on", "off", "toggle"],
                            "description": "Picture action to perform"
                        }
                    },
                    "required": ["action"]
                }),
            },
            Tool {
                name: "volume".to_string(),
                description: "Control TV volume".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "action": {
                            "type": "string",
                            "enum": ["up", "down"],
                            "description": "Volume action to perform"
                        }
                    },
                    "required": ["action"]
                }),
            },
            Tool {
                name: "brightness".to_string(),
                description: "Control TV brightness".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "action": {
                            "type": "string",
                            "enum": ["up", "down", "min", "max"],
                            "description": "Brightness action to perform"
                        }
                    },
                    "required": ["action"]
                }),
            },
            Tool {
                name: "display_toggle".to_string(),
                description: "Toggle TV display".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {}
                }),
            },
            Tool {
                name: "mute_toggle".to_string(),
                description: "Toggle TV mute".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {}
                }),
            },
            Tool {
                name: "input_hdmi".to_string(),
                description: "Select HDMI input".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "port": {
                            "type": "number",
                            "description": "HDMI port number",
                            "minimum": 1,
                            "maximum": 4
                        }
                    },
                    "required": ["port"]
                }),
            },
            Tool {
                name: "status".to_string(),
                description: "Get TV status".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {}
                }),
            },
        ]
    }

    pub async fn handle_request(&self, request: JsonRpcRequest) -> JsonRpcResponse {
        match request.method.as_str() {
            "initialize" => {
                let result = InitializeResult {
                    protocol_version: "2024-11-05".to_string(),
                    capabilities: ServerCapabilities {
                        tools: Some(ToolsCapability {
                            list_changed: Some(false),
                        }),
                        logging: None,
                        prompts: None,
                        resources: None,
                    },
                    server_info: ServerInfo {
                        name: "sony-bravia-mcp".to_string(),
                        version: "0.4.1".to_string(),
                    },
                };
                JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: request.id,
                    result: Some(serde_json::to_value(result).unwrap()),
                    error: None,
                }
            }
            "tools/list" => {
                let tools = self.get_tools();
                JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: request.id,
                    result: Some(json!({ "tools": tools })),
                    error: None,
                }
            }
            "tools/call" => {
                if let Some(params) = request.params {
                    if let Ok(call_params) = serde_json::from_value::<CallToolParams>(params) {
                        let result = self.handle_tool_call(call_params).await;
                        JsonRpcResponse {
                            jsonrpc: "2.0".to_string(),
                            id: request.id,
                            result: Some(serde_json::to_value(result).unwrap()),
                            error: None,
                        }
                    } else {
                        JsonRpcResponse {
                            jsonrpc: "2.0".to_string(),
                            id: request.id,
                            result: None,
                            error: Some(JsonRpcError {
                                code: -32602,
                                message: "Invalid params".to_string(),
                                data: None,
                            }),
                        }
                    }
                } else {
                    JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: request.id,
                        result: None,
                        error: Some(JsonRpcError {
                            code: -32602,
                            message: "Missing params".to_string(),
                            data: None,
                        }),
                    }
                }
            }
            _ => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: None,
                error: Some(JsonRpcError {
                    code: -32601,
                    message: "Method not found".to_string(),
                    data: None,
                }),
            },
        }
    }

    async fn handle_tool_call(&self, params: CallToolParams) -> CallToolResult {
        let result = match params.name.as_str() {
            "power" => {
                let action = params.arguments
                    .as_ref()
                    .and_then(|v| v.get("action"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                self.execute_power_action(action)
            }
            "picture" => {
                let action = params.arguments
                    .as_ref()
                    .and_then(|v| v.get("action"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                self.execute_picture_action(action)
            }
            "volume" => {
                let action = params.arguments
                    .as_ref()
                    .and_then(|v| v.get("action"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                self.execute_volume_action(action)
            }
            "brightness" => {
                let action = params.arguments
                    .as_ref()
                    .and_then(|v| v.get("action"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                self.execute_brightness_action(action)
            }
            "display_toggle" => self.execute_display_toggle(),
            "mute_toggle" => self.execute_mute_toggle(),
            "input_hdmi" => {
                let port = params.arguments
                    .as_ref()
                    .and_then(|v| v.get("port"))
                    .and_then(|v| v.as_u64())
                    .unwrap_or(1) as u8;
                self.execute_input_hdmi(port)
            }
            "status" => self.get_status(),
            _ => Err(format!("Unknown tool: {}", params.name)),
        };

        match result {
            Ok(message) => CallToolResult {
                content: vec![ToolContent::Text { text: message }],
                is_error: None,
            },
            Err(error) => CallToolResult {
                content: vec![ToolContent::Text { text: error }],
                is_error: Some(true),
            },
        }
    }
}

pub async fn start_mcp_server(device_path: String) -> Result<(), Box<dyn std::error::Error>> {
    let server = SonyBraviaServer::new(device_path)?;
    
    let stdin = stdin();
    let stdout = stdout();
    
    let mut reader = FramedRead::new(stdin, LinesCodec::new());
    let mut writer = FramedWrite::new(stdout, LinesCodec::new());
    
    while let Some(line_result) = reader.next().await {
        match line_result {
            Ok(line) => {
                if let Ok(request) = serde_json::from_str::<JsonRpcRequest>(&line) {
                    let response = server.handle_request(request).await;
                    let response_json = serde_json::to_string(&response)?;
                    if let Err(e) = writer.send(response_json).await {
                        eprintln!("Error sending response: {}", e);
                        break;
                    }
                } else {
                    eprintln!("Failed to parse JSON-RPC request: {}", line);
                }
            }
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                break;
            }
        }
    }
    
    Ok(())
}

