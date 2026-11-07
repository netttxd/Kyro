// Zoom and Freelook: the state behind their keys, wheel and camera.

use std::cell::Cell;

const ZOOM_MIN: f32 = 2.0;
const ZOOM_MAX: f32 = 128.0;
/// Four wheel steps to each doubling.
const ZOOM_STEP: f32 = 1.19;

thread_local! {
    static ZOOM: Cell<(bool, f32)> = const { Cell::new((false, 4.0)) };
    static EASED: Cell<(f32, Option<std::time::Instant>)> = const { Cell::new((1.0, None)) };
    static LOOK: Cell<Option<(f32, f32)>> = const { Cell::new(None) };
    static NESTED: Cell<bool> = const { Cell::new(false) };
}

/// The key bound to one of the ring's KEYS, or its default.
pub fn bound_key(game: &crate::game::Game<'_>, name: &str) -> String {
    use horizon_client::options_ui::kyro_ring as ring;
    let default = ring::KEYS.iter().find(|k| k[0] == name).map_or("key.keyboard.unknown", |k| k[2]);
    game.native_mod_key(crate::game::port_kyro::ID, name, default)
}

/// Shift, Ctrl and Alt held now, as the ring's bind masks keep them.
pub fn held_modifiers(game: &crate::game::Game<'_>) -> i32 {
    use horizon_client::options_ui::kyro_ring as ring;
    let bit = |held: bool, bit: i32| if held { bit } else { 0 };
    bit(game.modifiers.shift_key(), ring::SHIFT_BIT) | bit(game.modifiers.control_key(), ring::CTRL_BIT) | bit(game.modifiers.alt_key(), ring::ALT_BIT)
}

/// Binds one of Kyro's keys and saves the options.
pub fn save_key(game: &mut crate::game::Game<'_>, name: &str, key: &str) {
    game.settings.mods.keys.entry(crate::game::port_kyro::ID.to_owned()).or_default().insert(name.to_owned(), key.to_owned());
    // The save must hold your own menu blur, not Glass's stand-in.
    let shown = game.settings.menu_blur;
    game.settings.menu_blur = horizon_client::options_ui::kyro_ring::own_blur().unwrap_or(shown);
    game.settings.save(&crate::game::Game::options_path());
    game.settings.menu_blur = shown;
}

pub fn zoom_key(playing: bool, pressed: bool, start: usize) {
    let (held, level) = ZOOM.get();
    if pressed && playing && !held {
        ZOOM.set((true, (start as f32).clamp(ZOOM_MIN, ZOOM_MAX)));
    } else if !pressed {
        ZOOM.set((false, level));
    }
}

/// The wheel while zooming changes how far; returns whether it used the scroll.
pub fn zoom_wheel(steps: f32) -> bool {
    use horizon_client::options_ui::kyro_ring as ring;
    if LOOK.get().is_some() && steps != 0.0 && ring::value("freelook", "scroll") == 1 {
        let share = horizon_client::options_ui::kyro_world::look_distance() * if steps > 0.0 { 0.85 } else { 1.0 / 0.85 };
        horizon_client::options_ui::kyro_world::set_look_distance(share.clamp(0.35, 3.0));
        return true;
    }
    let (held, level) = ZOOM.get();
    if !held || steps == 0.0 || ring::value("zoom", "scroll") == 0 {
        return false;
    }
    let next = level * if steps > 0.0 { ZOOM_STEP } else { 1.0 / ZOOM_STEP };
    ZOOM.set((true, next.clamp(ZOOM_MIN, ZOOM_MAX)));
    true
}

/// The zoom to apply now: factor, smooth, and how far the mouse turns.
pub fn zoom_view() -> Option<(f32, bool, f32)> {
    let (held, level) = ZOOM.get();
    let (eased, then) = EASED.get();
    let now = std::time::Instant::now();
    let target = if held { level } else { 1.0 };
    let passed = then.map_or(0.0, |t| now.duration_since(t).as_secs_f32()).min(0.1);
    // By ratio, so each doubling takes as long; out is the quicker, as Zoomify's.
    let share = 1.0 - (-passed * if held { 9.0 } else { 14.0 }).exp();
    let smooth = horizon_client::options_ui::kyro_ring::value("zoom", "smooth") == 1;
    let next = if smooth { eased * (target / eased).powf(share) } else { target };
    let next = if (next - target).abs() < 0.01 { target } else { next };
    EASED.set((next, Some(now)));
    (next > 1.0).then(|| (next, false, 1.0 / next))
}

// The freelook key: down keeps the camera's angles apart from the player's.
thread_local! {
    static STEPPED_BACK: Cell<bool> = const { Cell::new(false) };
}

pub fn looking() -> bool {
    LOOK.get().is_some()
}

/// Sets the flag and returns what it was.
pub fn set_stepped_back(on: bool) -> bool {
    STEPPED_BACK.replace(on)
}

static RAISED: std::sync::Mutex<Vec<(u32, std::time::Instant)>> = std::sync::Mutex::new(Vec::new());

/// Complete Shield Fixes: a player's shield shows raised only after the 5 ticks it takes to block.
pub fn shield_delay(game: &mut crate::game::Game<'_>) {
    use horizon_client::options_ui::kyro_ring as ring;
    let Ok(mut raised) = RAISED.lock() else {
        return;
    };
    if !ring::on("shield_status") || ring::value("shield_status", "delay") == 0 {
        raised.clear();
        return;
    }
    let now = std::time::Instant::now();
    let mut held = Vec::new();
    for m in game.mobs.iter_mut().filter(|m| &*m.kind.id == "minecraft:player") {
        let known = raised.iter().find(|r| r.0 == m.id).map(|r| r.1);
        if m.extra.using_item > 0 && (m.extra.blocking || known.is_some()) {
            let since = known.unwrap_or(now);
            m.extra.blocking = now.duration_since(since).as_millis() >= 250;
            held.push((m.id, since));
        }
    }
    *raised = held;
}

thread_local! {
    static OUTLINED: Cell<Option<[i32; 3]>> = const { Cell::new(None) };
}

pub fn set_outlined(pos: [i32; 3]) {
    OUTLINED.set(Some(pos));
}

pub fn take_outlined() -> Option<[i32; 3]> {
    OUTLINED.take()
}

pub fn look_key(playing: bool, pressed: bool, yaw: f32, pitch: f32) {
    if pressed && playing {
        LOOK.set(Some((yaw, pitch)));
    } else if !pressed {
        LOOK.set(None);
        horizon_client::options_ui::kyro_world::set_look_distance(1.0);
    }
}

/// The camera's own angles, once per nesting: the hooks call the original inside themselves.
pub fn begin() -> Option<(f32, f32)> {
    if NESTED.get() {
        return None;
    }
    let look = LOOK.get();
    if look.is_some() {
        NESTED.set(true);
    }
    look
}

pub fn end() {
    NESTED.set(false);
}

pub fn set_look(yaw: f32, pitch: f32) {
    LOOK.set(Some((yaw, pitch)));
}

thread_local! {
    static SPRINT: Cell<bool> = const { Cell::new(false) };
}

// Whether Sprint just turned on or off.
thread_local! {
    static SPRINT_TAPPED: Cell<bool> = const { Cell::new(false) };
}

/// Toggle mode: one tap of the sprint key keeps sprinting, the next stops.
pub fn sprint_tap() {
    SPRINT_TAPPED.set(!SPRINT_TAPPED.get());
}

pub fn sprint_tapped() -> bool {
    SPRINT_TAPPED.get()
}

pub fn sprint_changed(want: bool) -> bool {
    SPRINT.replace(want) != want
}

thread_local! {
    static SPEED: Cell<Option<(std::time::Instant, f64, f64, f64)>> = const { Cell::new(None) };
}

/// Horizontal blocks per second, measured over about a quarter second.
pub fn speed(x: f64, z: f64) -> f64 {
    let now = std::time::Instant::now();
    match SPEED.get() {
        Some((when, last_x, last_z, shown)) => {
            let seconds = now.duration_since(when).as_secs_f64();
            if seconds < 0.25 {
                return shown;
            }
            let value = (x - last_x).hypot(z - last_z) / seconds;
            SPEED.set(Some((now, x, z, value)));
            value
        }
        None => {
            SPEED.set(Some((now, x, z, 0.0)));
            0.0
        }
    }
}

thread_local! {
    static LAST_HIT: Cell<Option<std::time::Instant>> = const { Cell::new(None) };
    static COMBO: Cell<u32> = const { Cell::new(0) };
}

