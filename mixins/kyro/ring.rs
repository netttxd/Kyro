// The Right Shift menu: cards you swipe through, then each category's rows and settings.
// It takes over Options > Mods while open, as Mod Menu does.
// Themes color all of it through the palette below.

use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};
use std::time::Instant;

use super::{OptionsUi, Page};
use super::kyro_world::own_name;
use crate::hud::{HudAtlas, HudBatch, rgba};
use crate::text::Text;

const NAMES: [&str; 6] = ["HUD", "PVP", "VISUAL", "CLIENT", "CAPE", "THEMES"];
const HEADER: f32 = 24.0;
const ROW: f32 = 18.0;

/// Kyro's keys, for Options > Mods and the menu's Key rows: name, label, default key.
pub const KEYS: [[&str; 3]; 5] = [
    ["menu", "Open the Kyro menu", "key.keyboard.right.shift"],
    ["zoom", "Zoom (hold)", "key.keyboard.c"],
    ["freelook", "Freelook (hold)", "key.keyboard.left.alt"],
    ["lyric_earlier", "Lyrics earlier", "key.keyboard.left.bracket"],
    ["lyric_later", "Lyrics later", "key.keyboard.right.bracket"],
];

/// Mods by category: id, name, and whether it works in Rust yet.
const MODS: [&[(&str, &str, bool)]; 6] = [
    &[("hud_editor", "HUD editor", true), ("fps", "FPS", true), ("cps", "CPS", true), ("coordinates", "Coordinates", true), ("keystrokes", "Keystrokes", true), ("armor", "Armor", true), ("clock", "Clock", true), ("day", "Day", true), ("light", "Light", true), ("session", "Session", true), ("reach", "Reach", true), ("server", "Server", true), ("tps", "TPS", true), ("kill_feed", "Kill feed", true), ("totems", "Item counter", true), ("potions", "Potions", true), ("speed", "Speed", true), ("ping", "Ping", true), ("scoreboard", "Scoreboard", true), ("action_bar", "Action bar", true), ("inventory", "Inventory", true), ("lyrics", "Lyrics", true)],
    &[("#", "Hits", true), ("hit_markers", "Hit markers", true), ("hit_color", "Hit color", true), ("damage_numbers", "Damage numbers", true), ("combo", "Combo", true), ("cooldown_indicator", "Cooldown indicator", true), ("#", "Reach", true), ("reach_hitbox", "Hitboxes", true), ("#", "Warnings", true), ("low_health", "Low health glow", true), ("durability_warning", "Gear warning", true), ("shield_status", "Shield", true)],
    &[("#", "Crosshair", true), ("crosshair", "Crosshair", true), ("crosshair_editor", "Crosshair editor", true), ("#", "Camera", true), ("held_items", "Held items", true), ("fov_control", "FOV control", true), ("hurt_cam", "Hurt cam", true), ("motion_blur", "Motion blur", true), ("color_saturation", "Color saturation", true), ("#", "World", true), ("fullbright", "Fullbright", true), ("clear_fluids", "Clear fluids", true), ("world_look", "World look", true), ("low_obstruction", "Low obstruction", true), ("particles", "Hide particles", true), ("#", "Items", true), ("block_outline", "Block outline", true), ("item_highlighter", "Item highlighter", true), ("shulker_preview", "Shulker preview", true), ("glint_color", "Glint color", true), ("shiny_potions", "Shiny potions", true), ("food_saturation", "Food saturation", true), ("item_physics", "Item physics", true), ("tnt_timer", "TNT timer", true), ("#", "Players", true), ("skin_layers_3d", "3D skin layers", true), ("own_nametag", "Own nametag", true)],
    &[("#", "Controls", true), ("ring_menu", "Menu key", true), ("zoom", "Zoom", true), ("freelook", "Freelook", true), ("sprint", "Sprint", true), ("#", "Chat", true), ("better_chat", "Better chat", true), ("#", "Tab list", true), ("tab_ping", "Ping numbers", true)],
    &[("#", "Options", true), ("wavy_capes", "Wavy capes", true), ("tint_capes", "Cape color", true), ("#", "Animated", true), ("cape_black_hole", "Black hole", true), ("cape_hyperspace", "Hyperspace", true), ("cape_reactor", "Reactor", true), ("cape_hellfire", "Hellfire", true), ("cape_glitch", "Glitch", true), ("cape_thunderhead", "Thunderhead", true), ("cape_the_watcher", "The watcher", true), ("cape_kaleidoscope", "Kaleidoscope", true), ("cape_blood_code", "Blood code", true), ("cape_overdrive", "Overdrive", true), ("cape_supernova", "Supernova", true), ("cape_sky_dragon", "Sky dragon", true), ("cape_maelstrom", "Maelstrom", true), ("cape_northern_lights", "Northern lights", true), ("cape_soul_reaper", "Soul reaper", true), ("cape_magma_core", "Magma core", true), ("cape_hypnosis", "Hypnosis", true), ("cape_clockwork", "Clockwork", true), ("cape_swarm", "Swarm", true), ("cape_rift", "Rift", true), ("#", "Still", true), ("cape_crest", "Crest", true), ("cape_red_crane", "Red crane", true), ("cape_first_snow", "First snow", true), ("cape_kyro_sigil", "Kyro sigil", true), ("cape_kintsugi", "Kintsugi", true)],
    &[("#", "Themes", true), ("theme_classic", "Classic", true), ("theme_obsidian", "Obsidian", true), ("theme_crimson", "Crimson", true), ("theme_frost", "Frost", true), ("theme_sakura", "Sakura", true), ("theme_glass", "Glass", true), ("#", "Animated", true), ("theme_nebula", "Nebula", true), ("theme_ember", "Ember", true), ("theme_aurora", "Aurora", true), ("theme_petals", "Petals", true), ("theme_signal", "Signal", true), ("#", "Colors", true), ("theme_colors", "Custom colors", true)],
];

/// A mod's settings: key, label, min, max, default. 0..1 is a switch, key "color" is swatches.
fn settings_of(id: &str) -> &'static [(&'static str, &'static str, i32, i32, i32)] {
    match id {
        "zoom" => &[("level", "Starting zoom", 2, 16, 4), ("smooth", "Smooth", 0, 1, 1), ("scroll", "Scroll to zoom", 0, 1, 1), ("key_zoom", "Key", 0, 0, 0)],
        "fullbright" => &[("strength", "Strength", 10, 100, 100)],
        "scoreboard" | "action_bar" => &[("panel", "Background", 0, 1, 0)],
        "theme_glass" => &[("blur", "Blur", 0, 20, 7), ("dim", "Dim", 0, 80, 30)],
        "tint_capes" => &[("tint_color", "Color", 0, 0, 0)],
        "theme_colors" => &[("paper_color", "Background", 0, 0, -1), ("panel_color", "Panels", 0, 0, -1), ("line_color", "Lines", 0, 0, -1), ("text_color", "Text", 0, 0, -1), ("muted_color", "Muted text", 0, 0, -1), ("selected_color", "Selected row", 0, 0, -1), ("selected_text_color", "Selected text", 0, 0, -1), ("accent_color", "Accent", 0, 0, -1)],
        "clear_fluids" => &[("water", "Water", 0, 1, 1), ("lava", "Lava", 0, 1, 1), ("powder_snow", "Powder snow", 0, 1, 1), ("distance", "Distance", 20, 100, 100)],
        "fov_control" => &[("mode", "Mode", 0, 2, 1), ("transition", "Transition", 0, 2, 1)],
        "wavy_capes" => &[("wave", "Wave", 20, 200, 100)],
        "cooldown_indicator" => &[("bar", "Bar instead of ring", 0, 1, 0), ("color", "Color", 0, 7, 0), ("opponent", "Opponent swing", 0, 1, 1)],
        "world_look" => &[("time", "Time", 0, 6, 0), ("weather", "Weather", 0, 3, 1), ("fog", "Fog", 0, 2, 0)],
        "low_obstruction" => &[("pumpkin", "Hide pumpkin", 0, 1, 1), ("toasts", "Hide pop-ups", 0, 1, 0), ("fire", "Fire", 0, 2, 1), ("fire_opacity", "Fire opacity", 30, 100, 70)],
        "block_outline" => &[("color", "Color", 0, 7, 0), ("opacity", "Opacity", 10, 100, 80), ("thickness", "Thickness", 50, 400, 150)],
        "better_chat" => &[("timestamps", "Timestamps", 0, 1, 1), ("stack", "Stack repeats", 0, 1, 1), ("mentions", "Mark your name", 0, 1, 1), ("text_filter", "Hide words", 0, 0, 0), ("search", "Search box", 0, 1, 1), ("heads", "Player heads", 0, 1, 1), ("keep", "Keep chat", 0, 1, 1), ("copy", "Right-click copies", 0, 1, 1)],
        "item_highlighter" => &[("enchanted", "Enchanted", 0, 1, 1), ("custom", "One color", 0, 1, 0), ("color", "Color", 0, 7, 2)],
        "kill_feed" | "durability_warning" => &[("panel", "Background", 0, 1, 0)],
        "inventory" => &[("panel", "Background", 0, 1, 0)],
        "held_items" => &[("scale", "Scale", 30, 200, 100), ("x", "Left / right", -100, 100, 0), ("y", "Up / down", -100, 100, 0), ("z", "Forward / back", -100, 100, 0),
            ("pitch", "Pitch", -90, 90, 0), ("roll", "Roll", -90, 90, 0), ("swing", "Swing speed", 25, 300, 100), ("ignore_haste", "Ignore Haste", 0, 1, 0)],
        "hit_color" => &[("color", "Color", 0, 7, 6), ("note_launch", "Applies next launch", 0, 0, 0)],
        "motion_blur" => &[("strength", "Strength", 10, 90, 50), ("match_refresh", "Match refresh rate", 0, 1, 1)],
        "color_saturation" => &[("saturation", "Saturation", 0, 200, 125), ("vibrance", "Vibrance", 0, 100, 0), ("brightness", "Brightness", -50, 50, 0),
            ("contrast", "Contrast", 50, 150, 100), ("gamma", "Gamma", 50, 200, 100), ("hue", "Hue", -180, 180, 0)],
        "glint_color" => &[("color", "Color", 0, 7, 1), ("speed", "Speed", 0, 100, 50), ("strength", "Strength", 0, 100, 75), ("glint_style", "Style", 0, 2, 0), ("armor", "Own armor color", 0, 1, 0), ("armor_color", "Armor color", 0, 7, 5)],
        "reach_hitbox" => &[("items", "Dropped items", 0, 1, 0), ("living", "Living only", 0, 1, 1), ("color", "Color", 0, 7, 0), ("box_style", "Style", 0, 3, 0), ("players", "Players only", 0, 1, 1), ("reach", "Color in reach", 0, 1, 1), ("reach_color", "Reach color", 0, 7, 1), ("look", "Look arrow", 0, 1, 0), ("thickness", "Thickness", 50, 400, 100)],
        "skin_layers_3d" => &[("distance", "Distance", 8, 64, 24)],
        "hit_markers" => &[("crit", "Crit marker", 0, 1, 1), ("kill", "Kill marker", 0, 1, 1), ("color", "Color", 0, 7, 0), ("size", "Size", 2, 8, 4), ("gap", "Gap", 1, 8, 3), ("duration", "Duration", 150, 800, 350)],
        "low_health" => &[("below", "Below hearts", 0, 2, 1), ("power", "Strength", 0, 2, 1)],
        "freelook" => &[("solo", "Singleplayer only", 0, 1, 0), ("scroll", "Scroll to zoom", 0, 1, 1), ("key_freelook", "Key", 0, 0, 0), ("text_blocked", "Blocked servers", 0, 0, 0)],
        "sprint" => &[("always", "Always sprint", 0, 1, 0)],
        "particles" => &[("amount", "Amount", 0, 100, 100), ("explosions", "Explosions", 0, 1, 1), ("blocks", "Blocks", 0, 1, 1), ("potions", "Potions", 0, 1, 1)],
        "lyrics" => &[("offset", "Offset (ms)", -500, 500, 0), ("delay", "Lyric delay (ms)", -5000, 5000, 0), ("key_lyric_earlier", "Earlier key", 0, 0, 0), ("key_lyric_later", "Later key", 0, 0, 0), ("online", "Online lyrics", 0, 1, 1), ("word_for_word", "Word for word", 0, 1, 1), ("panel", "Background", 0, 1, 0)],
        "totems" => &[("count_totems", "Totems", 0, 1, 1), ("count_pearls", "Pearls", 0, 1, 0), ("count_gapples", "Enchanted apples", 0, 1, 0), ("count_golden_apples", "Golden apples", 0, 1, 0),
            ("count_arrows", "Arrows", 0, 1, 0), ("count_crystals", "Crystals", 0, 1, 0), ("count_wind_charges", "Wind charges", 0, 1, 0), ("count_xp_bottles", "XP bottles", 0, 1, 0),
            ("count_held", "Held item", 0, 1, 0), ("count_zero", "Show at zero", 0, 1, 1), ("text_color", "Value color", 0, 7, -1), ("panel", "Background", 0, 1, 0)],
        "armor" => &[("value", "Value", 0, 2, 0), ("layout", "Horizontal", 0, 1, 0), ("bar", "Durability bar", 0, 1, 1), ("text_color", "Value color", 0, 7, -1), ("panel", "Background", 0, 1, 0)],
        "potions" => &[("icons", "Icons", 0, 1, 0), ("label_color", "Label color", 0, 7, -1), ("text_color", "Value color", 0, 7, -1), ("panel", "Background", 0, 1, 0)],
        "ping" => &[("graph", "Graph", 0, 1, 0), ("label_color", "Label color", 0, 7, -1), ("text_color", "Value color", 0, 7, -1), ("panel", "Background", 0, 1, 0)],
        "combo" => &[("popup", "Popup", 0, 1, 0), ("from", "Popup from", 0, 2, 1), ("label_color", "Label color", 0, 7, -1), ("text_color", "Value color", 0, 7, -1), ("panel", "Background", 0, 1, 0)],
        "fps" | "cps" | "coordinates" | "speed" | "clock" | "day" | "light" | "session" | "reach" | "tps" =>
            &[("label_color", "Label color", 0, 7, -1), ("text_color", "Value color", 0, 7, -1), ("panel", "Background", 0, 1, 0)],
        "keystrokes" => &[("keys_move", "Movement keys", 0, 1, 1), ("keys_jump", "Jump", 0, 1, 1), ("keys_sneak", "Sneak", 0, 1, 1), ("keys_mouse", "Mouse", 0, 1, 1), ("keys_cps", "CPS on mouse", 0, 1, 1), ("label_color", "Text color", 0, 7, 0), ("held_color", "Pressed color", 0, 7, -1), ("panel", "Background", 0, 1, 0)],
        "ring_menu" => &[("key_menu", "Key", 0, 0, 0)],
        "food_saturation" => &[("preview", "Preview food", 0, 1, 1), ("health", "Preview health", 0, 1, 1), ("exhaustion", "Exhaustion bar", 0, 1, 1), ("tooltip", "Food tooltip", 0, 1, 1)],
        "server" => &[("hide", "Hide address", 0, 1, 0), ("label_color", "Label color", 0, 7, -1), ("text_color", "Value color", 0, 7, -1), ("panel", "Background", 0, 1, 0)],
        "shield_status" => &[("enabled_color", "Ready color", 0, 7, 4), ("disabled_color", "Disabled color", 0, 7, 1), ("fade", "Fade to ready", 0, 1, 1), ("sounds", "Shield sounds", 0, 1, 1), ("delay", "Show real block", 0, 1, 1), ("position", "Position", 0, 2, 0)],
        "hurt_cam" => &[("strength", "Strength", 0, 100, 30)],
        "crosshair" => &[("style", "Style", 0, 10, 0), ("length", "Length", 1, 10, 4), ("gap", "Gap", 0, 10, 2), ("thickness", "Thickness", 1, 5, 1),
            ("outline", "Outline", 0, 1, 1), ("color", "Color", 0, 7, 0), ("highlight", "Target highlight", 0, 1, 1)],
        _ => &[],
    }
}

/// A color setting's value as RGB: 0 to 7 is a preset, CUSTOM plus RGB is picked freely.
pub fn color_of(value: i32) -> u32 {
    if value >= CUSTOM { value as u32 & 0xffffff } else { COLORS[value.clamp(0, 7) as usize] }
}

const CUSTOM: i32 = 0x1000000;

/// The open color picker: setting, hue, saturation, brightness.
static PICKER: Mutex<Option<(&'static str, &'static str, f32, f32, f32)>> = Mutex::new(None);
const PICK_W: f32 = 132.0;
const PICK_H: f32 = 112.0;

fn hsv(h: f32, s: f32, v: f32) -> u32 {
    let sector = (h * 6.0).rem_euclid(6.0);
    let c = v * s;
    let x = c * (1.0 - (sector % 2.0 - 1.0).abs());
    let (r, g, b) = match sector as u32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    let m = v - c;
    let at = |t: f32| ((t + m) * 255.0).round() as u32;
    at(r) << 16 | at(g) << 8 | at(b)
}

fn open_picker(id: &'static str, key: &'static str) {
    if let Ok(mut p) = PICKER.lock() {
        // Starts on the setting's current color.
        let c = color_of(value(id, key));
        let [r, g, b] = [(c >> 16 & 255) as f32 / 255.0, (c >> 8 & 255) as f32 / 255.0, (c & 255) as f32 / 255.0];
        let (top, low) = (r.max(g).max(b), r.min(g).min(b));
        let d = top - low;
        let hue = if d == 0.0 {
            0.0
        } else if top == r {
            ((g - b) / d).rem_euclid(6.0) / 6.0
        } else if top == g {
            ((b - r) / d + 2.0) / 6.0
        } else {
            ((r - g) / d + 4.0) / 6.0
        };
        *p = Some((id, key, hue, if top > 0.0 { d / top } else { 0.0 }, top));
    }
}

