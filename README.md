# sony-bravia-cli
CLI to control Sony Bravia TV's over RS-232

Can be used as a CLI or HTTP API (`--http-server`).

## Supported TV's
The following TV's have been confirmed working with sony-bravia-cli:
- XBR-55X950G

## Installation
- [Arch Linux AUR](https://aur.archlinux.org/packages/sony-bravia-cli)

## CLI Usage

```
sony-bravia-cli --dev /dev/ttyUSB0 [OPTIONS]
```

### Power & Input
| Flag | Description |
|------|-------------|
| `--power on\|off` | Power on/off |
| `--power-query` | Query power state |
| `--input hdmi1-5\|component1-3\|video1-3\|pc1\|shared1` | Select input |
| `--input-toggle` | Toggle input |
| `--input-query` | Query input state |
| `--standby enable\|disable` | Standby control (BZ35F series) |

### Volume & Mute
| Flag | Description |
|------|-------------|
| `--volume up\|down` | Volume up/down |
| `--volume-set 0-100` | Set volume |
| `--volume-query` | Query volume |
| `--mute toggle\|on\|off` | Mute control |
| `--mute-query` | Query mute state |

### Picture
| Flag | Description |
|------|-------------|
| `--picture toggle\|on\|off` | Picture on/off |
| `--picture-mode vivid\|standard\|cinema\|custom\|game\|graphics` | Picture mode |
| `--picture-mode-toggle` | Toggle picture mode |
| `--brightness up\|down` | Brightness control |
| `--brightness-set 0-50` | Set brightness |
| `--contrast up\|down` | Contrast control |
| `--contrast-set 0-50` | Set contrast |
| `--color up\|down` | Color control |
| `--color-set 0-50` | Set color |
| `--sharpness up\|down` | Sharpness control |
| `--sharpness-set 0-50` | Set sharpness |
| `--hue-red up\|down` | Hue red control |
| `--hue-red-set 0-100` | Set hue red |
| `--hue-green up\|down` | Hue green control |
| `--hue-green-set 0-100` | Set hue green |
| `--cine-motion off\|auto` | Cine motion |

### Screen
| Flag | Description |
|------|-------------|
| `--display` | Toggle display |
| `--wide toggle\|wide_zoom\|full\|zoom\|normal\|pc_normal\|pc_full1\|pc_full2` | Wide mode |
| `--auto-wide toggle\|on\|off` | Auto wide |
| `--four-three-mode toggle\|off\|wide_zoom\|normal` | 4:3 mode |
| `--h-shift up\|down` | H shift control |
| `--h-shift-set 0-134` | Set H shift |
| `--v-size up\|down` | V size control |
| `--v-size-set 0-99` | Set V size |
| `--v-shift up\|down` | V shift control |
| `--v-shift-set 0-99` | Set V shift |

### Sound
| Flag | Description |
|------|-------------|
| `--sound-mode toggle\|standard\|cinema\|sports\|music\|game` | Sound mode |
| `--speaker toggle\|on\|off` | Speaker control |

### Other
| Flag | Description |
|------|-------------|
| `--sleep 0-255` | Sleep timer (minutes) |
| `--sleep-toggle` | Toggle sleep timer |
| `--language eng\|jpn\|...` | Set language (3-letter code) |
| `--sircs <button>` | SIRCS remote emulation |
| `--product-info query` | Query product info |
| `--device-id query` | Query device ID |
| `--status` | Show TV status |

### SIRCS Buttons
`input`, `power`, `wide-mode`, `dot`, `display`, `return`, `options`, `home`, `up`, `down`, `left`, `right`, `select`, `1`-`0`, `cc`, `volume-up`, `volume-down`, `muting`, `channel-up`, `channel-down`, `jump`

## HTTP API

Start server:
```
sony-bravia-cli --dev /dev/ttyUSB0 --http-server --http-host 0.0.0.0 --http-port 8000
```

### Query Routes (GET)
| Route | Description |
|-------|-------------|
| `/status` | Power status |
| `/power` | Power state |
| `/volume` | Volume level |
| `/input` | Input state |
| `/mute` | Mute state |
| `/product-info` | Product info |
| `/device-id` | Device ID |

### Control Routes (POST)
| Route | Description |
|-------|-------------|
| `/power/{on\|off}` | Power control |
| `/standby/{enable\|disable}` | Standby control |
| `/input/{hdmi1\|component1\|...}` | Input selection |
| `/input/toggle` | Toggle input |
| `/volume/{up\|down}` | Volume control |
| `/volume/set/{0-100}` | Set volume |
| `/mute/toggle` | Toggle mute |
| `/mute/{on\|off}` | Mute control |
| `/sleep/{0-255}` | Sleep timer |
| `/sleep/toggle` | Toggle sleep |
| `/picture/mode/{vivid\|standard\|...}` | Picture mode |
| `/picture/mode/toggle` | Toggle picture mode |
| `/picture/{off\|on\|toggle}` | Picture on/off |
| `/picture/brightness/{up\|down}` | Brightness control |
| `/picture/brightness/set/{0-50}` | Set brightness |
| `/picture/contrast/{up\|down}` | Contrast control |
| `/picture/contrast/set/{0-50}` | Set contrast |
| `/picture/color/{up\|down}` | Color control |
| `/picture/color/set/{0-50}` | Set color |
| `/picture/sharpness/{up\|down}` | Sharpness control |
| `/picture/sharpness/set/{0-50}` | Set sharpness |
| `/picture/hue/red/{up\|down}` | Hue red control |
| `/picture/hue/red/set/{0-100}` | Set hue red |
| `/picture/hue/green/{up\|down}` | Hue green control |
| `/picture/hue/green/set/{0-100}` | Set hue green |
| `/picture/cine-motion/{off\|auto}` | Cine motion |
| `/display/toggle` | Toggle display |
| `/screen/wide/{toggle\|widezoom\|full\|...}` | Wide mode |
| `/screen/auto-wide/{toggle\|on\|off}` | Auto wide |
| `/screen/4-3-mode/{toggle\|off\|widezoom\|normal}` | 4:3 mode |
| `/screen/h-shift/{up\|down}` | H shift control |
| `/screen/h-shift/set/{0-134}` | Set H shift |
| `/screen/v-size/{up\|down}` | V size control |
| `/screen/v-size/set/{0-99}` | Set V size |
| `/screen/v-shift/{up\|down}` | V shift control |
| `/screen/v-shift/set/{0-99}` | Set V shift |
| `/sound/mode/{toggle\|standard\|...}` | Sound mode |
| `/sound/speaker/{toggle\|on\|off}` | Speaker control |
| `/language/{eng\|jpn\|...}` | Set language |
| `/sircs/{button}` | SIRCS remote emulation |
