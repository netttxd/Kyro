// Fullbright: the game's night vision light at full, with no effect needed.
fn kyro_feat_light() -> core::ops::ControlFlow<f32> {
    use horizon_client::options_ui::kyro_ring as ring;
    if ring::on("fullbright") {
        core::ops::ControlFlow::Break(ring::value("fullbright", "strength") as f32 / 100.0)
    } else {
        core::ops::ControlFlow::Continue(())
    }
}