/// Beside the panel it belongs to, on whichever side has room.
fn picker_rect(size: (f32, f32), id: &str) -> [f32; 4] {
    let [px, py, pw, _] = settings_rect(size.0, size.1, id);
    let x = if px + pw + 8.0 + PICK_W < size.0 { px + pw + 8.0 } else { (px - 8.0 - PICK_W).max(0.0) };
    [x, py.min(size.1 - PICK_H).max(0.0), PICK_W, PICK_H]
}

fn draw_picker(batch: &mut HudBatch, atlas: &HudAtlas) {
    let Some((id, _, h, s, v)) = PICKER.lock().ok().and_then(|p| *p) else {
        return;
    };
    let [x, y, w, ph] = picker_rect((batch.width, batch.height), id);
    frame(batch, atlas, x, y, w, ph);
    let ink = |c: u32| vertex_color(c, 255);
    for col in 0..30 {
        for row in 0..20 {
            let c = hsv(h, col as f32 / 29.0, 1.0 - row as f32 / 19.0);
            batch.fill(atlas, x + 6.0 + col as f32 * 4.0, y + 6.0 + row as f32 * 4.0, 4.0, 4.0, ink(c));
        }
    }
    for col in 0..120 {
        batch.fill(atlas, x + 6.0 + col as f32, y + 90.0, 1.0, 12.0, ink(hsv(col as f32 / 120.0, 1.0, 1.0)));
    }
    batch.fill(atlas, x + 6.0 + s * 118.0, y + 6.0 + (1.0 - v) * 78.0, 2.0, 2.0, line());
    batch.fill(atlas, x + 6.0 + h * 119.0, y + 88.0, 1.0, 16.0, rgba(255, 255, 255, 255));
}

/// A click or drag while the picker is open; outside it, a click closes it.
fn picker_input(cursor: (f32, f32), size: (f32, f32), dragging: bool) -> bool {
    let Ok(mut lock) = PICKER.lock() else {
        return false;
    };
    let Some((id, key, mut h, mut s, mut v)) = *lock else {
        return false;
    };
    let [x, y, w, ph] = picker_rect(size, id);
    if !inside(cursor, [x, y, w, ph]) {
        if !dragging {
            *lock = None;
        }
        return true;
    }
    if cursor.1 >= y + 88.0 {
        h = ((cursor.0 - x - 6.0) / 120.0).clamp(0.0, 1.0);
    } else {
        s = ((cursor.0 - x - 6.0) / 120.0).clamp(0.0, 1.0);
        v = 1.0 - ((cursor.1 - y - 6.0) / 80.0).clamp(0.0, 1.0);
    }
    *lock = Some((id, key, h, s, v));
    drop(lock);
    set_value(id, key, CUSTOM | hsv(h, s, v) as i32);
    true
}

const COLORS: [u32; 8] = [0xffffff, 0xff5555, 0xffaa00, 0xffff55, 0x55ff55, 0x55ffff, 0x5555ff, 0xff55ff];
const SETTINGS_W: f32 = 270.0;

#[derive(Default)]
struct State {
    list: bool,
    page: Option<(&'static str, &'static str)>,
    values: HashMap<String, HashMap<String, i32>>,
    selected: usize,
    by_key: bool,
    opened: Option<Instant>,
    query: String,
    on: HashMap<String, bool>,
    loaded: bool,
    saved: Vec<(String, String)>,
    scroll: f32,
}

static STATE: LazyLock<Mutex<State>> = LazyLock::new(Mutex::default);

fn with<T>(f: impl FnOnce(&mut State) -> T) -> T {
    f(&mut STATE.lock().unwrap_or_else(|e| e.into_inner()))
}

fn default_on(id: &str) -> bool {
    matches!(id, "theme_classic" | "scoreboard" | "action_bar" | "fps" | "cps" | "coordinates" | "keystrokes" | "zoom" | "freelook" | "potions" | "armor" | "crosshair" | "hit_markers" | "cooldown_indicator")
}

/// Whether a mod is switched on in the menu.
pub fn on(id: &str) -> bool {
    with(|s| s.on.get(id).copied().unwrap_or_else(|| default_on(id)))
}

/// A setting's value, or its default.
pub fn value(id: &str, key: &str) -> i32 {
    let default = settings_of(id).iter().find(|e| e.0 == key).map_or(0, |e| e.4);
    with(|s| s.values.get(id).and_then(|m| m.get(key)).copied().unwrap_or(default))
}

fn set_value(id: &str, key: &str, value: i32) {
    with(|s| {
        s.values.entry(id.to_owned()).or_default().insert(key.to_owned(), value);
        s.saved.push((format!("set_{id}_{key}"), value.to_string()));
    });
}

pub fn loaded() -> bool {
    with(|s| s.loaded)
}

/// Kyro's saved settings: one file per key, as the game's `native_setting` keeps them.
pub fn data_dir() -> Option<std::path::PathBuf> {
    std::env::current_dir().ok().map(|d| d.join("horizon-mods").join(".horizon-mod-data").join("kyro"))
}

/// Loads the saved settings from disk, for hooks that can run before the first frame does.
pub fn ensure_loaded() {
    if !loaded() {
        let dir = data_dir();
        load(|key| std::fs::read_to_string(dir.as_ref()?.join(key)).ok().map(|v| v.trim().to_owned()));
    }
}

/// A color's red, green and blue.
pub fn rgb(color: u32) -> [u32; 3] {
    [(color >> 16) & 255, (color >> 8) & 255, color & 255]
}

/// Reads the saved switches once.
pub fn load(get: impl Fn(&str) -> Option<String>) {
    with(|s| {
        for mods in MODS {
            for (id, _, _) in mods {
                if let Some(value) = get(&format!("on_{id}")) {
                    s.on.insert((*id).to_owned(), value == "true");
                }
                for (key, _, _, _, _) in settings_of(id) {
                    if let Some(value) = get(&format!("set_{id}_{key}")).and_then(|v| v.parse().ok()) {
                        s.values.entry((*id).to_owned()).or_default().insert((*key).to_owned(), value);
                    }
                }
            }
        }
        let extra = (0..15).map(|n| ("crosshair".to_owned(), format!("r{n}"))).chain(KEYS.iter().flat_map(|[k, _, _]| [("keys".to_owned(), format!("{k}_mods")), ("keys".to_owned(), format!("{k}_for"))])).chain(
            ELEMENTS.iter().flat_map(|e| SAVED_KEYS.map(|k| ("hud".to_owned(), format!("{}_{k}", e.0)))),
        );
        for (id, key) in extra {
            if let Some(value) = get(&format!("set_{id}_{key}")).and_then(|v| v.parse().ok()) {
                s.values.entry(id).or_default().insert(key, value);
            }
        }
        for (id, key) in TEXT_ROWS {
            if let (Some(v), Ok(mut texts)) = (get(&format!("set_{id}_{key}")), TEXTS.lock()) {
                texts.push((format!("{id}.{key}"), v.to_string()));
            }
        }
        s.loaded = true;
    });
}

pub fn take_saved() -> Vec<(String, String)> {
    with(|s| std::mem::take(&mut s.saved))
}

pub fn shows(options: &OptionsUi) -> bool {
    options.page() == Page::Mods && with(|s| s.by_key)
}

pub fn opened_by_key() {
    SEARCH_FOCUS.store(false, std::sync::atomic::Ordering::Relaxed);
    with(|s| {
        s.by_key = true;
        s.list = false;
        s.opened = Some(Instant::now());
    });
}

/// True while the menu is open or was until this frame.
pub fn by_key() -> bool {
    with(|s| s.by_key)
}

/// Right Shift closes from any page in one go.
pub fn reset_list() {
    with(|s| {
        s.list = false;
        s.page = None;
    });
}

pub fn closed() {
    if let Ok(mut held) = SLIDER.lock() {
        *held = None;
    }
    if let Ok(mut swipe) = SWIPE.lock() {
        *swipe = None;
    }
    take_listening();
    with(|s| {
        s.by_key = false;
        s.list = false;
        s.page = None;
    });
}

/// Esc on the mod list goes back to the ring; on the ring it closes the menu.
pub fn back() -> bool {
    if let Some(bind) = listening() {
        clear_bind(bind);
        return true;
    }
    if just_bound() {
        return true;
    }
    if TEXT_FOCUS.lock().is_ok_and(|mut f| f.take().is_some()) {
        return true;
    }
    if PICKER.lock().is_ok_and(|mut p| p.take().is_some()) {
        return true;
    }
    if ed(|e| e.popup.take()).is_some() {
        return true;
    }
    if SEARCH_FOCUS.swap(false, std::sync::atomic::Ordering::Relaxed) {
        with(|s| s.query.clear());
        return true;
    }
    if with(|s| s.page.is_none() && !std::mem::take(&mut s.query).is_empty()) {
        return true;
    }
    with(|s| {
        if s.page.take().is_some_and(|p| p.0.ends_with("_editor")) {
            s.opened = None;
            true
        } else if s.list {
            s.list = false;
            s.opened = None;
            true
        } else {
            false
        }
    })
}

/// A flat box, the shape every row, swatch and switch is drawn with.
fn notched(batch: &mut HudBatch, atlas: &HudAtlas, x: f32, y: f32, w: f32, h: f32, color: u32) {
    batch.fill(atlas, x, y, w, h, color);
}

fn frame(batch: &mut HudBatch, atlas: &HudAtlas, x: f32, y: f32, w: f32, h: f32) {
    batch.fill(atlas, x + 3.0, y + 3.0, w, h, line());
    batch.fill(atlas, x, y, w, h, panel());
}

fn text(batch: &mut HudBatch, atlas: &HudAtlas, value: &str, x: f32, y: f32, color: u32, shadow: bool) {
    batch.text_styled(atlas, &Text::literal(value), x, y, color, shadow);
}

fn centered(batch: &mut HudBatch, atlas: &HudAtlas, value: &str, cx: f32, y: f32, color: u32) {
    text(batch, atlas, value, cx - HudBatch::text_width(atlas, value) / 2.0, y, color, false);
}

fn age(s: &State) -> f32 {
    s.opened.map_or(1.0, |t| t.elapsed().as_secs_f32())
}

const CAPES: usize = 4;

/// What a list shows: its category's rows, or everything matching the search.
fn items(category: usize) -> Vec<(&'static str, &'static str, bool)> {
    let query = with(|s| s.query.to_lowercase());
    if query.is_empty() {
        return MODS[category].iter().filter(|m| m.0 != "hud_editor").copied().collect();
    }
    let hit = |id: &str, name: &str| name.to_lowercase().contains(&query) || settings_of(id).iter().any(|e| e.1.to_lowercase().contains(&query));
    MODS.iter().take(CAPES).flat_map(|c| c.iter()).filter(|m| m.0 != "#" && hit(m.0, m.1)).copied().collect()
}

fn settings_rect(width: f32, height: f32, id: &str) -> [f32; 4] {
    if let Some(index) = ed(|e| e.popup).filter(|i| ELEMENTS[*i].0 == id) {
        return popup_rect(index, (width, height));
    }
    let rows = settings_of(id).len() + usize::from(id != "ring_menu");
    let preview = if id.starts_with("cape_") { 148.0 } else { 0.0 };
    let h = (rows + app_rows(id)) as f32 * ROW + HEADER + 8.0 + preview;
    let [x, y, w, room] = sheet_area((width, height));
    [x, if h <= room { y } else { y + (room - h) / 2.0 }, w, h]
}

/// The dark box behind a row's key or text, so it reads as something to click.
fn field(batch: &mut HudBatch, atlas: &HudAtlas, right: f32, y: f32, shown: &str) {
    let w = HudBatch::text_width(atlas, shown) + 8.0;
    notched(batch, atlas, right - w + 4.0, y - 1.0, w, 13.0, line());
    notched(batch, atlas, right - w + 5.0, y, w - 2.0, 11.0, sunk());
}

/// How far a page shrinks to fit a short window, and the cursor as the unshrunk page sees it.
fn fit(h: f32, height: f32) -> f32 {
    ((height - 16.0) / h).min(1.0)
}

fn fitted(c: (f32, f32), [x, y, w, h]: [f32; 4], height: f32) -> (f32, f32) {
    let (k, cx, cy) = (fit(h, height), x + w / 2.0, y + h / 2.0);
    (cx + (c.0 - cx) / k, cy + (c.1 - cy) / k)
}

/// `lean`: 0 is off, 1 is on, anything between is the knob on its way.
fn switch_at(batch: &mut HudBatch, atlas: &HudAtlas, right: f32, y: f32, lean: f32) {
    let eased = lean * lean * (3.0 - 2.0 * lean);
    batch.fill(atlas, right - 23.0, y, 24.0, 11.0, line());
    batch.fill(atlas, right - 22.0, y + 1.0, 22.0, 9.0, sunk());
    batch.fill(atlas, right - 22.0, y + 1.0, 22.0 * eased, 9.0, sel());
    batch.fill(atlas, right - 22.0 + 12.0 * eased, y + 1.0, 10.0, 9.0, if eased > 0.5 { seltxt() } else { line() });
}

fn switch_of(batch: &mut HudBatch, atlas: &HudAtlas, right: f32, y: f32, id: &str) {
    let share = toggle_share(id);
    switch_at(batch, atlas, right, y, if on(id) { share } else { 1.0 - share });
}

const CELL: f32 = 8.0;

fn canvas_rect(width: f32, height: f32) -> [f32; 4] {
    let (w, h) = (15.0 * CELL + 16.0, HEADER + 15.0 * CELL + 14.0);
    [(width - w) / 2.0, (height - h) / 2.0, w, h]
}

/// One row of the drawn crosshair, a bit a cell.
pub fn crosshair_row(row: usize) -> i32 {
    value("crosshair", &format!("r{row}"))
}

fn draw_canvas(batch: &mut HudBatch, atlas: &HudAtlas, cursor: (f32, f32)) {
    let [x, y, w, h] = canvas_rect(batch.width, batch.height);
    frame(batch, atlas, x, y, w, h);
    batch.fill(atlas, x, y, w, HEADER - 4.0, sel());
    if cursor.0 >= x && cursor.0 < x + 26.0 && cursor.1 >= y && cursor.1 < y + HEADER - 4.0 {
        batch.fill(atlas, x, y, 22.0, HEADER - 4.0, ca(6, 50));
    }
    let uv = white_uv(batch, atlas);
    let (tip, mid) = ([x + 8.0, y + 10.0], y + 10.0);
    stroke(batch, uv, tip, [x + 13.0, mid - 5.0], 1.5, seltxt(), seltxt());
    stroke(batch, uv, tip, [x + 13.0, mid + 5.0], 1.5, seltxt(), seltxt());
    text(batch, atlas, "Draw crosshair", x + 26.0, y + 6.0, seltxt(), false);
    let (gx, gy) = (x + 8.0, y + HEADER + 4.0);
    for row in 0..15 {
        let bits = crosshair_row(row);
        for col in 0..15 {
            let (cx, cy) = (gx + col as f32 * CELL, gy + row as f32 * CELL);
            let hot = cursor.0 >= cx && cursor.0 < cx + CELL && cursor.1 >= cy && cursor.1 < cy + CELL;
            let fill = if bits >> col & 1 == 1 {
                accent()
            } else if hot {
                ca(3, 90)
            } else if row == 7 && col == 7 {
                line()
            } else {
                sunk()
            };
            batch.fill(atlas, cx, cy, CELL - 1.0, CELL - 1.0, fill);
        }
    }
}

fn click_canvas(cursor: (f32, f32), size: (f32, f32)) {
    let [x, y, _, _] = canvas_rect(size.0, size.1);
    if cursor.0 >= x && cursor.0 < x + 26.0 && cursor.1 >= y && cursor.1 < y + HEADER {
        back();
        return;
    }
    if let Some((col, row)) = canvas_cell(cursor, x, y) {
        // The first cell decides: the whole stroke paints, or the whole stroke erases.
        let paint = crosshair_row(row) >> col & 1 == 0;
        PAINT.store(paint, std::sync::atomic::Ordering::Relaxed);
        paint_cell(col, row, paint);
    }
}

static PAINT: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(true);

fn canvas_cell(cursor: (f32, f32), x: f32, y: f32) -> Option<(usize, usize)> {
    let col = ((cursor.0 - x - 8.0) / CELL).floor();
    let row = ((cursor.1 - y - HEADER - 4.0) / CELL).floor();
    ((0.0..15.0).contains(&col) && (0.0..15.0).contains(&row)).then_some((col as usize, row as usize))
}

fn paint_cell(col: usize, row: usize, paint: bool) {
    let bits = crosshair_row(row);
    let next = if paint { bits | 1 << col } else { bits & !(1 << col) };
    if next != bits {
        set_value("crosshair", &format!("r{row}"), next);
    }
}

/// Every HUD element the editor handles.
const ELEMENTS: [(&str, &str); 23] = [("fps", "FPS"), ("cps", "CPS"), ("coordinates", "Coordinates"), ("speed", "Speed"), ("clock", "Clock"), ("day", "Day"), ("light", "Light"), ("session", "Session"), ("reach", "Reach"), ("server", "Server"), ("tps", "TPS"), ("kill_feed", "Kill feed"), ("scoreboard", "Scoreboard"), ("action_bar", "Action bar"),
    ("totems", "Item counter"), ("combo", "Combo"), ("durability_warning", "Gear warning"), ("ping", "Ping"), ("armor", "Armor"),
    ("potions", "Potions"), ("keystrokes", "Keystrokes"), ("inventory", "Inventory"), ("lyrics", "Lyrics")];
const TOOLS: [&str; 4] = ["Snap", "Elements", "Reset", "Done"];
const TOOL_W: f32 = 64.0;

#[derive(Clone, Copy, PartialEq)]
enum Grab {
    Move(f32, f32),
    /// The handle's direction, -1, 0 or 1 on each axis.
    Size(i32, i32),
}

