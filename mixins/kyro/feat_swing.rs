// Held items: how fast your arm swings, as Swing. Display only: hits land as they always do.
fn kyro_feat_swing(value: i32, animation: SwingAnimation) -> i32 {
    use crate::options_ui::kyro_ring as ring;
    if !ring::on("held_items") {
        return value;
    }
    let base = if ring::value("held_items", "ignore_haste") == 1 { animation.duration } else { value };
    (base as f32 * 100.0 / ring::value("held_items", "swing").max(1) as f32).round().max(1.0) as i32
}
