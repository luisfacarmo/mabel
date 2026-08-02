# Visual Reference Notes — Soundcore App

## Screenshots Analyzed (from user + manual)

### Home / Device List Screen
- White background, very clean
- Header: "soundcore" logo centered, bell icon left, headphone icon right
- Section: "My Devices" label
- Device cards: large white rounded cards with shadow
  - Left: product image (photo, not icon)
  - Right: device name (bold, 16px), connection status below (green dot + "Connected" or gray "Not Connected")
- Bottom: "+ Add Devices" button (text only, blue/teal)
- Cards have subtle shadow, no visible border

### Device Detail Screen (connected)
- Top: device image (large, centered, with gradient/glow background)
- Below image: device name centered
- Battery indicators: 3 small circles with percentage (Left ear, Right ear, Case)
- Sections below as list items:
  - "Usage Tips" with dismiss X
  - "Intelligent Noise Cancellation" with toggle (teal when on)
  - Mode selector: segmented control (Noise Cancellation | Normal) - 2 options as rounded pills
  - "Wind Noise Reduction" with toggle
  - "Sound Effects" with chevron > and subtitle "Custom"
  - "Gaming Mode" with toggle

### Custom EQ Screen
- Header: "Custom EQ" title, back arrow, share and save icons
- Tab bar: "ANC Form" | "ANC Form+Open-Ear Form" (selected in blue)
- EQ visualization:
  - Grid with dots (circles) at intersection points
  - X-axis: frequencies (200, 400, 800, 1.6k, 3.2k, 6.4k, 12.8kHz)
  - Y-axis: implied dB levels
  - Dots are open circles, some filled when adjusted
  - Two rows: one for each form
- Bottom: "Custom" button (teal, rounded), gear icon

## Key Visual Patterns

| Element | Style |
|---------|-------|
| Background | Pure white (#ffffff) or very light gray (#f8f9fa) |
| Cards | White, rounded (16px), subtle shadow, no border or very light border |
| Accent color | Teal/Cyan (#00BCD4 to #4DD0E1) |
| Toggles | Teal when on, gray when off, iOS-style rounded |
| Typography | SF Pro / system font, weights 400/500/600 |
| Icons | Thin line icons, not filled |
| Spacing | Generous whitespace, 16-24px gaps |
| Status indicator | Small green dot + text |
| Chevrons | Light gray ">" for navigation items |
| Sections | No visible dividers, just spacing + label |
| Mode selectors | Segmented rounded pills or icon cards |

## Differences from Our Current Dark Mockup

The real Soundcore app is LIGHT themed. Options for Mabel:
1. **Light theme only** — match the app exactly
2. **Dark theme only** — premium desktop feel (original plan)
3. **Both** — light default, dark optional (more work)
4. **Hybrid** — light content area, dark sidebar (compromise)