#[derive(Default)]
struct Editor {
    selected: Option<usize>,
    grab: Option<Grab>,
    snap: bool,
    list: bool,
    popup: Option<usize>,
    bar_hidden: bool,
    undo: Vec<Vec<(usize, [i32; 6])>>,
    /// More than one element picked; the selected one leads.
    group: Vec<usize>,
    /// Where each picked element stood when the drag began.
    moving: Vec<(usize, f32, f32)>,
    /// The box being dragged out over empty space.
    band: Option<[(f32, f32); 2]>,
    cursor: (f32, f32),
    size: (f32, f32),
}

static EDITOR: LazyLock<Mutex<Editor>> = LazyLock::new(|| Mutex::new(Editor { snap: true, ..Default::default() }));
/// Each element's natural size and unmoved place, as last drawn.
static SIZES: LazyLock<Mutex<HashMap<&'static str, [f32; 4]>>> = LazyLock::new(Default::default);

static SCREEN: Mutex<(f32, f32)> = Mutex::new((0.0, 0.0));
static SHIFT: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// The GUI's size this frame: moved elements hang off its edges.
pub fn set_screen(w: f32, h: f32) {
    refresh_palette();
    if let Ok(mut own) = SCREEN.lock() {
        *own = (w, h);
    }
}

fn screen() -> (f32, f32) {
    SCREEN.lock().map_or((0.0, 0.0), |s| *s)
}

pub fn set_shift(held: bool) {
    SHIFT.store(held, std::sync::atomic::Ordering::Relaxed);
}

fn ed<T>(f: impl FnOnce(&mut Editor) -> T) -> T {
    f(&mut EDITOR.lock().unwrap_or_else(std::sync::PoisonError::into_inner))
}

fn key_of(index: usize, what: &str) -> String {
    format!("{}_{what}", ELEMENTS[index].0)
}

fn scale_of(index: usize) -> f32 {
    let s = value("hud", &key_of(index, "s"));
    if s == 0 { 1.0 } else { s as f32 / 100.0 }
}

/// An element's screen box: x, y, drawn width, drawn height.
fn element_box(index: usize) -> Option<[f32; 4]> {
    let known = SIZES.lock().ok()?.get(ELEMENTS[index].0).copied()?;
    let [w, h, dx, dy] = known;
    let moved = value("hud", &key_of(index, "p")) == 1;
    let s = scale_of(index);
    let (x, y) = if moved { anchored(index, w * s, h * s) } else { (dx, dy) };
    Some([x, y, w * s, h * s])
}

/// Where a moved element sits: its offset from the edge or centre it hangs off, kept on screen.
fn anchored(index: usize, w: f32, h: f32) -> (f32, f32) {
    let (sw, sh) = screen();
    let at = |what: &str, anchor: &str, length: f32, full: f32| {
        let saved = value("hud", &key_of(index, what)) as f32;
        let spot = match value("hud", &key_of(index, anchor)) {
            2 => ((full - length) / 2.0).floor() + saved,
            3 => full - length + saved,
            _ => saved,
        };
        if full > 0.0 { spot.clamp(0.0, (full - length).max(0.0)) } else { spot }
    };
    (at("x", "ax", w, sw), at("y", "ay", h, sh))
}

/// The nearest edge by thirds of the screen, as Meteor's: 1 start, 2 centre, 3 end. 0 is unset.
fn anchor_of(at: f32, length: f32, full: f32) -> i32 {
    let third = full / 3.0;
    let (near, far) = (at <= third, at + length >= third * 2.0);
    if near == far {
        2
    } else if near {
        1
    } else {
        3
    }
}

/// Where an element draws: x, y, scale, whether it has its panel. Also tells the editor its size.
/// Whether the HUD editor is open.
pub fn editing() -> bool {
    with(|s| s.by_key && s.page.is_some_and(|p| p.0 == "hud_editor"))
}

/// Whether the player has moved this element.
pub fn is_moved(id: &str) -> bool {
    ELEMENTS.iter().position(|e| e.0 == id).is_some_and(|index| value("hud", &key_of(index, "p")) == 1)
}

pub fn place(id: &str, w: f32, h: f32, default: (f32, f32)) -> (f32, f32, f32, bool) {
    let Some(index) = ELEMENTS.iter().position(|e| e.0 == id) else {
        return (default.0, default.1, 1.0, false);
    };
    if let Ok(mut sizes) = SIZES.lock() {
        sizes.insert(ELEMENTS[index].0, [w, h, default.0, default.1]);
    }
    let moved = value("hud", &key_of(index, "p")) == 1;
    // Saved before anchors existed: hang it off the nearest edge now.
    if moved && value("hud", &key_of(index, "ax")) == 0 && screen().0 > 0.0 {
        move_to(index, value("hud", &key_of(index, "x")) as f32, value("hud", &key_of(index, "y")) as f32);
    }
    let s = scale_of(index);
    let (x, y) = if moved { anchored(index, w * s, h * s) } else { default };
    // Just switched on: it pops in from its own middle.
    let t = toggle_share(id);
    let pop = if t < 1.0 { 0.72 + 0.28 * (1.0 - (1.0 - t) * (1.0 - t)) } else { 1.0 };
    (x + w * s * (1.0 - pop) / 2.0, y + h * s * (1.0 - pop) / 2.0, s * pop, value(id, "panel") == 1)
}

const SAVED_KEYS: [&str; 6] = ["x", "y", "s", "p", "ax", "ay"];

/// What the next change acts on: the group, or the selected element.
fn chosen() -> Vec<usize> {
    ed(|e| if e.group.is_empty() { e.selected.into_iter().collect() } else { e.group.clone() })
}

fn remember(indices: &[usize]) {
    let snapshot: Vec<(usize, [i32; 6])> = indices.iter().map(|i| (*i, SAVED_KEYS.map(|k| value("hud", &key_of(*i, k))))).collect();
    ed(|e| {
        e.undo.push(snapshot);
        if e.undo.len() > 50 {
            e.undo.remove(0);
        }
    });
}

fn move_to(index: usize, x: f32, y: f32) {
    let (sw, sh) = screen();
    let [_, _, w, h] = element_box(index).unwrap_or_default();
    let offset = |at: f32, length: f32, full: f32, anchor: i32| match anchor {
        2 => at - ((full - length) / 2.0).floor(),
        3 => at - (full - length),
        _ => at,
    };
    let (ax, ay) = (anchor_of(x, w, sw), anchor_of(y, h, sh));
    set_value("hud", &key_of(index, "x"), offset(x, w, sw, ax).round() as i32);
    set_value("hud", &key_of(index, "y"), offset(y, h, sh, ay).round() as i32);
    set_value("hud", &key_of(index, "ax"), ax);
    set_value("hud", &key_of(index, "ay"), ay);
    set_value("hud", &key_of(index, "p"), 1);
}

fn set_scale(index: usize, scale: f32) {
    set_value("hud", &key_of(index, "s"), (scale * 100.0).round().clamp(50.0, 300.0) as i32);
}

/// Snaps a box to the screen's edges and centre and to the other elements' edges.
fn snapped(index: usize, skip: &[usize], x: f32, y: f32, w: f32, h: f32, size: (f32, f32)) -> (f32, f32) {
    const NEAR: f32 = 4.0;
    let mut xs = vec![0.0, size.0, size.0 / 2.0];
    let mut ys = vec![0.0, size.1, size.1 / 2.0];
    for other in (0..ELEMENTS.len()).filter(|o| *o != index && !skip.contains(o) && on(ELEMENTS[*o].0)) {
        if let Some([ox, oy, ow, oh]) = element_box(other) {
            xs.extend([ox, ox + ow]);
            ys.extend([oy, oy + oh]);
        }
    }
    let fit = |at: f32, length: f32, lines: &[f32]| {
        let mut best = at;
        let mut gap = NEAR;
        let mut guide = None;
        for line in lines {
            for (edge, shift) in [(at, 0.0), (at + length, length), (at + length / 2.0, length / 2.0)] {
                if (edge - line).abs() < gap {
                    gap = (edge - line).abs();
                    best = line - shift;
                    guide = Some(*line);
                }
            }
        }
        (best, guide)
    };
    let ((sx, gx), (sy, gy)) = (fit(x, w, &xs), fit(y, h, &ys));
    if let Ok(mut guides) = GUIDES.lock() {
        *guides = (gx, gy);
    }
    (sx, sy)
}

static GUIDES: Mutex<(Option<f32>, Option<f32>)> = Mutex::new((None, None));
static LAST_CLICK: Mutex<Option<(usize, Instant)>> = Mutex::new(None);

fn handles(b: [f32; 4]) -> [(i32, i32, f32, f32); 8] {
    let [x, y, w, h] = b;
    let (mx, my, rx, by) = (x + w / 2.0, y + h / 2.0, x + w, y + h);
    [(-1, -1, x, y), (0, -1, mx, y), (1, -1, rx, y), (-1, 0, x, my), (1, 0, rx, my), (-1, 1, x, by), (0, 1, mx, by), (1, 1, rx, by)]
}

fn tool_rect(i: usize, size: (f32, f32)) -> [f32; 4] {
    [size.0 / 2.0 - 2.0 * TOOL_W + i as f32 * TOOL_W + 4.0, 6.0, TOOL_W - 8.0, 18.0]
}

fn inside(c: (f32, f32), [x, y, w, h]: [f32; 4]) -> bool {
    c.0 >= x && c.0 < x + w && c.1 >= y && c.1 < y + h
}

/// The editor's list: elements under group titles, in the same layout as the pages.
fn element_items() -> Vec<(&'static str, &'static str, bool)> {
    const GROUPS: [(&str, &[&str]); 4] = [
        ("Info", &["fps", "cps", "coordinates", "speed", "clock", "day", "light", "session", "ping", "tps", "server"]),
        ("Combat", &["combo", "reach", "kill_feed", "durability_warning"]),
        ("Items", &["totems", "armor", "potions", "inventory"]),
        ("Screen", &["keystrokes", "scoreboard", "action_bar", "lyrics"]),
    ];
    let mut out = Vec::new();
    for (title, ids) in GROUPS {
        out.push(("#", title, true));
        out.extend(ELEMENTS.iter().filter(|e| ids.contains(&e.0)).map(|e| (e.0, e.1, true)));
    }
    out
}

fn list_rect(size: (f32, f32)) -> [f32; 4] {
    let h = layout(&element_items()).1 * ROW + 8.0 + HEADER;
    [size.0 / 2.0 - 150.0, 34.0, 300.0, h]
}

/// Beside the element, on the side of the screen with room: left half opens right, right half opens left.
fn popup_rect(index: usize, size: (f32, f32)) -> [f32; 4] {
    let [x, y, w, _] = element_box(index).unwrap_or([size.0 / 2.0, size.1 / 2.0, 0.0, 0.0]);
    let h = (settings_of(ELEMENTS[index].0).len() + 1 + app_rows(ELEMENTS[index].0)) as f32 * ROW + HEADER + 8.0;
    let px = if x + w / 2.0 < size.0 / 2.0 { x + w + 6.0 } else { x - SETTINGS_W - 6.0 };
    [px.clamp(0.0, (size.0 - SETTINGS_W).max(0.0)), y.min(size.1 - h).max(0.0), SETTINGS_W, h]
}

fn draw_hud_editor(batch: &mut HudBatch, atlas: &HudAtlas, cursor: (f32, f32)) {
    let size = (batch.width, batch.height);
    let white = ink();
    set_screen(size.0, size.1);
    let (selected, snap, list, popup, hidden, group, band) = ed(|e| {
        e.cursor = cursor;
        e.size = size;
        (e.selected, e.snap, e.list, e.popup, e.bar_hidden, e.group.clone(), e.band)
    });
    // Switched in the list: a box closes on what left, and rings what arrived.
    for (index, (id, _)) in ELEMENTS.iter().enumerate() {
        let t = toggle_share(id);
        if let Some([x, y, w, h]) = element_box(index).filter(|_| t < 1.0) {
            let (cx, cy) = (x + w / 2.0, y + h / 2.0);
            let k = if on(id) { 1.0 + 0.25 * t } else { 1.0 - 0.3 * t };
            let (bw, bh) = (w * k, h * k);
            let (bx, by) = (cx - bw / 2.0, cy - bh / 2.0);
            let alpha = ((1.0 - t) * 200.0) as u8;
            if !on(id) {
                batch.fill(atlas, bx, by, bw, bh, ca(1, alpha));
            }
            for edge in [[bx, by, bw, 1.0], [bx, by + bh - 1.0, bw, 1.0], [bx, by, 1.0, bh], [bx + bw - 1.0, by, 1.0, bh]] {
                batch.fill(atlas, edge[0], edge[1], edge[2], edge[3], ca(7, alpha));
            }
        }
    }
    for (index, (id, name)) in ELEMENTS.iter().enumerate() {
        let Some(b) = element_box(index).filter(|_| on(id)) else {
            continue;
        };
        let [x, y, w, h] = b;
        // The real element shows through; only the one under the cursor is marked and named.
        if cursor.0 >= x && cursor.0 < x + w && cursor.1 >= y && cursor.1 < y + h {
            for edge in [[x, y, w, 0.5], [x, y + h - 0.5, w, 0.5], [x, y, 0.5, h], [x + w - 0.5, y, 0.5, h]] {
                batch.fill(atlas, edge[0], edge[1], edge[2], edge[3], rgba(255, 255, 255, 150));
            }
            if selected != Some(index) {
                let tw = HudBatch::text_width(atlas, name);
                batch.fill(atlas, x, (y - 11.0).max(0.0), tw + 6.0, 10.0, sel());
                text(batch, atlas, name, x + 3.0, (y - 10.0).max(1.0), seltxt(), false);
            }
        }
        if selected == Some(index) || group.contains(&index) {
            for edge in [[x, y, w, 1.0], [x, y + h - 1.0, w, 1.0], [x, y, 1.0, h], [x + w - 1.0, y, 1.0, h]] {
                batch.fill(atlas, edge[0], edge[1], edge[2], edge[3], accent());
            }
        }
        if selected == Some(index) && group.len() < 2 {
            for (_, _, hx, hy) in handles(b) {
                notched(batch, atlas, hx - 3.0, hy - 3.0, 6.0, 6.0, ink());
                notched(batch, atlas, hx - 2.0, hy - 2.0, 4.0, 4.0, ca(1, 255));
            }
            let tag = format!("{name} {}%", (scale_of(index) * 100.0).round());
            let tw = HudBatch::text_width(atlas, &tag);
            batch.fill(atlas, x, (y - 11.0).max(0.0), tw + 6.0, 10.0, sel());
            text(batch, atlas, &tag, x + 3.0, (y - 10.0).max(1.0), seltxt(), false);
        }
    }
    if let Some([from, to]) = band {
        let (x, y, w, h) = (from.0.min(to.0), from.1.min(to.1), (from.0 - to.0).abs(), (from.1 - to.1).abs());
        batch.fill(atlas, x, y, w, h, ca(7, 50));
        for edge in [[x, y, w, 1.0], [x, y + h - 1.0, w, 1.0], [x, y, 1.0, h], [x + w - 1.0, y, 1.0, h]] {
            batch.fill(atlas, edge[0], edge[1], edge[2], edge[3], accent());
        }
    }
    let (gx, gy) = GUIDES.lock().map_or((None, None), |g| *g);
    if let Some(x) = gx {
        batch.fill(atlas, x - 0.25, 0.0, 0.5, size.1, accent());
    }
    if let Some(y) = gy {
        batch.fill(atlas, 0.0, y - 0.25, size.0, 0.5, accent());
    }
    if !hidden {
        for (i, tool) in TOOLS.iter().enumerate() {
            let r = tool_rect(i, size);
            let lit = (i == 0 && snap) || (i == 1 && list);
            batch.fill(atlas, r[0] + 2.0, r[1] + 2.0, r[2], r[3], if lit { accent() } else { line() });
            batch.fill(atlas, r[0], r[1], r[2], r[3], if lit { sel() } else if inside(cursor, r) { ca(1, 255) } else { panel() });
            let ink = if lit { seltxt() } else { white };
            let tool = if i == 2 && selected.is_none() && group.is_empty() { "Reset all" } else { *tool };
            centered(batch, atlas, tool, r[0] + r[2] / 2.0, r[1] + 5.0, ink);
        }
        let tip = "Drag to move / corners resize / wheel sizes / double-click for settings / Shift or drag a box selects more";
        let tw = HudBatch::text_width(atlas, tip);
        let tk = ((size.0 - 12.0) / (tw + 10.0)).min(1.0);
        let start = batch.vertices.len();
        let (tx, ty) = (size.0 / 2.0 - tw / 2.0, size.1 - 39.0);
        batch.fill(atlas, tx - 5.0, ty - 2.0, tw + 10.0, 12.0, sel());
        text(batch, atlas, tip, tx, ty, seltxt(), false);
        batch.scale_since(start, size.0 / 2.0, ty + 4.0, tk, tk);
    }
    let (list_age, popup_age) = pop_ages(list, popup);
    if list {
        let [x, y, w, h] = list_rect(size);
        let first = batch.vertices.len();
        frame(batch, atlas, x, y, w, h);
        batch.fill(atlas, x, y, w, HEADER - 4.0, sel());
        text(batch, atlas, "Elements", x + 9.0, y + 6.0, seltxt(), false);
        let count = format!("{} / {}", ELEMENTS.iter().filter(|e| on(e.0)).count(), ELEMENTS.len());
        text(batch, atlas, &count, x + w - 9.0 - HudBatch::text_width(atlas, &count), y + 6.0, seltxt(), false);
        let shown = element_items();
        let (places, _) = layout(&shown);
        for ((id, name, _), (col, row)) in shown.iter().zip(places) {
            let (rx, ry) = (x + 6.0 + col * 146.0, y + HEADER + 1.0 + row * ROW);
            if *id == "#" {
                text(batch, atlas, name, rx, ry + 3.0, soft(), false);
                continue;
            }
            text(batch, atlas, name, rx, ry + 3.0, white, false);
            switch_of(batch, atlas, rx + 136.0, ry + 1.0, id);
        }
        ease_in(batch, first, [x, y, w, h], list_age);
    }
    if let Some(index) = popup {
        let first = batch.vertices.len();
        draw_panel(batch, atlas, cursor, ELEMENTS[index]);
        ease_in(batch, first, popup_rect(index, size), popup_age);
    }
}

