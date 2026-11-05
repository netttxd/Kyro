// The Right Shift menu: four categories around your face, then each one's mods.
// It takes over Options > Mods while open, as Mod Menu does. Art is drawn from
// the same 16x16 sprites the Java client uses.

use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};
use std::time::Instant;

use super::{OptionsUi, Page};
use crate::hud::{HudAtlas, HudBatch, Rect, rgba};
use crate::text::Text;

const NAMES: [&str; 5] = ["HUD", "PVP", "VISUAL", "CLIENT", "CAPE"];
const RADIUS: f32 = 78.0;
const HEADER: f32 = 24.0;
const ROW: f32 = 18.0;
const PANEL_W: f32 = 340.0;

/// Mods by category: id, name, and whether it works in Rust yet.
const MODS: [&[(&str, &str, bool)]; 5] = [
    &[("fps", "FPS", true), ("cps", "CPS", true), ("coordinates", "Coordinates", true), ("keystrokes", "Keystrokes", true), ("armor", "Armor", true), ("clock", "Clock", true), ("day", "Day", true), ("light", "Light", true), ("session", "Session", true), ("reach", "Reach", true), ("server", "Server", true), ("tps", "TPS", true), ("kill_feed", "Kill feed", true), ("totems", "Item counter", true), ("potions", "Potions", true), ("speed", "Speed", true), ("ping", "Ping", true), ("scoreboard", "Scoreboard", true), ("action_bar", "Action bar", true), ("inventory", "Inventory", true), ("lyrics", "Lyrics", true)],
    &[("#", "Hits", true), ("hit_markers", "Hit markers", true), ("hit_color", "Hit color", true), ("damage_numbers", "Damage numbers", true), ("combo", "Combo", true), ("cooldown_indicator", "Cooldown indicator", true), ("#", "Reach", true), ("reach_hitbox", "Hitboxes", true), ("#", "Warnings", true), ("low_health", "Low health glow", true), ("durability_warning", "Gear warning", true), ("shield_status", "Shield", true)],
    &[("#", "Crosshair", true), ("crosshair", "Crosshair", true), ("crosshair_editor", "Crosshair editor", true), ("#", "Camera", true), ("held_items", "Held items", true), ("fov_control", "FOV control", true), ("hurt_cam", "Hurt cam", true), ("motion_blur", "Motion blur", true), ("color_saturation", "Color saturation", true), ("#", "World", true), ("fullbright", "Fullbright", true), ("clear_fluids", "Clear fluids", true), ("world_look", "World look", true), ("low_obstruction", "Low obstruction", true), ("particles", "Hide particles", true), ("#", "Items", true), ("block_outline", "Block outline", true), ("item_highlighter", "Item highlighter", true), ("shulker_preview", "Shulker preview", true), ("glint_color", "Glint color", true), ("shiny_potions", "Shiny potions", true), ("food_saturation", "Food saturation", true), ("item_physics", "Item physics", true), ("tnt_timer", "TNT timer", true), ("#", "Players", true), ("skin_layers_3d", "3D skin layers", true), ("own_nametag", "Own nametag", true)],
    &[("#", "Controls", true), ("ring_menu", "Menu key", true), ("zoom", "Zoom", true), ("freelook", "Freelook", true), ("sprint", "Sprint", true), ("#", "Chat", true), ("better_chat", "Better chat", true), ("#", "Tab list", true), ("tab_ping", "Ping numbers", true)],
    &[("wavy_capes", "Wavy capes", true), ("cape_aurora", "Aurora", true), ("cape_bamboo", "Bamboo", true), ("cape_ember", "Ember", true), ("cape_crimson_dragon", "Crimson dragon", true), ("cape_fallen_angel", "Fallen angel", true), ("cape_frost", "Frost", true), ("cape_fuji_sunset", "Fuji sunset", true), ("cape_galaxy", "Galaxy", true), ("cape_inferno", "Inferno", true), ("cape_koi_pond", "Koi pond", true), ("cape_lantern_night", "Lantern night", true), ("cape_reaper", "Reaper", true), ("cape_sakura", "Sakura", true), ("cape_starfield", "Starfield", true), ("cape_starlight", "Starlight", true), ("cape_watcher", "Watcher", true), ("cape_wisteria", "Wisteria", true)],
];

/// A mod's settings: key, label, min, max, default. 0..1 is a switch, key "color" is swatches.
fn settings_of(id: &str) -> &'static [(&'static str, &'static str, i32, i32, i32)] {
    match id {
        "zoom" => &[("level", "Starting zoom", 2, 16, 4), ("smooth", "Smooth", 0, 1, 1), ("scroll", "Scroll to zoom", 0, 1, 1), ("key_zoom", "Key", 0, 0, 0)],
        "fullbright" => &[("strength", "Strength", 10, 100, 100)],
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
        "lyrics" => &[("offset", "Offset (ms)", -500, 500, 0), ("delay", "Lyric delay (ms)", -5000, 5000, 0), ("key_lyric_earlier", "Earlier key", 0, 0, 0), ("key_lyric_later", "Later key", 0, 0, 0), ("online", "Online lyrics", 0, 1, 1), ("guess", "Guess word timing", 0, 1, 0), ("panel", "Background", 0, 1, 0)],
        "totems" => &[("count_totems", "Totems", 0, 1, 1), ("count_pearls", "Pearls", 0, 1, 0), ("count_gapples", "Enchanted apples", 0, 1, 0), ("count_golden_apples", "Golden apples", 0, 1, 0),
            ("count_arrows", "Arrows", 0, 1, 0), ("count_crystals", "Crystals", 0, 1, 0), ("count_wind_charges", "Wind charges", 0, 1, 0), ("count_xp_bottles", "XP bottles", 0, 1, 0),
            ("count_held", "Held item", 0, 1, 0), ("count_zero", "Show at zero", 0, 1, 1), ("text_color", "Value color", 0, 7, -1), ("panel", "Background", 0, 1, 0)],
        "armor" => &[("value", "Value", 0, 2, 0), ("layout", "Horizontal", 0, 1, 0), ("bar", "Durability bar", 0, 1, 1), ("text_color", "Value color", 0, 7, 0), ("panel", "Background", 0, 1, 0)],
        "potions" => &[("icons", "Icons", 0, 1, 0), ("label_color", "Label color", 0, 7, 0), ("text_color", "Value color", 0, 7, -1), ("panel", "Background", 0, 1, 0)],
        "ping" => &[("graph", "Graph", 0, 1, 0), ("label_color", "Label color", 0, 7, 0), ("text_color", "Value color", 0, 7, -1), ("panel", "Background", 0, 1, 0)],
        "combo" => &[("popup", "Popup", 0, 1, 0), ("from", "Popup from", 0, 2, 1), ("label_color", "Label color", 0, 7, 0), ("text_color", "Value color", 0, 7, -1), ("panel", "Background", 0, 1, 0)],
        "fps" | "cps" | "coordinates" | "speed" | "clock" | "day" | "light" | "session" | "reach" | "tps" =>
            &[("label_color", "Label color", 0, 7, 0), ("text_color", "Value color", 0, 7, -1), ("panel", "Background", 0, 1, 0)],
        "keystrokes" => &[("keys_move", "Movement keys", 0, 1, 1), ("keys_jump", "Jump", 0, 1, 1), ("keys_sneak", "Sneak", 0, 1, 1), ("keys_mouse", "Mouse", 0, 1, 1), ("keys_cps", "CPS on mouse", 0, 1, 1), ("label_color", "Text color", 0, 7, 0), ("held_color", "Pressed color", 0, 7, -1), ("panel", "Background", 0, 1, 0)],
        "ring_menu" => &[("key_menu", "Key", 0, 0, 0)],
        "food_saturation" => &[("preview", "Preview food", 0, 1, 1), ("health", "Preview health", 0, 1, 1), ("exhaustion", "Exhaustion bar", 0, 1, 1), ("tooltip", "Food tooltip", 0, 1, 1)],
        "server" => &[("hide", "Hide address", 0, 1, 0), ("label_color", "Label color", 0, 7, 0), ("text_color", "Value color", 0, 7, -1), ("panel", "Background", 0, 1, 0)],
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
    let [px, py, pw, _] = if id == "capes" { panel_rect(size.0, size.1, CAPES) } else { settings_rect(size.0, size.1, id) };
    let x = if px + pw + 8.0 + PICK_W < size.0 { px + pw + 8.0 } else { (px - 8.0 - PICK_W).max(0.0) };
    [x, py.min(size.1 - PICK_H).max(0.0), PICK_W, PICK_H]
}

