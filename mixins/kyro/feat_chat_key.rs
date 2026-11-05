// Backspace edits the search box; Esc and Enter leave it without closing the chat.
fn kyro_feat_chat_key(key: winit::keyboard::KeyCode) -> core::ops::ControlFlow<ChatAction> {
    use crate::options_ui::kyro_ring as ring;
    use winit::keyboard::KeyCode as K;
    if !ring::chat_focus() {
        return core::ops::ControlFlow::Continue(());
    }
    match key {
        K::Backspace => ring::chat_backspace(),
        K::Escape | K::Enter | K::NumpadEnter => ring::set_chat_focus(false),
        _ => {}
    }
    core::ops::ControlFlow::Break(ChatAction::None)
}
