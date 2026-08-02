# Reference Materials — Mabel

## Source

- **Manual PDF:** `soundcore-space-one-pro-manual.pdf` — Official Soundcore Space One Pro user manual
- **Screenshots:** `screenshots/` — Extracted page renders and individual images from the manual
- **App screenshot:** User-provided screenshot of Soundcore App (Controls screen with AeroClip)

## Features Extracted from Manual (Space One Pro A3062)

### Physical Controls
- NC Button: Switch ANC / Transparency (Normal mode can be enabled via app)
- Power Button: On/Off (2s hold), Dual Connection pairing (double press)
- Volume Up/Down: Press once
- Multi-function Button: Play/Pause, Next/Prev track, Answer/Reject calls, Voice Assistant

### App-Controllable Features (what Mabel needs to implement)

| Feature | Description | Manual Page |
|---------|-------------|-------------|
| **Adaptive Noise Cancelling** | Real-time calculation based on environment + wearing | p6 |
| **Custom Noise Cancelling** | Manual level control | p6 |
| **Custom Transparency** | Manual transparency level | p6 |
| **Normal Mode** | Can be enabled via app (added to NC button cycle) | p5 |
| **ANC Mode Cycle** | Configure which modes the NC button cycles through | p5 |
| **Sidetone** | Hear your own voice during calls (on by default, toggle via app) | p7 |
| **Easy Chat** | Auto-lower volume when talking, configurable timeout | p7 |
| **LDAC** | Higher quality audio codec (Android 8.0+), disables Dolby | p8 |
| **Dolby Audio** | Enhanced audio processing (disabled when LDAC on) | p1 |
| **Dual Connections** | Connect to 2 devices simultaneously, manage device list | p4 |
| **Equalizer / HearID** | Custom EQ, sound profiles | p1 |
| **Firmware Update** | OTA firmware update via app | p10 |
| **Auto Power Off** | Configurable duration (30min increments) | protocol |
| **Limit High Volume** | Volume limiter with dB threshold | protocol |
| **Low Battery Prompt** | Toggle voice prompt on low battery | protocol |
| **Voice Prompt** | Toggle ambient sound mode voice prompts | protocol |
| **Button Configuration** | Configure double-press action | protocol |
| **Wind Noise Reduction** | Toggle for wind noise suppression | protocol |

### Hardware Specs
- Driver: 40mm
- Bluetooth: 5.3, range 15m
- Battery: 350mAh x2
- Playtime: 60H (ANC off), 40H (ANC on)
- Charging: 2h full, 5min = 8h playback
- Input: USB-C + 3.5mm AUX
- Impedance: 16 ohm
- Frequency: 20Hz - 40kHz

### Visual Reference (from manual screenshots)
- Page 4: App screenshots showing Dual Connections UI flow
- Page 6: App screenshots showing ANC and Transparency controls
- Page 7: App screenshots showing Sidetone and Easy Chat
- Page 8: App screenshots showing LDAC activation flow

## UI Design Direction
- Dark theme (matches official Soundcore app)
- Teal/cyan accent color (#4DD0E1 / turquoise — Soundcore brand)
- Card-based layout with rounded corners
- Clean typography, minimal icons
- Desktop-optimized (fullscreen, not mobile-sized)