static POPS: Mutex<(bool, Option<usize>, Option<Instant>, Option<Instant>)> = Mutex::new((false, None, None, None));

/// Seconds since the Elements list and the popup opened, for the frame that draws them.
fn pop_ages(list: bool, popup: Option<usize>) -> (f32, f32) {
    let Ok(mut pops) = POPS.lock() else {
        return (1.0, 1.0);
    };
    let now = Instant::now();
    if list && !pops.0 {
        pops.2 = Some(now);
    }
    if popup.is_some() && pops.1 != popup {
        pops.3 = Some(now);
    }
    pops.0 = list;
    pops.1 = popup;
    let age = |t: Option<Instant>| t.map_or(1.0, |t| t.elapsed().as_secs_f32());
    (age(pops.2), age(pops.3))
}

/// A panel grows a little and fades in over its first moment.
fn ease_in(batch: &mut HudBatch, first: usize, [x, y, w, h]: [f32; 4], age: f32) {
    let t = (age / 0.13).min(1.0);
    if t >= 1.0 {
        return;
    }
    let eased = 1.0 - (1.0 - t) * (1.0 - t);
    let scale = 0.94 + 0.06 * eased;
    batch.scale_since(first, x + w / 2.0, y + h / 2.0, scale, scale);
    fade_since(batch, first, eased);
}

fn click_hud_editor(cursor: (f32, f32), size: (f32, f32), right: bool) {
    let (selected, list, popup, hidden) = ed(|e| {
        e.grab = None;
        (e.selected, e.list, e.popup, e.bar_hidden)
    });
    if let Some(index) = popup {
        if inside(cursor, popup_rect(index, size)) {
            click_panel(cursor, size, ELEMENTS[index].0);
            return;
        }
        ed(|e| e.popup = None);
    }
    if !hidden {
        for i in 0..TOOLS.len() {
            if inside(cursor, tool_rect(i, size)) {
                match i {
                    0 => ed(|e| e.snap = !e.snap),
                    1 => ed(|e| e.list = !e.list),
                    2 => {
                        // What's selected, or everything with nothing selected.
                        let mut picks = chosen();
                        if picks.is_empty() {
                            picks = (0..ELEMENTS.len()).collect();
                        }
                        remember(&picks);
                        for index in picks {
                            for key in ["p", "s", "ax", "ay"] {
                                set_value("hud", &key_of(index, key), 0);
                            }
                        }
                    }
                    _ => {
                        back();
                    }
                }
                return;
            }
        }
    }
    if list {
        let r = list_rect(size);
        if inside(cursor, r) {
            let col = ((cursor.0 - r[0] - 6.0) / 146.0).floor().clamp(0.0, 1.0);
            let row = ((cursor.1 - r[1] - HEADER + 1.0) / ROW).floor().max(0.0);
            let shown = element_items();
            let (places, _) = layout(&shown);
            let found = shown.iter().zip(places).find(|(m, p)| m.0 != "#" && p.0 == col && p.1 == row).and_then(|(m, _)| ELEMENTS.iter().position(|e| e.0 == m.0));
            if let Some(index) = found {
                if right {
                    ed(|e| {
                        e.selected = Some(index);
                        e.popup = Some(index);
                    });
                } else {
                    toggle(ELEMENTS[index].0);
                }
            }
            return;
        }
    }
    if !right && ed(|e| e.group.len()) < 2 && let Some(index) = selected && let Some(b) = element_box(index) {
        if let Some((hx, hy, _, _)) = handles(b).into_iter().find(|h| (cursor.0 - h.2).abs() <= 3.0 && (cursor.1 - h.3).abs() <= 3.0) {
            remember(&[index]);
            ed(|e| e.grab = Some(Grab::Size(hx, hy)));
            return;
        }
    }
    let hit = (0..ELEMENTS.len()).rev().find(|i| on(ELEMENTS[*i].0) && element_box(*i).is_some_and(|b| inside(cursor, b)));
    if let (Some(index), Ok(mut last)) = (hit.filter(|_| !right), LAST_CLICK.lock()) {
        let twice = last.is_some_and(|(i, t)| i == index && t.elapsed().as_millis() < 350);
        *last = if twice { None } else { Some((index, Instant::now())) };
        if twice {
            ed(|e| {
                e.selected = Some(index);
                e.group.clear();
                e.popup = Some(index);
            });
            return;
        }
    }
    match hit {
        Some(index) if right => ed(|e| {
            e.selected = Some(index);
            e.group.clear();
            e.popup = Some(index);
        }),
        // Shift adds to the pick, or takes back out.
        Some(index) if SHIFT.load(std::sync::atomic::Ordering::Relaxed) => ed(|e| {
            if e.group.is_empty() {
                e.group.extend(e.selected);
            }
            match e.group.iter().position(|g| *g == index) {
                Some(at) => {
                    e.group.remove(at);
                }
                None => e.group.push(index),
            }
            e.selected = e.group.last().copied();
        }),
        Some(index) => {
            let b = element_box(index).unwrap_or_default();
            ed(|e| {
                if !e.group.contains(&index) {
                    e.group.clear();
                }
                e.selected = Some(index);
                e.grab = Some(Grab::Move(cursor.0 - b[0], cursor.1 - b[1]));
            });
            let picks = chosen();
            remember(&picks);
            let starts: Vec<(usize, f32, f32)> = picks.iter().filter_map(|i| element_box(*i).map(|b| (*i, b[0], b[1]))).collect();
            ed(|e| e.moving = starts);
        }
        None => ed(|e| {
            e.selected = None;
            e.group.clear();
            e.band = Some([cursor, cursor]);
        }),
    }
}

/// The button came up: a dragged box is done.
pub fn editor_release() {
    ed(|e| e.band = None);
    if let Ok(mut held) = SLIDER.lock() {
        *held = None;
    }
    if let Ok(mut guides) = GUIDES.lock() {
        *guides = (None, None);
    }
    let held = SWIPE.lock().ok().and_then(|mut s| s.take());
    if let Some((from, at)) = held {
        let moved = at - from;
        let step = (CARD_W + CARD_GAP) * deck_scale(screen().1);
        if let Ok(mut deck) = DECK.lock() {
            deck.0 = (deck.0 - moved / step).rem_euclid(CARDS as f32);
        }
        if moved.abs() > 36.0 {
            let cards = (moved.abs() / step).round().max(1.0) as i32;
            step_card(if moved < 0.0 { cards } else { -cards });
        } else if let Some(card) = card_at((at, CARD_Y.load(std::sync::atomic::Ordering::Relaxed) as f32), screen()) {
            if card == with(|s| s.selected) {
                open_card(card);
            } else {
                with(|s| s.selected = card);
            }
        }
    }
}

/// The pointer moved with the button held.
pub fn drag(cursor: (f32, f32), size: (f32, f32)) {
    if let Some((id, key, min, max)) = SLIDER.lock().ok().and_then(|h| *h) {
        let popup = ed(|e| e.popup).is_some_and(|i| ELEMENTS[i].0 == id);
        let c = if popup { cursor } else { page_cursor(cursor, size) };
        let [x, y, w, h] = settings_rect(size.0, size.1, id);
        let c = fitted(c, [x, y, w, h], panel_room(id, size));
        slide(c.0, x + w - 12.0, id, key, min, max);
        return;
    }
    if let Ok(mut swipe) = SWIPE.lock()
        && let Some(held) = swipe.as_mut()
    {
        held.1 = cursor.0;
        return;
    }
    if picker_input(cursor, size, true) {
        return;
    }
    if with(|s| s.page).is_some_and(|p| p.0 == "crosshair_editor") {
        let [x, y, _, _] = canvas_rect(size.0, size.1);
        if let Some((col, row)) = canvas_cell(cursor, x, y) {
            paint_cell(col, row, PAINT.load(std::sync::atomic::Ordering::Relaxed));
        }
        return;
    }
    if with(|s| s.page).is_none_or(|p| p.0 != "hud_editor") {
        return;
    }
    if let Some([from, _]) = ed(|e| e.band) {
        let (x0, x1, y0, y1) = (from.0.min(cursor.0), from.0.max(cursor.0), from.1.min(cursor.1), from.1.max(cursor.1));
        let caught: Vec<usize> = (0..ELEMENTS.len())
            .filter(|i| on(ELEMENTS[*i].0) && element_box(*i).is_some_and(|b| b[0] < x1 && b[0] + b[2] > x0 && b[1] < y1 && b[1] + b[3] > y0))
            .collect();
        ed(|e| {
            e.band = Some([from, cursor]);
            e.selected = caught.last().copied();
            e.group = caught;
        });
        return;
    }
    let (selected, grab, snap) = ed(|e| (e.selected, e.grab, e.snap));
    let (Some(index), Some(grab)) = (selected, grab) else {
        return;
    };
    let Some([x, y, w, h]) = element_box(index) else {
        return;
    };
    match grab {
        Grab::Move(dx, dy) => {
            let moving = ed(|e| e.moving.clone());
            let Some(&(_, lead_x, lead_y)) = moving.iter().find(|m| m.0 == index) else {
                return;
            };
            // The whole group stops together at the screen's edge, so nobody bunches up.
            let room = |shift: f32, spots: Vec<(f32, f32)>, full: f32| {
                let least = spots.iter().map(|s| -s.0).fold(f32::MIN, f32::max);
                let most = spots.iter().map(|s| full - s.1 - s.0).fold(f32::MAX, f32::min);
                shift.clamp(least, most.max(least))
            };
            let boxes: Vec<[f32; 4]> = moving.iter().map(|m| {
                let [_, _, bw, bh] = element_box(m.0).unwrap_or_default();
                [m.1, m.2, bw, bh]
            }).collect();
            let fit = |sx: f32, sy: f32| (room(sx, boxes.iter().map(|b| (b[0], b[2])).collect(), size.0), room(sy, boxes.iter().map(|b| (b[1], b[3])).collect(), size.1));
            let (mut sx, mut sy) = fit(cursor.0 - dx - lead_x, cursor.1 - dy - lead_y);
            if snap {
                let skip: Vec<usize> = moving.iter().map(|m| m.0).collect();
                let (nx, ny) = snapped(index, &skip, lead_x + sx, lead_y + sy, w, h, size);
                (sx, sy) = fit(nx - lead_x, ny - lead_y);
            }
            for (other, from_x, from_y) in moving {
                move_to(other, from_x + sx, from_y + sy);
            }
        }
        Grab::Size(hx, hy) => {
            // The opposite edge stays put.
            let (fx, fy) = (if hx < 0 { x + w } else { x }, if hy < 0 { y + h } else { y });
            let wide = if hx != 0 { (cursor.0 - fx).abs() / w.max(1.0) } else { 0.0 };
            let tall = if hy != 0 { (cursor.1 - fy).abs() / h.max(1.0) } else { 0.0 };
            let next = (scale_of(index) * wide.max(tall)).clamp(0.5, 3.0);
            let before = scale_of(index);
            set_scale(index, next);
            // The stored scale is rounded: place by what it became, and pin the element where it is.
            let ratio = scale_of(index) / before;
            set_value("hud", &key_of(index, "p"), 1);
            move_to(index, if hx < 0 { fx - w * ratio } else { x }, if hy < 0 { fy - h * ratio } else { y });
        }
    }
}

/// The wheel in the HUD editor resizes what it is over; returns whether it used the scroll.
pub fn editor_wheel(amount: f32) -> bool {
    if with(|s| s.by_key && s.page.is_none_or(|p| !p.0.ends_with("_editor"))) {
        if with(|s| s.list) {
            with(|s| s.scroll -= amount * ROW_H * 2.0);
        } else {
            step_card(-(amount.signum() as i32));
        }
        return true;
    }
    if with(|s| s.page).is_none_or(|p| p.0 != "hud_editor") {
        return false;
    }
    let cursor = ed(|e| e.cursor);
    if let Some(index) = (0..ELEMENTS.len()).rev().find(|i| on(ELEMENTS[*i].0) && element_box(*i).is_some_and(|b| inside(cursor, b))) {
        remember(&[index]);
        set_scale(index, scale_of(index) + amount.signum() * 0.05);
    }
    true
}

/// The HUD editor's keys; returns whether it used the key.
pub fn editor_key(key: winit::keyboard::KeyCode, ctrl: bool) -> bool {
    use winit::keyboard::KeyCode as K;
    if with(|s| s.page).is_none_or(|p| p.0 != "hud_editor") {
        return false;
    }
    let selected = ed(|e| e.selected);
    let nudge = |dx: f32, dy: f32| {
        let picks = chosen();
        remember(&picks);
        for index in picks {
            if let Some(b) = element_box(index) {
                move_to(index, b[0] + dx, b[1] + dy);
            }
        }
    };
    match key {
        K::Tab => {
            let start = selected.map_or(0, |s| s + 1);
            let next = (0..ELEMENTS.len()).map(|i| (start + i) % ELEMENTS.len()).find(|i| on(ELEMENTS[*i].0));
            ed(|e| {
                e.selected = next;
                e.group.clear();
            });
        }
        K::KeyA if ctrl => {
            let all: Vec<usize> = (0..ELEMENTS.len()).filter(|i| on(ELEMENTS[*i].0)).collect();
            ed(|e| {
                e.selected = all.last().copied();
                e.group = all;
            });
        }
        K::ArrowLeft => nudge(-1.0, 0.0),
        K::ArrowRight => nudge(1.0, 0.0),
        K::ArrowUp => nudge(0.0, -1.0),
        K::ArrowDown => nudge(0.0, 1.0),
        K::Equal | K::NumpadAdd | K::Minus | K::NumpadSubtract => {
            let picks = chosen();
            remember(&picks);
            let step = if matches!(key, K::Equal | K::NumpadAdd) { 0.05 } else { -0.05 };
            for index in picks {
                set_scale(index, scale_of(index) + step);
            }
        }
        K::Space => {
            if let Some(index) = selected {
                toggle(ELEMENTS[index].0);
            }
        }
        K::KeyZ if ctrl => {
            for (index, values) in ed(|e| e.undo.pop()).unwrap_or_default() {
                for (key, value) in SAVED_KEYS.iter().zip(values) {
                    set_value("hud", &key_of(index, key), value);
                }
            }
        }
        K::KeyS => ed(|e| e.snap = !e.snap),
        K::KeyH => ed(|e| e.bar_hidden = !e.bar_hidden),
        _ => return false,
    }
    true
}

fn draw_settings(batch: &mut HudBatch, atlas: &HudAtlas, cursor: (f32, f32), (id, name): (&'static str, &'static str)) {
    if id == "crosshair_editor" {
        draw_canvas(batch, atlas, cursor);
        return;
    }
    if id == "hud_editor" {
        draw_hud_editor(batch, atlas, cursor);
        return;
    }
    draw_panel(batch, atlas, cursor, (id, name));
}

fn panel_room(id: &str, size: (f32, f32)) -> f32 {
    if ed(|e| e.popup).is_some_and(|i| ELEMENTS[i].0 == id) { size.1 } else { sheet_area(size)[3] + 16.0 }
}