/// You hit something: restarts the marker and adds to the combo.
pub fn hit() {
    let live = LAST_HIT.get().is_some_and(|t| t.elapsed().as_secs_f32() < 2.5);
    COMBO.set(if live { COMBO.get() + 1 } else { 1 });
    LAST_HIT.set(Some(std::time::Instant::now()));
    EXHAUSTION.set(EXHAUSTION.get() + 0.1);
}

thread_local! {
    static EXHAUSTION: Cell<f32> = const { Cell::new(0.0) };
    static BURN: Cell<Option<(u64, [f64; 3], bool, i32, f32, f32)>> = const { Cell::new(None) };
}

/// Exhaustion never reaches the client, so this adds up what vanilla charges for what's seen here.
/// shortcut: swimming, mining and hits taken are left out, they barely move it. The server's value
/// would need a server mod, as AppleSkin's own does.
pub fn track_exhaustion(game: &crate::game::Game<'_>, food: i32, saturation: f32, health: f32) {
    let Some(p) = game.player.as_ref() else {
        return;
    };
    let pos = [p.position.x, p.position.y, p.position.z];
    let ground = p.ground.on_ground;
    let last = BURN.get();
    if last.is_some_and(|l| l.0 == game.ticks) {
        return;
    }
    BURN.set(Some((game.ticks, pos, ground, food, saturation, health)));
    let Some((_, was, grounded, last_food, last_saturation, last_health)) = last else {
        return;
    };
    // A point of saturation or food just went: the server's counter passed 4 and started over.
    if food < last_food || saturation < last_saturation {
        EXHAUSTION.set(0.0);
        return;
    }
    let mut burned = 0.0;
    if p.sprinting && ground {
        // The cap skips teleports.
        burned += 0.1 * ((pos[0] - was[0]).hypot(pos[2] - was[2]) as f32).min(1.0);
    }
    if grounded && !ground && pos[1] > was[1] {
        burned += if p.sprinting { 0.2 } else { 0.05 };
    }
    // Natural regeneration costs 6 for each point healed.
    if food >= 18 {
        burned += 6.0 * (health - last_health).clamp(0.0, 1.0);
    }
    EXHAUSTION.set((EXHAUSTION.get() + burned).min(4.0));
}

pub fn exhaustion() -> f32 {
    EXHAUSTION.get()
}

/// The health this food level would regenerate, from AppleSkin's FoodHelper.
pub fn regen_health(mut food: i32, mut saturation: f32, mut exhaustion: f32) -> f32 {
    let mut health = 0.0;
    while food >= 18 {
        while exhaustion > 4.0 {
            exhaustion -= 4.0;
            if saturation > 0.0 {
                saturation = (saturation - 1.0).max(0.0);
            } else {
                food -= 1;
            }
        }
        if food >= 20 && saturation > f32::MIN_POSITIVE {
            let limited = saturation.min(6.0);
            let runs = ((4.000_000_5 - exhaustion) / limited).ceil().max(1.0);
            health += limited / 6.0 * runs;
            exhaustion += limited * runs;
        } else if food >= 18 {
            health += 1.0;
            exhaustion += 6.0;
        }
    }
    health
}

/// How strong the hit marker is now, 1 fading to 0 over its duration.
pub fn marker() -> f32 {
    let life = horizon_client::options_ui::kyro_ring::value("hit_markers", "duration") as f32 / 1000.0;
    LAST_HIT.get().map_or(0.0, |t| (1.0 - t.elapsed().as_secs_f32() / life).max(0.0))
}

static PING_SAMPLES: std::sync::Mutex<(Option<std::time::Instant>, Vec<i32>)> = std::sync::Mutex::new((None, Vec::new()));

/// The ping history, oldest first, one sample a second.
pub fn ping_samples(ms: i32) -> Vec<i32> {
    let Ok(mut samples) = PING_SAMPLES.lock() else {
        return Vec::new();
    };
    if samples.0.is_none_or(|t| t.elapsed().as_secs() >= 1) {
        samples.0 = Some(std::time::Instant::now());
        samples.1.push(ms);
        if samples.1.len() > 60 {
            samples.1.remove(0);
        }
    }
    samples.1.clone()
}

thread_local! {
    static CHARGE: Cell<(f32, Option<std::time::Instant>)> = const { Cell::new((1.0, None)) };
}

/// Whether your attack filled up again within the last 220 ms.
pub fn charged_pulse(strength: f32) -> bool {
    let (last, at) = CHARGE.get();
    let at = if last < 1.0 && strength >= 1.0 { Some(std::time::Instant::now()) } else { at };
    CHARGE.set((strength, at));
    at.is_some_and(|t| t.elapsed().as_millis() < 220)
}

static TARGET_NAME: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());

/// A hit player's name, from the remote hook that can read it.
pub fn set_target_name(name: &str) {
    if let Ok(mut own) = TARGET_NAME.lock() {
        name.clone_into(&mut own);
    }
}


thread_local! {
    static REACH: Cell<Option<f64>> = const { Cell::new(None) };
}

/// How far your last hit reached, along your aim.
pub fn set_reach(distance: f64) {
    REACH.set(Some(distance));
}

pub fn reach() -> Option<f64> {
    REACH.get()
}

static BROKEN: std::sync::Mutex<Vec<(u32, std::time::Instant)>> = std::sync::Mutex::new(Vec::new());

/// A player's shield was disabled: the server only says it happened, so the wait is the vanilla 5 seconds.
pub fn shield_broke(id: u32) {
    if let Ok(mut broken) = BROKEN.lock() {
        broken.retain(|b| b.0 != id);
        broken.push((id, std::time::Instant::now()));
    }
}

/// The entity id that stands for you: the mob list doesn't hold your own entity.
pub const YOU: u32 = u32::MAX;

/// The share of that wait left for an entity; 0 when its shield is fine.
pub fn shield_wait(id: u32) -> f32 {
    let Ok(mut broken) = BROKEN.lock() else {
        return 0.0;
    };
    broken.retain(|b| b.1.elapsed().as_secs() < 5);
    broken.iter().find(|b| b.0 == id).map_or(0.0, |b| 1.0 - b.1.elapsed().as_secs_f32() / 5.0)
}

static DIED: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(-1);

/// The server's death event for an entity.
pub fn entity_died(id: i32) {
    DIED.store(id, std::sync::atomic::Ordering::Relaxed);
}

struct FeedEntry {
    name: String,
    weapon: Option<crate::game::ItemStack>,
    player: bool,
    death: bool,
    streak: u32,
    at: std::time::Instant,
}

#[derive(Default)]
struct Kills {
    /// What you hit last: id, name, is a player, when.
    target: Option<(u32, String, bool, std::time::Instant)>,
    entries: Vec<FeedEntry>,
    streak: u32,
    dead: bool,
    killed: Option<std::time::Instant>,
}

static KILLS: std::sync::LazyLock<std::sync::Mutex<Kills>> = std::sync::LazyLock::new(Default::default);

/// CombatTracker and KillFeed: something you hit in the last 3 seconds died. Each HUD frame.
/// shortcut: Java uses the death event; here it is health 0, or in singleplayer the mob vanishing.
pub fn track_kills(game: &crate::game::Game<'_>) {
    // Taking a hit breaks the combo.
    if game.player.as_ref().is_some_and(|p| p.health.hurt.cooldown > 0) {
        COMBO.set(0);
    }
    let Ok(mut k) = KILLS.lock() else {
        return;
    };
    let now = std::time::Instant::now();
    // A dead mob stays in the crosshair a moment and would count again.
    if since_hit() < 0.1 && let Some(mob) = game.mob_hit().and_then(|m| game.mobs.get(m.0)).filter(|m| m.health > 0.0) {
        let player = &*mob.kind.id == "minecraft:player";
        let name = if player {
            TARGET_NAME.lock().map(|n| n.clone()).unwrap_or_default()
        } else {
            mob.custom_name.as_deref().map_or_else(|| mob.kind.id.trim_start_matches("minecraft:").replace('_', " "), str::to_owned)
        };
        HIT_ID.store(mob.id as i32, std::sync::atomic::Ordering::Relaxed);
        k.target = Some((mob.id, name, player, now));
    }
    if let Some((id, name, player, at)) = k.target.clone() {
        let mob = game.mobs.iter().find(|m| m.id == id && !m.removed);
        let died = DIED.swap(-1, std::sync::atomic::Ordering::Relaxed) == id as i32;
        let gone = died || mob.map_or(horizon_client::options_ui::kyro_world::server().is_empty(), |m| m.health <= 0.0);
        if now.duration_since(at).as_secs() >= 3 {
            k.target = None;
        } else if gone {
            k.target = None;
            k.killed = Some(now);
            if player {
                k.streak += 1;
            }
            let streak = if player { k.streak } else { 0 };
            k.entries.insert(0, FeedEntry { name, weapon: game.held().cloned(), player, death: false, streak, at: now });
        }
    }
    let dead = game.player.as_ref().is_some_and(|p| p.health.health() as f32 <= 0.0);
    if dead && !k.dead {
        k.streak = 0;
        k.entries.insert(0, FeedEntry { name: "You died".to_owned(), weapon: None, player: true, death: true, streak: 0, at: now });
    }
    k.dead = dead;
    k.entries.retain(|e| now.duration_since(e.at).as_secs() < 6);
    k.entries.truncate(4);
}

