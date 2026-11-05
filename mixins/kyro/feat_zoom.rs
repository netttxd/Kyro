// The zoom in effect, unless another mod's came first.
fn kyro_feat_zoom(game: &crate::game::Game<'_>, zoom: &mut Option<crate::game::native_hooks::NativeZoom>) {
    use crate::game::{kyro_feat as f, port_kyro as p};
    if zoom.is_none()
        && game.native_on(p::ID)
        && let Some((factor, smooth, turn)) = f::zoom_view()
    {
        *zoom = Some(crate::game::native_hooks::NativeZoom { factor, smooth, turn });
    }
}