fn draw_panel(batch: &mut HudBatch, atlas: &HudAtlas, cursor: (f32, f32), (id, name): (&'static str, &'static str)) {
    let [x, y, w, h] = settings_rect(batch.width, batch.height, id);
    let room = panel_room(id, (batch.width, batch.height));
    let cursor = fitted(cursor, [x, y, w, h], room);
    let start = batch.vertices.len();
    frame(batch, atlas, x, y, w, h);
    batch.fill(atlas, x, y, w, HEADER - 4.0, sel());
    text(batch, atlas, name, x + 9.0, y + 6.0, seltxt(), false);
    if id != "ring_menu" {
        let state = if on(id) { "ON" } else { "OFF" };
        text(batch, atlas, state, x + w - 9.0 - HudBatch::text_width(atlas, state), y + 6.0, seltxt(), false);
    }
    if id.starts_with("cape_") {
        cape_art(batch, atlas, id, x + w / 2.0 - 43.0, y + HEADER + ROW + 8.0, 86.0, 136.0);
    }
    let right = x + w - 12.0;
    let white = ink();
    let mut ry = y + HEADER + 5.0;
    if id == "ring_menu" {
        ry -= ROW; // A key, not a module: nothing to switch.
    } else {
        text(batch, atlas, "Enabled", x + 11.0, ry + 2.0, white, false);
        switch_of(batch, atlas, right, ry, id);
    }
    for (key, label, min, max, _) in settings_of(id) {
        ry += ROW;
        if cursor.0 >= x && cursor.0 < x + w && cursor.1 >= ry - 4.0 && cursor.1 < ry + ROW - 4.0 {
            notched(batch, atlas, x + 6.0, ry - 4.0, w - 12.0, ROW - 1.0, ca(3, 30));
        }
        if key.starts_with("note_") {
            text(batch, atlas, label, x + 11.0, ry + 2.0, soft(), false);
            continue;
        }
        text(batch, atlas, label, x + 11.0, ry + 2.0, white, false);
        let current = value(id, key);
        if TEXT_ROWS.contains(&(id, *key)) {
            let editing = TEXT_FOCUS.lock().is_ok_and(|f| *f == Some((id, *key)));
            let value = text_value(id, key);
            let tail: String = value.chars().rev().take(22).collect::<Vec<_>>().into_iter().rev().collect();
            let shown = if editing { format!("{tail}_") } else if value.is_empty() { "Click to type".to_owned() } else { tail };
            let ink = if editing { accent() } else if value.is_empty() { soft() } else { white };
            field(batch, atlas, right, ry, &shown);
            text(batch, atlas, &shown, right - HudBatch::text_width(atlas, &shown), ry + 2.0, ink, false);
        } else if let Some(bind) = key.strip_prefix("key_") {
            let waiting = LISTENING.lock().is_ok_and(|l| *l == Some(bind));
            let shown = if waiting {
                "Press a key, Esc clears".to_owned()
            } else {
                KEY_NAMES.lock().ok().and_then(|n| n.iter().find(|k| k.0 == bind).map(|k| k.1.clone())).unwrap_or_default()
            };
            field(batch, atlas, right, ry, &shown);
            text(batch, atlas, &shown, right - HudBatch::text_width(atlas, &shown), ry + 2.0, if waiting { accent() } else { white }, false);
        } else if key.ends_with("color") {
            for (i, color) in COLORS.iter().enumerate() {
                let bx = right - 80.0 + i as f32 * 10.0;
                notched(batch, atlas, bx - 1.0, ry, 10.0, 11.0, if i as i32 == current { white } else { line() });
                notched(batch, atlas, bx, ry + 1.0, 8.0, 9.0, vertex_color(*color, 255));
            }
            // The custom swatch shows the picked color, or a rainbow until there is one.
            let bx = right - 90.0;
            notched(batch, atlas, bx - 1.0, ry, 10.0, 11.0, if current >= CUSTOM { white } else { line() });
            if current >= CUSTOM {
                notched(batch, atlas, bx, ry + 1.0, 8.0, 9.0, vertex_color(color_of(current), 255));
            } else {
                notched(batch, atlas, bx, ry + 1.0, 8.0, 9.0, sunk());
                batch.fill(atlas, bx + 1.5, ry + 5.0, 5.0, 1.0, white);
                batch.fill(atlas, bx + 3.5, ry + 3.0, 1.0, 5.0, white);
            }
        } else if *min == 0 && *max == 1 {
            let share = toggle_share(&format!("{id}.{key}"));
            switch_at(batch, atlas, right, ry, if current == 1 { share } else { 1.0 - share });
        } else {
            let choices: &[&str] = match *key {
                "time" => &["Real", "Sunrise", "Morning", "Noon", "Sunset", "Night", "Midnight"],
                "weather" => &["Real", "Clear", "Rain", "Thunder"],
                "fog" => &["Normal", "Less", "None"],
                "below" => &["2", "3", "4"],
                "value" => &["Durability", "Percent", "Off"],
                "from" => &["2", "3", "5"],
                "box_style" => &["Box", "Flat", "Corners", "Filled"],
                "glint_style" => &["Normal", "Soft", "Solid"],
                "fire" => &["Vanilla", "Low", "Lowest"],
                "position" => &["Normal", "Low", "Side"],
                "mode" => &["Vanilla", "No speed", "Static"],
                "transition" => &["Instant", "Vanilla", "Smooth"],
                "power" => &["Subtle", "Normal", "Strong"],
                "style" => &["Cross", "Cross dot", "Dot", "Circle", "T", "X", "Circle dot", "Square", "Chevron", "Dots", "Custom"],
                _ => &[],
            };
            let shown = if let Some(choice) = choices.get(current as usize) {
                (*choice).to_owned()
            } else if current == 0 && *min == 0 {
                "Off".to_owned()
            } else if id == "zoom" {
                format!("{current}x")
            } else {
                current.to_string()
            };
            let track = right - SLIDER_W;
            let share = glide(&format!("{id}.{key}"), (current - min) as f32 / (max - min).max(1) as f32);
            batch.fill(atlas, track, ry + 4.0, SLIDER_W, 3.0, line());
            batch.fill(atlas, track, ry + 4.0, SLIDER_W * share, 3.0, accent());
            batch.fill(atlas, track + SLIDER_W * share - 3.0, ry + 1.0, 6.0, 9.0, sel());
            batch.fill(atlas, track + SLIDER_W * share - 1.0, ry + 3.0, 2.0, 5.0, seltxt());
            text(batch, atlas, &shown, track - 7.0 - HudBatch::text_width(atlas, &shown), ry + 2.0, white, false);
        }
    }
    if id == "lyrics" {
        ry += ROW;
        text(batch, atlas, "Music apps", x + 11.0, ry + 2.0, soft(), false);
        let apps = media_apps();
        if apps.is_empty() {
            ry += ROW;
            text(batch, atlas, "Play some music and it shows up here", x + 11.0, ry + 2.0, soft(), false);
        }
        let off = apps_off();
        for app in &apps {
            ry += ROW;
            if cursor.0 >= x && cursor.0 < x + w && cursor.1 >= ry - 4.0 && cursor.1 < ry + ROW - 4.0 {
                notched(batch, atlas, x + 6.0, ry - 4.0, w - 12.0, ROW - 1.0, ca(3, 30));
            }
            text(batch, atlas, &app_name(app), x + 11.0, ry + 2.0, white, false);
            let share = toggle_share(app);
            switch_at(batch, atlas, right, ry, if off.contains(app) { 1.0 - share } else { share });
        }
    }
    let pop = fit(h, room);
    batch.scale_since(start, x + w / 2.0, y + h / 2.0, pop, pop);
}

static MEDIA_APPS: Mutex<Vec<String>> = Mutex::new(Vec::new());

/// The apps Windows says are playing music, kept for the whole session.
pub fn set_media_apps(apps: &[&str]) {
    if let Ok(mut known) = MEDIA_APPS.lock() {
        for app in apps.iter().filter(|a| !a.is_empty()) {
            if !known.iter().any(|k| k == app) {
                known.push((*app).to_owned());
            }
        }
    }
}

fn apps_off() -> Vec<String> {
    text_value("lyrics", "text_apps_off").split('|').filter(|a| !a.is_empty()).map(str::to_owned).collect()
}

/// Everything seen now, and anything switched off before, so its switch can be turned back on.
fn media_apps() -> Vec<String> {
    let mut apps = MEDIA_APPS.lock().map_or(Vec::new(), |k| k.clone());
    for app in apps_off() {
        if !apps.contains(&app) {
            apps.push(app);
        }
    }
    apps.sort_by_key(|a| app_name(a).to_lowercase());
    apps.truncate(12);
    apps
}

/// Rows the Lyrics sheet adds for its app switches: a title, then one each, or a hint.
fn app_rows(id: &str) -> usize {
    if id == "lyrics" { 1 + media_apps().len().max(1) } else { 0 }
}

/// Windows names apps by long ids: known ones read nicely, the rest lose the id's packaging.
fn app_name(id: &str) -> String {
    let low = id.to_lowercase();
    const KNOWN: [(&str, &str); 16] = [("spotify", "Spotify"), ("applemusic", "Apple Music"), ("itunes", "iTunes"), ("soundcloud", "SoundCloud"), ("tidal", "Tidal"), ("deezer", "Deezer"), ("amazon", "Amazon Music"), ("youtube", "YouTube Music"), ("brave", "Brave"), ("chrome", "Chrome"), ("msedge", "Edge"), ("firefox", "Firefox"), ("308046b0af4a39cb", "Firefox"), ("opera", "Opera"), ("vlc", "VLC"), ("foobar", "foobar2000")];
    if let Some((_, name)) = KNOWN.iter().find(|k| low.contains(k.0)) {
        return (*name).to_owned();
    }
    let tail = id.rsplit(['\\', '/']).next().unwrap_or(id);
    let tail = tail.split('!').next().unwrap_or(tail).split('_').next().unwrap_or(tail).trim_end_matches(".exe");
    tail.rsplit('.').next().unwrap_or(tail).to_owned()
}

fn toggle_app(app: &str) {
    mark_toggle(app);
    let mut off = apps_off();
    if off.iter().any(|a| a == app) {
        off.retain(|a| a != app);
    } else {
        off.push(app.to_owned());
    }
    set_text("lyrics", "text_apps_off", off.join("|"));
}

fn click_settings(cursor: (f32, f32), size: (f32, f32), id: &'static str, right: bool) {
    if id == "crosshair_editor" {
        click_canvas(cursor, size);
        return;
    }
    if id == "hud_editor" {
        click_hud_editor(cursor, size, right);
        return;
    }
    click_panel(cursor, size, id);
}

fn click_panel(cursor: (f32, f32), size: (f32, f32), id: &'static str) {
    let [x, y, w, h] = settings_rect(size.0, size.1, id);
    let cursor = fitted(cursor, [x, y, w, h], panel_room(id, size));
    let row = ((cursor.1 - y - HEADER - 1.0) / ROW).floor();
    if row < 0.0 || cursor.0 < x || cursor.0 >= x + w {
        return;
    }
    let row = if id == "ring_menu" { row + 1.0 } else { row };
    let right = x + w - 12.0;
    if row == 0.0 {
        toggle(id);
        return;
    }
    if id == "lyrics" && row as usize >= settings_of(id).len() + 2 {
        if let Some(app) = media_apps().get(row as usize - settings_of(id).len() - 2) {
            toggle_app(app);
        }
        return;
    }
    let Some((key, _, min, max, _)) = settings_of(id).get(row as usize - 1).copied() else {
        return;
    };
    if key.starts_with("note_") {
        return;
    }
    if TEXT_ROWS.contains(&(id, key)) {
        if let Ok(mut f) = TEXT_FOCUS.lock() {
            *f = Some((id, key));
        }
    } else if let Some(bind) = key.strip_prefix("key_") {
        if let Ok(mut waiting) = LISTENING.lock() {
            *waiting = ["zoom", "freelook", "menu", "lyric_earlier", "lyric_later"].into_iter().find(|k| *k == bind);
        }
    } else if key.ends_with("color") {
        if cursor.0 >= right - 91.0 && cursor.0 < right - 81.0 {
            open_picker(id, key);
            return;
        }
        let index = ((cursor.0 - (right - 80.0)) / 10.0).floor();
        if (0.0..8.0).contains(&index) {
            set_value(id, key, index as i32);
        }
    } else if min == 0 && max == 1 {
        mark_toggle(&format!("{id}.{key}"));
        set_value(id, key, 1 - value(id, key));
    } else {
        if cursor.0 >= right - SLIDER_W - 6.0 {
            if let Ok(mut held) = SLIDER.lock() {
                *held = Some((id, key, min, max));
            }
            slide(cursor.0, right, id, key, min, max);
        }
    }
}

fn slide(at: f32, right: f32, id: &str, key: &str, min: i32, max: i32) {
    let share = ((at - (right - SLIDER_W)) / SLIDER_W).clamp(0.0, 1.0);
    set_value(id, key, min + (share * (max - min) as f32).round() as i32);
}

static GLIDES: LazyLock<Mutex<HashMap<String, (f32, Instant)>>> = LazyLock::new(Default::default);

/// Where a slider's knob is drawn: on its way to `target`. A knob not drawn lately starts there.
fn glide(name: &str, target: f32) -> f32 {
    let Ok(mut glides) = GLIDES.lock() else {
        return target;
    };
    let now = Instant::now();
    let (at, then) = glides.get(name).copied().unwrap_or((target, now));
    let dt = now.duration_since(then).as_secs_f32();
    let next = if dt > 0.25 { target } else { at + (target - at) * (1.0 - (-dt * 28.0).exp()) };
    glides.insert(name.to_owned(), (next, now));
    next
}

const SLIDER_W: f32 = 92.0;
static SLIDER: Mutex<Option<(&'static str, &'static str, i32, i32)>> = Mutex::new(None);

static LISTENING: Mutex<Option<&'static str>> = Mutex::new(None);
static TEXT_FOCUS: Mutex<Option<(&'static str, &'static str)>> = Mutex::new(None);
static TEXTS: Mutex<Vec<(String, String)>> = Mutex::new(Vec::new());
const TEXT_ROWS: [(&str, &str); 3] = [("better_chat", "text_filter"), ("freelook", "text_blocked"), ("lyrics", "text_apps_off")];

/// A text setting; comma separated lists go through here.
pub fn text_value(id: &str, key: &str) -> String {
    let name = format!("{id}.{key}");
    TEXTS.lock().ok().and_then(|t| t.iter().find(|e| e.0 == name).map(|e| e.1.clone())).unwrap_or_default()
}

fn set_text(id: &str, key: &str, value: String) {
    let name = format!("{id}.{key}");
    if let Ok(mut texts) = TEXTS.lock() {
        texts.retain(|e| e.0 != name);
        texts.push((name, value.clone()));
    }
    with(|s| s.saved.push((format!("set_{id}_{key}"), value)));
}
static MENU_KEY: Mutex<String> = Mutex::new(String::new());
static CLOSE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// A key pressed on the menu: Game hooks never see these, so the menu's own key handler calls this.
/// Returns whether the press was used.
pub fn menu_key(key: winit::keyboard::KeyCode, typed: Option<&str>) -> bool {
    // The press that opened the menu is not a press on it.
    if !text_focused() && with(|s| s.by_key && age(s) < 0.15) {
        return true;
    }
    let focus = TEXT_FOCUS.lock().ok().and_then(|f| *f);
    if let Some((id, field)) = focus {
        use winit::keyboard::KeyCode as K;
        let mut value = text_value(id, field);
        if matches!(key, K::Escape | K::Enter | K::NumpadEnter) {
            if let Ok(mut f) = TEXT_FOCUS.lock() {
                *f = None;
            }
        } else if key == K::Backspace {
            value.pop();
            set_text(id, field, value);
        } else if let Some(t) = typed.filter(|t| t.chars().all(|c| !c.is_control())) {
            if value.len() < 120 {
                value.push_str(t);
                set_text(id, field, value);
            }
        }
        return true;
    }
    let name = horizon_client_input::keybinds::key_name(key);
    if just_bound() {
        return true;
    }
    // The real keyboard's presses only come here while Options is open; their releases go to feat_key.
    if let Some(bind) = listening() {
        match name {
            Some("key.keyboard.escape") => clear_bind(bind),
            Some(n) if is_modifier(n) => hold_modifier(n),
            Some(n) => pick_bind(bind, n, HELD.load(std::sync::atomic::Ordering::Relaxed)),
            None => {}
        }
        return true;
    }
    if with(|s| s.by_key && !s.list && s.page.is_none()) {
        use winit::keyboard::KeyCode as K;
        match key {
            K::KeyA | K::ArrowLeft => step_card(-1),
            K::KeyD | K::ArrowRight => step_card(1),
            K::Enter | K::NumpadEnter => open_card(with(|s| s.selected)),
            _ => {}
        }
        if matches!(key, K::KeyA | K::ArrowLeft | K::KeyD | K::ArrowRight | K::Enter | K::NumpadEnter) {
            return true;
        }
    }
    if with(|s| s.by_key && s.list && s.page.is_none_or(|p| !p.0.ends_with("_editor"))) {
        use winit::keyboard::KeyCode as K;
        let has = with(|s| !s.query.is_empty());
        if key == K::Backspace || key == K::Escape && has {
            with(|s| {
                if key == K::Escape {
                    s.query.clear();
                } else {
                    s.query.pop();
                }
            });
            return true;
        }
        let closing = !has && is_menu_key(name);
        if let Some(t) = typed.filter(|t| !closing && t.chars().all(|c| !c.is_control()) && !t.is_empty()) {
            with(|s| {
                if s.query.len() < 24 {
                    s.query.push_str(t);
                }
                s.list = true;
            });
            return true;
        }
    }
    if with(|s| s.by_key) && is_menu_key(name) {
        CLOSE.store(true, std::sync::atomic::Ordering::Relaxed);
        return true;
    }
    false
}

/// Whether the menu key asked the ring to close.
pub fn take_close() -> bool {
    CLOSE.swap(false, std::sync::atomic::Ordering::Relaxed)
}

/// Whether this key press is the menu key, modifiers included.
fn is_menu_key(name: Option<&str>) -> bool {
    let held = HELD.load(std::sync::atomic::Ordering::Relaxed);
    name.is_some_and(|n| MENU_KEY.lock().is_ok_and(|m| *m == n)) && name.is_some_and(|n| bind_ok("menu", n, held))
}

pub fn set_menu_key(key: &str) {
    if let Ok(mut own) = MENU_KEY.lock() && *own != key {
        key.clone_into(&mut own);
    }
}
static KEY_NAMES: Mutex<Vec<(&'static str, String)>> = Mutex::new(Vec::new());

/// The mod key waiting for its next press, cleared by reading it.
fn take_listening() -> Option<&'static str> {
    LISTENING.lock().ok()?.take()
}

/// The key waiting for a new bind, if any.
pub fn listening() -> Option<&'static str> {
    *LISTENING.lock().ok()?
}

static HELD_MODIFIER: Mutex<Option<String>> = Mutex::new(None);
static BOUND_AT: Mutex<Option<Instant>> = Mutex::new(None);

/// A modifier went down while binding: it may be part of a symbol, so it waits.
pub fn hold_modifier(key: &str) {
    if let Ok(mut held) = HELD_MODIFIER.lock() {
        *held = Some(key.to_owned());
    }
}

/// True when the modifier came up with nothing pressed under it: the bind is the modifier.
pub fn released_alone(key: &str) -> bool {
    HELD_MODIFIER.lock().is_ok_and(|mut h| h.as_deref() == Some(key) && h.take().is_some())
}

pub fn is_modifier(key: &str) -> bool {
    matches!(key, "key.keyboard.left.shift" | "key.keyboard.right.shift" | "key.keyboard.left.control" | "key.keyboard.right.control" | "key.keyboard.left.alt" | "key.keyboard.right.alt")
}

static PICKED: Mutex<Option<(&'static str, &'static str)>> = Mutex::new(None);

fn pick_bind(bind: &'static str, key: &'static str, modifiers: i32) {
    finish_bind(bind, key, modifiers);
    if let Ok(mut picked) = PICKED.lock() {
        *picked = Some((bind, key));
    }
}

/// Esc while binding: clears the bind, except the menu key, which stays as it was.
fn clear_bind(bind: &'static str) {
    if bind == "menu" {
        cancel_bind();
    } else {
        pick_bind(bind, "key.keyboard.unknown", 0);
    }
}

