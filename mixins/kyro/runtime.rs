// Kyro on Horizon: the mod's identity. Its code is in the feat_*, ring and hud files.

/// The mod's id (its settings folder), name, version and description.
pub const ID: &str = "kyro";
pub const NAME: &str = "Kyro";
pub const VERSION: &str = "2.0.0";
pub const DESCRIPTION: &str = "HUD, PvP and visual tools with a ring menu and drag-and-drop HUD editor.";

/// The mod's keys for Options > Mods: name, label, default key.
pub const KEYS: &[[&str; 3]] = &[
    ["menu", "Open the Kyro menu", "key.keyboard.right.shift"],
    ["zoom", "Zoom (hold)", "key.keyboard.c"],
    ["freelook", "Freelook (hold)", "key.keyboard.left.alt"],
    ["lyric_earlier", "Lyrics earlier", "key.keyboard.left.bracket"],
    ["lyric_later", "Lyrics later", "key.keyboard.right.bracket"],
];
