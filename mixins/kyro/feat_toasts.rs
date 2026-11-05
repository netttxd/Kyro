// Low obstruction: advancement and recipe pop-ups dropped instead of shown.
fn kyro_feat_toasts(game: &mut crate::game::Game<'_>) -> core::ops::ControlFlow<()> {
    use horizon_client::options_ui::kyro_ring as ring;
    if ring::on("low_obstruction") && ring::value("low_obstruction", "toasts") == 1 {
        game.adv.toasts.clear();
        return core::ops::ControlFlow::Break(());
    }
    core::ops::ControlFlow::Continue(())
}
