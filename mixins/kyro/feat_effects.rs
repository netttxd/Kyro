// The Potions element replaces the game's own effect icons.
fn kyro_feat_effects() -> core::ops::ControlFlow<()> {
    if horizon_client::options_ui::kyro_ring::on("potions") {
        return core::ops::ControlFlow::Break(());
    }
    core::ops::ControlFlow::Continue(())
}
