// Esc steps back from a list to the ring; on the ring it closes as any page does.
fn kyro_ring_keys(
    options: &mut crate::options_ui::OptionsUi,
    key: winit::keyboard::KeyCode,
    typed: Option<&str>,
) -> core::ops::ControlFlow<Vec<Event>> {
    use crate::options_ui::kyro_ring as m;
    if m::shows(options) && m::menu_key(key, typed) {
        return core::ops::ControlFlow::Break(Vec::new());
    }
    if m::shows(options) && key == winit::keyboard::KeyCode::Escape && m::back() {
        return core::ops::ControlFlow::Break(Vec::new());
    }
    core::ops::ControlFlow::Continue(())
}
