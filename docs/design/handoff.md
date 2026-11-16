# Design handoff: theme foundation

Design canvas: <https://claude.ai/code/artifact/1dc20798-49ef-46ec-b5c8-9d2bea121653> ("Girovolante — identidade visual e telas v1", in pt-BR). It has one design system board and eight screens: six for the PC program (1280×800 window) and two for the phone app (844×390, landscape).

This handoff covers the visual foundation only. No product screen is implemented yet (phase 0).

- The product name in the UI is **Gamepad Volante**, not the canvas's working name "Girovolante" (see `GLOSSARY.md`).
- The UI is in pt-BR only. Android strings live in `res/values/strings.xml`; desktop strings will be constants in one module when the first screen is built.

## Where things live

| What | Where |
| --- | --- |
| Tokens, source of truth (ADR 0003) | `docs/design/tokens.json` |
| egui tokens, fonts, components | `desktop/gw-theme/src/` (`tokens.rs`, `fonts.rs`, `lib.rs`) |
| egui catalogue | `cargo run -p gw-theme --example showcase` (CI uploads `showcase.exe` as the `theme-showcase-windows` artifact) |
| Compose tokens, theme, components | `mobile/app/src/main/java/.../ui/theme/` (`Tokens.kt`, `Theme.kt`, `Components.kt`) |
| Compose catalogue | `Showcase.kt`, shown by `MainActivity` |
| Font files (one copy for both sides) | `mobile/app/src/main/res/font/` |
| Drift, font and contrast tests | `desktop/gw-theme/tests/tokens.rs`, `mobile/app/src/test/.../TokensTest.kt` |

To change a token: edit `tokens.json`, then `tokens.rs` and `Tokens.kt`. The tests fail until all three agree.

## Design rules kept in code

- **One accent** (lavender `#8B9CFF`), for actions only. Green, amber and red mean status. On Android every Material colour role that would default to a second hue is mapped to a neutral.
- **Status never relies on colour alone.** `badge` / `StatusBadge` always take a text and always draw the dot; `alert` always draws an icon.
- **Contrast 4.5:1.** A Rust test computes the WCAG ratio of every text colour on the surfaces it is drawn on, including badge and alert tints. All pairs pass.
- **Touch targets:** 40 px on the PC (egui `interact_size`), 56 dp on the phone (`GwButton` minimum height).

## Tokens

The JSON key `color.text_2` is `TEXT_2` in Rust (`gw_theme::tokens`) and `Tokens.COLOR_TEXT_2` in Kotlin. Other groups keep their prefix on both sides (`RADIUS_CARD`, `SIZE_BUTTON`, `SPACE_MD`).

### Colours

The first eleven are named on the design system board. The rest are used by the screens without being named there; they are named here by role.

| Token | Hex | egui (`apply`) | Compose (`GwTheme`) |
| --- | --- | --- | --- |
| `bg` | `#0E1116` | `panel_fill`, `extreme_bg_color`, text field fill | `background`, `surface`, `surfaceContainerLowest` |
| `surface` | `#161A22` | `card()` fill, `faint_bg_color` | `surfaceContainer`, `surfaceContainerLow`, `surfaceTint` |
| `surface_2` | `#1D222C` | secondary button, `window_fill`, `code_bg_color` | `surfaceVariant`, `surfaceContainerHigh(est)` |
| `border` | `#343B49` | widget and window stroke, toggle-off fill | `outline` |
| `text` | `#E8EBF1` | widget `fg_stroke` | `onBackground`, `onSurface`, `on*Container` |
| `text_2` | `#9AA3B5` | help text, "off" badge tint | `onSurfaceVariant` |
| `accent` | `#8B9CFF` | primary button, focus border and ring | `primary` |
| `ok` | `#3DD68C` | `Tone::Ok` | `Tone.Ok` |
| `warn` | `#F2B640` | `Tone::Warn`, `warn_fg_color` | `Tone.Warn` |
| `err` | `#FF6B6B` | danger button, error border, `Tone::Err` tint | `error`, `Tone.Err` tint |
| `neutral` | `#AEB6C6` | `Tone::Off` text | `secondary`, `tertiary`, `Tone.Off` text |
| `title_bar` | `#0B0D12` | token only | token only |
| `sidebar` | `#12151C` | token only | token only |
| `divider` | `#222834` | token only | token only |
| `card_border` | `#2A303C` | `card()` stroke, non-interactive stroke | `outlineVariant` |
| `selected` | `#2A3142` | hovered/active widget fill, `selection` | `primaryContainer`, `secondaryContainer`, `tertiaryContainer` |
| `text_label` | `#C9CFDB` | field label, ghost button text | ghost button text |
| `accent_text` | `#A9B5FF` | `hyperlink_color`, `Tone::Info` text | `Tone.Info` text |
| `accent_text_hover` | `#C9D0FF` | token only | token only |
| `err_text` | `#FF7B7B` | `error_fg_color`, `Tone::Err` text and icon | `Tone.Err` text |
| `err_help` | `#FF8A8A` | field error message | token only |
| `on_accent` | `#0E1116` | text on primary and danger buttons | `onPrimary`, `onSecondary`, `onTertiary`, `onError` |
| `radio_off` | `#5A6274` | token only | token only |
| `dark_screen` | `#000000` | token only | `GwColors.darkScreen` |
| `dark_screen_text` | `#8A93A6` | token only | `GwColors.darkScreenText` |

