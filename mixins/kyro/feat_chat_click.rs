// A click on the search box takes the typing; anywhere else gives it back.
fn kyro_feat_chat_click(this: &ChatScreen, x: f64, y: f64) -> core::ops::ControlFlow<bool> {
    use crate::options_ui::kyro_ring as ring;
    let left = f64::from(this.width) - 134.0;
    let inside = ring::chat_search_on() && x >= left && x < left + 130.0 && (4.0..18.0).contains(&y);
    ring::set_chat_focus(inside);
    if inside { core::ops::ControlFlow::Break(true) } else { core::ops::ControlFlow::Continue(()) }
}
