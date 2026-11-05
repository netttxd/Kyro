// The ring menu instead of Options > Mods while Right Shift has it open.
fn kyro_ring_draw(
    options: &mut crate::options_ui::OptionsUi,
    batch: &mut crate::hud::HudBatch,
    atlas: &crate::hud::HudAtlas,
    cursor: (f32, f32),
) -> core::ops::ControlFlow<()> {
    use crate::options_ui::kyro_ring as m;
    if !m::shows(options) {
        return core::ops::ControlFlow::Continue(());
    }
    m::draw(batch, atlas, cursor);
    core::ops::ControlFlow::Break(())
}
