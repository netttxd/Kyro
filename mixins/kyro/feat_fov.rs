// FOV control: no speed changes (mode 1), or none at all (mode 2). Spyglass stays, and mode 1 keeps the bow.
fn kyro_feat_fov(game: &crate::game::Game<'_>) -> core::ops::ControlFlow<Option<FovPlayer>> {
    use horizon_client::options_ui::kyro_ring as ring;
    let mode = ring::value("fov_control", "mode");
    if !ring::on("fov_control") || mode == 0 {
        return core::ops::ControlFlow::Continue(());
    }
    let item = game.using.and_then(|(slot, _)| game.stack_in(slot)).map(|s| s.item.to_string());
    match item.as_deref() {
        Some("minecraft:spyglass") => core::ops::ControlFlow::Continue(()),
        Some("minecraft:bow") if mode == 1 => core::ops::ControlFlow::Continue(()),
        _ => core::ops::ControlFlow::Break(None),
    }
}