/// The bind just made in the menu, for the frame hook to save.
pub fn take_picked() -> Option<(&'static str, &'static str)> {
    PICKED.lock().ok()?.take()
}

/// Stops listening and leaves the bind as it was.
pub fn cancel_bind() {
    take_listening();
    if let Ok(mut held) = HELD_MODIFIER.lock() {
        *held = None;
    }
    if let Ok(mut at) = BOUND_AT.lock() {
        *at = Some(Instant::now());
    }
}

/// Ends the bind. `modifiers`: 1 Shift, 2 Ctrl, 4 Alt; AltGr is Ctrl and Alt together.
pub fn finish_bind(name: &'static str, key: &str, modifiers: i32) {
    cancel_bind();
    set_value("keys", &format!("{name}_mods"), modifiers);
    set_value("keys", &format!("{name}_for"), key_mark(key));
}

/// A short mark of a key's name, so a modifier saved with one key is not applied to another.
fn key_mark(key: &str) -> i32 {
    key.bytes().fold(7_i32, |mark, b| mark.wrapping_mul(31).wrapping_add(i32::from(b))) & 0x7fff_ffff
}

/// The modifiers saved for a bind, if it still holds the key they were saved with:
/// Options > Mods can change the key without knowing about them.
fn bind_modifiers(name: &str, key: &str) -> i32 {
    if value("keys", &format!("{name}_for")) == key_mark(key) { value("keys", &format!("{name}_mods")) } else { 0 }
}

static HELD: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

/// The modifiers down this frame, for the menu's own key handling.
pub fn set_held(modifiers: i32) {
    HELD.store(modifiers, std::sync::atomic::Ordering::Relaxed);
}

static OWN_BLUR: Mutex<Option<i32>> = Mutex::new(None);

/// Your own menu blur option while Glass stands in for it.
pub fn own_blur() -> Option<i32> {
    OWN_BLUR.lock().ok().and_then(|b| *b)
}

pub fn set_own_blur(blur: Option<i32>) {
    if let Ok(mut own) = OWN_BLUR.lock() {
        *own = blur;
    }
}

/// True right after a bind ended, so that key press does nothing else.
pub fn just_bound() -> bool {
    BOUND_AT.lock().is_ok_and(|at| at.is_some_and(|t| t.elapsed().as_millis() < 80))
}

/// Whether the modifiers held match what a bind asked for. A bind without any takes all.
pub fn bind_ok(name: &str, key: &str, modifiers: i32) -> bool {
    let wanted = bind_modifiers(name, key);
    wanted == 0 || wanted == modifiers
}

/// The key a mod key is on now, for its row.
pub fn set_key_name(name: &'static str, key: &str) {
    if let Ok(mut names) = KEY_NAMES.lock() {
        names.retain(|k| k.0 != name);
        let base = key.trim_start_matches("key.keyboard.").trim_start_matches("key.mouse.").replace('.', " ");
        let prefix = match bind_modifiers(name, key) {
            1 => "Shift + ",
            2 => "Ctrl + ",
            3 => "Ctrl + Shift + ",
            4 => "Alt + ",
            5 => "Alt + Shift + ",
            6 => "AltGr + ",
            7 => "AltGr + Shift + ",
            _ => "",
        };
        names.push((name, if base == "unknown" { "None".to_owned() } else { format!("{prefix}{base}") }));
    }
}

/// The RGB color `t` of the way from `a` to `b`.
pub fn mix(a: u32, b: u32, t: f32) -> u32 {
    let at = |shift: u32| {
        let (x, y) = (((a >> shift) & 255) as f32, ((b >> shift) & 255) as f32);
        ((x + (y - x) * t) as u32) << shift
    };
    at(16) | at(8) | at(0)
}

pub fn text_focused() -> bool {
    TEXT_FOCUS.lock().is_ok_and(|f| f.is_some())
}

pub fn text_paste(pasted: &str) {
    if let Some((id, key)) = TEXT_FOCUS.lock().ok().and_then(|f| *f) {
        let clean: String = pasted.chars().filter(|c| !c.is_control()).collect();
        set_text(id, key, text_value(id, key) + &clean);
    }
}

/// What Ctrl+C takes from the focused row.
pub fn text_copy() -> String {
    TEXT_FOCUS.lock().ok().and_then(|f| *f).map_or(String::new(), |(id, key)| text_value(id, key))
}

/// The picked cape color as a vertex color, if any.
pub fn cape_tint() -> Option<u32> {
    on("tint_capes").then(|| vertex_color(color_of(value("tint_capes", "tint_color")), 255))
}

/// The cape you wear, by id.
pub fn cape() -> Option<String> {
    with(|s| s.on.iter().find(|(k, v)| **v && k.starts_with("cape_")).map(|(k, _)| k.clone()))
}

static TOGGLED: Mutex<Vec<(String, bool, Instant)>> = Mutex::new(Vec::new());

/// How far a switch has moved since its last toggle, 0 to 1.
fn toggle_share(id: &str) -> f32 {
    TOGGLED.lock().ok().and_then(|t| t.iter().find(|e| e.0 == id).map(|e| (e.2.elapsed().as_secs_f32() / 0.14).min(1.0))).unwrap_or(1.0)
}

/// Starts a switch's slide.
fn mark_toggle(name: &str) {
    if let Ok(mut toggled) = TOGGLED.lock() {
        toggled.retain(|e| e.0 != name && e.2.elapsed().as_secs_f32() < 1.0);
        toggled.push((name.to_owned(), true, Instant::now()));
    }
}

fn toggle(id: &str) {
    mark_toggle(id);
    if id.starts_with("theme_") && id != "theme_colors" {
        with(|s| {
            for (other, _, _) in THEMES {
                s.on.insert(other.to_owned(), other == id);
                s.saved.push((format!("on_{other}"), (other == id).to_string()));
            }
        });
        return;
    }
    let enabled = !on(id);
    with(|s| {
        if enabled && id.starts_with("cape_") {
            // One cape at a time.
            let others: Vec<String> = s.on.iter().filter(|(k, v)| **v && k.starts_with("cape_")).map(|(k, _)| k.clone()).collect();
            for other in others {
                s.on.insert(other.clone(), false);
                s.saved.push((format!("on_{other}"), "false".to_owned()));
            }
        }
        s.on.insert(id.to_owned(), enabled);
        s.saved.push((format!("on_{id}"), enabled.to_string()));
    });
}

// ---- Themes: colors in the order background, panels, lines, text, muted, selected, selected text, accent.
const THEMES: [(&str, &str, [u32; 8]); 11] = [
    ("theme_classic", "Classic", [0xcdc8b0, 0xdad5bd, 0xa8a28b, 0x4b483e, 0x7b7665, 0x4b483e, 0xdad5bd, 0x4b483e]),
    ("theme_obsidian", "Obsidian", [0x0e0e12, 0x1a1a20, 0x3a3a44, 0xe8e2d0, 0x8d8a80, 0xd8b65a, 0x14120c, 0xd8b65a]),
    ("theme_crimson", "Crimson", [0x120609, 0x220c10, 0x5a1a22, 0xf2e4e0, 0xa8787a, 0xd0202c, 0xfff4f0, 0xff3b3b]),
    ("theme_frost", "Frost", [0xc6d4dc, 0xdbe5ea, 0x9db0bc, 0x2b3f52, 0x66808f, 0x2b3f52, 0xe6eff3, 0x3a86c8]),
    ("theme_sakura", "Sakura", [0xecd3d6, 0xf5e5e6, 0xcfa8b0, 0x5a2a40, 0x9a6a7c, 0x5a2a40, 0xfbeef0, 0xd8407a]),
    ("theme_glass", "Glass", [0x1a1e24, 0x20262e, 0x7a8490, 0xf2f5f8, 0xb4bec8, 0xf2f5f8, 0x14181e, 0x7fd0ff]),
    ("theme_nebula", "Nebula", [0x0a0820, 0x171238, 0x3a2e6e, 0xe8e4ff, 0x9088c0, 0x7a4cff, 0xffffff, 0xff5cc8]),
    ("theme_ember", "Ember", [0x0e0604, 0x1e0e08, 0x5a2a14, 0xffe8d0, 0xb08868, 0xff6a14, 0x1a0800, 0xffb02e]),
    ("theme_aurora", "Aurora", [0x04101a, 0x0a1e28, 0x1e4a50, 0xdcfff0, 0x7ab0a0, 0x18d890, 0x04140e, 0xb060ff]),
    ("theme_petals", "Petals", [0x2a1224, 0x3a1a30, 0x6e3a58, 0xffe6ee, 0xc090a8, 0xff7aa8, 0x2a0a1a, 0xffc0d8]),
    ("theme_signal", "Signal", [0xcdc8b0, 0xdad5bd, 0xa8a28b, 0x4b483e, 0x7b7665, 0x4b483e, 0xdad5bd, 0xb8452f]),
];
const GLASS: usize = 5;
const FIRST_ANIMATED: usize = 6;
const COLOR_KEYS: [&str; 8] = ["paper_color", "panel_color", "line_color", "text_color", "muted_color", "selected_color", "selected_text_color", "accent_color"];
static PALETTE: Mutex<[u32; 8]> = Mutex::new(THEMES[0].2);
static LOOK: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

fn theme() -> usize {
    THEMES.iter().position(|t| on(t.0)).unwrap_or(0)
}

/// Reads the theme once a frame, with any custom colors over it.
fn refresh_palette() {
    let index = theme();
    let mut colors = THEMES[index].2;
    if on("theme_colors") {
        for (slot, key) in COLOR_KEYS.iter().enumerate() {
            let v = value("theme_colors", key);
            // -1 keeps the theme's own color.
            if v >= 0 {
                colors[slot] = color_of(v);
            }
        }
    }
    LOOK.store(index, std::sync::atomic::Ordering::Relaxed);
    if let Ok(mut p) = PALETTE.lock() {
        *p = colors;
    }
}

fn tok(slot: usize) -> u32 {
    PALETTE.lock().map_or(0, |p| p[slot])
}

fn ca(slot: usize, alpha: u8) -> u32 {
    vertex_color(tok(slot), alpha)
}

/// An RGB color as a vertex color.
pub fn vertex_color(c: u32, alpha: u8) -> u32 {
    rgba((c >> 16) as u8, (c >> 8) as u8, c as u8, alpha)
}

fn look() -> usize {
    LOOK.load(std::sync::atomic::Ordering::Relaxed)
}

fn sunk() -> u32 { ca(0, 255) }
fn line() -> u32 { ca(2, 255) }
fn ink() -> u32 { ca(3, 255) }
fn soft() -> u32 { ca(4, 255) }
fn sel() -> u32 { ca(5, 255) }
fn seltxt() -> u32 { ca(6, 255) }
fn accent() -> u32 { ca(7, 255) }

/// Panels thin out over Glass and the animated backgrounds, so what is behind shows.
fn panel() -> u32 {
    let l = look();
    ca(1, if l == GLASS { 150 } else if l >= FIRST_ANIMATED { 226 } else { 255 })
}

/// How opaque HUD element backgrounds are.
const PANEL_ALPHA: u8 = 0xd8;

/// An element's background: the theme's panel color, as 0xAARRGGBB.
pub fn hud_panel() -> u32 {
    u32::from(PANEL_ALPHA) << 24 | tok(1)
}

/// For elements the game draws itself: paints the six vertices held back under them as the
/// theme's panel, and turns their white text to the theme's text.
pub fn back_panel(vertices: &mut [crate::hud::HudVertex], [x, y, w, h]: [f32; 4]) {
    let (c, text) = (tok(1), tok(3));
    let color = vertex_color(c, PANEL_ALPHA);
    let corners = [[x, y], [x + w, y], [x + w, y + h], [x, y + h]];
    for (v, i) in vertices.iter_mut().zip([0, 1, 2, 0, 2, 3]) {
        v.pos = corners[i];
        v.color = color;
    }
    for v in vertices.iter_mut().skip(6) {
        if v.color & 0x00ff_ffff == 0x00ff_ffff {
            v.color = v.color & 0xff00_0000 | vertex_color(text, 0);
        }
    }
}

/// Default text on a background takes the theme's text color; a color you picked stays.
pub fn panel_text(panel: bool, chosen: i32, default: u32) -> u32 {
    if chosen >= 0 { color_of(chosen) } else if panel { tok(3) } else { default }
}

/// Default values on a background take the theme's accent; a color you picked stays.
pub fn panel_value(panel: bool, chosen: i32, default: u32) -> u32 {
    if chosen >= 0 { color_of(chosen) } else if panel { tok(7) } else { default }
}

/// Blur for the frame hook while Glass is open.
pub fn glass_blur() -> Option<i32> {
    (by_key() && theme() == GLASS).then(|| value("theme_glass", "blur"))
}

// ---- Shapes: vertex colors do the gradients, so everything stays smooth at any size.
fn white_uv(batch: &mut HudBatch, atlas: &HudAtlas) -> [f32; 2] {
    let at = batch.vertices.len();
    batch.fill(atlas, 0.0, 0.0, 0.0, 0.0, 0);
    let uv = batch.vertices[at].uv;
    batch.vertices.truncate(at);
    uv
}

fn shade(batch: &mut HudBatch, uv: [f32; 2], p: [[f32; 2]; 4], c: [u32; 4]) {
    for i in [0, 1, 2, 0, 2, 3] {
        batch.vertices.push(crate::hud::HudVertex { pos: p[i], uv, color: c[i], depth: 0.0, glint_uv: [0.0, 0.0], text: 0 });
    }
}

/// An RGB color at a brightness, for additive drawing, which ignores alpha.
fn lit(rgb: u32, a: f32) -> u32 {
    let k = a.clamp(0.0, 1.0);
    let f = |s: u32| (((rgb >> s) & 255) as f32 * k) as u8;
    rgba(f(16), f(8), f(0), (k * 255.0) as u8)
}

/// A soft round light. Additive: draw inside an additive run.
fn glow(batch: &mut HudBatch, uv: [f32; 2], x: f32, y: f32, r: f32, rgb: u32, a: f32) {
    let segments = if r < 14.0 { 8 } else { 18 };
    let (c0, c1, c2) = (lit(rgb, a), lit(rgb, a * 0.42), lit(rgb, 0.0));
    let at = |radius: f32, i: usize| {
        let d = i as f32 * std::f32::consts::TAU / segments as f32;
        [x + radius * d.cos(), y + radius * d.sin()]
    };
    for i in 0..segments {
        let (m0, m1, o0, o1) = (at(r * 0.34, i), at(r * 0.34, i + 1), at(r, i), at(r, i + 1));
        shade(batch, uv, [[x, y], m0, m1, [x, y]], [c0, c1, c1, c0]);
        shade(batch, uv, [m0, o0, o1, m1], [c1, c2, c2, c1]);
    }
}

fn disc(batch: &mut HudBatch, uv: [f32; 2], x: f32, y: f32, r: f32, color: u32) {
    const SEGMENTS: usize = 10;
    let at = |i: usize| {
        let d = i as f32 * std::f32::consts::TAU / SEGMENTS as f32;
        [x + r * d.cos(), y + r * d.sin()]
    };
    for i in 0..SEGMENTS {
        shade(batch, uv, [[x, y], at(i), at(i + 1), [x, y]], [color; 4]);
    }
}

fn stroke(batch: &mut HudBatch, uv: [f32; 2], a: [f32; 2], b: [f32; 2], width: f32, c0: u32, c1: u32) {
    let (dx, dy) = (b[0] - a[0], b[1] - a[1]);
    let n = (dx * dx + dy * dy).sqrt().max(0.001);
    let (px, py) = (-dy / n * width / 2.0, dx / n * width / 2.0);
    shade(batch, uv, [[a[0] + px, a[1] + py], [b[0] + px, b[1] + py], [b[0] - px, b[1] - py], [a[0] - px, a[1] - py]], [c0, c1, c1, c0]);
}

fn arc(batch: &mut HudBatch, uv: [f32; 2], c: (f32, f32), r: f32, from: f32, to: f32, width: f32, color: u32) {
    let steps = ((to - from).abs() * r / 4.0).ceil().max(2.0) as usize;
    for i in 0..steps {
        let (a, b) = (from + (to - from) * i as f32 / steps as f32, from + (to - from) * (i + 1) as f32 / steps as f32);
        stroke(batch, uv, [c.0 + r * a.cos(), c.1 + r * a.sin()], [c.0 + r * b.cos(), c.1 + r * b.sin()], width, color, color);
    }
}

fn hsh(i: usize, k: usize) -> f32 {
    let v = ((i as f32 * 127.1 + k as f32 * 311.7).sin() * 43758.547).abs();
    v - v.floor()
}

fn clock() -> f32 {
    static START: LazyLock<Instant> = LazyLock::new(Instant::now);
    START.elapsed().as_secs_f32()
}

fn fade_since(batch: &mut HudBatch, start: usize, alpha: f32) {
    for v in &mut batch.vertices[start..] {
        let a = (((v.color >> 24) & 255) as f32 * alpha) as u32;
        v.color = v.color & 0x00ff_ffff | a << 24;
    }
}

// ---- Animated backgrounds.
fn additive(batch: &mut HudBatch, f: impl FnOnce(&mut HudBatch)) {
    let start = batch.vertices.len();
    f(batch);
    batch.blend_since(start, crate::hud::Blend::Additive);
}

