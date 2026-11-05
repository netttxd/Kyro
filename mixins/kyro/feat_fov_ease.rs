// FOV control's transition: Camera.tick with the share of the remaining change per tick (vanilla 0.5).
fn kyro_feat_fov_ease(camera: &mut Camera, eye: Option<f32>, target: f32) -> core::ops::ControlFlow<()> {
    use crate::options_ui::kyro_ring as ring;
    if !ring::on("fov_control") {
        return core::ops::ControlFlow::Continue(());
    }
    if let Some(eye) = eye {
        camera.eye_height_old = camera.eye_height;
        camera.eye_height += (eye - camera.eye_height) * 0.5;
    }
    let rate = [1.0, 0.5, 0.25][ring::value("fov_control", "transition").clamp(0, 2) as usize];
    camera.old_fov_modifier = camera.fov_modifier;
    camera.fov_modifier = (camera.fov_modifier + (target - camera.fov_modifier) * rate).clamp(0.1, 1.5);
    core::ops::ControlFlow::Break(())
}
