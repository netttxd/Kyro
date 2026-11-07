// What the game tells Kyro each frame, for the hooks that act on it: your eyes, reach and name,
// shields, the server, players, and the chat's search, hover and heads.

use std::sync::Mutex;
use std::time::Instant;

use super::kyro_ring::{color_of, mix, on, value};

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
    mix(ready, off, wait)
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
fn sender_skin(line: &str) -> Option<String> {
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