fn draw_picker(batch: &mut HudBatch, atlas: &HudAtlas) {
    let Some((id, _, h, s, v)) = PICKER.lock().ok().and_then(|p| *p) else {
        return;
    };
    let [x, y, w, ph] = picker_rect((batch.width, batch.height), id);
    frame(batch, atlas, x, y, w, ph);
    let ink = |c: u32| rgba((c >> 16) as u8, (c >> 8) as u8, c as u8, 255);
    for col in 0..30 {
        for row in 0..20 {
            let c = hsv(h, col as f32 / 29.0, 1.0 - row as f32 / 19.0);
            batch.fill(atlas, x + 6.0 + col as f32 * 4.0, y + 6.0 + row as f32 * 4.0, 4.0, 4.0, ink(c));
        }
    }
    for col in 0..120 {
        batch.fill(atlas, x + 6.0 + col as f32, y + 90.0, 1.0, 12.0, ink(hsv(col as f32 / 120.0, 1.0, 1.0)));
    }
    batch.fill(atlas, x + 6.0 + s * 118.0, y + 6.0 + (1.0 - v) * 78.0, 2.0, 2.0, rgba(0, 0, 0, 255));
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

pub const COLORS: [u32; 8] = [0xffffff, 0xff5555, 0xffaa00, 0xffff55, 0x55ff55, 0x55ffff, 0x5555ff, 0xff55ff];
const BLOCKS: f32 = 10.0;
const BLOCK: f32 = 6.0;
const STEPPER: f32 = 14.0;
const SETTINGS_W: f32 = 270.0;

const SCROLL: [&str; 16] = ["................", "..kkkkkkkkkkkk..", ".kwwwwwwwwwwllk.", ".kkkkkkkkkkkkk..", "..klwwwwwwwlk...", "..klwddddwlk....", "..klwwwwwwlk....", "..klwdddwwlk....", "..klwwwwwwlk....", "..klwddddwlk....", "..klwwwwwwlk....", "..klwddwwwlk....", ".kkkkkkkkkkkkk..", ".kwwwwwwwwwwllk.", "..kkkkkkkkkkkk..", "................"];
const SWORDS: [&str; 16] = ["kk............kk", "kwk..........kck", "klwk........kcbk", ".klwk......kcbk.", "..klwk....kcbk..", "...klwk..kcbk...", "....klwkkcbk....", ".....klwcbk.....", ".....kcwlbk.....", "....kcbkklwk....", ".kk.kbk..klk.kk.", ".kykyk....kykyk.", "..kyok....koyk..", ".ktkqk....kqktk.", "ktk.kk....kk.ktk", "kk............kk"];
const EYE: [&str; 16] = ["................", "................", ".....kkkkkk.....", "...kkwwwwwwkk...", "..kwwwkkkkwwlk..", ".kwwwkcbbbkwllk.", "kwwwkcwbbnnkwllk", "kwwwkbbkknnkwlgk", "kwwwkbbkknnkwlgk", "kwwlkbbbnnnklggk", ".kwllkbnnnklggk.", "..kllgkkkkgggk..", "...kkggggggkk...", ".....kkkkkk.....", "................", "................"];
const CRYSTAL: [&str; 16] = [".......kk.......", "......kwck......", ".....kwwcbk.....", "....kwwccbbk....", "...kwwcccbbnk...", "..kwwwcccbbbnk..", ".kwwcccccbbbnnk.", ".kkkkkkkkkkkkkk.", ".kcccbbbbbnnnnk.", "..kccbbbbbnnnk..", "...kcbbbbnnnk...", "....kcbbbnnk....", ".....kcbnnk.....", "......kbnk......", ".......kk.......", "................"];
const BACK: [&str; 16] = ["................", "....kk..........", "...kyk..........", "..kyyk..kkkkk...", ".kyyyykkyyyyyk..", "kyyyyyyyyooyyyk.", ".kyyyyoooookkyyk", "..koyk.kkk..koyk", "...kok......koyk", "....kk......koyk", "............koyk", "..........kkyyk.", "....kkkkkkyyyk..", "....kyyyyyyok...", "....kkkkkkkk....", "................"];
const CAPE: [&str; 16] = ["................", "....kkkkkkkk....", "...krrrrrrmmk...", "...kyyyyyyook...", "...krprrrrmmk...", "..kprrrrrrrmmk..", "..kprrrrrrrmmk..", "..kprrrrrrrmmk..", "..kprrrrrrrmmk..", ".kprrrrrrrrrmmk.", ".kprrrrrrrrrmmk.", ".kprrrrrrrrrmmk.", ".kprkrrkrrkrmmk.", ".kkk.kk.kk.kkkk.", "................", "................"];
const SPRITES: [&[&str; 16]; 5] = [&SCROLL, &SWORDS, &EYE, &CRYSTAL, &CAPE];
const FACE: [&str; 8] = ["tttttttt", "tttttttt", "tssssstt", "ssssssss", "swnssnws", "sssuusss", "ssuuuuss", "ssssssss"];

#[derive(Default)]
struct State {
    list: bool,
    page: Option<(&'static str, &'static str)>,
    values: HashMap<String, HashMap<String, i32>>,
    selected: usize,
    by_key: bool,
    opened: Option<Instant>,
    query: String,
    hover_since: Option<Instant>,
    on: HashMap<String, bool>,
    loaded: bool,
    saved: Vec<(String, String)>,
}

static STATE: LazyLock<Mutex<State>> = LazyLock::new(Mutex::default);

fn with<T>(f: impl FnOnce(&mut State) -> T) -> T {
    f(&mut STATE.lock().unwrap_or_else(|e| e.into_inner()))
}

fn default_on(id: &str) -> bool {
    matches!(id, "scoreboard" | "action_bar" | "fps" | "cps" | "coordinates" | "keystrokes" | "zoom" | "freelook" | "potions" | "armor" | "crosshair" | "hit_markers" | "cooldown_indicator")
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

/// Reads the saved switches once.
pub fn load(get: impl Fn(&str) -> Option<String>) {
    with(|s| {
        for mods in MODS {
            for (id, _, _) in mods {
                if let Some(value) = get(&format!("on_{id}")) {
                    s.on.insert((*id).to_owned(), value == "true");
                }
            }
        }
        for mods in MODS {
            for (id, _, _) in mods {
                for (key, _, _, _, _) in settings_of(id) {
                    if let Some(value) = get(&format!("set_{id}_{key}")).and_then(|v| v.parse().ok()) {
                        s.values.entry((*id).to_owned()).or_default().insert((*key).to_owned(), value);
                    }
                }
            }
        }
        if let Some(value) = get("set_capes_tint").and_then(|v| v.parse().ok()) {
            s.values.entry("capes".to_owned()).or_default().insert("tint".to_owned(), value);
        }
        let extra = (0..15).map(|n| ("crosshair".to_owned(), format!("r{n}"))).chain(
            ELEMENTS.iter().flat_map(|e| ["x", "y", "s", "b", "p", "ax", "ay"].map(|k| ("hud".to_owned(), format!("{}_{k}", e.0)))),
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
    with(|s| {
        s.by_key = true;
        s.list = false;
        s.opened = Some(Instant::now());
        s.hover_since = s.opened;
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
    with(|s| {
        s.by_key = false;
        s.list = false;
        s.page = None;
    });
}

/// Esc on the mod list goes back to the ring; on the ring it closes the menu.
pub fn back() -> bool {
    if take_listening().is_some() {
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
    if with(|s| s.page.is_none() && !std::mem::take(&mut s.query).is_empty()) {
        return true;
    }
    with(|s| {
        if s.page.take().is_some() {
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

fn pixel_color(c: char) -> u32 {
    match c {
        'k' => rgba(12, 12, 24, 255),
        'w' => rgba(255, 255, 255, 255),
        'l' => rgba(216, 220, 232, 255),
        'g' => rgba(152, 160, 184, 255),
        'd' => rgba(88, 96, 128, 255),
        'y' => rgba(255, 224, 96, 255),
        'o' => rgba(232, 152, 32, 255),
        'q' => rgba(168, 92, 16, 255),
        'b' => rgba(72, 120, 232, 255),
        'c' => rgba(168, 208, 255, 255),
        'n' => rgba(32, 48, 128, 255),
        'r' => rgba(240, 80, 80, 255),
        'p' => rgba(255, 156, 140, 255),
        'm' => rgba(152, 40, 56, 255),
        't' => rgba(120, 78, 40, 255),
        'u' => rgba(112, 68, 32, 255),
        's' => rgba(214, 160, 120, 255),
        _ => 0,
    }
}

fn sprite(batch: &mut HudBatch, atlas: &HudAtlas, rows: &[&str], x: f32, y: f32, scale: f32) {
    for (j, row) in rows.iter().enumerate() {
        for (i, c) in row.chars().enumerate() {
            let color = pixel_color(c);
            if color != 0 {
                batch.fill(atlas, x + i as f32 * scale, y + j as f32 * scale, scale, scale, color);
            }
        }
    }
}

/// The pixel rounded corner used everywhere: two steps cut from each corner.
fn notched(batch: &mut HudBatch, atlas: &HudAtlas, x: f32, y: f32, w: f32, h: f32, color: u32) {
    batch.fill(atlas, x + 2.0, y, w - 4.0, 1.0, color);
    batch.fill(atlas, x + 1.0, y + 1.0, w - 2.0, 1.0, color);
    batch.fill(atlas, x, y + 2.0, w, h - 4.0, color);
    batch.fill(atlas, x + 1.0, y + h - 2.0, w - 2.0, 1.0, color);
    batch.fill(atlas, x + 2.0, y + h - 1.0, w - 4.0, 1.0, color);
}

fn frame(batch: &mut HudBatch, atlas: &HudAtlas, x: f32, y: f32, w: f32, h: f32) {
    notched(batch, atlas, x - 3.0, y - 3.0, w + 6.0, h + 6.0, rgba(0, 0, 0, 255));
    notched(batch, atlas, x - 2.0, y - 2.0, w + 4.0, h + 4.0, rgba(255, 255, 255, 255));
    notched(batch, atlas, x - 1.0, y - 1.0, w + 2.0, h + 2.0, rgba(127, 147, 224, 255));
    batch.fill_gradient(atlas, (x, y, w, h), rgba(58, 86, 200, 255), rgba(14, 24, 88, 255));
}

fn text(batch: &mut HudBatch, atlas: &HudAtlas, value: &str, x: f32, y: f32, color: u32, shadow: bool) {
    batch.text_styled(atlas, &Text::literal(value), x, y, color, shadow);
}

fn centered(batch: &mut HudBatch, atlas: &HudAtlas, value: &str, cx: f32, y: f32, color: u32) {
    text(batch, atlas, value, cx - HudBatch::text_width(atlas, value) / 2.0, y, color, true);
}

fn back_ease(t: f32) -> f32 {
    let u = t.clamp(0.0, 1.0) - 1.0;
    1.0 + 2.2 * u * u * u + 1.2 * u * u
}

fn age(s: &State) -> f32 {
    s.opened.map_or(1.0, |t| t.elapsed().as_secs_f32())
}

fn center(batch: &HudBatch) -> (f32, f32) {
    (batch.width / 2.0, batch.height / 2.0 + 10.0)
}

fn pick(cursor: (f32, f32), c: (f32, f32)) -> Option<usize> {
    let (dx, dy) = (cursor.0 - c.0, cursor.1 - c.1);
    if dx * dx + dy * dy < 900.0 {
        return None;
    }
    let turn = dy.atan2(dx) + std::f32::consts::FRAC_PI_2;
    Some(((turn / (std::f32::consts::TAU / 5.0)).round() as i32).rem_euclid(5) as usize)
}

const CAPES: usize = 4;
const CELL_W: f32 = 36.0;
const CELL_H: f32 = 54.0;

/// The cape under the cursor on the cape page: its index in the category.
fn cape_at(cursor: (f32, f32), x: f32, y: f32) -> Option<usize> {
    let col = ((cursor.0 - x - 8.0) / CELL_W).floor();
    let row = ((cursor.1 - y - HEADER - 4.0) / CELL_H).floor();
    if !(0.0..6.0).contains(&col) || !(0.0..3.0).contains(&row) {
        return None;
    }
    let index = 1 + row as usize * 6 + col as usize;
    (index < MODS[CAPES].len()).then_some(index)
}

/// The cape's outer face, the part of its texture that shows from behind.
fn cape_art(batch: &mut HudBatch, atlas: &HudAtlas, id: &str, x: f32, y: f32, w: f32, h: f32) {
    let Some(r) = atlas.sprites.get(cape_sprite(id).as_str()).copied() else {
        batch.fill(atlas, x, y, w, h, rgba(10, 16, 56, 255));
        return;
    };
    let (du, dv) = (r.u1 - r.u0, r.v1 - r.v0);
    let art = Rect { u0: r.u0 + du / 64.0, v0: r.v0 + dv / 32.0, u1: r.u0 + du * 11.0 / 64.0, v1: r.v0 + dv * 17.0 / 32.0, ..r };
    batch.rect_uv(x, y, w, h, art, cape_tint().unwrap_or(rgba(255, 255, 255, 255)));
}

fn wavy_row(x: f32, y: f32) -> [f32; 4] {
    [x + 232.0, y + HEADER + 138.0, 100.0, ROW]
}

fn draw_capes(batch: &mut HudBatch, atlas: &HudAtlas, cursor: (f32, f32)) {
    let opened = with(|s| age(s));
    let [x, y, w, h] = panel_rect(batch.width, batch.height, CAPES);
    let start = batch.vertices.len();
    let white = rgba(255, 255, 255, 255);
    frame(batch, atlas, x, y, w, h);
    sprite(batch, atlas, &BACK, x + 5.0, y + 4.0, 1.0);
    text(batch, atlas, "Capes", x + 26.0, y + 8.0, white, false);
    batch.fill(atlas, x + 5.0, y + HEADER - 2.0, w - 10.0, 1.0, rgba(127, 147, 224, 255));
    let hovered = cape_at(cursor, x, y);
    for (index, (id, _, _)) in MODS[CAPES].iter().enumerate().skip(1) {
        let cx = x + 8.0 + ((index - 1) % 6) as f32 * CELL_W;
        let cy = y + HEADER + 4.0 + ((index - 1) / 6) as f32 * CELL_H;
        let edge = if on(id) { rgba(255, 224, 96, 255) } else if hovered == Some(index) { white } else { rgba(0, 0, 0, 255) };
        batch.fill(atlas, cx, cy, 32.0, 50.0, edge);
        cape_art(batch, atlas, id, cx + 1.0, cy + 1.0, 30.0, 48.0);
    }
    // The preview: what you point at, else what you wear.
    let shown = hovered.or_else(|| MODS[CAPES].iter().position(|m| m.0.starts_with("cape_") && on(m.0)));
    let px = x + 252.0;
    let py = y + HEADER + 6.0;
    batch.fill(atlas, px - 1.0, py - 1.0, 62.0, 98.0, rgba(0, 0, 0, 255));
    match shown {
        Some(index) => {
            cape_art(batch, atlas, MODS[CAPES][index].0, px, py, 60.0, 96.0);
            centered(batch, atlas, MODS[CAPES][index].1, px + 30.0, py + 102.0, white);
        }
        None => {
            batch.fill(atlas, px, py, 60.0, 96.0, rgba(10, 16, 56, 255));
            centered(batch, atlas, "No cape", px + 30.0, py + 102.0, rgba(150, 160, 200, 255));
        }
    }
    let [wx, wy, ww, wh] = wavy_row(x, y);
    // Color: the first swatch is the cape's own colors.
    let tint = value("capes", "tint");
    for i in 0..=COLORS.len() {
        let bx = wx + 5.0 + i as f32 * 10.0;
        notched(batch, atlas, bx - 1.0, wy - 13.0, 10.0, 11.0, if i as i32 == tint { white } else { rgba(0, 0, 0, 255) });
        let fill = if i == 0 {
            rgba(10, 16, 56, 255)
        } else {
            let color = COLORS[i - 1];
            rgba((color >> 16) as u8, (color >> 8) as u8, color as u8, 255)
        };
        notched(batch, atlas, bx, wy - 12.0, 8.0, 9.0, fill);
    }
    let bx = wx + 5.0 + 9.0 * 10.0;
    notched(batch, atlas, bx - 1.0, wy - 13.0, 10.0, 11.0, if tint >= CUSTOM { white } else { rgba(0, 0, 0, 255) });
    if tint >= CUSTOM {
        let c = color_of(tint);
        notched(batch, atlas, bx, wy - 12.0, 8.0, 9.0, rgba((c >> 16) as u8, (c >> 8) as u8, c as u8, 255));
    } else {
        notched(batch, atlas, bx, wy - 12.0, 8.0, 9.0, rgba(30, 44, 120, 255));
        centered(batch, atlas, "+", bx + 4.0, wy - 12.0, white);
    }
    if cursor.0 >= wx && cursor.0 < wx + ww && cursor.1 >= wy && cursor.1 < wy + wh {
        notched(batch, atlas, wx, wy - 2.0, ww, wh - 1.0, rgba(255, 255, 255, 60));
    }
    text(batch, atlas, "Wavy", wx + 5.0, wy + 2.0, white, false);
    switch(batch, atlas, wx + ww - 6.0, wy, on("wavy_capes"));
    let pop = back_ease(opened * 14.0);
    batch.scale_since(start, x + w / 2.0, y + h / 2.0, pop, pop);
}

fn click_capes(cursor: (f32, f32), size: (f32, f32), right_click: bool) {
    let [x, y, _, _] = panel_rect(size.0, size.1, CAPES);
    if cursor.0 >= x && cursor.0 < x + 26.0 && cursor.1 >= y && cursor.1 < y + HEADER {
        back();
        return;
    }
    let [wx, wy, ww, wh] = wavy_row(x, y);
    if cursor.1 >= wy - 13.0 && cursor.1 < wy - 2.0 {
        let index = ((cursor.0 - wx - 4.0) / 10.0).floor();
        if index >= 0.0 && (index as usize) <= COLORS.len() {
            set_value("capes", "tint", index as i32);
        } else if index as usize == COLORS.len() + 1 {
            open_picker("capes", "tint");
        }
        return;
    }
    if cursor.0 >= wx && cursor.0 < wx + ww && cursor.1 >= wy && cursor.1 < wy + wh {
        if right_click {
            with(|s| {
                s.page = Some(("wavy_capes", "Wavy capes"));
                s.opened = Some(Instant::now());
            });
        } else {
            toggle("wavy_capes");
        }
    } else if let Some(index) = cape_at(cursor, x, y) {
        toggle(MODS[CAPES][index].0);
    }
}

/// What a list shows: its category's rows, or everything matching the search.
fn items(category: usize) -> Vec<(&'static str, &'static str, bool)> {
    let query = with(|s| s.query.to_lowercase());
    if query.is_empty() {
        return MODS[category].to_vec();
    }
    let hit = |id: &str, name: &str| name.to_lowercase().contains(&query) || settings_of(id).iter().any(|e| e.1.to_lowercase().contains(&query));
    MODS.iter().take(CAPES).skip(1).flat_map(|c| c.iter()).filter(|m| m.0 != "#" && hit(m.0, m.1)).copied().collect()
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

fn showing_capes(category: usize) -> bool {
    category == CAPES && with(|s| s.query.is_empty())
}

fn panel_rect(width: f32, height: f32, category: usize) -> [f32; 4] {
    if showing_capes(category) {
        let h = HEADER + 3.0 * CELL_H + 10.0;
        return [(width - PANEL_W) / 2.0, (height - h) / 2.0, PANEL_W, h];
    }
    let rows = layout(&items(category)).1.max(1.0);
    let h = rows * ROW + HEADER + 8.0;
    [(width - PANEL_W) / 2.0, (height - h) / 2.0, PANEL_W, h]
}

pub fn draw(batch: &mut HudBatch, atlas: &HudAtlas, cursor: (f32, f32)) {
    let (w, h) = (batch.width, batch.height);
    if !editing() {
        batch.fill(atlas, 0.0, 0.0, w, h, rgba(0, 0, 24, 104));
    }
    if let Some(page) = with(|s| s.page) {
        draw_settings(batch, atlas, cursor, page);
    } else if with(|s| s.list) {
        draw_list(batch, atlas, cursor);
    } else {
        draw_ring(batch, atlas, cursor);
    }
    draw_picker(batch, atlas);
}

fn draw_ring(batch: &mut HudBatch, atlas: &HudAtlas, cursor: (f32, f32)) {
    let c = center(batch);
    if let Some(picked) = pick(cursor, c) {
        with(|s| {
            if s.selected != picked {
                s.selected = picked;
                s.hover_since = Some(Instant::now());
            }
        });
    }
    let (selected, opened, hover) = with(|s| (s.selected, age(s), s.hover_since.map_or(1.0, |t| t.elapsed().as_secs_f32())));
    notched(batch, atlas, c.0 - 29.0, c.1 - 29.0, 58.0, 58.0, rgba(0, 0, 0, 255));
    notched(batch, atlas, c.0 - 28.0, c.1 - 28.0, 56.0, 56.0, rgba(255, 255, 255, 255));
    // Your own face, hat layer on top; the pixel face until your skin has a sprite.
    let skin = OWN_SKIN.lock().map(|n| n.clone()).unwrap_or_default();
    if let Some(r) = atlas.sprites.get(skin.as_str()).copied() {
        let (du, dv) = (r.u1 - r.u0, r.v1 - r.v0);
        for (u, size) in [(8.0, 48.0), (40.0, 52.0)] {
            let part = Rect { u0: r.u0 + du * u / 64.0, v0: r.v0 + dv * 8.0 / 64.0, u1: r.u0 + du * (u + 8.0) / 64.0, v1: r.v0 + dv * 16.0 / 64.0, ..r };
            batch.rect_uv(c.0 - size / 2.0, c.1 - size / 2.0, size, size, part, rgba(255, 255, 255, 255));
        }
    } else {
        sprite(batch, atlas, &FACE, c.0 - 24.0, c.1 - 24.0, 6.0);
    }
    for i in 0..NAMES.len() {
        let angle = -std::f32::consts::FRAC_PI_2 + i as f32 * std::f32::consts::TAU / 5.0;
        let out = RADIUS * back_ease(opened * 10.0 - i as f32 * 0.25);
        let (x, y) = (c.0 + out * angle.cos(), c.1 + out * angle.sin());
        let picked = i == selected;
        let scale = if picked { 3.0 } else { 2.0 };
        let hop = if picked { 5.0 * (hover * 9.0).min(std::f32::consts::PI).sin() } else { 0.0 };
        let half = 8.0 * scale;
        sprite(batch, atlas, SPRITES[i].as_slice(), (x - half).round(), (y - half - hop).round(), scale);
        let color = if picked { rgba(255, 232, 74, 255) } else { rgba(255, 255, 255, 255) };
        centered(batch, atlas, NAMES[i], x, y + half + 2.0, color);
    }
}

fn draw_list(batch: &mut HudBatch, atlas: &HudAtlas, cursor: (f32, f32)) {
    let (category, opened, query) = with(|s| (s.selected, age(s), s.query.clone()));
    if showing_capes(category) {
        draw_capes(batch, atlas, cursor);
        return;
    }
    let [x, y, w, h] = panel_rect(batch.width, batch.height, category);
    let cursor = fitted(cursor, [x, y, w, h], batch.height);
    let start = batch.vertices.len();
    frame(batch, atlas, x, y, w, h);
    sprite(batch, atlas, &BACK, x + 5.0, y + 4.0, 1.0);
    let white = rgba(255, 255, 255, 255);
    let title = if query.is_empty() { NAMES[category] } else { "SEARCH" };
    text(batch, atlas, title, x + 26.0, y + 8.0, white, false);
    // Search, top right: typing goes here.
    let (bx, bw) = (x + w - 116.0, 110.0);
    notched(batch, atlas, bx, y + 5.0, bw, 14.0, rgba(0, 0, 0, 255));
    notched(batch, atlas, bx + 1.0, y + 6.0, bw - 2.0, 12.0, rgba(10, 16, 56, 255));
    if query.is_empty() {
        text(batch, atlas, "Search", bx + 4.0, y + 8.0, rgba(120, 132, 184, 255), false);
    } else {
        text(batch, atlas, &format!("{query}_"), bx + 4.0, y + 8.0, white, false);
    }
    // Each page has its own accent under the title.
    let accent = [rgba(127, 147, 224, 255), rgba(224, 84, 84, 255), rgba(84, 200, 224, 255), rgba(255, 200, 90, 255), rgba(224, 96, 170, 255)][category.min(4)];
    batch.fill(atlas, x + 5.0, y + HEADER - 2.0, w - 10.0, 1.0, if query.is_empty() { accent } else { white });
    let column = (w - 12.0) / 2.0;
    let shown = items(category);
    let (places, _) = layout(&shown);
    if shown.is_empty() {
        text(batch, atlas, "Nothing found", x + 11.0, y + HEADER + 5.0, rgba(150, 160, 200, 255), false);
    }
    for ((id, name, _), (col, row)) in shown.iter().zip(places) {
        let rx = x + 6.0 + col * column;
        let ry = y + HEADER + 3.0 + row * ROW;
        if *id == "#" {
            text(batch, atlas, name, rx + 5.0, ry + 4.0, accent, false);
            continue;
        }
        if cursor.0 >= rx && cursor.0 < rx + column && cursor.1 >= ry - 2.0 && cursor.1 < ry + ROW - 2.0 {
            notched(batch, atlas, rx, ry - 2.0, column - 2.0, ROW - 1.0, rgba(255, 255, 255, 80));
        }
        text(batch, atlas, name, rx + 5.0, ry + 3.0, white, false);
        if !id.ends_with("_editor") && *id != "ring_menu" {
            switch(batch, atlas, rx + column - 8.0, ry + 1.0, on(id));
        }
        if !settings_of(id).is_empty() || id.ends_with("_editor") {
            text(batch, atlas, ">", rx + column - 42.0, ry + 3.0, rgba(150, 160, 200, 255), false);
        }
    }
    let pop = back_ease(opened * 14.0) * fit(h, batch.height);
    batch.scale_since(start, x + w / 2.0, y + h / 2.0, pop, pop);
}

fn settings_rect(width: f32, height: f32, id: &str) -> [f32; 4] {
    if let Some(index) = ed(|e| e.popup).filter(|i| ELEMENTS[*i].0 == id) {
        return popup_rect(index, (width, height));
    }
    let rows = settings_of(id).len() + usize::from(id != "ring_menu");
    let h = rows as f32 * ROW + HEADER + 8.0;
    [(width - SETTINGS_W) / 2.0, (height - h) / 2.0, SETTINGS_W, h]
}

/// The dark box behind a row's key or text, so it reads as something to click.
fn field(batch: &mut HudBatch, atlas: &HudAtlas, right: f32, y: f32, shown: &str) {
    let w = HudBatch::text_width(atlas, shown) + 8.0;
    notched(batch, atlas, right - w + 4.0, y - 1.0, w, 13.0, rgba(0, 0, 0, 255));
    notched(batch, atlas, right - w + 5.0, y, w - 2.0, 11.0, rgba(10, 16, 56, 255));
}

/// How far a page shrinks to fit a short window, and the cursor as the unshrunk page sees it.
fn fit(h: f32, height: f32) -> f32 {
    ((height - 16.0) / h).min(1.0)
}

fn fitted(c: (f32, f32), [x, y, w, h]: [f32; 4], height: f32) -> (f32, f32) {
    let (k, cx, cy) = (fit(h, height), x + w / 2.0, y + h / 2.0);
    (cx + (c.0 - cx) / k, cy + (c.1 - cy) / k)
}

fn switch(batch: &mut HudBatch, atlas: &HudAtlas, right: f32, y: f32, enabled: bool) {
    notched(batch, atlas, right - 23.0, y, 24.0, 11.0, rgba(0, 0, 0, 255));
    notched(batch, atlas, right - 22.0, y + 1.0, 22.0, 9.0, if enabled { rgba(47, 157, 85, 255) } else { rgba(10, 16, 56, 255) });
    let knob = if enabled { right - 10.0 } else { right - 22.0 };
    notched(batch, atlas, knob, y + 1.0, 10.0, 9.0, if enabled { rgba(255, 255, 255, 255) } else { rgba(120, 132, 184, 255) });
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
    sprite(batch, atlas, &BACK, x + 5.0, y + 4.0, 1.0);
    text(batch, atlas, "Draw crosshair", x + 26.0, y + 8.0, rgba(255, 255, 255, 255), false);
    let (gx, gy) = (x + 8.0, y + HEADER + 4.0);
    for row in 0..15 {
        let bits = crosshair_row(row);
        for col in 0..15 {
            let (cx, cy) = (gx + col as f32 * CELL, gy + row as f32 * CELL);
            let hot = cursor.0 >= cx && cursor.0 < cx + CELL && cursor.1 >= cy && cursor.1 < cy + CELL;
            let fill = if bits >> col & 1 == 1 {
                rgba(255, 224, 96, 255)
            } else if hot {
                rgba(90, 104, 170, 255)
            } else if row == 7 && col == 7 {
                rgba(40, 52, 110, 255)
            } else {
                rgba(10, 16, 56, 255)
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
const POP_W: f32 = 110.0;

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
    (x, y, s, value(id, "panel") == 1)
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
fn snapped(index: usize, x: f32, y: f32, w: f32, h: f32, size: (f32, f32)) -> (f32, f32) {
    const NEAR: f32 = 4.0;
    let mut xs = vec![0.0, size.0, size.0 / 2.0];
    let mut ys = vec![0.0, size.1, size.1 / 2.0];
    for other in (0..ELEMENTS.len()).filter(|o| *o != index && on(ELEMENTS[*o].0)) {
        if let Some([ox, oy, ow, oh]) = element_box(other) {
            xs.extend([ox, ox + ow]);
            ys.extend([oy, oy + oh]);
        }
    }
    let fit = |at: f32, length: f32, lines: &[f32]| {
        let mut best = at;
        let mut gap = NEAR;
        for line in lines {
            for (edge, shift) in [(at, 0.0), (at + length, length), (at + length / 2.0, length / 2.0)] {
                if (edge - line).abs() < gap {
                    gap = (edge - line).abs();
                    best = line - shift;
                }
            }
        }
        best
    };
    (fit(x, w, &xs), fit(y, h, &ys))
}

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
    let h = layout(&element_items()).1 * ROW + 8.0;
    [size.0 / 2.0 - 150.0, 34.0, 300.0, h]
}

/// Beside the element, on the side of the screen with room: left half opens right, right half opens left.
fn popup_rect(index: usize, size: (f32, f32)) -> [f32; 4] {
    let [x, y, w, _] = element_box(index).unwrap_or([size.0 / 2.0, size.1 / 2.0, 0.0, 0.0]);
    let h = (settings_of(ELEMENTS[index].0).len() + 1) as f32 * ROW + HEADER + 8.0;
    let px = if x + w / 2.0 < size.0 / 2.0 { x + w + 6.0 } else { x - SETTINGS_W - 6.0 };
    [px.clamp(0.0, (size.0 - SETTINGS_W).max(0.0)), y.min(size.1 - h).max(0.0), SETTINGS_W, h]
}

fn draw_hud_editor(batch: &mut HudBatch, atlas: &HudAtlas, cursor: (f32, f32)) {
    let size = (batch.width, batch.height);
    let white = rgba(255, 255, 255, 255);
    set_screen(size.0, size.1);
    let (selected, snap, list, popup, hidden, group, band) = ed(|e| {
        e.cursor = cursor;
        e.size = size;
        (e.selected, e.snap, e.list, e.popup, e.bar_hidden, e.group.clone(), e.band)
    });
    for (index, (id, name)) in ELEMENTS.iter().enumerate() {
        let Some(b) = element_box(index).filter(|_| on(id)) else {
            continue;
        };
        let [x, y, w, h] = b;
        // The real element shows through; only the one under the cursor is marked and named.
        if cursor.0 >= x && cursor.0 < x + w && cursor.1 >= y && cursor.1 < y + h {
            notched(batch, atlas, x, y, w, h, rgba(255, 255, 255, 40));
            text(batch, atlas, name, x, y - 10.0, white, true);
        }
        if selected == Some(index) || group.contains(&index) {
            for edge in [[x, y, w, 1.0], [x, y + h - 1.0, w, 1.0], [x, y, 1.0, h], [x + w - 1.0, y, 1.0, h]] {
                batch.fill(atlas, edge[0], edge[1], edge[2], edge[3], rgba(127, 147, 224, 255));
            }
        }
        if selected == Some(index) && group.len() < 2 {
            for (_, _, hx, hy) in handles(b) {
                notched(batch, atlas, hx - 3.0, hy - 3.0, 6.0, 6.0, rgba(0, 0, 0, 255));
                notched(batch, atlas, hx - 2.0, hy - 2.0, 4.0, 4.0, rgba(255, 224, 96, 255));
            }
            let tag = format!("{name} {}%", (scale_of(index) * 100.0).round());
            text(batch, atlas, &tag, x, (y - 11.0).max(0.0), white, true);
        }
    }
    if let Some([from, to]) = band {
        let (x, y, w, h) = (from.0.min(to.0), from.1.min(to.1), (from.0 - to.0).abs(), (from.1 - to.1).abs());
        batch.fill(atlas, x, y, w, h, rgba(127, 147, 224, 50));
        for edge in [[x, y, w, 1.0], [x, y + h - 1.0, w, 1.0], [x, y, 1.0, h], [x + w - 1.0, y, 1.0, h]] {
            batch.fill(atlas, edge[0], edge[1], edge[2], edge[3], rgba(127, 147, 224, 255));
        }
    }
    if !hidden {
        for (i, tool) in TOOLS.iter().enumerate() {
            let r = tool_rect(i, size);
            let lit = (i == 0 && snap) || (i == 1 && list);
            frame(batch, atlas, r[0], r[1], r[2], r[3]);
            if inside(cursor, r) {
                notched(batch, atlas, r[0] + 2.0, r[1] + 2.0, r[2] - 4.0, r[3] - 4.0, rgba(255, 255, 255, 60));
            }
            let ink = if lit { rgba(255, 224, 96, 255) } else { white };
            let tool = if i == 2 && selected.is_none() && group.is_empty() { "Reset all" } else { *tool };
            centered(batch, atlas, tool, r[0] + r[2] / 2.0, r[1] + 5.0, ink);
        }
    }
    if list {
        let [x, y, w, h] = list_rect(size);
        frame(batch, atlas, x, y, w, h);
        let shown = element_items();
        let (places, _) = layout(&shown);
        for ((id, name, _), (col, row)) in shown.iter().zip(places) {
            let (rx, ry) = (x + 6.0 + col * 146.0, y + 5.0 + row * ROW);
            if *id == "#" {
                text(batch, atlas, name, rx, ry + 3.0, rgba(255, 224, 96, 255), false);
                continue;
            }
            text(batch, atlas, name, rx, ry + 3.0, white, false);
            switch(batch, atlas, rx + 136.0, ry + 1.0, on(id));
        }
    }
    if let Some(index) = popup {
        draw_panel(batch, atlas, cursor, ELEMENTS[index]);
    }
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
            let row = ((cursor.1 - r[1] - 3.0) / ROW).floor().max(0.0);
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
            remember(&chosen());
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
}

/// The pointer moved with the button held.
pub fn drag(cursor: (f32, f32), size: (f32, f32)) {
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
            let (mut nx, mut ny) = ((cursor.0 - dx).clamp(0.0, (size.0 - w).max(0.0)), (cursor.1 - dy).clamp(0.0, (size.1 - h).max(0.0)));
            if snap {
                (nx, ny) = snapped(index, nx, ny, w, h, size);
            }
            for other in chosen().into_iter().filter(|i| *i != index) {
                if let Some(b) = element_box(other) {
                    move_to(other, (b[0] + nx - x).clamp(0.0, (size.0 - b[2]).max(0.0)), (b[1] + ny - y).clamp(0.0, (size.1 - b[3]).max(0.0)));
                }
            }
            move_to(index, nx, ny);
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

fn draw_panel(batch: &mut HudBatch, atlas: &HudAtlas, cursor: (f32, f32), (id, name): (&'static str, &'static str)) {
    let opened = with(|s| age(s));
    let [x, y, w, h] = settings_rect(batch.width, batch.height, id);
    let cursor = fitted(cursor, [x, y, w, h], batch.height);
    let start = batch.vertices.len();
    frame(batch, atlas, x, y, w, h);
    sprite(batch, atlas, &BACK, x + 5.0, y + 4.0, 1.0);
    text(batch, atlas, name, x + 26.0, y + 8.0, rgba(255, 255, 255, 255), false);
    batch.fill(atlas, x + 5.0, y + HEADER - 2.0, w - 10.0, 1.0, rgba(127, 147, 224, 255));
    let right = x + w - 12.0;
    let white = rgba(255, 255, 255, 255);
    let mut ry = y + HEADER + 5.0;
    if id == "ring_menu" {
        ry -= ROW; // A key, not a module: nothing to switch.
    } else {
        text(batch, atlas, "Enabled", x + 11.0, ry + 2.0, white, false);
        switch(batch, atlas, right, ry, on(id));
    }
    for (key, label, min, max, _) in settings_of(id) {
        ry += ROW;
        if cursor.0 >= x && cursor.0 < x + w && cursor.1 >= ry - 4.0 && cursor.1 < ry + ROW - 4.0 {
            notched(batch, atlas, x + 6.0, ry - 4.0, w - 12.0, ROW - 1.0, rgba(255, 255, 255, 60));
        }
        if key.starts_with("note_") {
            text(batch, atlas, label, x + 11.0, ry + 2.0, rgba(150, 160, 200, 255), false);
            continue;
        }
        text(batch, atlas, label, x + 11.0, ry + 2.0, white, false);
        let current = value(id, key);
        if TEXT_ROWS.contains(&(id, *key)) {
            let editing = TEXT_FOCUS.lock().is_ok_and(|f| *f == Some((id, *key)));
            let value = text_value(id, key);
            let tail: String = value.chars().rev().take(22).collect::<Vec<_>>().into_iter().rev().collect();
            let shown = if editing { format!("{tail}_") } else if value.is_empty() { "Click to type".to_owned() } else { tail };
            let ink = if editing { rgba(255, 224, 96, 255) } else if value.is_empty() { rgba(150, 160, 200, 255) } else { white };
            field(batch, atlas, right, ry, &shown);
            text(batch, atlas, &shown, right - HudBatch::text_width(atlas, &shown), ry + 2.0, ink, false);
        } else if let Some(bind) = key.strip_prefix("key_") {
            let waiting = LISTENING.lock().is_ok_and(|l| *l == Some(bind));
            let shown = if waiting {
                "Press a key".to_owned()
            } else {
                KEY_NAMES.lock().ok().and_then(|n| n.iter().find(|k| k.0 == bind).map(|k| k.1.clone())).unwrap_or_default()
            };
            field(batch, atlas, right, ry, &shown);
            text(batch, atlas, &shown, right - HudBatch::text_width(atlas, &shown), ry + 2.0, if waiting { rgba(255, 224, 96, 255) } else { white }, false);
        } else if key.ends_with("color") {
            for (i, color) in COLORS.iter().enumerate() {
                let bx = right - 80.0 + i as f32 * 10.0;
                notched(batch, atlas, bx - 1.0, ry, 10.0, 11.0, if i as i32 == current { white } else { rgba(0, 0, 0, 255) });
                let [r, g, b] = [(color >> 16) as u8, (color >> 8) as u8, *color as u8];
                notched(batch, atlas, bx, ry + 1.0, 8.0, 9.0, rgba(r, g, b, 255));
            }
            // The custom swatch shows the picked color, or a rainbow until there is one.
            let bx = right - 90.0;
            notched(batch, atlas, bx - 1.0, ry, 10.0, 11.0, if current >= CUSTOM { white } else { rgba(0, 0, 0, 255) });
            if current >= CUSTOM {
                let c = color_of(current);
                notched(batch, atlas, bx, ry + 1.0, 8.0, 9.0, rgba((c >> 16) as u8, (c >> 8) as u8, c as u8, 255));
            } else {
                notched(batch, atlas, bx, ry + 1.0, 8.0, 9.0, rgba(30, 44, 120, 255));
                centered(batch, atlas, "+", bx + 4.0, ry + 1.0, white);
            }
        } else if *min == 0 && *max == 1 {
            switch(batch, atlas, right, ry, current == 1);
        } else {
            let steps = max - min + 1;
            let (count, width) = if steps as f32 <= BLOCKS { (steps, BLOCKS * BLOCK / steps as f32) } else { (BLOCKS as i32, BLOCK) };
            let share = (current - min) as f32 / (max - min) as f32;
            let filled = if steps as f32 <= BLOCKS {
                current - min + 1
            } else if current == 0 {
                0 // Empty means off.
            } else {
                ((share * BLOCKS).round() as i32).max(1)
            };
            for i in 0..count {
                let bx = right - STEPPER - BLOCKS * BLOCK + i as f32 * width;
                batch.fill(atlas, bx, ry, width - 1.0, 11.0, rgba(0, 0, 0, 255));
                batch.fill(atlas, bx + 1.0, ry + 1.0, width - 3.0, 9.0, if i < filled { rgba(255, 224, 96, 255) } else { rgba(10, 16, 56, 255) });
            }
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
            let minus = right - STEPPER - BLOCKS * BLOCK - STEPPER;
            for (bx, sign) in [(minus, "-"), (right - 11.0, "+")] {
                notched(batch, atlas, bx, ry, 11.0, 11.0, rgba(30, 44, 120, 255));
                centered(batch, atlas, sign, bx + 5.5, ry + 2.0, white);
            }
            text(batch, atlas, &shown, minus - 4.0 - HudBatch::text_width(atlas, &shown), ry + 2.0, white, false);
        }
    }
    let pop = back_ease(opened * 14.0) * fit(h, batch.height);
    batch.scale_since(start, x + w / 2.0, y + h / 2.0, pop, pop);
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
    let cursor = fitted(cursor, [x, y, w, h], size.1);
    if cursor.0 >= x && cursor.0 < x + 26.0 && cursor.1 >= y && cursor.1 < y + HEADER {
        back();
        return;
    }
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
        set_value(id, key, 1 - value(id, key));
    } else {
        let left = right - STEPPER - BLOCKS * BLOCK;
        if cursor.0 >= right - 11.0 {
            set_value(id, key, (value(id, key) + 1).min(max));
        } else if cursor.0 >= left - STEPPER && cursor.0 < left {
            set_value(id, key, (value(id, key) - 1).max(min));
        } else if cursor.0 >= left && cursor.0 < left + BLOCKS * BLOCK {
            let steps = max - min + 1;
            let share = ((cursor.0 - left) / (BLOCKS * BLOCK)).clamp(0.0, 1.0);
            let picked = if steps as f32 <= BLOCKS {
                min + ((share * steps as f32) as i32).min(steps - 1)
            } else {
                min + (share * (max - min) as f32).round() as i32
            };
            set_value(id, key, picked);
        }
    }
}

static OWN_NAME: Mutex<String> = Mutex::new(String::new());

static OWN_EYE: Mutex<[f64; 3]> = Mutex::new([0.0; 3]);

/// Where your eyes are, for Hitboxes' reach color.
pub fn set_eye(eye: [f64; 3]) {
    if let Ok(mut own) = OWN_EYE.lock() {
        *own = eye;
    }
}

pub fn eye() -> [f64; 3] {
    OWN_EYE.lock().map_or([0.0; 3], |e| *e)
}

thread_local! {
    static RAISED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// The next player model posed leaves out its flat outer layers.
pub fn set_raised() {
    RAISED.set(true);
}

pub fn take_raised() -> bool {
    RAISED.take()
}

static OWN_SKIN: Mutex<String> = Mutex::new(String::new());

/// Your skin's sprite, for the face on the ring.
pub fn set_own_skin(name: &str) {
    if let Ok(mut own) = OWN_SKIN.lock()
        && *own != name
    {
        *own = name.to_owned();
    }
}

static LISTENING: Mutex<Option<&'static str>> = Mutex::new(None);
static TEXT_FOCUS: Mutex<Option<(&'static str, &'static str)>> = Mutex::new(None);
static TEXTS: Mutex<Vec<(String, String)>> = Mutex::new(Vec::new());
const TEXT_ROWS: [(&str, &str); 2] = [("better_chat", "text_filter"), ("freelook", "text_blocked")];

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
static PICKED: Mutex<Option<(&'static str, &'static str)>> = Mutex::new(None);
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
    if let Some(bind) = take_listening() {
        if key != winit::keyboard::KeyCode::Escape && let (Some(name), Ok(mut picked)) = (name, PICKED.lock()) {
            *picked = Some((bind, name));
        }
        return true;
    }
    if with(|s| s.by_key && s.page.is_none()) {
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
        let closing = !has && name.is_some_and(|n| MENU_KEY.lock().is_ok_and(|m| *m == n));
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
    if with(|s| s.by_key) && name.is_some_and(|n| MENU_KEY.lock().is_ok_and(|m| *m == n)) {
        CLOSE.store(true, std::sync::atomic::Ordering::Relaxed);
        return true;
    }
    false
}

/// The mod key just picked, for the frame hook to save.
pub fn take_picked() -> Option<(&'static str, &'static str)> {
    PICKED.lock().ok()?.take()
}

/// Whether the menu key asked the ring to close.
pub fn take_close() -> bool {
    CLOSE.swap(false, std::sync::atomic::Ordering::Relaxed)
}

pub fn set_menu_key(key: &str) {
    if let Ok(mut own) = MENU_KEY.lock() && *own != key {
        key.clone_into(&mut own);
    }
}
static KEY_NAMES: Mutex<Vec<(&'static str, String)>> = Mutex::new(Vec::new());

/// The mod key waiting for its next press, cleared by reading it.
pub fn take_listening() -> Option<&'static str> {
    LISTENING.lock().ok()?.take()
}

/// The key a mod key is on now, for its row.
pub fn set_key_name(name: &'static str, key: &str) {
    if let Ok(mut names) = KEY_NAMES.lock() {
        names.retain(|k| k.0 != name);
        names.push((name, key.trim_start_matches("key.keyboard.").trim_start_matches("key.mouse.").replace('.', " ")));
    }
}

static REACH: Mutex<f64> = Mutex::new(3.0);

/// Your attack range now, for the hitbox color.
pub fn set_reach(reach: f64) {
    if let Ok(mut own) = REACH.lock() {
        *own = reach;
    }
}

pub fn reach() -> f64 {
    REACH.lock().map_or(3.0, |r| *r)
}

static LOOK_DISTANCE: Mutex<f32> = Mutex::new(1.0);

/// Freelook's camera distance, as a share of vanilla's; 1 when not freelooking.
pub fn set_look_distance(share: f32) {
    if let Ok(mut own) = LOOK_DISTANCE.lock() {
        *own = share;
    }
}

pub fn look_distance() -> f32 {
    // With the option off, a distance set earlier stops counting.
    if value("freelook", "scroll") != 1 {
        return 1.0;
    }
    LOOK_DISTANCE.lock().map_or(1.0, |d| *d)
}

static SHIELD_TINT: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

/// The color your shield draws in now; 0 for none.
pub fn set_shield_tint(rgb: u32) {
    SHIELD_TINT.store(rgb, std::sync::atomic::Ordering::Relaxed);
}

pub fn shield_tint() -> u32 {
    TINT_HELD.get().unwrap_or(0)
}

/// Your own shield's color now.
pub fn own_shield_color() -> u32 {
    SHIELD_TINT.load(std::sync::atomic::Ordering::Relaxed)
}

thread_local! {
    static TINT_HELD: std::cell::Cell<Option<u32>> = const { std::cell::Cell::new(None) };
}

/// The color shields take while a holder's items draw; None ends it, so the GUI stays plain.
pub fn hold_shield_tint(tint: Option<u32>) {
    TINT_HELD.set(tint);
}

/// The status color for a share of the wait left, from the Shield status settings.
pub fn shield_color(wait: f32) -> u32 {
    let (ready, off) = (color_of(value("shield_status", "enabled_color")), color_of(value("shield_status", "disabled_color")));
    if wait <= 0.0 {
        return ready;
    }
    if value("shield_status", "fade") != 1 {
        return off;
    }
    let at = |shift: u32| {
        let (a, b) = (((ready >> shift) & 255) as f32, ((off >> shift) & 255) as f32);
        ((a + (b - a) * wait) as u32) << shift
    };
    at(16) | at(8) | at(0)
}

static SHIELD_ARMS: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0);

/// Which arms hold a shield this frame.
pub fn set_shield_arms(left: bool, right: bool) {
    SHIELD_ARMS.store(u8::from(left) | u8::from(right) << 1, std::sync::atomic::Ordering::Relaxed);
}

pub fn shield_in(left: bool) -> bool {
    SHIELD_ARMS.load(std::sync::atomic::Ordering::Relaxed) & if left { 1 } else { 2 } != 0
}

static CHAT_QUERY: Mutex<String> = Mutex::new(String::new());
static CHAT_FOCUS: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

pub fn chat_search_on() -> bool {
    on("better_chat") && value("better_chat", "search") == 1
}

pub fn chat_query() -> String {
    CHAT_QUERY.lock().map(|q| q.clone()).unwrap_or_default()
}

pub fn chat_focus() -> bool {
    CHAT_FOCUS.load(std::sync::atomic::Ordering::Relaxed)
}

pub fn set_chat_focus(focus: bool) {
    CHAT_FOCUS.store(focus, std::sync::atomic::Ordering::Relaxed);
}

pub fn chat_type(text: &str) {
    if let Ok(mut q) = CHAT_QUERY.lock() && q.len() < 40 {
        q.push_str(text);
    }
}

pub fn chat_backspace() {
    if let Ok(mut q) = CHAT_QUERY.lock() {
        q.pop();
    }
}

/// The chat closed: the search goes with it.
pub fn chat_closed() {
    set_chat_focus(false);
    if let Ok(mut q) = CHAT_QUERY.lock() && !q.is_empty() {
        q.clear();
    }
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

static PLAYERS: Mutex<(Option<Instant>, Vec<(String, String, u128, String)>)> = Mutex::new((None, Vec::new()));
static CHAT_HOVER: Mutex<String> = Mutex::new(String::new());
static CHAT_RESTORING: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static F3: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// The chat message under the cursor, for right-click copy.
pub fn set_chat_hover(message: String) {
    if let Ok(mut own) = CHAT_HOVER.lock() {
        *own = message;
    }
}

/// The hovered message without the mark and time Better chat put on it.
pub fn chat_copy() -> String {
    let text = CHAT_HOVER.lock().map(|t| t.clone()).unwrap_or_default();
    let text = text.trim_start().trim_start_matches("\u{258c} ");
    let stamped = matches!(text.as_bytes(), [a, b, b':', c, d, b' ', ..] if [a, b, c, d].iter().all(|n| n.is_ascii_digit()));
    (if stamped { &text[6..] } else { text }).to_owned()
}

/// Old lines going back into chat, which keep the time they already have.
pub fn set_chat_restoring(on: bool) {
    CHAT_RESTORING.store(on, std::sync::atomic::Ordering::Relaxed);
}

pub fn chat_restoring() -> bool {
    CHAT_RESTORING.load(std::sync::atomic::Ordering::Relaxed)
}

pub fn set_f3(held: bool) {
    F3.store(held, std::sync::atomic::Ordering::Relaxed);
}

/// Whether F3 is down: F3+D is the one clear that still clears.
pub fn f3_held() -> bool {
    F3.load(std::sync::atomic::Ordering::Relaxed)
}
static CHAT_SENDER: Mutex<Option<u128>> = Mutex::new(None);
static LINE_SKINS: Mutex<Vec<(String, String)>> = Mutex::new(Vec::new());
static PLAYER_SPOTS: Mutex<Vec<[f64; 3]>> = Mutex::new(Vec::new());

/// Where players stand this frame, for the hitbox filter.
pub fn set_player_spots(spots: Vec<[f64; 3]>) {
    if let Ok(mut own) = PLAYER_SPOTS.lock() {
        *own = spots;
    }
}

pub fn player_at(position: [f64; 3]) -> bool {
    PLAYER_SPOTS.lock().is_ok_and(|s| s.iter().any(|p| (p[0] - position[0]).abs() < 0.75 && (p[1] - position[1]).abs() < 0.75 && (p[2] - position[2]).abs() < 0.75))
}

/// The server named who sent the next chat line.
pub fn set_chat_sender(id: u128) {
    if let Ok(mut own) = CHAT_SENDER.lock() {
        *own = Some(id);
    }
}

/// The next line's skin: the sender the server named, else the name it opens with.
pub fn take_line_skin(line: &str) -> Option<String> {
    let named = CHAT_SENDER.lock().ok().and_then(|mut s| s.take());
    let known = named.and_then(|id| PLAYERS.lock().ok()?.1.iter().find(|p| p.2 == id).map(|p| p.1.clone()));
    known.or_else(|| sender_skin(line))
}

/// Remembers whose face a shown line gets, by how it starts.
pub fn remember_line(line: &str, skin: String) {
    if let Ok(mut lines) = LINE_SKINS.lock() {
        lines.push((line.chars().take(24).collect(), skin));
        if lines.len() > 200 {
            lines.remove(0);
        }
    }
}

pub fn line_skin(line: &str) -> Option<String> {
    let start: String = line.chars().take(24).collect();
    LINE_SKINS.lock().ok()?.iter().rev().find(|l| l.0 == start).map(|l| l.1.clone())
}

/// Whether the player list is due a refresh, once a second while heads are on.
pub fn players_due() -> bool {
    on("better_chat") && value("better_chat", "heads") == 1 && PLAYERS.lock().is_ok_and(|p| p.0.is_none_or(|t| t.elapsed().as_secs() >= 1))
}

/// Names, skins, ids and tab-list nicknames of the players online, for chat heads.
pub fn set_players(players: Vec<(String, String, u128, String)>) {
    if let Ok(mut own) = PLAYERS.lock() {
        *own = (Some(Instant::now()), players);
    }
}

/// The skin of the player whose name a chat line opens with; timestamps and brackets are skipped.
pub fn sender_skin(line: &str) -> Option<String> {
    let players = PLAYERS.lock().ok()?;
    // The sender comes before the colon or arrow, after any rank; a server's nickname is matched too.
    let opening = line.split([':', '\u{bb}', '>', '\u{2192}']).next().unwrap_or(line);
    opening.split_whitespace().take(6).find_map(|word| {
        let name = word.trim_matches(|c: char| !c.is_alphanumeric() && c != '_');
        let real = |p: &&(String, String, u128, String)| p.0.chars().all(|c| c.is_alphanumeric() || c == '_');
        players.1.iter().filter(real).find(|p| !name.is_empty() && (p.0.eq_ignore_ascii_case(name) || p.3.eq_ignore_ascii_case(name))).map(|p| p.1.clone())
    })
}

static SERVER: Mutex<String> = Mutex::new(String::new());

static SERVER_SET: Mutex<Option<Instant>> = Mutex::new(None);

/// The address you joined, for the Server element.
pub fn set_server(address: &str) {
    if let Ok(mut own) = SERVER.lock() {
        address.clone_into(&mut own);
    }
    if let Ok(mut at) = SERVER_SET.lock() {
        *at = Some(Instant::now());
    }
}

/// At the menu the address goes, unless a join has only just started.
pub fn server_left() {
    if SERVER_SET.lock().is_ok_and(|at| at.is_none_or(|t| t.elapsed().as_secs() >= 30))
        && let Ok(mut own) = SERVER.lock()
    {
        own.clear();
    }
}

pub fn server() -> String {
    SERVER.lock().map(|s| s.clone()).unwrap_or_default()
}

/// Your name, for Better chat's mark.
pub fn set_own_name(name: String) {
    if let Ok(mut own) = OWN_NAME.lock() {
        *own = name;
    }
}

pub fn own_name() -> String {
    OWN_NAME.lock().map(|n| n.clone()).unwrap_or_default()
}

/// The cape you wear: its sprite name, if any.
pub fn cape() -> Option<String> {
    let id = with(|s| s.on.iter().find(|(k, v)| **v && k.starts_with("cape_")).map(|(k, _)| k.clone()))?;
    Some(cape_sprite(&id))
}

/// This moment's sprite of a cape: 8 frames a second as in Java, the grey copy when tinted.
fn cape_sprite(id: &str) -> String {
    static START: LazyLock<Instant> = LazyLock::new(Instant::now);
    let name = &id[5..];
    let frames = match name {
        "starlight" | "galaxy" => 8,
        "aurora" | "ember" | "bamboo" | "fuji_sunset" | "koi_pond" | "lantern_night" | "sakura" | "wisteria" => 16,
        _ => 0,
    };
    let frame = if frames == 0 { String::new() } else { format!("_{}", START.elapsed().as_millis() / 125 % frames) };
    let grey = if cape_tint().is_some() { "_g" } else { "" };
    format!("kyro_capes:entity/{name}{frame}{grey}")
}

/// The picked cape color as a vertex color, if any.
pub fn cape_tint() -> Option<u32> {
    let index = value("capes", "tint");
    (index > 0).then(|| {
        let color = if index >= CUSTOM { color_of(index) } else { COLORS[(index - 1) as usize] };
        rgba((color >> 16) as u8, (color >> 8) as u8, color as u8, 255)
    })
}

fn toggle(id: &str) {
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

pub fn click(cursor: (f32, f32), size: (f32, f32), right_click: bool) {
    if picker_input(cursor, size, false) {
        return;
    }
    if let Some((id, _)) = with(|s| s.page) {
        click_settings(cursor, size, id, right_click);
    } else if with(|s| s.list) {
        let category = with(|s| s.selected);
        if showing_capes(category) {
            click_capes(cursor, size, right_click);
            return;
        }
        let [x, y, w, h] = panel_rect(size.0, size.1, category);
        let cursor = fitted(cursor, [x, y, w, h], size.1);
        if cursor.0 >= x && cursor.0 < x + 26.0 && cursor.1 >= y && cursor.1 < y + HEADER {
            back();
            return;
        }
        let column = (w - 12.0) / 2.0;
        let col = ((cursor.0 - x - 6.0) / column).floor();
        let row = ((cursor.1 - y - HEADER - 1.0) / ROW).floor();
        let shown = items(category);
        let (places, _) = layout(&shown);
        let found = shown.iter().zip(places).find(|(m, p)| m.0 != "#" && p.0 == col && p.1 == row).map(|(m, _)| *m);
        if let Some((id, name, _)) = found {
            let on_switch = cursor.0 >= x + 6.0 + (col + 1.0) * column - 36.0;
            let has_page = id.ends_with("_editor") || !settings_of(id).is_empty();
            if has_page && (right_click || !on_switch) {
                with(|s| {
                    s.page = Some((id, name));
                    s.opened = Some(Instant::now());
                });
            } else {
                toggle(id);
            }
        }
    } else {
        let c = (size.0 / 2.0, size.1 / 2.0 + 10.0);
        if let Some(picked) = pick(cursor, c) {
            with(|s| {
                s.selected = picked; // The click's own spot, not the last hover.
                if picked == 0 {
                    s.page = Some(("hud_editor", "HUD editor"));
                } else {
                    s.list = true;
                }
                s.opened = Some(Instant::now());
            });
        }
    }
}
