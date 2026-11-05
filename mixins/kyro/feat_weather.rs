// World look: the weather you see.
// shortcut: game logic always asks at partial 1.0, drawing at the frame's fraction, so 1.0 stays real; a frame landing on exactly 1.0 shows real weather once.
fn kyro_feat_weather(partial: f32) -> core::ops::ControlFlow<(f32, f32)> {
    use crate::options_ui::kyro_ring as ring;
    if partial == 1.0 || !ring::on("world_look") {
        return core::ops::ControlFlow::Continue(());
    }
    match ring::value("world_look", "weather") {
        1 => core::ops::ControlFlow::Break((0.0, 0.0)),
        2 => core::ops::ControlFlow::Break((1.0, 0.0)),
        3 => core::ops::ControlFlow::Break((1.0, 1.0)),
        _ => core::ops::ControlFlow::Continue(()),
    }
}
