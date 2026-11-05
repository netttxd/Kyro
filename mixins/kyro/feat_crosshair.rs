// The custom crosshair replaces the vanilla sprite (hud.rs draws it).
fn kyro_feat_crosshair() -> core::ops::ControlFlow<Vec<GuiDraw>> {
    if crate::options_ui::kyro_ring::on("crosshair") {
        core::ops::ControlFlow::Break(Vec::new())
    } else {
        core::ops::ControlFlow::Continue(())
    }
}