fn fx_nebula(batch: &mut HudBatch, uv: [f32; 2], t: f32, w: f32, h: f32, s: f32) {
    let (star, a, b) = (tok(3), tok(7), tok(5));
    additive(batch, |batch| {
        for i in 0..8 {
            let fi = i as f32;
            let x = w * (0.5 + 0.48 * (t * 0.07 + fi * 1.9).sin());
            let y = h * (0.5 + 0.46 * (t * 0.05 + fi * 2.6).cos());
            glow(batch, uv, x, y, w.max(h) * (0.3 + 0.1 * (t * 0.13 + fi).sin()), if i % 2 == 0 { b } else { a }, 0.2);
        }
        for i in 0..300 {
            let z = hsh(i, 5);
            let x = (hsh(i, 1) * w - t * (4.0 + z * 16.0) * s).rem_euclid(w);
            let y = hsh(i, 2) * h;
            let k = 0.3 + 0.7 * (t * (0.6 + hsh(i, 3) * 2.0) + i as f32).sin().abs();
            disc(batch, uv, x, y, (0.45 + z * 0.8) * s, lit(star, k));
            if i % 14 == 0 {
                glow(batch, uv, x, y, 8.0 * s * (0.6 + z), if i % 28 == 0 { a } else { star }, k * 0.7);
            }
        }
        let n = (t / 3.0).floor();
        let p = t / 3.0 - n;
        if p < 0.22 {
            let i = n as usize;
            let (x, y) = (w * (0.3 + 0.8 * hsh(i, 1)) - p * w * 1.8, h * 0.55 * hsh(i, 2) + p * h * 1.2);
            let fade = 1.0 - p / 0.22 * 0.6;
            stroke(batch, uv, [x, y], [x + 130.0 * s, y - 65.0 * s], 1.4 * s, lit(star, fade), lit(star, 0.0));
            glow(batch, uv, x, y, 9.0 * s, star, fade);
        }
    });
}

fn fx_ember(batch: &mut HudBatch, uv: [f32; 2], t: f32, w: f32, h: f32, s: f32) {
    let (hot, deep) = (tok(7), tok(5));
    additive(batch, |batch| {
        for i in 0..46 {
            let fi = i as f32;
            let p = (hsh(i, 2) + t * (0.1 + hsh(i, 1) * 0.12)).fract();
            let x = w * hsh(i, 3) + w * 0.03 * (t * (0.7 + hsh(i, 4)) + fi).sin();
            let flicker = 0.8 + 0.2 * (t * 7.0 + fi).sin();
            glow(batch, uv, x, h * (1.1 - p * 0.62), h * (0.3 - 0.22 * p), if p < 0.45 { hot } else { deep }, 0.17 * (1.0 - p) * flicker);
        }
        for i in 0..240 {
            let fi = i as f32;
            let p = (hsh(i, 7) + t * (0.06 + hsh(i, 6) * 0.17)).fract();
            let x = hsh(i, 8) * w + w * 0.018 * (t * (0.9 + hsh(i, 9) * 2.0) + fi).sin() + p * w * 0.05;
            let y = h * (1.04 - p * 1.1);
            let z = (0.8 + hsh(i, 10) * 1.8) * s * (1.0 - p * 0.6);
            let a = (1.0 - p).powf(1.3);
            glow(batch, uv, x, y, z * 5.0, deep, a * 0.55);
            disc(batch, uv, x, y, z * 0.8, lit(hot, a));
        }
    });
}

fn fx_aurora(batch: &mut HudBatch, atlas: &HudAtlas, uv: [f32; 2], t: f32, w: f32, h: f32, s: f32) {
    batch.fill_gradient(atlas, (0.0, 0.0, w, h), ca(0, 255), ca(2, 255));
    let (star, a, b) = (tok(3), tok(7), tok(5));
    additive(batch, |batch| {
        for i in 0..190 {
            disc(batch, uv, hsh(i, 1) * w, hsh(i, 2) * h * 0.8, (0.4 + hsh(i, 3) * 0.8) * s, lit(star, 0.25 + 0.55 * (t * 0.8 + i as f32).sin().abs()));
        }
        let step = (w / 300.0).max(1.5);
        for band in 0..3 {
            let fb = band as f32;
            let color = if band == 1 { a } else { b };
            let mut x = -step;
            while x < w + step {
                let u = x / w;
                let y = h * (0.02 + 0.17 * fb) + h * 0.075 * (u * 5.0 + t * 0.35 + fb * 2.0).sin() + h * 0.032 * (u * 13.0 - t * 0.6).sin();
                let l = h * (0.62 + 0.14 * (u * 9.0 + t * 0.5 + fb).sin());
                let k = ((0.06 + 0.05 * (u * 31.0 + t * 1.1 + fb).sin() + 0.035 * (u * 77.0 - t * 1.7).sin()) * (0.65 + 0.35 * (u * 3.0 + t * 0.3 + fb).sin()) * 2.6).max(0.0);
                let (top, mid) = (lit(color, 0.0), lit(color, k));
                let split = y + l * 0.12;
                shade(batch, uv, [[x, y], [x + step, y], [x + step, split], [x, split]], [top, top, mid, mid]);
                shade(batch, uv, [[x, split], [x + step, split], [x + step, y + l], [x, y + l]], [mid, mid, top, top]);
                x += step;
            }
        }
    });
}

fn fx_petals(batch: &mut HudBatch, uv: [f32; 2], t: f32, w: f32, h: f32, s: f32) {
    let (a, b) = (ca(5, 255), ca(7, 255));
    let (ga, gb) = (tok(5), tok(7));
    additive(batch, |batch| {
        glow(batch, uv, w * 0.76, h * 0.22, h * 0.72, ga, 0.2);
        glow(batch, uv, w * 0.16, h * 0.92, h * 0.58, gb, 0.11);
    });
    let gust = w * (0.05 * (t * 0.32).sin() + 0.03 * (t * 0.85).sin());
    for i in 0..120 {
        let fi = i as f32;
        let d = 0.35 + 0.65 * hsh(i, 5);
        let p = (hsh(i, 2) + t * (0.035 + hsh(i, 1) * 0.05) * (0.6 + d * 0.6)).fract();
        let x = (hsh(i, 3) * w + gust * (1.0 + p * 3.0) * d + p * w * 0.14 + w * 0.012 * (t * 1.2 + fi).sin()).rem_euclid(w);
        let y = p * (h + 30.0 * s) - 15.0 * s;
        let z = (3.5 + hsh(i, 6) * 6.0) * s * d;
        let turn = t * (1.0 + hsh(i, 7) * 2.0) + fi;
        let (sn, cs) = (turn * 0.7).sin_cos();
        let flat = 0.28 + 0.72 * turn.cos().abs();
        let at = |px: f32, py: f32| {
            let qy = py * flat;
            [x + px * cs - qy * sn, y + px * sn + qy * cs]
        };
        let base = if i % 3 == 0 { b } else { a };
        let color = base & 0x00ff_ffff | (((0.32 + 0.62 * d) * 255.0) as u32) << 24;
        // A leaf shape: pointed at both ends, widest above the middle.
        let outline = [(0.0, -1.0), (0.42, -0.72), (0.5, -0.25), (0.34, 0.3), (0.0, 1.0), (-0.34, 0.3), (-0.5, -0.25), (-0.42, -0.72)];
        let center = at(0.0, -0.1 * z);
        for k in 0..outline.len() {
            let (p0, p1) = (outline[k], outline[(k + 1) % outline.len()]);
            shade(batch, uv, [center, at(p0.0 * z, p0.1 * z), at(p1.0 * z, p1.1 * z), center], [color; 4]);
        }
    }
}

fn fx_signal(batch: &mut HudBatch, atlas: &HudAtlas, uv: [f32; 2], t: f32, w: f32, h: f32, s: f32) {
    let grid = 40.0 * s;
    let (nx, ny) = ((w / grid).ceil() as usize, (h / grid).ceil() as usize);
    for i in 0..=nx {
        batch.fill(atlas, i as f32 * grid, 0.0, 0.5, h, ca(2, 110));
    }
    for j in 0..=ny {
        batch.fill(atlas, 0.0, j as f32 * grid, w, 0.5, ca(2, 110));
    }
    for i in 0..70 {
        let k = (t * (0.5 + hsh(i, 4)) + i as f32 * 2.1).sin().max(0.0).powi(3);
        let color = if i % 7 == 0 { ca(7, (k * 180.0) as u8) } else { ca(3, (k * 44.0) as u8) };
        batch.fill(atlas, (hsh(i, 2) * nx as f32).floor() * grid + 0.5, (hsh(i, 3) * ny as f32).floor() * grid + 0.5, grid - 0.5, grid - 0.5, color);
    }
    let y = ((t * 0.11) % 1.2 - 0.1) * h;
    batch.fill_gradient(atlas, (0.0, y - 80.0 * s, w, 80.0 * s), ca(7, 0), ca(7, 80));
    batch.fill(atlas, 0.0, y, w, 0.75, accent());
    let (cx, cy, r) = (w * 0.84, h * 0.74, w.min(h) * 0.24);
    let an = t * 0.9;
    for k in 0..48 {
        let (a0, a1) = (an - 1.1 * k as f32 / 48.0, an - 1.1 * (k + 1) as f32 / 48.0);
        let (c0, c1) = (ca(7, (110.0 * (1.0 - k as f32 / 48.0)) as u8), ca(7, (110.0 * (1.0 - (k + 1) as f32 / 48.0)) as u8));
        shade(batch, uv, [[cx, cy], [cx + r * a0.cos(), cy + r * a0.sin()], [cx + r * a1.cos(), cy + r * a1.sin()], [cx, cy]], [c0, c0, c1, c1]);
    }
    for m in [0.33, 0.66, 1.0] {
        arc(batch, uv, (cx, cy), r * m, 0.0, std::f32::consts::TAU, 0.6, ca(3, 140));
    }
    for i in 0..72 {
        let d = i as f32 * std::f32::consts::TAU / 72.0;
        let o = r * if i % 6 == 0 { 1.07 } else { 1.03 };
        stroke(batch, uv, [cx + r * d.cos(), cy + r * d.sin()], [cx + o * d.cos(), cy + o * d.sin()], 0.6, ca(3, 140), ca(3, 140));
    }
    stroke(batch, uv, [cx, cy], [cx + r * an.cos(), cy + r * an.sin()], 0.9, accent(), accent());
    for i in 0..24 {
        let fi = i as f32;
        let b = (13.0 + 11.0 * (t * 2.4 + fi * 0.6).sin() + 7.0 * (t * 4.0 + fi).sin()) * s;
        batch.fill(atlas, (22.0 + fi * 6.5) * s, h - 18.0 * s - b, 3.6 * s, b, ca(3, 190));
    }
}

/// What sits behind the menu.
fn backdrop(batch: &mut HudBatch, atlas: &HudAtlas, home: bool) {
    let (w, h) = (batch.width, batch.height);
    let l = look();
    if l == GLASS {
        batch.fill(atlas, 0.0, 0.0, w, h, rgba(0, 0, 0, (value("theme_glass", "dim") as f32 * 2.55) as u8));
    } else if l >= FIRST_ANIMATED {
        batch.fill(atlas, 0.0, 0.0, w, h, sunk());
        let (uv, t, s) = (white_uv(batch, atlas), clock(), w.max(h) / 1100.0 * 1.6);
        match l {
            6 => fx_nebula(batch, uv, t, w, h, s),
            7 => fx_ember(batch, uv, t, w, h, s),
            8 => fx_aurora(batch, atlas, uv, t, w, h, s),
            9 => fx_petals(batch, uv, t, w, h, s),
            _ => fx_signal(batch, atlas, uv, t, w, h, s),
        }
    } else if home {
        batch.fill(atlas, 0.0, 0.0, w, h, ca(0, 190));
    } else {
        batch.fill(atlas, 0.0, 0.0, w, h, sunk());
        let mut x = 0.0;
        while x < w {
            batch.fill(atlas, x, 0.0, 0.5, h, ca(2, 46));
            x += 8.0;
        }
        let mut y = 0.0;
        while y < h {
            batch.fill(atlas, 0.0, y, w, 0.5, ca(2, 46));
            y += 8.0;
        }
    }
}

pub fn draw(batch: &mut HudBatch, atlas: &HudAtlas, cursor: (f32, f32)) {
    refresh_palette();
    let page = with(|s| s.page);
    let full = page.is_some_and(|p| p.0.ends_with("_editor"));
    if !editing() {
        backdrop(batch, atlas, !full && !with(|s| s.list));
    }
    if let Some(page) = page.filter(|_| full) {
        draw_settings(batch, atlas, cursor, page);
    } else if with(|s| s.list) {
        draw_page(batch, atlas, cursor);
    } else {
        draw_deck(batch, atlas, cursor);
    }
    draw_picker(batch, atlas);
}

fn big(batch: &mut HudBatch, atlas: &HudAtlas, value: &str, x: f32, y: f32, scale: f32, color: u32) {
    let start = batch.vertices.len();
    text(batch, atlas, value, x, y, color, false);
    batch.scale_since(start, x, y, scale, scale);
}

// ---- The start screen: cards you swipe through.
const CARD_W: f32 = 158.0;
const CARD_H: f32 = 192.0;
const CARD_GAP: f32 = 16.0;
const CARDS: i32 = 6;
const EMBLEMS: [&str; 6] = ["ui_hud", "ui_pvp", "ui_visual", "ui_client", "ui_capes", "ui_themes"];
static DECK: Mutex<(f32, Option<Instant>)> = Mutex::new((2.0, None));
static SWIPE: Mutex<Option<(f32, f32)>> = Mutex::new(None);

fn wrap(offset: f32) -> f32 {
    (offset + 3.0).rem_euclid(CARDS as f32) - 3.0
}

fn deck_scale(height: f32) -> f32 {
    ((height - 74.0) / CARD_H).min(1.0)
}

fn swipe_offset() -> f32 {
    SWIPE.lock().ok().and_then(|s| *s).map_or(0.0, |(from, at)| at - from)
}

fn deck_position() -> f32 {
    let target = with(|s| s.selected) as f32;
    let Ok(mut deck) = DECK.lock() else {
        return target;
    };
    let now = Instant::now();
    let dt = deck.1.map_or(1.0, |t| now.duration_since(t).as_secs_f32()).min(0.1);
    deck.1 = Some(now);
    deck.0 = (deck.0 + wrap(target - deck.0) * (1.0 - (-dt * 15.0).exp())).rem_euclid(CARDS as f32);
    deck.0
}

fn card_count(index: usize) -> String {
    if index == 5 {
        return THEMES[look()].1.to_owned();
    }
    let rows = MODS[index].iter().filter(|m| m.0 != "#" && !m.0.ends_with("_editor") && m.0 != "ring_menu");
    let (all, active) = rows.fold((0, 0), |(a, b), m| (a + 1, b + usize::from(on(m.0))));
    format!("{active} / {all}")
}

fn card_at(cursor: (f32, f32), size: (f32, f32)) -> Option<usize> {
    let k = deck_scale(size.1);
    let (cx, cy) = (size.0 / 2.0, size.1 / 2.0 + 8.0);
    let selected = with(|s| s.selected);
    let dy = (cursor.1 - cy).abs();
    if dy > CARD_H * k / 2.0 {
        return None;
    }
    let step = (CARD_W + CARD_GAP) * k;
    let dx = cursor.0 - cx;
    if dx.abs() < CARD_W * k / 2.0 {
        Some(selected)
    } else if dx.abs() < step + CARD_W * k / 2.0 {
        Some((selected as i32 + dx.signum() as i32).rem_euclid(CARDS) as usize)
    } else {
        None
    }
}

fn draw_card(batch: &mut HudBatch, atlas: &HudAtlas, index: usize, x: f32, y: f32, selected: bool) {
    let uv = white_uv(batch, atlas);
    batch.fill(atlas, x + 3.0, y + 3.0, CARD_W, CARD_H, if selected { accent() } else { line() });
    batch.fill(atlas, x, y, CARD_W, CARD_H, panel());
    let head = 28.0;
    batch.fill(atlas, x, y, CARD_W, head, if selected { sel() } else { ca(1, 0) });
    batch.fill(atlas, x, y + head, CARD_W, 0.5, line());
    let title = if selected { seltxt() } else { ink() };
    let name = ["HUD", "PvP", "Visual", "Client", "Capes", "Themes"][index];
    big(batch, atlas, name, x + 9.0, y + 7.0, 1.75, title);
    let count = card_count(index);
    text(batch, atlas, &count, x + CARD_W - 8.0 - HudBatch::text_width(atlas, &count), y + 11.0, title, false);
    let c = (x + CARD_W / 2.0, y + head + (CARD_H - head - 14.0) / 2.0 + 1.0);
    let spin = if selected { clock() * 0.14 } else { 0.0 };
    let tau = std::f32::consts::TAU;
    let r = 66.0;
    for i in 0..120 {
        let d = i as f32 / 120.0 * tau + spin;
        let o = r + if i % 10 == 0 { 5.0 } else { 2.5 };
        stroke(batch, uv, [c.0 + r * d.cos(), c.1 + r * d.sin()], [c.0 + o * d.cos(), c.1 + o * d.sin()], 0.4, ink(), ink());
    }
    for from in [-1.2, 1.9] {
        arc(batch, uv, c, r - 6.0, from + spin, from + 1.05 + spin, 2.4, ink());
    }
    for from in [0.35, 3.5] {
        arc(batch, uv, c, r - 11.0, from - spin * 1.6, from + 0.95 - spin * 1.6, 2.0, ink());
    }
    let mut d = -spin * 1.6;
    while d < tau - spin * 1.6 {
        arc(batch, uv, c, r - 14.5, d, d + 0.05, 0.45, soft());
        d += 0.12;
    }
    for sign in [-1.0, 1.0] {
        let (bx, by) = (c.0 + sign * (CARD_W / 2.0 - 8.0), y + head + 7.0);
        batch.fill(atlas, bx.min(bx - sign * 10.0), by, 10.0, 0.6, ink());
        batch.fill(atlas, bx, by, 0.6, 10.0, ink());
        let ey = y + CARD_H - 21.0;
        batch.fill(atlas, bx.min(bx - sign * 10.0), ey + 10.0, 10.0, 0.6, ink());
        batch.fill(atlas, bx, ey, 0.6, 10.0, ink());
    }
    if let Some(r) = atlas.sprites.get(format!("kyro_capes:entity/{}", EMBLEMS[index]).as_str()).copied() {
        let size = 96.0;
        batch.rect_uv(c.0 - size / 2.0, c.1 - size / 2.0, size, size, r, if selected { ink() } else { soft() });
    }
    let open = "Open";
    text(batch, atlas, open, x + CARD_W - 8.0 - HudBatch::text_width(atlas, open), y + CARD_H - 12.0, soft(), false);
}

