# Phase A — Tauri App Shell + React Frontend (Mock Data)

## Goal

Create the full premium UI for Mabel with mock/static data. No Bluetooth connection needed.
At the end of this phase, `npm run tauri dev` opens a window with all pages working visually.

## Design Principles Applied

- **Modular:** Each page is an independent component. Each hook has a single responsibility.
- **Abstract:** Mock data layer is identical in shape to what the real backend will provide — swapping from mocks to Tauri IPC changes 1 file, not 20.
- **No monoliths:** State is split into 3 hooks (connection, state, commands). No God Context.
- **No reinvention:** Radix UI Primitives for accessible interactive controls (slider, switch, tooltip, select). We style them, not rebuild them.

---

## Task A1: Scaffold Tauri v2 + React + Vite project

**Objective:** Bootstrap the app structure inside `apps/mabel-app/`.

**Steps:**
1. Run `npm create tauri-app@latest` with React + TypeScript + Vite template inside `apps/mabel-app/`
2. Verify the generated structure (`src/`, `src-tauri/`, `package.json`, `vite.config.ts`)
3. Run `npm install` to install deps
4. Run `npm run tauri dev` — confirm empty Tauri window opens
5. Clean up default template content (remove boilerplate React code)

**Test:** `npm run tauri dev` opens a blank window with no errors in console.

**Demo:** Empty Tauri window renders on screen.

---

## Task A2: Add Tailwind CSS + Radix UI + Framer Motion + design tokens

**Objective:** Setup styling infrastructure, UI primitives, and the Mabel design system.

**Steps:**
1. Install Tailwind CSS v4: `npm install tailwindcss @tailwindcss/vite`
2. Configure Vite plugin for Tailwind
3. Install Framer Motion: `npm install framer-motion`
4. Install Lucide React icons: `npm install lucide-react`
5. Install Radix UI Primitives (unstyled, accessible):
   ```
   npm install @radix-ui/react-slider @radix-ui/react-switch @radix-ui/react-tooltip @radix-ui/react-select
   ```
6. Create `src/styles/globals.css` with Tailwind directives + CSS variables:
   - Background: `#0f0f0f` (near-black)
   - Surface: `#1a1a1a` (cards)
   - Surface hover: `#242424`
   - Accent: `#4DD0E1` (Soundcore teal/cyan)
   - Accent hover: `#26C6DA`
   - Text primary: `#ffffff`
   - Text secondary: `#a0a0a0`
   - Border: `#2a2a2a`
   - Success: `#4CAF50`
   - Warning: `#FFC107`
   - Error: `#F44336`
7. Define font: Inter (or system-ui fallback)
8. Verify: app renders with dark background

**Test:** App renders with pure dark background, no white flash on load.

**Demo:** Dark empty window with correct background color.

---

## Task A3: App layout — sidebar navigation + content area

**Objective:** Build the main shell layout with a left sidebar and routed content area.

**Steps:**
1. Install React Router: `npm install react-router-dom`
2. Create `src/components/layout/Sidebar.tsx`:
   - Fixed left panel (~64px wide, icon-only)
   - Navigation items: Home, Sound (EQ), ANC, Controls, Settings
   - Each item: Lucide icon + Radix Tooltip on hover
   - Active item highlighted with accent color + sliding marker
   - Bottom: device connection indicator (dot green/red)
3. Create `src/components/layout/AppLayout.tsx`:
   - Sidebar on left
   - Content area fills remaining space with padding
   - Top bar with device name + battery indicator (small)
4. Create `src/pages/` folder with placeholder pages:
   - `HomePage.tsx`
   - `SoundPage.tsx`
   - `AncPage.tsx`
   - `ControlsPage.tsx`
   - `SettingsPage.tsx`
5. Wire up routes in `App.tsx`

**Test:** Click each nav item → corresponding page renders. Active state shows correctly.

**Demo:** Navigable app shell with sidebar, all 5 pages accessible (content still placeholder).

---

## Task A4: Mock data layer (3 modular hooks)

**Objective:** Create TypeScript types and mock data that mirror the A3062 state. Split into 3 independent hooks for separation of concerns.