"Token only" means the constant exists but no component uses it yet; the screen that needs it will.

Tints (`alpha.*`): badge background 0.14 (info 0.16), alert background 0.08, alert border 0.45, focus ring 0.25, selected option 0.07, disabled 0.45 (egui `disabled_alpha`, Compose disabled button colours).

### Typography

Fonts: Space Grotesk (titles), Inter (interface), JetBrains Mono (numbers). egui has no font weights, so each face is its own family, named `"Inter-600"`.

| Token | Face · size | egui | Compose |
| --- | --- | --- | --- |
| `text.brand` | Space Grotesk 700 · 18 | `TEXT_BRAND` | `GwType.brand` |
| `text.screen_title` | Space Grotesk 600 · 28 | `TextStyle::Heading` | `headlineMedium`; `titleLarge` at 26 (`text_size.mobile_title`) |
| `text.card_title` | Space Grotesk 600 · 18 | `TEXT_CARD_TITLE` | `titleMedium` |
| `text.body` | Inter 400 · 14 | `TextStyle::Body` | `bodyLarge`, `bodyMedium` |
| `text.label` | Inter 600 · 13 | `TEXT_LABEL_TEXT` (field label) | `labelMedium` |
| `text.help` | Inter 400 · 12 | `TextStyle::Small` | `bodySmall` |
| `text.error_help` | Inter 500 · 12 | field error message | token only |
| `text.alert_title` | Inter 600 · 14 | alert title | token only |
| `text.caption` | Inter 600 · 12, uppercase, 0.08em | `TEXT_CAPTION` (tracking and uppercase left to the caller) | `labelSmall` (tracking applied) |
| `text.button` | Inter 600 · 14 (13 small, 15 large) | `TextStyle::Button` | `labelLarge` at 16 (`text_size.mobile_button`) |
| `text.badge` | Inter 600 · 12 | badge | `GwType.badge` at 13 (`text_size.mobile_badge`) |
| `text.mono` | JetBrains Mono 400 · 12 | `TextStyle::Monospace` | `GwType.mono` |
| `text.number` | JetBrains Mono 600 · 22 | `TEXT_NUMBER` | `GwType.number` |
| `text.number_large` | JetBrains Mono 600 · 32 | `TEXT_NUMBER_LARGE` | `GwType.numberLarge` |

### Radii, sizes, spacing

| Token | Value | egui | Compose |
| --- | --- | --- | --- |
| `radius.segment` | 6 | token only | `shapes.extraSmall` |
| `radius.control` | 8 | widget radius, button, field | `shapes.small` |
| `radius.alert` | 10 | alert, `menu_corner_radius` | token only |
| `radius.card` | 12 | `card()` | `shapes.medium` |
| `radius.modal` | 14 | `window_corner_radius` | `shapes.extraLarge` |
| `radius.mobile_button` | 12 | n/a | `GwButton` |
| `radius.mobile_tile` | 14 | n/a | `shapes.large` |
| `size.touch_target` / `size.button` / `size.field` | 40 | `interact_size`, button and field height | n/a |
| `size.button_small` / `size.button_large` | 32 / 48 | `ButtonSize::Small` / `Large` | n/a |
| `size.mobile_touch_target` | 56 | n/a | `GwButton` minimum height |
| `size.badge` / `size.badge_dot` | 24 / 6 | badge | n/a |
| `size.mobile_badge` / `size.mobile_badge_dot` | 26 / 7 | n/a | `StatusBadge` |
| `size.alert_icon` | 20 | alert icon | n/a |
| `size.slider_track` | 6 | `slider_rail_height` | token only |
| `size.focus_ring` | 3 | field focus ring | token only |
| `space.xs` … `space.xxl` | 4, 6, 8, 12, 16, 24 | `item_spacing` = 8 | layout gaps |
| `space.card_padding` | 20 | `card()` | token only |
| `space.*_padding_x` | button 16 / 12 / 20, field 12, badge 10, alert 16×14; mobile button 24, mobile badge 12 | component padding | component padding |

