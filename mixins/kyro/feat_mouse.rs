// While Freelook is held the mouse turns the camera's angles, not the player's.
fn kyro_feat_mouse(game: &mut crate::game::Game<'_>) -> std::ops::ControlFlow<()> {
    use crate::game::kyro_feat as f;
    let Some((look_yaw, look_pitch)) = f::begin() else {
        return std::ops::ControlFlow::Continue(());
    };
    let (yaw, pitch) = (game.yaw, game.pitch);
    // Turning also tilts your model's head, which must stay as it was.
    let head = game.player_model.as_ref().map(|m| (m.head_pitch, m.previous_head_pitch));
    game.yaw = look_yaw;
    game.pitch = look_pitch;
    game.apply_mouse_look();
    f::set_look(game.yaw, game.pitch);
    game.yaw = yaw;
    game.pitch = pitch;
    if let (Some(m), Some((tilt, before))) = (game.player_model.as_mut(), head) {
        m.head_pitch = tilt;
        m.previous_head_pitch = before;
    }
    f::end();
    std::ops::ControlFlow::Break(())
}
