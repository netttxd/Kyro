// Item highlighter: valuable drops you can see glow, each kind in its color. Never through walls.
fn kyro_feat_glow(game: &Game<'_>, id: u32) -> core::ops::ControlFlow<bool> {
    use horizon_client::options_ui::kyro_ring as ring;
    if !ring::on("item_highlighter") {
        return core::ops::ControlFlow::Continue(());
    }
    let Some(drop) = game.dropped.iter().find(|d| d.id == id) else {
        return core::ops::ControlFlow::Continue(());
    };
    let name = drop.stack.item.trim_start_matches("minecraft:");
    let color = match name {
        "totem_of_undying" | "golden_apple" | "enchanted_golden_apple" | "elytra" | "mace" => 0xffd166,
        "ender_pearl" | "end_crystal" | "respawn_anchor" | "wind_charge" => 0xc77dff,
        n if n.starts_with("netherite_") => 0xb07a5a,
        n if n.starts_with("diamond") => 0x5ee6ff,
        _ if ring::value("item_highlighter", "enchanted") == 1 && !drop.stack.enchantments().is_empty() => 0x9f8cff,
        _ => return core::ops::ControlFlow::Continue(()),
    };
    let color = if ring::value("item_highlighter", "custom") == 1 { ring::color_of(ring::value("item_highlighter", "color")) } else { color };
    let Some(eye) = game.eye_position() else {
        return core::ops::ControlFlow::Continue(());
    };
    let to = [drop.position.x - eye[0], drop.position.y + 0.125 - eye[1], drop.position.z - eye[2]];
    let length = (to[0] * to[0] + to[1] * to[1] + to[2] * to[2]).sqrt();
    if length > 0.0 && game.clip_ray(eye, to.map(|v| v / length), length).is_some() {
        return core::ops::ControlFlow::Continue(());
    }
    crate::game::kyro_feat::set_glow(color);
    core::ops::ControlFlow::Break(true)
}