/// How strong the kill marker is now: 1 fading to 0 over 1.6 times the marker's duration.
pub fn kill_marker() -> f32 {
    let life = horizon_client::options_ui::kyro_ring::value("hit_markers", "duration") as f32 / 1000.0 * 1.6;
    KILLS.lock().ok().and_then(|k| k.killed).map_or(0.0, |t| (1.0 - t.elapsed().as_secs_f32() / life).max(0.0))
}

/// The kill feed: weapon, name, streak; newest first.
pub fn draw_feed(game: &crate::game::Game<'_>, batch: &mut horizon_client::hud::HudBatch) {
    use horizon_client::options_ui::kyro_ring as ring;
    const ROW: f32 = 14.0;
    let Ok(k) = KILLS.lock() else {
        return;
    };
    let height = k.entries.len().max(1) as f32 * ROW + 8.0;
    let width = k.entries.iter().map(|e| {
        let streak = if e.streak > 1 { 6.0 + game.native_text_width(&format!("x{}", e.streak)) } else { 0.0 };
        22.0 + game.native_text_width(&e.name) + streak + 6.0
    }).fold(60.0_f32, f32::max);
    let (x, y, zoom, panel) = ring::place("kill_feed", width, height, ((batch.width * 0.25).floor(), (batch.height * 0.1).floor()));
    let first = batch.vertices.len();
    if panel {
        game.native_fill(batch, [x, y, width, height], ring::hud_panel());
    }
    for (index, entry) in k.entries.iter().enumerate() {
        let row = y + 4.0 + index as f32 * ROW;
        let age = entry.at.elapsed().as_secs_f32();
        let alpha = if age < 5.0 { 1.0 } else { (6.0 - age).max(0.0) };
        if let Some(weapon) = &entry.weapon {
            let start = batch.vertices.len();
            game.draw_item_model(batch, weapon, x + 6.0, row - 1.0);
            batch.scale_since(start, x + 6.0, row - 1.0, 0.75, 0.75);
        }
        let ink = if entry.death { 0xff8d8d } else { if entry.player { 0xedf3f4 } else { 0xb9c3ca } };
        lyric_text(game, batch, &entry.name, (x + 22.0, row + 3.0), 1.0, ink, alpha, true);
        if entry.streak > 1 {
            let shown = format!("x{}", entry.streak);
            lyric_text(game, batch, &shown, (x + width - 6.0 - game.native_text_width(&shown), row + 3.0), 1.0, ring::panel_value(panel, -1, 0xffd166), alpha, true);
        }
    }
    batch.scale_since(first, x, y, zoom, zoom);
}

static TPS: std::sync::Mutex<(i64, Option<std::time::Instant>, f32)> = std::sync::Mutex::new((-1, None, -1.0));
static CRIT: std::sync::Mutex<Option<(i32, std::time::Instant)>> = std::sync::Mutex::new(None);
static HIT_ID: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(-1);

/// A new server: TPS and the streak start over.
pub fn world_reset() {
    if let Ok(mut t) = TPS.lock() {
        *t = (-1, None, -1.0);
    }
    if let Ok(mut k) = KILLS.lock() {
        *k = Kills::default();
    }
}

/// ServerTps.java: ticks passed over real seconds passed, between the server's time updates.
pub fn tps_time(game_time: i64) {
    let Ok(mut t) = TPS.lock() else {
        return;
    };
    let now = std::time::Instant::now();
    if let Some(last) = t.1.filter(|_| t.0 >= 0 && game_time > t.0) {
        let seconds = now.duration_since(last).as_secs_f32();
        if seconds > 0.2 {
            let sample = ((game_time - t.0) as f32 / seconds).min(20.0);
            t.2 = if t.2 < 0.0 { sample } else { t.2 + (sample - t.2) * 0.35 };
        }
    } else if game_time < t.0 {
        t.2 = -1.0;
    }
    *t = (game_time, Some(now), t.2);
}

pub fn tps() -> f32 {
    TPS.lock().map_or(-1.0, |t| t.2)
}

pub fn crit(id: i32) {
    if let Ok(mut at) = CRIT.lock() {
        *at = Some((id, std::time::Instant::now()));
    }
}

static OPPONENT_SWING: std::sync::Mutex<Option<std::time::Instant>> = std::sync::Mutex::new(None);

/// A swing from the entity you last hit.
pub fn swing(id: i32) {
    if id == HIT_ID.load(std::sync::atomic::Ordering::Relaxed) && let Ok(mut at) = OPPONENT_SWING.lock() {
        *at = Some(std::time::Instant::now());
    }
}

/// 1 fading to 0 over the 350 ms after your opponent swings.
pub fn opponent_swing() -> f32 {
    OPPONENT_SWING.lock().ok().and_then(|s| *s).map_or(0.0, |t| (1.0 - t.elapsed().as_secs_f32() / 0.35).max(0.0))
}

/// Whether the server showed crit particles around your last hit.
pub fn last_hit_crit() -> bool {
    since_hit() < 1.0 && CRIT.lock().ok().and_then(|c| *c).is_some_and(|(id, t)| {
        id == HIT_ID.load(std::sync::atomic::Ordering::Relaxed) && t.elapsed().as_secs_f32() < since_hit() + 0.5
    })
}

/// Seconds since your last hit.
pub fn since_hit() -> f32 {
    LAST_HIT.get().map_or(f32::MAX, |t| t.elapsed().as_secs_f32())
}

/// Hits in a row, each within 2.5 seconds of the last.
pub fn combo() -> u32 {
    if LAST_HIT.get().is_some_and(|t| t.elapsed().as_secs_f32() < 2.5) { COMBO.get() } else { 0 }
}

thread_local! {
    static GLOW: Cell<Option<u32>> = const { Cell::new(None) };
}

/// The glow color of the drop about to be outlined.
pub fn set_glow(color: u32) {
    GLOW.set(Some(color));
}

pub fn take_glow() -> Option<u32> {
    GLOW.take()
}

/// Spotify through Windows media controls, and LRCLIB lyrics, as in Java: a hidden PowerShell reader.
#[derive(Default)]
struct Media {
    title: String,
    artist: String,
    position: i64,
    playing: bool,
    duration: i64,
    /// The cover as runs of near-equal pixels: x, y, length, RGB; and its average color.
    cover: Vec<(u8, u8, u8, u32)>,
    /// Goes up with each new cover, so its finished vertices are only built once.
    cover_id: u32,
    accent: u32,
    /// The lyric lookup for this song has finished.
    done: bool,
    /// Sung milliseconds per letter, this song's own pace.
    pace: f32,
    seen: Option<std::time::Instant>,
    /// Start, text, and each word's start when the source has them.
    lyrics: Vec<(i64, String, Vec<i64>)>,
}

static MEDIA: std::sync::LazyLock<std::sync::Mutex<Media>> = std::sync::LazyLock::new(Default::default);
static MEDIA_STARTED: std::sync::Once = std::sync::Once::new();
const MEDIA_SCRIPT: &str = r#"param($parent, $mode, $data, $appsOffFile, $onlineFile)
$ErrorActionPreference = 'Stop'
[Console]::OutputEncoding = New-Object System.Text.UTF8Encoding $false
Add-Type -AssemblyName System.Runtime.WindowsRuntime
Add-Type -AssemblyName System.Drawing
$null = [Windows.Media.Control.GlobalSystemMediaTransportControlsSessionManager, Windows.Media.Control, ContentType = WindowsRuntime]
$null = [Windows.Storage.Streams.IRandomAccessStreamWithContentType, Windows.Storage.Streams, ContentType = WindowsRuntime]
$asTask = [System.WindowsRuntimeSystemExtensions].GetMethods() | Where-Object {
    $_.Name -eq 'AsTask' -and $_.IsGenericMethod -and $_.GetParameters().Count -eq 1 -and
    $_.GetParameters()[0].ParameterType.Name -eq 'IAsyncOperation`1' } | Select-Object -First 1