**Steps:**
1. Create `src/lib/types.ts` with full device state interface:
   ```typescript
   interface DeviceState {
     battery: { level: number; maxLevel: number };
     firmware: string;
     serialNumber: string;
     soundModes: {
       ambientSoundMode: 'noiseCanceling' | 'transparency' | 'normal';
       noiseCancelingMode: 'adaptive' | 'custom';
       adaptiveNcLevel: number; // 1-5
       customNcLevel: number; // 1-5
       customTransparency: number; // 1-5
       windNoiseReduction: boolean;
     };
     ambientSoundModeCycle: {
       noiseCanceling: boolean;
       transparency: boolean;
       normal: boolean;
     };
     equalizer: {
       preset: string | null;
       bands: number[]; // 10 values, 0-180 range
     };
     buttonConfig: {
       doublePressAction: string | null;
     };
     toggles: {
       dolbyAudio: boolean;
       ldac: boolean;
       sideTone: boolean;
       voicePrompt: boolean;
       lowBatteryPrompt: boolean;
     };
     autoPowerOff: number; // minutes, 0 = disabled
     limitHighVolume: {
       enabled: boolean;
       dbLimit: number;
     };
     dualConnections: {
       enabled: boolean;
       devices: Array<{ name: string; connected: boolean }>;
     };
   }

   type ConnectionStatus = 'connected' | 'disconnected' | 'reconnecting';
   ```
2. Create `src/lib/mock-data.ts` with realistic default state (from OpenSCQ30 test vector)
3. Create 3 independent hooks:
   - `src/hooks/useConnection.ts` — `{ status, connect(), disconnect() }`
   - `src/hooks/useDeviceState.ts` — `{ state: DeviceState | null }` (readonly, from events)
   - `src/hooks/useCommands.ts` — `{ setSoundModes(), setEqualizer(), setToggle(), ... }`
4. Create `src/providers/DeviceProvider.tsx` — thin wrapper that composes the 3 hooks
5. Each hook is independently testable and replaceable

**Why 3 hooks:** When we swap mocks for real Tauri IPC (Phase E), we change the implementation inside each hook without touching any component. The interface stays identical.

**Test:** Each hook returns typed data. Components access only what they need.

**Demo:** Console.log shows full mock device state on app load.

---

## Task A5: Home page — device overview + battery

**Objective:** Build the main home page showing device image, name, connection status, and battery.

**Steps:**
1. Create `src/components/home/DeviceHero.tsx`:
   - Large centered headphone illustration/silhouette (SVG or PNG)
   - Device name below: "Space One Pro"
   - Connection status badge (green dot + "Connected")
2. Create `src/components/home/BatteryRing.tsx`:
   - Circular progress ring (SVG arc)
   - Battery percentage in center (large number)
   - Color transitions: green (>50%) → yellow (20-50%) → red (<20%)
   - Animated fill on mount (Framer Motion)
3. Create `src/components/home/QuickStatus.tsx`:
   - Row of status chips: current ANC mode, EQ preset, LDAC on/off
   - Tappable to navigate to respective page
4. Assemble in `src/pages/HomePage.tsx`

**Test:** Battery ring renders with correct percentage from mock data. Color matches level.

**Demo:** Home page with headphone image, animated battery ring, and quick status chips.

---

## Task A6: ANC page — mode selector + level sliders

**Objective:** Build the noise cancellation control page.

**Steps:**
1. Create `src/components/anc/ModeSelector.tsx`:
   - 3 large cards/pills: Noise Canceling, Transparency, Normal
   - Active mode highlighted with accent border + fill
   - Animated transition between modes (Framer Motion)
2. Create `src/components/anc/LevelSlider.tsx`:
   - Uses `@radix-ui/react-slider` with custom Tailwind styling
   - 5 discrete steps (1-5)
   - Shows current value label
3. Create `src/components/anc/NcModeToggle.tsx`:
   - Sub-selector within NC: "Adaptive" vs "Custom"
   - Shows appropriate slider based on selection
4. Create `src/components/anc/WindNoiseToggle.tsx`:
   - Uses `@radix-ui/react-switch` with custom styling
5. Create `src/components/anc/CycleConfig.tsx`:
   - 3 checkboxes: which modes NC button cycles through
6. Assemble in `src/pages/AncPage.tsx`:
   - Mode selector at top
   - Contextual controls below (change based on selected mode)
   - Wind noise + cycle config at bottom

**Test:** Switching modes updates UI. Sliders move. Toggles toggle. All from mock state.

**Demo:** Full ANC page with interactive mode switching, sliders, and toggles.

---

## Task A7: Controls page — button configuration

**Objective:** Build the button controls configuration page (matching the Soundcore app style).

**Steps:**
1. Create `src/components/controls/ActionCard.tsx`:
   - Card showing: gesture name, current action, chevron to expand
   - Example: "Double Tap" → "Bass Up"
2. Create `src/components/controls/ActionSelector.tsx`:
   - Uses `@radix-ui/react-select` for the action picker
   - Options: BassUp (only option for A3062 currently)
   - Checkmark on current selection
3. Assemble in `src/pages/ControlsPage.tsx`:
   - Header with "Controls" title
   - Section "Audio": Double Press action card
   - Section "Call (Not Adjustable)": info-only display

**Test:** Selecting an action updates the card display.

**Demo:** Controls page matching the Soundcore app style from user screenshot.

---

