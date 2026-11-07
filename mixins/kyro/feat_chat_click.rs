// A click on the search box takes the typing; anywhere else gives it back.
fn kyro_feat_chat_click(this: &ChatScreen, x: f64, y: f64) -> core::ops::ControlFlow<bool> {
    let [left, top, w, h] = crate::options_ui::kyro_world::chat_search_box(this.width as f32).map(f64::from);
    let inside = crate::options_ui::kyro_world::chat_search_on() && x >= left && x < left + w && y >= top && y < top + h;
    crate::options_ui::kyro_world::set_chat_focus(inside);
    if inside { core::ops::ControlFlow::Break(true) } else { core::ops::ControlFlow::Continue(()) }
}