Toggle (40×22) and slider thumb (18) sizes are tokens only: those components are not built yet.

## Components

| Component | egui (`gw_theme`) | Compose |
| --- | --- | --- |
| Button | `button(ui, label, ButtonKind::{Primary, Secondary, Ghost, Danger}, ButtonSize::{Small, Medium, Large})` | `GwButton(text, onClick, kind = ButtonKind.{Primary, Secondary, Ghost})` |
| Status badge | `badge(ui, Tone::{Ok, Warn, Err, Info, Off}, label)` | `StatusBadge(tone, text)` |
| Field | `field(ui, label, &mut value, Note::{None, Help, Error})` | not built: no phone screen has a text field |
| Alert | `alert(ui, tone, title, body)` | not built: no phone screen uses the alert box |
| Card | `card()` (an `egui::Frame`) | Material `Card` picks up `surfaceContainer*` and `shapes.medium` |

## Screens → requirements → components

| Screen (board) | Requirements | Components it needs |
| --- | --- | --- |
| 01 · PC · Status | R9 (status window, tray icon), R8 (missing driver notice), R6 (Recentralizar in the top bar), R3 (incl. Nova 2 Lite disconnected) and R7 (device states), R10 (active profile) | card, badge, button, alert; to build: sidebar nav, top-bar pill, profile selector, key-value row, tray menu |
| 02 · PC · Pareamento | R2 (QR, token, network card choice) | card, badge, button, field, alert; to build: QR view, radio option list |
| 03 · PC · Perfis | R10, R9 (profile editor entry) | button, badge, alert; to build: table, icon button, executable chip, confirm dialog, toast |
| 04 · PC · Editor de perfil — Direção | R5, R6 (recentre button choice), R10 (executables) | field, button, badge, alert; to build: tabs, radio mode cards, slider with value box, select, steering preview bar |
| 05 · PC · Editor de perfil — Combos e botões da tela | R11, R12 | button, badge, field, alert; to build: table, key cap, Xbox button pill, segmented control, phone layout preview |
| 06 · PC · Teste ao vivo | R9 (live test), R7, R14 (test vibration) | card, badge, button, alert, number text; to build: steering bar, trigger bars, stick view, button pills, latency chart |
| 07 · Celular · Pareamento | R2 (incl. camera denied), R1 (phone without rotation sensor) | `GwButton`, `StatusBadge`; to build: camera viewfinder |
| 08 · Celular · Direção | R13 (incl. "Tela escura" button), R12, R6, R7, R1 (screen stays on) | `GwButton`, `StatusBadge`; to build: on-screen game button tile, angle indicator, output bar, dark-screen mode |

## Gaps

### Requirement without a screen or state