fn draw_deck(batch: &mut HudBatch, atlas: &HudAtlas, _cursor: (f32, f32)) {
    let (w, h) = (batch.width, batch.height);
    let k = deck_scale(h);
    let (cx, cy) = (w / 2.0, h / 2.0 + 8.0);
    let top = (cy - CARD_H * k / 2.0 - 30.0).max(4.0);
    let title = "K Y R O";
    big(batch, atlas, title, cx - HudBatch::text_width(atlas, title), top, 2.0, ink());
    let sub = format!("{} / 26.3", own_name());
    centered(batch, atlas, if own_name().is_empty() { "26.3" } else { &sub }, cx, top + 19.0, soft());
    let position = deck_position();
    let step = (CARD_W + CARD_GAP) * k;
    let drag = swipe_offset() / step;
    let mut order: Vec<(usize, f32)> = (0..CARDS as usize).map(|i| (i, wrap(i as f32 - position) + drag)).filter(|c| c.1.abs() < 1.75).collect();
    order.sort_by(|a, b| b.1.abs().total_cmp(&a.1.abs()));
    for (index, offset) in order {
        let start = batch.vertices.len();
        let near = offset.abs().min(1.0);
        let (x, y) = (cx + offset * step, cy);
        draw_card(batch, atlas, index, x - CARD_W / 2.0, y - CARD_H / 2.0, offset.abs() < 0.5);
        let scale = k * (1.0 - 0.14 * near);
        batch.scale_since(start, x, y, scale, scale);
        fade_since(batch, start, 1.0 - 0.5 * near);
    }
    let selected = with(|s| s.selected);
    let dots = cy + CARD_H * k / 2.0 + 9.0;
    for i in 0..CARDS as usize {
        batch.fill(atlas, cx - 33.0 + i as f32 * 12.0, dots, 7.0, 7.0, if i == selected { ink() } else { line() });
    }
    centered(batch, atlas, "Swipe or A / D     Enter opens", cx, dots + 12.0, ink());
}

fn open_card(index: usize) {
    // The deck waits here, so coming back shows this card without sliding to it.
    if let Ok(mut deck) = DECK.lock() {
        deck.0 = index as f32;
    }
    with(|s| {
        s.selected = index;
        s.list = true;
        s.page = None;
        s.scroll = 0.0;
        s.opened = Some(Instant::now());
    });
}

fn step_card(by: i32) {
    with(|s| s.selected = (s.selected as i32 + by).rem_euclid(CARDS) as usize);
}

// ---- The menu page: tabs on top, rows on the left, the sheet on the right.
const PAGE_W: f32 = 572.0;
const ROW_H: f32 = 15.0;
const PAGE_TOP: f32 = 46.0;
const SHEET_GAP: f32 = 10.0;

fn page_scale(size: (f32, f32)) -> f32 {
    ((size.0 - 10.0) / PAGE_W).min(1.0)
}

fn page_rect(size: (f32, f32)) -> [f32; 4] {
    let k = page_scale(size);
    let h = (size.1 / k - 14.0).min(430.0);
    [(size.0 - PAGE_W) / 2.0, (size.1 - h) / 2.0, PAGE_W, h]
}

fn page_cursor(c: (f32, f32), size: (f32, f32)) -> (f32, f32) {
    let k = page_scale(size);
    ((c.0 - size.0 / 2.0) / k + size.0 / 2.0, (c.1 - size.1 / 2.0) / k + size.1 / 2.0)
}

fn rows_rect(size: (f32, f32)) -> [f32; 4] {
    let [x, y, w, h] = page_rect(size);
    let top = if shows_banner() { BANNER_H + 6.0 } else { 0.0 };
    [x, y + PAGE_TOP + top, w - SETTINGS_W - SHEET_GAP, h - PAGE_TOP - 12.0 - top]
}

const BANNER_H: f32 = 40.0;

fn shows_banner() -> bool {
    with(|s| s.selected == 0 && s.query.is_empty())
}

fn banner_rect(size: (f32, f32)) -> [f32; 4] {
    let [x, y, w, _] = page_rect(size);
    [x, y + PAGE_TOP, w - SETTINGS_W - SHEET_GAP - 6.0, BANNER_H]
}

fn draw_banner(batch: &mut HudBatch, atlas: &HudAtlas, c: (f32, f32), size: (f32, f32)) {
    let [x, y, w, h] = banner_rect(size);
    let hot = inside(c, [x, y, w, h]);
    let lift = if hot { 2.0 } else { 0.0 };
    batch.fill(atlas, x + 3.0, y + 3.0, w, h, accent());
    batch.fill(atlas, x - lift / 2.0, y - lift / 2.0, w + lift, h + lift, sel());
    if let Some(r) = atlas.sprites.get("kyro_capes:entity/ui_hud").copied() {
        batch.rect_uv(x + 4.0, y + 2.0, 36.0, 36.0, r, seltxt());
    }
    big(batch, atlas, "HUD EDITOR", x + 44.0, y + 8.0, 1.4, seltxt());
    text(batch, atlas, "Move, resize and restyle your HUD", x + 44.0, y + 24.0, seltxt(), false);
    big(batch, atlas, ">", x + w - 16.0, y + 13.0, 1.6, seltxt());
}

fn sheet_area(size: (f32, f32)) -> [f32; 4] {
    let [x, y, w, h] = page_rect(size);
    [x + w - SETTINGS_W, y + PAGE_TOP, SETTINGS_W, h - PAGE_TOP - 12.0]
}

fn tab_rect(index: usize, size: (f32, f32)) -> [f32; 4] {
    let [x, y, w, _] = page_rect(size);
    let tw = (w - 5.0 * 4.0) / 6.0;
    [x + index as f32 * (tw + 4.0), y + 24.0, tw, 15.0]
}

fn row_at(cursor: (f32, f32), size: (f32, f32), shown: &[(&'static str, &'static str, bool)]) -> Option<usize> {
    let [x, y, w, h] = rows_rect(size);
    if !inside(cursor, [x, y, w, h]) {
        return None;
    }
    let scroll = with(|s| s.scroll);
    let row = ((cursor.1 - y + scroll) / ROW_H).floor() as usize;
    (row < shown.len() && shown[row].0 != "#").then_some(row)
}

static SEARCH_FOCUS: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

fn search_rect(size: (f32, f32)) -> [f32; 4] {
    let [x, y, w, _] = page_rect(size);
    [x + w - 130.0, y + 7.0, 130.0, 13.0]
}

fn sheet_busy() -> bool {
    text_focused() || SLIDER.lock().is_ok_and(|h| h.is_some()) || LISTENING.lock().is_ok_and(|l| l.is_some()) || PICKER.lock().is_ok_and(|p| p.is_some())
}

fn draw_page(batch: &mut HudBatch, atlas: &HudAtlas, cursor: (f32, f32)) {
    let size = (batch.width, batch.height);
    let k = page_scale(size);
    let c = page_cursor(cursor, size);
    let start = batch.vertices.len();
    let [x, y, w, h] = page_rect(size);
    let (category, query) = with(|s| (s.selected, s.query.clone()));
    let mut tx = x;
    while tx < x + w {
        batch.fill(atlas, tx, y, 2.0, 4.0, ca(3, 130));
        batch.fill(atlas, tx + 6.0, y, 1.0, 4.0, ca(3, 130));
        tx += 10.0;
    }
    big(batch, atlas, "K Y R O", x, y + 8.0, 1.5, ink());
    let [bx, by, bw, bh] = search_rect(size);
    let focused = SEARCH_FOCUS.load(std::sync::atomic::Ordering::Relaxed) || !query.is_empty();
    if focused {
        batch.fill(atlas, bx - 1.0, by - 1.0, bw + 2.0, bh + 2.0, accent());
    }
    batch.fill(atlas, bx, by, bw, bh, if focused { ca(1, 255) } else if inside(c, [bx, by, bw, bh]) { ca(1, 255) } else { panel() });
    if focused {
        let caret = if clock().fract() < 0.55 { "_" } else { "" };
        text(batch, atlas, &format!("{query}{caret}"), bx + 5.0, by + 3.0, ink(), false);
    } else {
        text(batch, atlas, "Search", bx + 5.0, by + 3.0, soft(), false);
    }
    for (index, name) in ["HUD", "PvP", "Visual", "Client", "Capes", "Themes"].iter().enumerate() {
        let [rx, ry, rw, rh] = tab_rect(index, size);
        let picked = index == category && query.is_empty();
        batch.fill(atlas, rx + 2.0, ry + 2.0, rw, rh, if picked { accent() } else { line() });
        batch.fill(atlas, rx, ry, rw, rh, if picked { sel() } else if inside(c, [rx, ry, rw, rh]) { ca(1, 255) } else { panel() });
        text(batch, atlas, name, rx + 7.0, ry + 4.0, if picked { seltxt() } else { ink() }, false);
    }
    if shows_banner() {
        draw_banner(batch, atlas, c, size);
    }
    let shown = items(category);
    let [lx, ly, lw, lh] = rows_rect(size);
    let most = (shown.len() as f32 * ROW_H - lh).max(0.0);
    let scroll = with(|s| {
        s.scroll = s.scroll.clamp(0.0, most);
        s.scroll
    });
    let hovered = row_at(c, size, &shown);
    if let Some(row) = hovered.filter(|_| !sheet_busy()) {
        let (id, name, _) = shown[row];
        if !id.ends_with("_editor") {
            with(|s| s.page = Some((id, name)));
        }
    }
    let current = with(|s| s.page).map(|p| p.0);
    let rows = batch.vertices.len();
    if shown.is_empty() {
        text(batch, atlas, "Nothing found", lx + 6.0, ly + 5.0, soft(), false);
    }
    for (i, (id, name, _)) in shown.iter().enumerate() {
        let ry = ly + i as f32 * ROW_H - scroll;
        if ry + ROW_H < ly || ry > ly + lh {
            continue;
        }
        if *id == "#" {
            text(batch, atlas, name, lx, ry + 5.0, soft(), false);
            let tail = lx + HudBatch::text_width(atlas, name) + 6.0;
            batch.fill(atlas, tail, ry + 9.0, lx + lw - tail, 0.5, line());
            continue;
        }
        let picked = current == Some(*id);
        let shift = if picked { 4.0 } else { 0.0 };
        if picked {
            batch.fill(atlas, lx + shift + 2.0, ry + 2.0, lw - 6.0, ROW_H - 2.0, accent());
        }
        batch.fill(atlas, lx + shift, ry, lw - 6.0, ROW_H - 2.0, if picked { sel() } else if hovered == Some(i) { ca(1, 255) } else { panel() });
        let enabled = on(id);
        let plain = id.ends_with("_editor") || *id == "ring_menu";
        let mark = if picked { seltxt() } else if enabled && !plain { ink() } else { line() };
        batch.fill(atlas, lx + shift + 5.0, ry + 4.0, 5.0, 5.0, mark);
        text(batch, atlas, name, lx + shift + 15.0, ry + 3.0, if picked { seltxt() } else { ink() }, false);
        let state = if plain { ">" } else if enabled { "ON" } else { "OFF" };
        let right = lx + shift + lw - 12.0 - HudBatch::text_width(atlas, state);
        text(batch, atlas, state, right, ry + 3.0, if picked { seltxt() } else if enabled { ink() } else { soft() }, false);
    }
    batch.clip_since(rows, lx - 1.0, ly, lx + lw + 8.0, ly + lh);
    if most > 0.0 {
        let bar = lh * lh / (lh + most);
        batch.fill(atlas, lx + lw + 1.0, ly + (lh - bar) * scroll / most, 1.5, bar, line());
    }
    if let Some(page) = with(|s| s.page) {
        draw_panel(batch, atlas, c, page);
    } else {
        let [sx, sy, sw, _] = sheet_area(size);
        frame(batch, atlas, sx, sy, sw, 48.0);
        text(batch, atlas, "Point at a row to see its settings.", sx + 10.0, sy + 20.0, soft(), false);
    }
    let foot = "Esc  Back        Wheel  Scroll        Click a row to switch it";
    text(batch, atlas, foot, x, y + h - 9.0, soft(), false);
    batch.scale_since(start, size.0 / 2.0, size.1 / 2.0, k, k);
}

fn click_page(cursor: (f32, f32), size: (f32, f32), right_click: bool) {
    let c = page_cursor(cursor, size);
    for index in 0..NAMES.len() {
        if inside(c, tab_rect(index, size)) {
            with(|s| s.query.clear());
            SEARCH_FOCUS.store(false, std::sync::atomic::Ordering::Relaxed);
            open_card(index);
            return;
        }
    }
    let on_search = inside(c, search_rect(size));
    SEARCH_FOCUS.store(on_search, std::sync::atomic::Ordering::Relaxed);
    if on_search {
        return;
    }
    if shows_banner() && inside(c, banner_rect(size)) {
        with(|s| {
            s.page = Some(("hud_editor", "HUD editor"));
            s.opened = Some(Instant::now());
        });
        return;
    }
    let category = with(|s| s.selected);
    let shown = items(category);
    if let Some(row) = row_at(c, size, &shown) {
        let (id, name, _) = shown[row];
        if id.ends_with("_editor") {
            with(|s| {
                s.page = Some((id, name));
                s.opened = Some(Instant::now());
            });
        } else if id != "ring_menu" && !right_click {
            toggle(id);
        }
        return;
    }
    if let Some((id, _)) = with(|s| s.page) {
        click_panel(c, size, id);
    }
}

/// A cape's page in its sheet: cells of 88x68, 8 across; a page is 256x128 in the same scale.
pub fn cape_uv(id: &str, r: [f32; 4]) -> [f32; 4] {
    const CELL_PX: (f32, f32) = (88.0, 68.0);
    static START: LazyLock<Instant> = LazyLock::new(Instant::now);
    let animated = !matches!(&id[5..], "crest" | "red_crane" | "first_snow" | "kyro_sigil" | "kintsugi");
    let (sheet_w, sheet_h) = if animated { (704.0, 272.0) } else { CELL_PX };
    let frame = if animated { (START.elapsed().as_millis() / 50 % 32) as u32 } else { 0 };
    let (du, dv) = ((r[2] - r[0]) / sheet_w, (r[3] - r[1]) / sheet_h);
    let (u, v) = (r[0] + (frame % 8) as f32 * CELL_PX.0 * du, r[1] + (frame / 8) as f32 * CELL_PX.1 * dv);
    [u, v, u + 256.0 * du, v + 128.0 * dv]
}

/// The sprite holding a cape's sheet.
pub fn cape_name(id: &str) -> String {
    format!("kyro_capes:entity/{}", &id[5..])
}

/// The outside of a cape, swaying a little.
fn cape_art(batch: &mut HudBatch, atlas: &HudAtlas, id: &str, x: f32, y: f32, w: f32, h: f32) {
    let Some(r) = atlas.sprites.get(cape_name(id).as_str()).copied() else {
        batch.fill(atlas, x, y, w, h, sunk());
        return;
    };
    let p = cape_uv(id, [r.u0, r.v0, r.u1, r.v1]);
    let (du, dv) = (p[2] - p[0], p[3] - p[1]);
    let (u0, u1, v0, v1) = (p[0] + du / 64.0, p[0] + du * 11.0 / 64.0, p[1] + dv / 32.0, p[1] + dv * 17.0 / 32.0);
    let t = clock() * 3.5;
    let tint = cape_tint().unwrap_or(rgba(255, 255, 255, 255));
    const STRIPS: usize = 32;
    let bend = |i: usize| {
        let share = i as f32 / STRIPS as f32;
        (t - share * 7.0).sin() * share * 4.0
    };
    for i in 0..STRIPS {
        let (a, b) = (i as f32 / STRIPS as f32, (i + 1) as f32 / STRIPS as f32);
        let (ya, yb, sa, sb) = (y + h * a, y + h * b, bend(i), bend(i + 1));
        let (va, vb) = (v0 + (v1 - v0) * a, v0 + (v1 - v0) * b);
        batch.quad([[x + sa, ya], [x + w + sa, ya], [x + w + sb, yb], [x + sb, yb]], [[u0, va], [u1, va], [u1, vb], [u0, vb]], tint);
    }
}

static CARD_Y: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

pub fn click(cursor: (f32, f32), size: (f32, f32), right_click: bool) {
    if picker_input(cursor, size, false) {
        return;
    }
    let page = with(|s| s.page);
    if let Some((id, _)) = page.filter(|p| p.0.ends_with("_editor")) {
        click_settings(cursor, size, id, right_click);
    } else if with(|s| s.list) {
        click_page(cursor, size, right_click);
    } else if let Ok(mut swipe) = SWIPE.lock() {
        set_screen(size.0, size.1);
        *swipe = Some((cursor.0, cursor.0));
        CARD_Y.store(cursor.1 as i32, std::sync::atomic::Ordering::Relaxed);
    }
}

/// Each item's column and row: group titles take a whole row; the rest fill two columns.
fn layout(items: &[(&'static str, &'static str, bool)]) -> (Vec<(f32, f32)>, f32) {
    let (mut col, mut row) = (0.0, 0.0);
    let mut places = Vec::with_capacity(items.len());
    for item in items {
        if item.0 == "#" {
            if col > 0.0 {
                row += 1.0;
                col = 0.0;
            }
            places.push((0.0, row));
            row += 1.0;
        } else {
            places.push((col, row));
            if col > 0.0 {
                row += 1.0;
                col = 0.0;
            } else {
                col = 1.0;
            }
        }
    }
    (places, if col > 0.0 { row + 1.0 } else { row })
}
