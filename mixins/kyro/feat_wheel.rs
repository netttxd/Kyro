// While zooming, the wheel changes how far instead of moving the hotbar.
fn kyro_feat_wheel(game: &crate::game::Game<'_>, steps: f32, claimed: &mut bool) {
    use crate::game::{kyro_feat as f, port_kyro as p};
    if game.native_on(p::ID) && f::zoom_wheel(steps) {
        *claimed = true;
    }
}