None. The five gaps found first were added to the canvas (#6):

- **R3**: "Nova 2 Lite desconectado" error state on 01 · PC · Status.
- **R7**: no setting. The Nova 2 Lite always gives a short vibration on connection loss, as R7 says (the toggle added for #6 was removed in #7).
- **R13**: labelled "Tela escura" button in the top bar of 08 · Celular · Direção (double tap in the centre still leaves).
- **R2 / R13**: "Câmera bloqueada" error state on 07 · Celular · Pareamento.
- **R1**: "Celular incompatível" error state on 07 · Celular · Pareamento, shown on app start.

### Screen or state without a requirement

None. Each item found first was decided against the PRD (#7):

| Item | Screen | Decision |
| --- | --- | --- |
| Automatic reconnection while the session token is valid | 02 · PC · Pareamento | Kept; now a check under R2 (backs the open-to-drive metric). |
| Tray menu: status, open window, Recentralizar, quit and show the controller | 01 · PC · Status | Kept; now a check under R9. The "Perfil ›" submenu was removed. |
| Editable UDP port, "Abrir firewall" | 02 · PC · Pareamento | Removed. The port is shown read-only; the firewall note stays (PRD risk). |
| "Iniciar com o Windows" toggle | design system board | Removed. |
| "Exportar registro", fixed "manual" profile | 06 · PC · Teste ao vivo | Removed. The test uses the active profile. |
| Undo after deleting a profile, open the profile folder or TOML in an editor | 03 · PC · Perfis | Removed. The confirm dialog stays; an invalid TOML keeps "Recarregar". |
| "Copiar de outro perfil" | 05 · PC · Editor — Combos | Removed. |
| Vibration-on-loss toggle | 06 · PC · Teste ao vivo, design system board | Removed: R7 says the Nova 2 Lite always vibrates. |

### Conflict with a decision

- **Layer button (#8): the design follows plan A, pending the Q2 spike.** 05 · PC · Editor — Combos and 06 · PC · Teste ao vivo now use L4/R4 as the layer: in the GameSir app both back buttons send a held macro LB + RB + R3, and the program decodes that signature. The layer field is read-only and shows whether the signature is detected. Because LB, RB and R3 are part of the signature, they cannot be the second button of a combo (validation shown on board 05); the gear combos moved from LB/RB to X/A.
  - If the Q2 spike (`spikes/q1-xinput-probe`) shows the macro does not stay held, switch to plan B instead: View (Back) as the layer, in the PRD (R11, Q2) and on boards 05 and 06.

### Inside the design

- **Sizes (#9): resolved, the board wins.** The screens now use the design system board's values, which `tokens.json` already held: screen title 28, card title 18, caption 12, help 12, field height 40. `tokens.json` is unchanged.
- **Phone touch target (#9): resolved.** The top-bar buttons of "Celular · Direção" (options and "Tela escura") are now 56 px (`size.mobile_touch_target`); the top bar grew to 72 px to hold them.
- **Button states (#10): defined on the design system board** ("Estados dos botões"). Not in the code yet; see the table below.
- **Shadows (#10): defined on the design system board** ("Sombras e tema"). Not in the code yet; see the table below.
- **Light theme (#10): none, by intent.** The product is dark only in v1: it is played at night, the window sits next to the game, and the phone saves battery with the dark screen (R13).
- **Alert title (#10): resolved, the canvas changed.** Alert titles are Inter 600 on every board, which matches `text.alert_title` and the code. Inter 700 is not embedded.

New values from #10, to add to `tokens.json`, `tokens.rs` and `Tokens.kt` together (the drift tests fail until all three agree):

| Proposed token | Value | Use |
| --- | --- | --- |
| `color.accent_hover` | `#A9B5FF` (same as `accent_text`) | primary button, hover |
| `color.accent_pressed` | `#7584E8` | primary button, pressed |
| `color.err_hover` | `#FF7B7B` (same as `err_text`) | danger button, hover |
| `color.err_pressed` | `#E85A5A` | danger button, pressed |
| (existing) `selected` / `border` | `#2A3142` / `#343B49` | secondary button, hover / pressed |
| (existing) `surface_2` / `selected` | `#1D222C` / `#2A3142` | ghost button, hover / pressed (text turns `text`) |
| `size.focus_outline`, `space.focus_offset` | 2, 2 | keyboard focus: solid `accent` outline, 2 px from the button; desktop only |
| `shadow.menu` | 0 16 40, black 0.50 | menu, tray menu, toast |
| `shadow.modal` | 0 24 60, black 0.60 | confirm dialog |
| `shadow.window` | 0 30 80, black 0.50 | window mockup only (the OS draws the real one) |

The phone has no hover: pressed applies to touch. `on_accent` text stays readable on `accent_pressed` and `err_pressed` (above 4.5:1).

### Deviations in this implementation

- One 10 px label on the combos screen uses JetBrains Mono 500; only 400 and 600 are embedded.
- Shadows and button states: egui keeps its defaults until the tokens above are added.

## Left for when the screens are built

- Components: slider, segmented control, tabs, radio option, table, chip, key cap, toast, modal, sidebar, top bar, tray menu, and the phone's game button tile.
- Icons: the canvas uses inline stroke SVGs. Only the four alert icons exist (drawn with the egui painter).
- Desktop strings module, and landscape lock plus keep-screen-on for the phone.
- Compose field and alert, if a phone screen needs them.

## Fonts and licences

All three families are under the SIL Open Font License 1.1. The licence texts are in `docs/design/fonts/`.

| Family | Weights embedded | Copyright | Licence file |
| --- | --- | --- | --- |
| Inter | 400, 500, 600 | The Inter Project Authors (<https://github.com/rsms/inter>) | `fonts/OFL-Inter.txt` |
| Space Grotesk | 600, 700 | The Space Grotesk Project Authors (<https://github.com/floriankarsten/space-grotesk>) | `fonts/OFL-SpaceGrotesk.txt` |
| JetBrains Mono | 400, 600 | The JetBrains Mono Project Authors (<https://github.com/JetBrains/JetBrainsMono>) | `fonts/OFL-JetBrainsMono.txt` |

The files are the static TTF instances served by Google Fonts, the same ones the canvas loads.

## Library versions

| Library | Version | Notes |
| --- | --- | --- |
| `egui` | 0.37.0 | theme uses `Visuals`, `Style` and `FontDefinitions`; the new class-based style engine is not used |
| `eframe` | 0.37.0 | dev-dependency of `gw-theme`, for the showcase only (`glow` renderer) |
| `serde_json` | 1.0.154 | dev-dependency, for the drift test |
| Compose BOM | 2026.09.00 | unchanged; `material3` comes from the BOM |
