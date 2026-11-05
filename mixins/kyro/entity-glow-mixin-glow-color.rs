// The color Item highlighter picked for the drop being drawn, else the team's.
fn kyro_glow_color(team: &Option<&sb::Team>) -> std::ops::ControlFlow<u32> {
    let _ = &team;
    match crate::game::kyro_feat::take_glow() {
        Some(color) => std::ops::ControlFlow::Break(color),
        None => std::ops::ControlFlow::Continue(()),
    }
}
