// Kyro's state for crates below the menu: the ring sets it each frame, and item code reads it.

/// The mod's id: its settings folder and its key binds.
pub const ID: &str = "kyro";

/// Whether Shiny potions is on.
pub static SHINY_POTIONS: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Whether an item gets the shimmer: a potion, while Shiny potions is on.
pub fn shines(item: &str) -> bool {
    SHINY_POTIONS.load(std::sync::atomic::Ordering::Relaxed) && matches!(item, "minecraft:potion" | "minecraft:splash_potion" | "minecraft:lingering_potion")
}
