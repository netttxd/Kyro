// Hitboxes: the game's own F3+B boxes, kept on by the ring.
fn kyro_feat_hitbox(path: &str) -> core::ops::ControlFlow<bool> {
    if path == "entity_hitboxes" && horizon_client::options_ui::kyro_ring::on("reach_hitbox") {
        return core::ops::ControlFlow::Break(true);
    }
    core::ops::ControlFlow::Continue(())
}