function Ms($v) { $n = 0.0; foreach ($part in ($v -split ':')) { $n = $n * 60 + [double]::Parse($part.TrimEnd('s'), [Globalization.CultureInfo]::InvariantCulture) }; [long]($n * 1000) }
function Await($op, $type) { $asTask.MakeGenericMethod($type).Invoke($null, @($op)).GetAwaiter().GetResult() }
# Video sites name songs for people: "Artist - Song (Official Video)" by "ArtistVEVO". Lyric sites want the song.
function Tidy-Track($title, $artist, $loose) {
    if (-not $loose) { return $title, $artist }
    $t = $title -replace '\s*[\(\[][^\)\]]*(official|video|audio|lyric|visuali[sz]er|\bhd\b|\b4k\b|\bmv\b)[^\)\]]*[\)\]]', ''
    $t = ($t -replace '\s*\|.*$', '' -replace '\s+(ft|feat)\.?\s.*$', '').Trim()
    $a = ($artist -replace 'VEVO$', '' -replace '\s*-\s*Topic$', '' -replace '\s*Official$', '').Trim()
    if ($t -match '^(.+?)\s+[-\u2013\u2014]\s+(.+)$') { $a = $Matches[1].Trim(); $t = $Matches[2].Trim() }
    return $t, $a
}
$managerType = [Windows.Media.Control.GlobalSystemMediaTransportControlsSessionManager]
$propsType = [Windows.Media.Control.GlobalSystemMediaTransportControlsSessionMediaProperties]
$manager = Await ($managerType::RequestAsync()) $managerType
$last = ''
$seen = $null
$retryAt = $null
while (Get-Process -Id $parent -ErrorAction SilentlyContinue) {
    try {
        # Every app Windows knows is playing music; the ones switched off in Lyrics are skipped.
        $all = @($manager.GetSessions())
        if ($mode -eq 'pos') {
            $names = ($all | ForEach-Object { $_.SourceAppUserModelId }) -join "`t"
            if ($names -ne $seen) { $seen = $names; [Console]::Out.WriteLine("S`t$names") }
        }
        $off = @(((Get-Content $appsOffFile -Raw -ErrorAction SilentlyContinue) -split '\|') | Where-Object { $_ })
        $open = @($all | Where-Object { $off -notcontains $_.SourceAppUserModelId })
        $live = @($open | Where-Object { $_.GetPlaybackInfo().PlaybackStatus -eq 'Playing' })
        if ($live.Count -gt 0) { $open = $live }
        # Spotify first, then what Windows thinks you are using, then whatever is left.
        $s = $open | Where-Object { $_.SourceAppUserModelId -like '*Spotify*' } | Select-Object -First 1
        if ($null -eq $s) {
            $now = $manager.GetCurrentSession()
            if ($null -ne $now) { $s = $open | Where-Object { $_.SourceAppUserModelId -eq $now.SourceAppUserModelId } | Select-Object -First 1 }
        }
        if ($null -eq $s) { $s = $open | Select-Object -First 1 }
        if ($null -eq $s) { if ($mode -eq 'pos') { [Console]::Out.WriteLine('N') } } else {
            $p = Await ($s.TryGetMediaPropertiesAsync()) $propsType
            $t = $s.GetTimelineProperties()
            $length = [long]$t.EndTime.TotalMilliseconds
            $playing = $s.GetPlaybackInfo().PlaybackStatus -eq 'Playing'
            $pos = $t.Position.TotalMilliseconds
            if ($playing) { $pos += ([DateTimeOffset]::UtcNow - $t.LastUpdatedTime).TotalMilliseconds }
            if ($mode -eq 'pos') { [Console]::Out.WriteLine("T`t$($p.Title)`t$($p.Artist)`t$([long]$pos)`t$([int]$playing)`t$length") }
            $key = "$($p.Title)|$($p.Artist)"
            $again = $retryAt -and (Get-Date) -ge $retryAt
            if ($mode -eq 'lyrics' -and ($key -ne $last -or $again)) {
                # Nothing found (a network blip, say): one more try half a minute later, as Java.
                $retryAt = $null
                $fresh = $key -ne $last
                $online = (Get-Content $onlineFile -ErrorAction SilentlyContinue) -ne '0'
                $last = $key
                $song, $singer = Tidy-Track $p.Title $p.Artist ($s.SourceAppUserModelId -notlike '*Spotify*')
                [Console]::Out.WriteLine('C')
                try {
                    if ($null -ne $p.Thumbnail) {
                        $st = Await ($p.Thumbnail.OpenReadAsync()) ([Windows.Storage.Streams.IRandomAccessStreamWithContentType])
                        $net = [System.IO.WindowsRuntimeStreamExtensions].GetMethods() | Where-Object { $_.Name -eq 'AsStreamForRead' -and $_.GetParameters().Count -eq 1 } | Select-Object -First 1 | ForEach-Object { $_.Invoke($null, @($st)) }
                        $img = [System.Drawing.Image]::FromStream($net)
                        # The sharp cover from Apple's catalog, as Java's CoverArt; Windows' thumbnail if none matches.
                        try {
                            # Covers stay on disk, as Java's: a song played before asks nobody.
                            $md5 = [System.Security.Cryptography.MD5]::Create()
                            $name = -join ($md5.ComputeHash([Text.Encoding]::UTF8.GetBytes($key)) | ForEach-Object { $_.ToString('x2') })
                            $folder = Join-Path $data 'covers'
                            $saved = Join-Path $folder "$name.png"
                            if (Test-Path $saved) {
                                $sharp = [System.Drawing.Image]::FromStream((New-Object System.IO.MemoryStream(,[IO.File]::ReadAllBytes($saved))))
                                $img.Dispose(); $img = $sharp
                                throw 'saved'
                            }
                            $clean = ($song -replace '\s+-\s.*$|\s*[\(\[].*$', '').Trim()
                            $first = ($singer -split '[,&]')[0].Trim()
                            if (-not $online) { throw 'offline' }
                            $found = Invoke-RestMethod -Uri ('https://itunes.apple.com/search?media=music&entity=song&limit=10&term=' + [uri]::EscapeDataString("$first $clean")) -TimeoutSec 8
                            $hit = $found.results | Where-Object { $_.trackName -and $_.trackName.StartsWith($clean, [StringComparison]::OrdinalIgnoreCase) -and $_.artistName -and $_.artistName.IndexOf($first, [StringComparison]::OrdinalIgnoreCase) -ge 0 } | Select-Object -First 1
                            if ($hit) {
                                $url = $hit.artworkUrl100 -replace '/[0-9]+x[0-9]+bb\.[a-z]+$', '/600x600bb.png'
                                $bytes = (Invoke-WebRequest -Uri $url -UseBasicParsing -TimeoutSec 10).Content
                                $sharp = [System.Drawing.Image]::FromStream((New-Object System.IO.MemoryStream(,$bytes)))
                                $null = New-Item -ItemType Directory -Force -Path $folder
                                [IO.File]::WriteAllBytes($saved, $bytes)
                                Get-ChildItem $folder -Filter *.png | Sort-Object LastWriteTime -Descending | Select-Object -Skip 200 | Remove-Item
                                $img.Dispose(); $img = $sharp
                            }
                        } catch { }
                        $bmp = New-Object System.Drawing.Bitmap 60, 60
                        $gfx = [System.Drawing.Graphics]::FromImage($bmp)
                        $gfx.InterpolationMode = [System.Drawing.Drawing2D.InterpolationMode]::HighQualityBicubic
                        $gfx.PixelOffsetMode = [System.Drawing.Drawing2D.PixelOffsetMode]::HighQuality
                        $gfx.DrawImage($img, 0, 0, 60, 60)
                        $gfx.Dispose()
                        $sb = New-Object System.Text.StringBuilder
                        for ($y = 0; $y -lt 60; $y++) { for ($x = 0; $x -lt 60; $x++) {
                            $null = $sb.Append($bmp.GetPixel($x, $y).ToArgb().ToString('x8').Substring(2))
                        } }
                        [Console]::Out.WriteLine("A`t$($sb.ToString())")
                        $bmp.Dispose(); $img.Dispose(); $net.Dispose()
                    }
                } catch { }
                # Exact word timings from Better Lyrics first, as Java's LrcLib; LRCLIB if it has none.
                $word = $false
                try {
                    if (-not $online) { throw 'offline' }
                    $u = 'https://api.betterlyrics.org/getLyrics?s=' + [uri]::EscapeDataString($song) + '&a=' + [uri]::EscapeDataString($singer) + '&al=' + [uri]::EscapeDataString($p.AlbumTitle) + '&d=' + [long]$t.EndTime.TotalSeconds
                    $b = Invoke-RestMethod -Uri $u -UserAgent 'Kyro (Minecraft mod; https://lrclib.net)' -TimeoutSec 6
                    if ($b.ttml -and ($null -eq $b.score -or $b.score -ge 0.8)) {
                        $doc = New-Object System.Xml.XmlDocument
                        $doc.XmlResolver = $null
                        $doc.PreserveWhitespace = $true
                        $doc.LoadXml($b.ttml)
                        foreach ($line in $doc.SelectNodes("//*[local-name()='p']")) {
                            $words = New-Object System.Collections.ArrayList
                            $starts = New-Object System.Collections.ArrayList
                            $joined = $false
                            foreach ($n in $line.ChildNodes) {
                                if ($n.LocalName -eq 'span' -and $n.GetAttribute('begin')) {
                                    # Syllables with no space between them are one word.
                                    if ($joined -and $words.Count -gt 0) { $words[$words.Count - 1] = $words[$words.Count - 1] + $n.InnerText.Trim() }
                                    else { $null = $words.Add($n.InnerText.Trim()); $null = $starts.Add((Ms $n.GetAttribute('begin'))) }
                                    $joined = $true
                                } elseif ($n.NodeType -ne 'Element' -and $n.Value -match '\s') { $joined = $false }
                            }
                            if ($words.Count -gt 0) {
                                [Console]::Out.WriteLine("Y`t$(Ms $line.GetAttribute('begin'))`t$($words -join ' ')`t$($starts -join ',')")
                                $word = $true
                            }
                        }
                    }
                } catch { }
                # Then QQ's word timings through the same service, as Java's QrcLyrics.
                try {
                    if (-not $word -and $online) {
                        $b = Invoke-RestMethod -Uri ($u -replace '/getLyrics', '/qq/getLyrics') -UserAgent 'Kyro (Minecraft mod; https://lrclib.net)' -TimeoutSec 6
                        $m = [regex]::Match([string]$b.lyrics, 'LyricContent="([^"]*)"')
                        if ($m.Success -and ($null -eq $b.score -or $b.score -ge 0.8)) {
                            $fold = { param($v) ($v.ToLowerInvariant() -replace '[^\p{L}\p{N}]', '') }
                            $body = [System.Net.WebUtility]::HtmlDecode($m.Groups[1].Value)
                            $ti = [regex]::Match($body, '\[ti:([^\]]*)\]')
                            $ar = [regex]::Match($body, '\[ar:([^\]]*)\]')
                            $lead = & $fold (($singer -split '[,&]')[0])
                            if ((-not $ti.Success -or (& $fold $ti.Groups[1].Value) -eq (& $fold $song)) -and (-not $ar.Success -or (& $fold $ar.Groups[1].Value).Contains($lead))) {
                                foreach ($raw in ($body -split "`r?`n")) {
                                    $line = [regex]::Match($raw.Trim(), '^\[(\d+),(\d+)\](.*)$')
                                    if (-not $line.Success) { continue }
                                    $words = New-Object System.Collections.ArrayList
                                    $starts = New-Object System.Collections.ArrayList
                                    $open = $false
                                    foreach ($seg in [regex]::Matches($line.Groups[3].Value, '([^()]*)\((\d+),(\d+)\)')) {
                                        $piece = $seg.Groups[1].Value
                                        $trim = $piece.Trim()
                                        if ($piece -match '^\s') { $open = $false }
                                        if ($trim) {
                                            if ($open -and $words.Count -gt 0) { $words[$words.Count - 1] = $words[$words.Count - 1] + $trim }
                                            else { $null = $words.Add($trim); $null = $starts.Add([long]$seg.Groups[2].Value) }
                                            $open = $true
                                        }
                                        if ($piece -match '\s$') { $open = $false }
                                    }
                                    if ($words.Count -gt 0) {
                                        [Console]::Out.WriteLine("Y`t$($line.Groups[1].Value)`t$($words -join ' ')`t$($starts -join ',')")
                                        $word = $true
                                    }
                                }
                            }
                        }
                    }
                } catch { }
                try {
                    if ($word -or -not $online) { throw 'done' }
                    $q = 'track_name=' + [uri]::EscapeDataString($song) + '&artist_name=' + [uri]::EscapeDataString($singer) + '&duration=' + [long]$t.EndTime.TotalSeconds
                    $r = $null
                    try { $r = Invoke-RestMethod -Uri "https://lrclib.net/api/get?$q" -UserAgent 'Kyro (https://lrclib.net)' -TimeoutSec 8 } catch { }
                    if ($null -eq $r -or -not $r.syncedLyrics) {
                        # As Java: a looser search on the cleaned title and first artist.
                        $name = ($song -replace '\s+-\s.*$|\s*[\(\[].*$', '').Trim()
                        $lead = ($singer -split '[,&]')[0].Trim()
                        $list = Invoke-RestMethod -Uri ('https://lrclib.net/api/search?q=' + [uri]::EscapeDataString("$name $lead")) -UserAgent 'Kyro (https://lrclib.net)' -TimeoutSec 8
                        $r = $list | Where-Object { $_.syncedLyrics } | Select-Object -First 1
                    }
                    foreach ($line in ($r.syncedLyrics -split "`n")) {
                        if ($line -match '^\[(\d+):(\d+\.\d+)\]\s*(.*)$') {
                            [Console]::Out.WriteLine("Y`t$([long](([int]$Matches[1] * 60 + [double]$Matches[2]) * 1000))`t$($Matches[3])")
                            $word = $true
                        }
                    }
                } catch { }
                [Console]::Out.WriteLine('D')
                if (-not $word -and $fresh) { $retryAt = (Get-Date).AddSeconds(30) }
            }
        }
        [Console]::Out.Flush()
    } catch { }
    Start-Sleep -Milliseconds $(if ($mode -eq 'pos') { 250 } else { 2000 })
}
"#;

