// Esc on a mod list goes back to the ring instead of closing the menu.
fn kyro_ring_esc(game: &crate::game::Game<'_>, key: winit::keyboard::KeyCode) -> core::ops::ControlFlow<Result<()>> {
    if horizon_client::options_ui::kyro_ring::editor_key(key, game.has_control_down()) {
        return core::ops::ControlFlow::Break(Ok(()));
    }
    if key == winit::keyboard::KeyCode::Escape && horizon_client::options_ui::kyro_ring::back() {
        return core::ops::ControlFlow::Break(Ok(()));
    }
    core::ops::ControlFlow::Continue(())
}
