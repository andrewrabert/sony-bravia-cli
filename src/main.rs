use clap::Parser;

mod cli;
mod http;
mod protocol;
mod transport;

use cli::Cli;
use protocol::*;
use transport::{Transport, TransportError};

fn run_cli(cli: &Cli, transport: &mut Transport) -> Result<(), TransportError> {
    // Power
    if let Some(action) = &cli.power {
        let action = match action.as_str() {
            "on" => PowerAction::On,
            "off" => PowerAction::Off,
            _ => unreachable!(),
        };
        transport.execute::<Power>(&action)?;
        println!("Power: {}", cli.power.as_ref().unwrap());
    }

    if cli.power_query {
        let state = transport.query::<Power>()?;
        println!("Power: {:?}", state);
    }

    // Input
    if let Some(input) = &cli.input {
        let action = match input.as_str() {
            "toggle" => InputType::Toggle,
            "hdmi1" => InputType::Hdmi(1),
            "hdmi2" => InputType::Hdmi(2),
            "hdmi3" => InputType::Hdmi(3),
            "hdmi4" => InputType::Hdmi(4),
            "hdmi5" => InputType::Hdmi(5),
            "component1" => InputType::Component(1),
            "component2" => InputType::Component(2),
            "component3" => InputType::Component(3),
            "video1" => InputType::Video(1),
            "video2" => InputType::Video(2),
            "video3" => InputType::Video(3),
            "pc1" => InputType::Pc(1),
            "shared1" => InputType::SharedInput(1),
            _ => {
                eprintln!("Invalid input: {}", input);
                return Ok(());
            }
        };
        transport.execute::<InputSelect>(&action)?;
        println!("Input: {}", input);
    }

    if cli.input_toggle {
        transport.execute::<InputSelect>(&InputType::Toggle)?;
        println!("Input toggled");
    }

    if cli.input_query {
        let state = transport.query::<InputSelect>()?;
        println!(
            "Input: type={:#04x} num={}",
            state.input_type, state.input_num
        );
    }

    // Volume
    if let Some(action) = &cli.volume {
        let action = match action.as_str() {
            "up" => VolumeAction::Up,
            "down" => VolumeAction::Down,
            _ => unreachable!(),
        };
        transport.execute::<Volume>(&action)?;
        println!("Volume: {}", cli.volume.as_ref().unwrap());
    }

    if let Some(level) = cli.volume_set {
        let val = VolumeValue::new(level).map_err(|_| TransportError::QueryNotSupported)?;
        transport.execute::<Volume>(&VolumeAction::Set(val))?;
        println!("Volume set: {}", level);
    }

    if cli.volume_query {
        let level = transport.query::<Volume>()?;
        println!("Volume: {}", level);
    }

    // Mute
    if let Some(action) = &cli.mute {
        let action = match action.as_str() {
            "toggle" => MuteAction::Toggle,
            "on" => MuteAction::Mute,
            "off" => MuteAction::Unmute,
            _ => unreachable!(),
        };
        transport.execute::<Muting>(&action)?;
        println!("Mute: {}", cli.mute.as_ref().unwrap());
    }

    if let Some(action) = &cli.mute_set {
        let action = match action.as_str() {
            "on" => MuteAction::Mute,
            "off" => MuteAction::Unmute,
            _ => unreachable!(),
        };
        transport.execute::<Muting>(&action)?;
        println!("Mute: {}", cli.mute_set.as_ref().unwrap());
    }

    if cli.mute_query {
        let state = transport.query::<Muting>()?;
        println!("Mute: {:?}", state);
    }

    // Sleep
    if let Some(sleep) = cli.sleep {
        let val = SleepMinutes::new(sleep).map_err(|_| TransportError::QueryNotSupported)?;
        transport.execute::<OffTimer>(&SleepAction::Set(val))?;
        println!("Sleep: {}", sleep);
    }

    if cli.sleep_toggle {
        transport.execute::<OffTimer>(&SleepAction::Toggle)?;
        println!("Sleep toggled");
    }

    // Display
    if cli.display {
        transport.execute::<Display>(&())?;
        println!("Display toggled");
    }

    // Picture on/off
    if let Some(action) = &cli.picture {
        let action = match action.as_str() {
            "toggle" => PictureOffAction::Toggle,
            "on" => PictureOffAction::On,
            "off" => PictureOffAction::Off,
            _ => unreachable!(),
        };
        transport.execute::<PictureOff>(&action)?;
        println!("Picture: {}", cli.picture.as_ref().unwrap());
    }

    // Picture mode
    if let Some(mode) = &cli.picture_mode {
        let action = match mode.as_str() {
            "toggle" => PictureModeAction::Toggle,
            "vivid" => PictureModeAction::Vivid,
            "standard" => PictureModeAction::Standard,
            "cinema" => PictureModeAction::Cinema,
            "custom" => PictureModeAction::Custom,
            "game" => PictureModeAction::Game,
            "graphics" => PictureModeAction::Graphics,
            _ => {
                eprintln!("Invalid picture mode: {}", mode);
                return Ok(());
            }
        };
        transport.execute::<PictureMode>(&action)?;
        println!("Picture mode: {}", mode);
    }

    if cli.picture_mode_toggle {
        transport.execute::<PictureMode>(&PictureModeAction::Toggle)?;
        println!("Picture mode toggled");
    }

    // Brightness
    if let Some(action) = &cli.brightness {
        let action = match action.as_str() {
            "up" => BrightnessAction::Up,
            "down" => BrightnessAction::Down,
            _ => unreachable!(),
        };
        transport.execute::<Brightness>(&action)?;
        println!("Brightness: {}", cli.brightness.as_ref().unwrap());
    }

    if let Some(level) = cli.brightness_set {
        let val = BrightnessValue::new(level).map_err(|_| TransportError::QueryNotSupported)?;
        transport.execute::<Brightness>(&BrightnessAction::Set(val))?;
        println!("Brightness set: {}", level);
    }

    // Contrast
    if let Some(action) = &cli.contrast {
        let action = match action.as_str() {
            "up" => ContrastAction::Up,
            "down" => ContrastAction::Down,
            _ => unreachable!(),
        };
        transport.execute::<Contrast>(&action)?;
        println!("Contrast: {}", cli.contrast.as_ref().unwrap());
    }

    if let Some(level) = cli.contrast_set {
        let val = ContrastValue::new(level).map_err(|_| TransportError::QueryNotSupported)?;
        transport.execute::<Contrast>(&ContrastAction::Set(val))?;
        println!("Contrast set: {}", level);
    }

    // Color
    if let Some(action) = &cli.color {
        let action = match action.as_str() {
            "up" => ColorAction::Up,
            "down" => ColorAction::Down,
            _ => unreachable!(),
        };
        transport.execute::<Color>(&action)?;
        println!("Color: {}", cli.color.as_ref().unwrap());
    }

    if let Some(level) = cli.color_set {
        let val = ColorValue::new(level).map_err(|_| TransportError::QueryNotSupported)?;
        transport.execute::<Color>(&ColorAction::Set(val))?;
        println!("Color set: {}", level);
    }

    // Sharpness
    if let Some(action) = &cli.sharpness {
        let action = match action.as_str() {
            "up" => SharpnessAction::Up,
            "down" => SharpnessAction::Down,
            _ => unreachable!(),
        };
        transport.execute::<Sharpness>(&action)?;
        println!("Sharpness: {}", cli.sharpness.as_ref().unwrap());
    }

    if let Some(level) = cli.sharpness_set {
        let val = SharpnessValue::new(level).map_err(|_| TransportError::QueryNotSupported)?;
        transport.execute::<Sharpness>(&SharpnessAction::Set(val))?;
        println!("Sharpness set: {}", level);
    }

    // Cine motion
    if let Some(action) = &cli.cine_motion {
        let action = match action.as_str() {
            "off" => CineMotionAction::Off,
            "auto" => CineMotionAction::Auto,
            _ => unreachable!(),
        };
        transport.execute::<CineMotion>(&action)?;
        println!("Cine motion: {}", cli.cine_motion.as_ref().unwrap());
    }

    // Wide
    if let Some(mode) = &cli.wide {
        let action = match mode.as_str() {
            "toggle" => WideAction::Toggle,
            "wide_zoom" => WideAction::WideZoom,
            "full" => WideAction::Full,
            "zoom" => WideAction::Zoom,
            "normal" => WideAction::Normal,
            "pc_normal" => WideAction::PcNormal,
            "pc_full1" => WideAction::PcFull1,
            "pc_full2" => WideAction::PcFull2,
            _ => {
                eprintln!("Invalid wide mode: {}", mode);
                return Ok(());
            }
        };
        transport.execute::<Wide>(&action)?;
        println!("Wide: {}", mode);
    }

    // Auto wide
    if let Some(action) = &cli.auto_wide {
        let action = match action.as_str() {
            "toggle" => AutoWideAction::Toggle,
            "on" => AutoWideAction::On,
            "off" => AutoWideAction::Off,
            _ => unreachable!(),
        };
        transport.execute::<AutoWide>(&action)?;
        println!("Auto wide: {}", cli.auto_wide.as_ref().unwrap());
    }

    // 4:3 mode
    if let Some(mode) = &cli.four_three_mode {
        let action = match mode.as_str() {
            "toggle" => FourThreeModeAction::Toggle,
            "off" => FourThreeModeAction::Off,
            "wide_zoom" => FourThreeModeAction::WideZoom,
            "normal" => FourThreeModeAction::Normal,
            _ => {
                eprintln!("Invalid 4:3 mode: {}", mode);
                return Ok(());
            }
        };
        transport.execute::<FourThreeMode>(&action)?;
        println!("4:3 mode: {}", mode);
    }

    // Sound mode
    if let Some(mode) = &cli.sound_mode {
        let action = match mode.as_str() {
            "toggle" => SoundModeAction::Toggle,
            "standard" => SoundModeAction::Standard,
            "cinema" => SoundModeAction::Cinema,
            "sports" => SoundModeAction::Sports,
            "music" => SoundModeAction::Music,
            "game" => SoundModeAction::Game,
            _ => {
                eprintln!("Invalid sound mode: {}", mode);
                return Ok(());
            }
        };
        transport.execute::<SoundMode>(&action)?;
        println!("Sound mode: {}", mode);
    }

    // Speaker
    if let Some(action) = &cli.speaker {
        let action = match action.as_str() {
            "toggle" => SpeakerAction::Toggle,
            "on" => SpeakerAction::On,
            "off" => SpeakerAction::Off,
            _ => unreachable!(),
        };
        transport.execute::<Speaker>(&action)?;
        println!("Speaker: {}", cli.speaker.as_ref().unwrap());
    }

    // SIRCS
    if let Some(button) = &cli.sircs {
        let button = match button.to_lowercase().as_str() {
            "input" => SircsButton::Input,
            "power" => SircsButton::Power,
            "wide_mode" => SircsButton::WideMode,
            "dot" => SircsButton::Dot,
            "display" => SircsButton::Display,
            "return" => SircsButton::Return,
            "options" => SircsButton::Options,
            "home" => SircsButton::Home,
            "cursor_up" => SircsButton::CursorUp,
            "cursor_down" => SircsButton::CursorDown,
            "cursor_left" => SircsButton::CursorLeft,
            "cursor_right" => SircsButton::CursorRight,
            "select" => SircsButton::Select,
            "num1" => SircsButton::Num1,
            "num2" => SircsButton::Num2,
            "num3" => SircsButton::Num3,
            "num4" => SircsButton::Num4,
            "num5" => SircsButton::Num5,
            "num6" => SircsButton::Num6,
            "num7" => SircsButton::Num7,
            "num8" => SircsButton::Num8,
            "num9" => SircsButton::Num9,
            "num0" => SircsButton::Num0,
            "closed_caption" => SircsButton::ClosedCaption,
            "volume_up" => SircsButton::VolumeUp,
            "volume_down" => SircsButton::VolumeDown,
            "muting" => SircsButton::Muting,
            "channel_up" => SircsButton::ChannelUp,
            "channel_down" => SircsButton::ChannelDown,
            "jump" => SircsButton::Jump,
            _ => {
                eprintln!("Invalid SIRCS button: {}", button);
                return Ok(());
            }
        };
        transport.execute::<Sircs>(&button)?;
        println!("SIRCS: {}", cli.sircs.as_ref().unwrap());
    }

    // Language
    if let Some(code) = &cli.language {
        let lang = LanguageCode::new(code).map_err(|_| TransportError::QueryNotSupported)?;
        transport.execute::<Language>(&lang)?;
        println!("Language: {}", code);
    }

    // Standby (BZ35F)
    if let Some(action) = &cli.standby {
        let action = match action.as_str() {
            "enable" => StandbyAction::Enable,
            "disable" => StandbyAction::Disable,
            _ => unreachable!(),
        };
        transport.execute::<Standby>(&action)?;
        println!("Standby: {}", cli.standby.as_ref().unwrap());
    }

    // Hue red
    if let Some(action) = &cli.hue_red {
        let action = match action.as_str() {
            "up" => HueAction::Up(HueChannel::Red),
            "down" => HueAction::Down(HueChannel::Red),
            _ => unreachable!(),
        };
        transport.execute::<Hue>(&action)?;
        println!("Hue red: {}", cli.hue_red.as_ref().unwrap());
    }

    if let Some(level) = cli.hue_red_set {
        let val = HueValue::new(level).map_err(|_| TransportError::QueryNotSupported)?;
        transport.execute::<Hue>(&HueAction::Set(HueChannel::Red, val))?;
        println!("Hue red set: {}", level);
    }

    // Hue green
    if let Some(action) = &cli.hue_green {
        let action = match action.as_str() {
            "up" => HueAction::Up(HueChannel::Green),
            "down" => HueAction::Down(HueChannel::Green),
            _ => unreachable!(),
        };
        transport.execute::<Hue>(&action)?;
        println!("Hue green: {}", cli.hue_green.as_ref().unwrap());
    }

    if let Some(level) = cli.hue_green_set {
        let val = HueValue::new(level).map_err(|_| TransportError::QueryNotSupported)?;
        transport.execute::<Hue>(&HueAction::Set(HueChannel::Green, val))?;
        println!("Hue green set: {}", level);
    }

    // H shift
    if let Some(action) = &cli.h_shift {
        let action = match action.as_str() {
            "up" => HShiftAction::Up,
            "down" => HShiftAction::Down,
            _ => unreachable!(),
        };
        transport.execute::<HShift>(&action)?;
        println!("H shift: {}", cli.h_shift.as_ref().unwrap());
    }

    if let Some(value) = cli.h_shift_set {
        let action = if value <= 67 {
            HShiftAction::SetMinus(67 - value)
        } else {
            HShiftAction::SetPlus(value - 67)
        };
        transport.execute::<HShift>(&action)?;
        println!("H shift set: {}", value);
    }

    // V size
    if let Some(action) = &cli.v_size {
        let action = match action.as_str() {
            "up" => VSizeAction::Up,
            "down" => VSizeAction::Down,
            _ => unreachable!(),
        };
        transport.execute::<VSize>(&action)?;
        println!("V size: {}", cli.v_size.as_ref().unwrap());
    }

    if let Some(value) = cli.v_size_set {
        let action = if value <= 49 {
            VSizeAction::SetMinus(49 - value)
        } else {
            VSizeAction::SetPlus(value - 49)
        };
        transport.execute::<VSize>(&action)?;
        println!("V size set: {}", value);
    }

    // V shift
    if let Some(action) = &cli.v_shift {
        let action = match action.as_str() {
            "up" => VShiftAction::Up,
            "down" => VShiftAction::Down,
            _ => unreachable!(),
        };
        transport.execute::<VShift>(&action)?;
        println!("V shift: {}", cli.v_shift.as_ref().unwrap());
    }

    if let Some(value) = cli.v_shift_set {
        let action = if value <= 49 {
            VShiftAction::SetMinus(49 - value)
        } else {
            VShiftAction::SetPlus(value - 49)
        };
        transport.execute::<VShift>(&action)?;
        println!("V shift set: {}", value);
    }

    // Product info
    if cli.product_info.is_some() {
        println!("=== Product Info ===");
        match transport.query::<ProductInfo1>() {
            Ok(data) => println!("Product Info 1: {:02x?}", data),
            Err(e) => println!("Product Info 1: error ({:?})", e),
        }
        match transport.query::<ProductInfo2>() {
            Ok(data) => println!("Product Info 2: {:02x?}", data),
            Err(e) => println!("Product Info 2: error ({:?})", e),
        }
        match transport.query::<ProductInfo3>() {
            Ok(data) => println!("Product Info 3: {:02x?}", data),
            Err(e) => println!("Product Info 3: error ({:?})", e),
        }
    }

    // Device ID
    if cli.device_id.is_some() {
        match transport.query::<IdCommand>() {
            Ok(data) => println!("Device ID: {:02x?}", data),
            Err(e) => println!("Device ID: error ({:?})", e),
        }
    }

    // Status
    if cli.status {
        println!("=== Status ===");
        match transport.query::<Power>() {
            Ok(state) => println!("Power: {:?}", state),
            Err(e) => println!("Power: error ({:?})", e),
        }
        match transport.query::<Volume>() {
            Ok(level) => println!("Volume: {}", level),
            Err(e) => println!("Volume: error ({:?})", e),
        }
        match transport.query::<Muting>() {
            Ok(state) => println!("Mute: {:?}", state),
            Err(e) => println!("Mute: error ({:?})", e),
        }
        match transport.query::<InputSelect>() {
            Ok(state) => println!(
                "Input: type={:#04x} num={}",
                state.input_type, state.input_num
            ),
            Err(e) => println!("Input: error ({:?})", e),
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if cli.http_server {
        if let Err(e) =
            http::start_http_server(cli.dev.clone(), cli.http_host.clone(), cli.http_port).await
        {
            eprintln!("Server error: {}", e);
            std::process::exit(1);
        }
        return;
    }

    let mut transport = match Transport::new(&cli.dev) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Failed to open serial port: {}", e);
            std::process::exit(1);
        }
    };

    if let Err(e) = run_cli(&cli, &mut transport) {
        eprintln!("Command error: {}", e);
        std::process::exit(1);
    }
}