fn media_reader(mode: &str) -> std::io::Result<()> {
    use std::io::BufRead;
    let path = std::env::temp_dir().join(format!("kyro-media-{mode}.ps1"));
    std::fs::write(&path, MEDIA_SCRIPT)?;
    use horizon_client::options_ui::kyro_ring as ring;
    let data = ring::data_dir().unwrap_or_default();
    let (apps_off, online) = (data.join(ring::set_file("lyrics", "text_apps_off")), data.join(ring::set_file("lyrics", "online")));
    let quoted = |p: &std::path::Path| p.display().to_string().replace('\'', "''");
    let mut command = std::process::Command::new("powershell.exe");
    command
        .args(["-NoProfile", "-NonInteractive", "-Command"])
        // Run as a script block: execution policy covers script files, so nothing has to bypass it.
        .arg(format!("& ([scriptblock]::Create([IO.File]::ReadAllText('{}'))) {} {mode} '{}' '{}' '{}'", quoted(&path), std::process::id(), quoted(&data), quoted(&apps_off), quoted(&online)))
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null());
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x0800_0000); // CREATE_NO_WINDOW
    }
    let mut child = command.spawn()?;
    let Some(out) = child.stdout.take() else {
        return Ok(());
    };
    for line in std::io::BufReader::new(out).lines() {
        let line = line?;
        media_line(&line.split('\t').collect::<Vec<_>>());
    }
    Ok(())
}

