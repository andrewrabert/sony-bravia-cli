use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "sony-bravia")]
#[command(about = "Sony Bravia TV RS232 control")]
pub struct Cli {
    /// Serial device path
    #[arg(long, required = true)]
    pub dev: String,

    // === Mode Control ===
    /// Power control: on, off
    #[arg(long, value_parser = ["on", "off"])]
    pub power: Option<String>,

    /// Query power state
    #[arg(long)]
    pub power_query: bool,

    /// Input selection: hdmi1-5, component1-3, video1-3, pc1, shared1
    #[arg(long, value_parser = parse_input)]
    pub input: Option<String>,

    /// Toggle input
    #[arg(long)]
    pub input_toggle: bool,

    /// Query input state
    #[arg(long)]
    pub input_query: bool,

    /// Volume control: up, down
    #[arg(long, value_parser = ["up", "down"])]
    pub volume: Option<String>,

    /// Set volume (0-100)
    #[arg(long, value_parser = clap::value_parser!(u8).range(0..=100))]
    pub volume_set: Option<u8>,

    /// Query volume level
    #[arg(long)]
    pub volume_query: bool,

    /// Mute control: toggle, on, off
    #[arg(long, value_parser = ["toggle", "on", "off"])]
    pub mute: Option<String>,

    /// Set mute: on, off
    #[arg(long, value_parser = ["on", "off"])]
    pub mute_set: Option<String>,

    /// Query mute state
    #[arg(long)]
    pub mute_query: bool,

    /// Sleep timer in minutes (0-255)
    #[arg(long, value_parser = clap::value_parser!(u8))]
    pub sleep: Option<u8>,

    /// Toggle sleep/off-timer
    #[arg(long)]
    pub sleep_toggle: bool,

    /// Toggle display
    #[arg(long)]
    pub display: bool,

    /// Picture off control: toggle, on, off
    #[arg(long, value_parser = ["toggle", "on", "off"])]
    pub picture: Option<String>,

    // === Picture ===
    /// Picture mode: vivid, standard, cinema, custom, game, graphics
    #[arg(long, value_parser = ["vivid", "standard", "cinema", "custom", "game", "graphics"])]
    pub picture_mode: Option<String>,

    /// Toggle picture mode
    #[arg(long)]
    pub picture_mode_toggle: bool,

    /// Brightness control: up, down
    #[arg(long, value_parser = ["up", "down"])]
    pub brightness: Option<String>,

    /// Set brightness (0-50)
    #[arg(long, value_parser = clap::value_parser!(u8).range(0..=50))]
    pub brightness_set: Option<u8>,

    /// Contrast control: up, down
    #[arg(long, value_parser = ["up", "down"])]
    pub contrast: Option<String>,

    /// Set contrast (0-50)
    #[arg(long, value_parser = clap::value_parser!(u8).range(0..=50))]
    pub contrast_set: Option<u8>,

    /// Color control: up, down
    #[arg(long, value_parser = ["up", "down"])]
    pub color: Option<String>,

    /// Set color (0-50)
    #[arg(long, value_parser = clap::value_parser!(u8).range(0..=50))]
    pub color_set: Option<u8>,

    /// Sharpness control: up, down
    #[arg(long, value_parser = ["up", "down"])]
    pub sharpness: Option<String>,

    /// Set sharpness (0-50)
    #[arg(long, value_parser = clap::value_parser!(u8).range(0..=50))]
    pub sharpness_set: Option<u8>,

    /// Cine motion: off, auto
    #[arg(long, value_parser = ["off", "auto"])]
    pub cine_motion: Option<String>,

    // === Screen ===
    /// Wide mode: toggle, wide_zoom, full, zoom, normal, pc_normal, pc_full1, pc_full2
    #[arg(long, value_parser = ["toggle", "wide_zoom", "full", "zoom", "normal", "pc_normal", "pc_full1", "pc_full2"])]
    pub wide: Option<String>,

    /// Auto wide: toggle, on, off
    #[arg(long, value_parser = ["toggle", "on", "off"])]
    pub auto_wide: Option<String>,

    /// 4:3 mode: toggle, off, wide_zoom, normal
    #[arg(long, value_parser = ["toggle", "off", "wide_zoom", "normal"])]
    pub four_three_mode: Option<String>,

