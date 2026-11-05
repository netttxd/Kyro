// Clicks on the ring and its lists.
fn kyro_ring_click(
    options: &mut crate::options_ui::OptionsUi,
    x: f32,
    y: f32,
    size: (f32, f32),
    drag: bool,
    right: bool,
) -> core::ops::ControlFlow<Vec<Event>> {
    use crate::options_ui::kyro_ring as m;
    if !m::shows(options) {
        return core::ops::ControlFlow::Continue(());
    }
    if drag {
        m::drag((x, y), size);
    } else {
        m::click((x, y), size, right);
    }
    core::ops::ControlFlow::Break(Vec::new())
}