/// One line of what's playing, in the reader's format.
fn media_line(parts: &[&str]) {
    let Ok(mut m) = MEDIA.lock() else {
        return;
    };
    match parts {
        ["N"] => m.title.clear(),
        ["S", apps @ ..] => horizon_client::options_ui::kyro_ring::set_media_apps(apps),
        ["C"] => {
            m.lyrics.clear();
            m.done = false;
            m.cover.clear();
            m.accent = 0;
        }
        ["A", hex] => {
            let pixels: Vec<u32> = hex.as_bytes().chunks_exact(6).filter_map(|c| u32::from_str_radix(std::str::from_utf8(c).ok()?, 16).ok()).collect();
            let count = pixels.len().max(1) as u32;
            let sum = |shift: u32| pixels.iter().map(|p| (p >> shift) & 255).sum::<u32>() / count;
            // The cover's mean, lifted so it reads on the game.
            let (r, g, b) = (sum(16), sum(8), sum(0));
            let top = r.max(g).max(b).max(1);
            m.accent = (r * 255 / top) << 16 | (g * 255 / top) << 8 | (b * 255 / top);
            // Neighbours that look alike draw as one rectangle: far fewer draws a frame.
            let near = |a: u32, b: u32| [16, 8, 0].iter().all(|s| ((a >> s) & 255).abs_diff((b >> s) & 255) <= 5);
            let mut runs: Vec<(u8, u8, u8, u32)> = Vec::new();
            if pixels.len() == 3600 {
                for (i, pixel) in pixels.iter().enumerate() {
                    let (x, y) = ((i % 60) as u8, (i / 60) as u8);
                    match runs.last_mut() {
                        Some(run) if run.1 == y && near(run.3, *pixel) => run.2 += 1,
                        _ => runs.push((x, y, 1, *pixel)),
                    }
                }
            }
            m.cover = runs;
            m.cover_id += 1;
        }
        ["D"] => {
            let mut rates: Vec<f32> = m.lyrics.windows(2).filter(|l| !l[0].1.is_empty()).map(|l| (l[1].0 - l[0].0) as f32 / letters(&l[0].1) as f32).collect();
            rates.sort_by(f32::total_cmp);
            m.pace = rates.get(rates.len() * 2 / 5).copied().unwrap_or(80.0);
            m.done = true;
        }
        ["T", title, artist, position, playing, length] => {
            m.title = (*title).to_owned();
            m.artist = (*artist).to_owned();
            // Spotify publishes its clock every 4.5 s and extrapolating from that is good to a few ms
            // (LyricHub measured it), so a small gap here is only poll noise to ease out, and anything
            // bigger is a seek or its correction, taken at once.
            let told: i64 = position.parse().unwrap_or(0);
            let playing = *playing == "1";
            let now = std::time::Instant::now();
            let running = m.seen.filter(|_| m.playing && playing).map(|t| m.position + now.duration_since(t).as_millis() as i64);
            m.position = match running {
                Some(r) if (told - r).abs() <= 60 => r + (told - r) / 4,
                _ => told,
            };
            m.seen = Some(now);
            m.playing = playing;
            m.duration = length.parse().unwrap_or(0);
        }
        ["Y", at, text] => {
            let at = at.parse().unwrap_or(0);
            m.lyrics.push((at, (*text).to_owned(), Vec::new()));
        }
        ["Y", at, text, starts] => {
            let at = at.parse().unwrap_or(0);
            // The drawing adds a lead for estimated words; exact ones have none.
            let starts = starts.split(',').filter_map(|s| s.parse::<i64>().ok()).map(|s| s + 150).collect();
            m.lyrics.push((at, (*text).to_owned(), starts));
        }
        _ => {}
    }
}

/// Timed lines out of LRC text.
fn synced_lines(lrc: &str) -> Vec<(i64, String)> {
    lrc.lines()
        .filter_map(|line| {
            let (at, text) = line.strip_prefix('[')?.split_once(']')?;
            let (minutes, seconds) = at.split_once(':')?;
            let ms = (minutes.parse::<f64>().ok()? * 60.0 + seconds.parse::<f64>().ok()?) * 1000.0;
            Some((ms as i64, text.trim().to_owned()))
        })
        .collect()
}

fn lrclib_lyrics(title: &str, artist: &str) -> Vec<(i64, String)> {
    let reply = ureq::get("https://lrclib.net/api/get")
        .query("track_name", title)
        .query("artist_name", artist)
        .set("User-Agent", "Kyro (https://lrclib.net)")
        .timeout(std::time::Duration::from_secs(10))
        .call();
    let body = reply.ok().and_then(|r| r.into_string().ok()).unwrap_or_default();
    let json: serde_json::Value = serde_json::from_str(&body).unwrap_or_default();
    synced_lines(json["syncedLyrics"].as_str().unwrap_or(""))
}

/// Spotify as Linux's playerctl or macOS's AppleScript tell it: title, artist, ms in, playing, ms long.
fn desktop_track() -> Option<(String, String, i64, bool, i64)> {
    let run = |program: &str, args: &[&str]| {
        let out = std::process::Command::new(program).args(args).output().ok().filter(|o| o.status.success())?;
        Some(String::from_utf8_lossy(&out.stdout).trim().to_owned())
    };
    if cfg!(target_os = "macos") {
        let told = run("osascript", &["-e", MAC_TRACK])?;
        let [title, artist, at, state, length] = told.split('\t').collect::<Vec<_>>().try_into().ok()?;
        return Some((title.to_owned(), artist.to_owned(), at.parse().ok()?, state == "playing", length.parse().ok()?));
    }
    let told = run("playerctl", &["-p", "spotify", "metadata", "--format", "{{title}}\t{{artist}}\t{{mpris:length}}\t{{status}}"])?;
    let [title, artist, length, state] = told.split('\t').collect::<Vec<_>>().try_into().ok()?;
    let at = run("playerctl", &["-p", "spotify", "position"])?.parse::<f64>().ok()?;
    Some((title.to_owned(), artist.to_owned(), (at * 1000.0) as i64, state == "Playing", length.parse::<i64>().ok()? / 1000))
}

const MAC_TRACK: &str = r#"if application "Spotify" is running then
tell application "Spotify" to return (name of current track) & tab & (artist of current track) & tab & ((player position * 1000) as integer) & tab & (player state as string) & tab & (duration of current track)
end if"#;

/// The reader away from Windows, polled each second.
/// shortcut: LRCLIB's synced lines and no cover; word timings and art need the Windows reader.
fn desktop_media(last: &mut String) {
    let Some((title, artist, at, playing, length)) = desktop_track() else {
        media_line(&["N"]);
        last.clear();
        return;
    };
    media_line(&["T", &title, &artist, &at.to_string(), if playing { "1" } else { "0" }, &length.to_string()]);
    let key = format!("{title}\t{artist}");
    if *last != key {
        key.clone_into(last);
        media_line(&["C"]);
        std::thread::spawn(move || {
            if horizon_client::options_ui::kyro_ring::value("lyrics", "online") == 1 {
                for (ms, text) in lrclib_lyrics(&title, &artist) {
                    media_line(&["Y", &ms.to_string(), &text]);
                }
            }
            media_line(&["D"]);
        });
    }
}

thread_local! {
    static SESSION: Cell<Option<std::time::Instant>> = const { Cell::new(None) };
}

static SESSION_WORLD: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());

/// Runs every frame in a world: a new dimension starts the clock over, as a new level does in Java.
pub fn session_tick(world: &str) {
    let moved = SESSION_WORLD.lock().is_ok_and(|mut w| {
        let moved = *w != world;
        if moved {
            world.clone_into(&mut w);
        }
        moved
    });
    if moved || SESSION.get().is_none() {
        SESSION.set(Some(std::time::Instant::now()));
    }
}

/// A new world or server: the next tick starts the clock.
pub fn session_reset() {
    SESSION.set(None);
}

/// Seconds in this world.
pub fn session_seconds() -> u64 {
    SESSION.get().map_or(0, |t| t.elapsed().as_secs())
}

static NUDGES: std::sync::Mutex<(Option<std::collections::HashMap<String, i64>>, Option<std::time::Instant>)> = std::sync::Mutex::new((None, None));

fn nudge_file() -> Option<std::path::PathBuf> {
    horizon_client::options_ui::kyro_ring::data_dir().map(|d| d.join("lyric_offsets.txt"))
}

fn nudges_loaded(slot: &mut Option<std::collections::HashMap<String, i64>>) -> &mut std::collections::HashMap<String, i64> {
    slot.get_or_insert_with(|| {
        let text = nudge_file().and_then(|p| std::fs::read_to_string(p).ok()).unwrap_or_default();
        text.lines().filter_map(|l| l.rsplit_once('\t')).filter_map(|(k, v)| Some((k.to_owned(), v.parse().ok()?))).collect()
    })
}

/// This song's saved timing fix, in milliseconds; positive shows lyrics earlier.
fn nudge_of(song: &str) -> i64 {
    NUDGES.lock().map_or(0, |mut n| nudges_loaded(&mut n.0).get(song).copied().unwrap_or(0))
}

/// Moves the playing song's lyrics earlier or later, and keeps it for next time.
pub fn nudge(delta: i64) {
    let Some(song) = MEDIA.lock().ok().filter(|m| !m.title.is_empty()).map(|m| format!("{} | {}", m.title, m.artist)) else {
        return;
    };
    let Ok(mut n) = NUDGES.lock() else {
        return;
    };
    let map = nudges_loaded(&mut n.0);
    *map.entry(song).or_insert(0) += delta;
    let text: String = map.iter().map(|(k, v)| format!("{k}\t{v}\n")).collect();
    if let Some(path) = nudge_file() {
        if let Err(error) = std::fs::create_dir_all(path.parent().unwrap_or(&path)).and_then(|()| std::fs::write(&path, text)) {
            log::warn!("kyro lyric offsets not written: {error}");
        }
    }
    n.1 = Some(std::time::Instant::now());
}