## Task A8: Settings page — toggles + config

**Objective:** Build the settings page with all remaining toggles and configuration options.

**Steps:**
1. Create `src/components/ui/ToggleRow.tsx`:
   - Reusable: label + description + `@radix-ui/react-switch`
   - Variants: with sub-label, with value display
2. Create `src/components/ui/SliderRow.tsx`:
   - Label + `@radix-ui/react-slider` + value display
3. Create `src/components/settings/DualConnectionsSection.tsx`:
   - Enable/disable toggle
   - Connected devices list (name + status)
4. Create `src/components/settings/AutoPowerOffSelector.tsx`:
   - Uses `@radix-ui/react-select`: Off, 30m, 60m, 90m, 120m
5. Create `src/components/settings/DeviceInfo.tsx`:
   - Firmware version display
   - Serial number display
6. Assemble in `src/pages/SettingsPage.tsx`:
   - Sections: Audio, Connections, Battery, Device
   - Audio: Dolby Audio, LDAC, Side Tone, Voice Prompt
   - Connections: Dual Connections
   - Battery: Auto Power Off, Limit High Volume, Low Battery Prompt
   - Device: Firmware, Serial Number

**Test:** All toggles toggle. Slider moves. Auto power off selector works.

**Demo:** Full settings page with all 18 features accessible.

---

## Task A9: Sound/EQ page — equalizer + presets

**Objective:** Build the equalizer page with 10-band visualizer and preset selection.

**Steps (v1 — visual only, no drag):**
1. Create `src/components/eq/PresetChips.tsx`:
   - Horizontal scrollable row of preset chips
   - Presets: Soundcore Signature, Bass Boost, Podcast, Classical, Bass Reducer, Treble Boost
   - Active preset highlighted
   - "Custom" chip appears when bands differ from any preset
2. Create `src/components/eq/EqualizerBands.tsx`:
   - 10 vertical bars (one per band)
   - Height reflects current value (0-180 mapped to visual range)
   - Frequency labels below: 100, 200, 400, 800, 1.6k, 3.2k, 6.4k, 12.8k Hz
   - Smooth curve line connecting bar tops (SVG path)
   - Animated height change on preset switch (Framer Motion)
   - **v1: No drag interaction** — selecting a preset updates bars
3. Create `src/components/eq/EqResetButton.tsx`:
   - "Reset to flat" button
4. Assemble in `src/pages/SoundPage.tsx`

**Future (A9b):** Add vertical drag on each bar for custom EQ editing.

**Test:** Selecting preset updates bars with animation. Reset flattens all.

**Demo:** Animated 10-band EQ visualizer with preset switching.

---

## Task A10: Window chrome + Tauri config

**Objective:** Configure the Tauri window for a premium desktop experience.

**Steps:**
1. Edit `tauri.conf.json`:
   - Remove default title bar (decorations: false)
   - Set minimum window size: 900x600
   - Set default size: 1100x750
   - Set background color to match app (#0f0f0f)
2. Create `src/components/layout/TitleBar.tsx`:
   - Custom draggable title bar (data-tauri-drag-region)
   - Window controls: minimize, maximize, close
   - App title: "Mabel"
3. Verify no white flash on startup (background color set in Tauri config)

**Test:** Window opens without title bar, custom controls work, drag region works.

**Demo:** Borderless window with custom controls.

---

## Task A11: Page transitions + micro-interactions

**Objective:** Add the final polish layer.

**Steps:**
1. Add page transition animations (Framer Motion `AnimatePresence` + `motion.div`)
2. Add hover/press micro-interactions on buttons and cards
3. Add loading/connecting skeleton state UI
4. Test all pages at different window sizes (responsive within 900px-1920px)

**Test:** Resize window — no layout breakage. Transitions are smooth.

**Demo:** Complete, polished frontend. All interactions work with mock data.

---

## Success Criteria (Phase A complete)

- [ ] `npm run tauri dev` opens the app window
- [ ] Dark theme renders without white flash
- [ ] All 5 pages are navigable via sidebar
- [ ] Home: battery ring animates, quick status shows
- [ ] ANC: mode switching works, Radix sliders functional
- [ ] Controls: action selector works
- [ ] Settings: all Radix toggles and configs functional
- [ ] EQ: bars animate on preset switch
- [ ] Mock data flows through 3 independent hooks
- [ ] Window is borderless with custom title bar
- [ ] Page transitions are smooth

## Execution Order (quick wins first)

```
A1 → A2 → A3 → A4 → A5 (milestone: "app looks real")
→ A7 (Controls, simplest page)
→ A8 (Settings, toggles are fast with Radix)
→ A6 (ANC, moderate complexity)
→ A9 (EQ, most complex visual)
→ A10 (Window chrome)
→ A11 (Polish)
```
