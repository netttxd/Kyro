// Backspace edits the search box; Esc and Enter leave it without closing the chat.
fn kyro_feat_chat_key(key: winit::keyboard::KeyCode) -> core::ops::ControlFlow<ChatAction> {
    use winit::keyboard::KeyCode as K;
    if !crate::options_ui::kyro_world::chat_focus() {
        return core::ops::ControlFlow::Continue(());
    }
    match key {
        K::Backspace => crate::options_ui::kyro_world::chat_backspace(),
        K::Escape | K::Enter | K::NumpadEnter => crate::options_ui::kyro_world::set_chat_focus(false),
        _ => {}
    }
    core::ops::ControlFlow::Break(ChatAction::None)
}