fn nudge_shown() -> bool {
    NUDGES.lock().is_ok_and(|n| n.1.is_some_and(|t| t.elapsed().as_millis() < 1500))
}

fn letters(line: &str) -> usize {
    line.chars().filter(|c| *c != ' ').count().max(1)
}

/// Word starts spread over the sung part of a line by word length, as Java does without word tags.
fn spread(line: &str, start: i64, next: i64, pace: f32) -> Vec<i64> {
    let total = letters(line);
    let sung = ((next - start).max(0) as f32 * 0.85).min(total as f32 * pace * 1.1) as i64;
    let mut before = 0;
    line.split(' ')
        .map(|word| {
            let at = start + sung * before as i64 / total as i64;
            before += word.chars().count();
            at
        })
        .collect()
}

struct Word {
    text: String,
    x: f32,
    w: f32,
    row: f32,
    at: i64,
    hit: Option<std::time::Instant>,
}

/// The lyric on screen and what its animation has reached.
struct Shown {
    index: i64,
    song: String,
    line: String,
    before: String,
    next: String,
    scale: f32,
    before_scale: f32,
    timed: bool,
    rows: usize,
    words: Vec<Word>,
    changed: std::time::Instant,
    share: f32,
}

static SHOWN: std::sync::Mutex<Option<Shown>> = std::sync::Mutex::new(None);

/// Text in a color and alpha at (x, y), grown by `scale` from that corner.
pub fn lyric_text(game: &crate::game::Game<'_>, batch: &mut horizon_client::hud::HudBatch, text: &str, at: (f32, f32), scale: f32, color: u32, alpha: f32, shadow: bool) {
    let start = batch.vertices.len();
    game.native_text(batch, text, [at.0, at.1], color, shadow);
    fade_since(batch, start, alpha);
    batch.scale_since(start, at.0, at.1, scale, scale);
}

fn fade_since(batch: &mut horizon_client::hud::HudBatch, start: usize, alpha: f32) {
    let mask = horizon_client::hud::rgba(0, 0, 0, 255);
    let shift = mask.trailing_zeros();
    for v in &mut batch.vertices[start..] {
        let old = ((v.color & mask) >> shift) as f32;
        v.color = (v.color & !mask) | (((old * alpha) as u32) << shift);
    }
}

