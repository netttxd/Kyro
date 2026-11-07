// The color Item highlighter picked for the drop being drawn, else the team's.
fn kyro_glow_color() -> std::ops::ControlFlow<u32> {
    match crate::game::kyro_feat::take_glow() {
        Some(color) => std::ops::ControlFlow::Break(color),
        None => std::ops::ControlFlow::Continue(()),
    }
}
