// While Freelook is held the mouse turns the camera's angles, not the player's.
fn kyro_feat_mouse(game: &mut crate::game::Game<'_>) -> std::ops::ControlFlow<()> {
    use crate::game::kyro_feat as f;
    let Some((look_yaw, look_pitch)) = f::begin() else {
        return std::ops::ControlFlow::Continue(());
    };
    let (yaw, pitch) = (game.yaw, game.pitch);
    game.yaw = look_yaw;
    game.pitch = look_pitch;
    game.apply_mouse_look();
    f::set_look(game.yaw, game.pitch);
    game.yaw = yaw;
    game.pitch = pitch;
    f::end();
    std::ops::ControlFlow::Break(())
}