/// The Lyrics element from HudElement.java: title, artist, equalizer, progress, and karaoke words
/// that slide in and pop as they are sung. Spotify's cover is not drawn: a note stands in for it.
pub fn draw_lyrics(game: &crate::game::Game<'_>, batch: &mut horizon_client::hud::HudBatch) {
    use horizon_client::options_ui::kyro_ring as ring;
    const W: f32 = 214.0;
    const H: f32 = 74.0;
    const PAD: f32 = 6.0;
    const COVER: f32 = 30.0;
    MEDIA_STARTED.call_once(|| {
        // Position and lyrics read apart, so a slow lookup never stalls the clock.
        if !cfg!(windows) {
            std::thread::spawn(|| {
                let mut last = String::new();
                loop {
                    desktop_media(&mut last);
                    std::thread::sleep(std::time::Duration::from_secs(1));
                }
            });
        }
        for mode in ["pos", "lyrics"].into_iter().filter(|_| cfg!(windows)) {
            std::thread::spawn(move || {
                // A reader that dies comes back; it only ends on its own with the game.
                loop {
                    if let Err(error) = media_reader(mode) {
                        log::warn!("kyro media reader stopped: {error}");
                    }
                    std::thread::sleep(std::time::Duration::from_secs(5));
                }
            });
        }
    });
    let (Ok(m), Ok(mut guard)) = (MEDIA.lock(), SHOWN.lock()) else {
        return;
    };
    // With nothing playing the box is just its one line.
    let (w, h) = if m.title.is_empty() { (game.native_text_width("Nothing playing") + 2.0 * PAD + 4.0, 8.0 + 2.0 * PAD) } else { (W, H) };
    let (x, y, zoom, panel) = ring::place("lyrics", w, h, (((batch.width - W) / 2.0).floor(), ((batch.height - H) * 0.76).floor()));
    let (ink, muted) = (0xffffff, 0xa8b0b8);
    let first = batch.vertices.len();
    if panel {
        game.native_fill(batch, [x, y, w, h], ring::hud_panel());
    }
    let accent = if m.accent != 0 { m.accent } else { 0x1db954 };
    if m.title.is_empty() {
        game.native_text(batch, "Nothing playing", [x + PAD + 2.0, y + PAD], muted, true);
        batch.scale_since(first, x, y, zoom, zoom);
        return;
    }
    let clock = std::time::Instant::now();
    let ahead = if m.playing { m.seen.map_or(0, |t| t.elapsed().as_millis() as i64) } else { 0 };
    let position = m.position + ahead;
    let shift = i64::from(ring::value("lyrics", "offset")) - i64::from(ring::value("lyrics", "delay"));
    let song = format!("{}\n{}", m.title, m.artist);
    let nudged = nudge_of(&format!("{} | {}", m.title, m.artist));
    let shift = shift + nudged;
    // Trim to a width, ending in "…".
    let trim = |text: &str, room: f32| {
        if game.native_text_width(text) <= room {
            return text.to_owned();
        }
        let mut out = String::new();
        for c in text.chars() {
            out.push(c);
            if game.native_text_width(&out) + game.native_text_width("…") > room {
                out.pop();
                break;
            }
        }
        out + "…"
    };
    // Cover place, title, artist.
    let (cx, cy) = (x + PAD, y + PAD);
    if !m.cover.is_empty() {
        // A cover is thousands of small rectangles: build them once per cover, then only copy them.
        thread_local! {
            static COVER: std::cell::RefCell<(u32, Vec<horizon_client::hud::HudVertex>)> = const { std::cell::RefCell::new((0, Vec::new())) };
        }
        COVER.with_borrow_mut(|(id, quads)| {
            if *id != m.cover_id {
                let start = batch.vertices.len();
                for (px, py, length, pixel) in &m.cover {
                    game.native_fill(batch, [f32::from(*px) * 0.5, f32::from(*py) * 0.5, f32::from(*length) * 0.5, 0.5], 0xff000000 | pixel);
                }
                *quads = batch.vertices.split_off(start);
                *id = m.cover_id;
            }
            batch.vertices.extend(quads.iter().map(|v| horizon_client::hud::HudVertex { pos: [v.pos[0] + cx, v.pos[1] + cy], ..*v }));
        });
    } else {
        game.native_fill(batch, [cx, cy, COVER, COVER], 0x461db954);
        game.native_text(batch, "♪", [cx + COVER / 2.0 - 3.0, cy + COVER / 2.0 - 4.0], ink, true);
    }
    let text_x = cx + COVER + 7.0;
    // Measuring a long title letter by letter adds up, so it happens once per song.
    thread_local! {
        static CUT: std::cell::RefCell<(String, String, String)> = const { std::cell::RefCell::new((String::new(), String::new(), String::new())) };
    }
    CUT.with_borrow_mut(|(key, title, artist)| {
        if *key != song {
            key.clone_from(&song);
            *title = trim(&m.title, x + W - PAD - 13.0 - text_x);
            *artist = trim(&m.artist, x + W - PAD - text_x);
        }
        game.native_text(batch, title, [text_x, cy + 4.0], ink, true);
        game.native_text(batch, artist, [text_x, cy + 16.0], muted, true);
    });
    // Equalizer: three bars that dance while playing and rest flat when paused.
    let seconds = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0.0, |t| (t.as_millis() % 1_000_000) as f32 / 1000.0);
    for bar in 0..3 {
        let b = bar as f32;
        let level = if m.playing { 0.35 + 0.65 * (seconds * (5.1 + b * 1.7) + b * 1.3).sin().abs() } else { 0.2 };
        let height = (9.0 * level).round().max(2.0);
        game.native_fill(batch, [x + W - PAD - 9.0 + b * 3.0, cy + 13.0 - height, 2.0, height], 0xff000000 | if m.playing { accent } else { muted });
    }
    let state = guard.get_or_insert_with(|| Shown {
        index: -2,
        song: String::new(),
        line: String::new(),
        before: String::new(),
        next: String::new(),
        scale: 1.0,
        before_scale: 1.0,
        timed: false,
        rows: 1,
        words: Vec::new(),
        changed: clock,
        share: 0.0,
    });
    // Progress, eased so a poll correcting the position glides.
    let share = if m.duration > 0 { position as f32 / m.duration as f32 } else { 0.0 };
    state.share = if (share - state.share).abs() > 0.05 { share } else { state.share + (share - state.share) * 0.25 };
    let bar_w = x + W - PAD - text_x;
    game.native_fill(batch, [text_x, cy + 27.0, bar_w, 2.0], 0x40ffffff);
    game.native_fill(batch, [text_x, cy + 27.0, (bar_w * state.share.clamp(0.0, 1.0)).round(), 2.0], 0xff000000 | accent);
    // Which line: -3 loading, -4 none, -1 before the first.
    let index = if !m.done {
        -3
    } else if m.lyrics.is_empty() {
        -4
    } else {
        // A line whose words light up comes early enough to read; a line that only appears, on time.
        let words = ring::value("lyrics", "word_for_word") == 1;
        m.lyrics.iter().rposition(|l| l.0 <= position + shift + if words { 260 } else { 100 }).map_or(-1, |i| i as i64)
    };
    if index != state.index || song != state.song {
        let room = W - 2.0 * PAD;
        let sung = usize::try_from(index).ok().and_then(|i| m.lyrics.get(i));
        let line = match index {
            -3 => "Loading lyrics".to_owned(),
            -4 => "No synced lyrics".to_owned(),
            _ => sung.filter(|l| !l.1.trim().is_empty()).map_or("♪".to_owned(), |l| l.1.clone()),
        };
        state.before = if song != state.song { String::new() } else { trim(&state.line, (room / state.scale).round()) };
        state.next = usize::try_from(index + 1).ok().filter(|_| index >= 0).and_then(|i| m.lyrics.get(i)).map_or(String::new(), |l| trim(&l.1, room));
        state.before_scale = state.scale;
        let full = game.native_text_width(&line);
        state.scale = if full * 1.15 <= room {
            1.15
        } else if full <= room {
            1.0
        } else {
            (2.0 * room * 0.9 / full).clamp(0.72, 1.0)
        };
        // Word for word uses real word times, or spreads the words over the line when a site has none.
        // Off, the whole line lights up at once.
        state.timed = sung.is_some_and(|l| !l.1.trim().is_empty()) && ring::value("lyrics", "word_for_word") == 1;
        let starts = sung.filter(|_| state.timed).map_or(Vec::new(), |l| {
            let after = m.lyrics.get(index as usize + 1).map_or(l.0 + 5000, |n| n.0);
            if l.2.len() == l.1.split(' ').count() { l.2.clone() } else { spread(&l.1, l.0, after, m.pace) }
        });
        let limit = (room / state.scale).round();
        let space = game.native_text_width(" ");
        let (mut at, mut row) = (0.0, 0.0);
        state.words.clear();
        for (i, word) in line.split(' ').enumerate() {
            let w = game.native_text_width(word);
            if at > 0.0 && at + w > limit {
                if row > 0.0 {
                    break; // Past two rows even at the smallest size: stop here.
                }
                row = 1.0;
                at = 0.0;
            }
            state.words.push(Word { text: word.to_owned(), x: at, w, row, at: starts.get(i).copied().unwrap_or(0), hit: None });
            at += w + space;
        }
        state.rows = if row > 0.0 { 2 } else { 1 };
        state.line = line;
        state.index = index;
        state.song = song;
        state.changed = clock;
    }
    let lyric_y = cy + COVER + 5.0;
    if nudge_shown() {
        let shown = format!("{nudged:+} ms");
        game.native_text(batch, &shown, [x + W - PAD - game.native_text_width(&shown), cy + 16.0], 0xffd166, true);
    }
    let since = clock.duration_since(state.changed).as_secs_f32();
    let eased = 1.0 - (1.0 - (since / 0.22).clamp(0.0, 1.0)).powi(3);
    if eased < 1.0 && !state.before.is_empty() && 1.0 - eased > 0.03 {
        // The old line flies up, shrinks a little, and fades.
        let scale = state.before_scale * (1.0 - 0.12 * eased);
        lyric_text(game, batch, &state.before, (x + PAD, lyric_y - 4.0 * eased + 4.0 * (1.0 - scale)), scale, muted, 1.0 - eased, true);
    }
    // Words drop in one after another; each pops, lifts, flashes and glows the moment it is sung.
    let word_now = position + 150 + shift;
    let reached = if state.timed { state.words.iter().filter(|w| word_now >= w.at).count() } else { state.words.len() };
    let (timed, scale) = (state.timed, state.scale);
    let (ox, oy) = (x + PAD, lyric_y + 4.0 * (1.0 - scale));
    for (i, word) in state.words.iter_mut().enumerate() {
        let sung = i < reached;
        if !sung {
            word.hit = None;
        } else if word.hit.is_none() {
            word.hit = Some(clock);
        }
        let enter = 1.0 - (1.0 - ((since - i as f32 * 0.035) / 0.22).clamp(0.0, 1.0)).powi(3);
        if enter <= 0.0 {
            continue;
        }
        let pop = match word.hit {
            Some(hit) if timed => (clock.duration_since(hit).as_secs_f32() / 0.32).clamp(0.0, 1.0),
            _ => 1.0,
        };
        let bump = if pop >= 1.0 {
            0.0
        } else if pop < 0.15 {
            pop / 0.15
        } else {
            (1.0 - (pop - 0.15) / 0.85).powi(2)
        };
        let grow = 1.0 + 0.38 * bump;
        let lift = -3.0 * bump + 7.0 * (1.0 - enter);
        let current = timed && sung && i + 1 == reached;
        let ink = if !timed {
            if index >= 0 { ink } else { muted }
        } else if !sung {
            muted
        } else if current {
            horizon_client::options_ui::kyro_ring::mix(ink, accent, 0.55 + 0.45 * pop)
        } else {
            horizon_client::options_ui::kyro_ring::mix(accent, ink, pop)
        };
        let alpha = enter * if sung { 1.0 } else { 0.6 };
        let middle = word.x + word.w / 2.0;
        let rows = word.row * 10.0;
        let at = (ox + scale * (middle - grow * word.w / 2.0), oy + scale * (4.0 + lift + rows - 4.0 * grow));
        if bump > 0.05 {
            for (dx, dy) in [(-1.0, 0.0), (1.0, 0.0), (0.0, -1.0), (0.0, 1.0)] {
                lyric_text(game, batch, &word.text, (at.0 + dx * scale * grow, at.1 + dy * scale * grow), scale * grow, accent, 0.43 * bump, false);
            }
        }
        lyric_text(game, batch, &word.text, at, scale * grow, ink, alpha, true);
        if current {
            // An underline that grows out from the middle of the word being sung.
            let half = (word.w / 2.0 * (pop * 1.6).min(1.0)).round();
            let start = batch.vertices.len();
            game.native_fill(batch, [ox + scale * (middle - half), oy + scale * (11.0 + rows), scale * half * 2.0, scale], 0xff000000 | accent);
            fade_since(batch, start, alpha);
        }
    }
    // The next line, faded, unless the current one needs two rows.
    if state.rows == 1 && !state.next.trim().is_empty() {
        lyric_text(game, batch, &state.next, (x + PAD, lyric_y + 15.0), 1.0, ink, 0.43, true);
    }
    batch.scale_since(first, x, y, zoom, zoom);
}

#[cfg(test)]
mod tests {
    use super::{regen_health, spread, synced_lines};

    #[test]
    fn lrc_lines_keep_their_times() {
        assert_eq!(synced_lines("[00:01.50] one\n[01:00.00]two\nnot timed"), vec![(1500, "one".to_owned()), (60000, "two".to_owned())]);
    }

    #[test]
    fn regen_stops_below_18_food() {
        assert_eq!(regen_health(17, 20.0, 0.0), 0.0);
        assert_eq!(regen_health(20, 0.0, 0.0), 3.0);
        assert!((regen_health(20, 6.0, 0.0) - 6.1667).abs() < 0.01);
    }

    #[test]
    fn words_start_by_letters_sung() {
        // 10 letters over 1000 ms at 85% sung: 4 letters then 6.
        assert_eq!(spread("four sixsix", 0, 1000, 1000.0), vec![0, 340]);
        // A long pause after the line does not stretch it: the song's pace caps it.
        assert_eq!(spread("four sixsix", 0, 100_000, 10.0), vec![0, 44]);
        assert_eq!(spread("one", 500, 400, 80.0), vec![500]);
    }
}
