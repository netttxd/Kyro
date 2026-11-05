// The wheel in the HUD editor resizes the element under the pointer.
fn kyro_ring_wheel(amount: f32) -> core::ops::ControlFlow<Result<()>> {
    if horizon_client::options_ui::kyro_ring::editor_wheel(amount) {
        return core::ops::ControlFlow::Break(Ok(()));
    }
    core::ops::ControlFlow::Continue(())
}
