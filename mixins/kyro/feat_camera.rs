// While Freelook is held the view uses the camera's angles.
fn kyro_feat_camera(
    game: &mut crate::game::Game<'_>,
    partial: f32,
) -> std::ops::ControlFlow<Option<(glam::DVec3, f32, f32)>> {
    use crate::game::kyro_feat as f;
    let Some((look_yaw, look_pitch)) = f::begin() else {
        return std::ops::ControlFlow::Continue(());
    };
    let (yaw, pitch) = (game.yaw, game.pitch);
    game.yaw = look_yaw;
    game.pitch = look_pitch;
    let view = game.camera_view(partial);
    game.yaw = yaw;
    game.pitch = pitch;
    f::end();
    std::ops::ControlFlow::Break(view)
}