    // === Sound ===
    /// Sound mode: toggle, standard, cinema, sports, music, game
    #[arg(long, value_parser = ["toggle", "standard", "cinema", "sports", "music", "game"])]
    pub sound_mode: Option<String>,

    /// Speaker control: toggle, on, off
    #[arg(long, value_parser = ["toggle", "on", "off"])]
    pub speaker: Option<String>,

    // === SIRCS ===
    /// SIRCS remote button emulation
    #[arg(long, value_parser = parse_sircs_button)]
    pub sircs: Option<String>,

    // === Language ===
    /// Language code (3 letters, e.g., eng, jpn, ger)
    #[arg(long)]
    pub language: Option<String>,

    // === BZ35F Series ===
    /// Standby control: enable, disable (BZ35F series)
    #[arg(long, value_parser = ["enable", "disable"])]
    pub standby: Option<String>,

    // === Advanced Picture ===
    /// Hue red control: up, down
    #[arg(long, value_parser = ["up", "down"])]
    pub hue_red: Option<String>,

    /// Set hue red (0-100)
    #[arg(long, value_parser = clap::value_parser!(u8).range(0..=100))]
    pub hue_red_set: Option<u8>,

    /// Hue green control: up, down
    #[arg(long, value_parser = ["up", "down"])]
    pub hue_green: Option<String>,

    /// Set hue green (0-100)
    #[arg(long, value_parser = clap::value_parser!(u8).range(0..=100))]
    pub hue_green_set: Option<u8>,

    /// H shift control: up, down
    #[arg(long, value_parser = ["up", "down"])]
    pub h_shift: Option<String>,

    /// Set H shift (0-134)
    #[arg(long, value_parser = clap::value_parser!(u8).range(0..=134))]
    pub h_shift_set: Option<u8>,

    /// V size control: up, down
    #[arg(long, value_parser = ["up", "down"])]
    pub v_size: Option<String>,

    /// Set V size (0-99)
    #[arg(long, value_parser = clap::value_parser!(u8).range(0..=99))]
    pub v_size_set: Option<u8>,

    /// V shift control: up, down
    #[arg(long, value_parser = ["up", "down"])]
    pub v_shift: Option<String>,

    /// Set V shift (0-99)
    #[arg(long, value_parser = clap::value_parser!(u8).range(0..=99))]
    pub v_shift_set: Option<u8>,

    // === Signage/Info ===
    /// Query product info (prints all 3 product info responses)
    #[arg(long, value_parser = ["query"])]
    pub product_info: Option<String>,

    /// Query device ID
    #[arg(long, value_parser = ["query"])]
    pub device_id: Option<String>,

    // === Status ===
    /// Show TV status
    #[arg(long)]
    pub status: bool,

    // === HTTP Server ===
    /// Start HTTP server
    #[arg(long)]
    pub http_server: bool,

    /// HTTP server port
    #[arg(long, default_value = "8000")]
    pub http_port: u16,

    /// HTTP server host
    #[arg(long, default_value = "127.0.0.1")]
    pub http_host: String,
}

fn parse_input(s: &str) -> Result<String, String> {
    let valid_inputs = [
        "hdmi1",
        "hdmi2",
        "hdmi3",
        "hdmi4",
        "hdmi5",
        "component1",
        "component2",
        "component3",
        "video1",
        "video2",
        "video3",
        "pc1",
        "shared1",
    ];
    if valid_inputs.contains(&s) {
        Ok(s.to_string())
    } else {
        Err(format!(
            "Invalid input: {}. Valid inputs: {}",
            s,
            valid_inputs.join(", ")
        ))
    }
}

fn parse_sircs_button(s: &str) -> Result<String, String> {
    let valid_buttons = [
        "input",
        "power",
        "wide_mode",
        "dot",
        "display",
        "return",
        "options",
        "home",
        "cursor_up",
        "cursor_down",
        "cursor_left",
        "cursor_right",
        "select",
        "num1",
        "num2",
        "num3",
        "num4",
        "num5",
        "num6",
        "num7",
        "num8",
        "num9",
        "num0",
        "closed_caption",
        "volume_up",
        "volume_down",
        "muting",
        "channel_up",
        "channel_down",
        "jump",
    ];
    if valid_buttons.contains(&s) {
        Ok(s.to_string())
    } else {
        Err(format!(
            "Invalid SIRCS button: {}. Valid buttons: {}",
            s,
            valid_buttons.join(", ")
        ))
    }
}
