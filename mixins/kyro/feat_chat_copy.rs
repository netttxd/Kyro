// Right-click copy: a right click in the chat screen puts the message under it on the clipboard.
fn kyro_feat_chat_copy(game: &mut crate::game::Game<'_>, event: &winit::event::WindowEvent) {
    use horizon_client::clipboard::Clipboard;
    use horizon_client::options_ui::kyro_ring as ring;
    if let winit::event::WindowEvent::MouseInput { state, button, .. } = event
        && *state == winit::event::ElementState::Pressed
        && *button == winit::event::MouseButton::Right
        && game.screen == crate::game::Screen::Chat
        && ring::on("better_chat")
        && ring::value("better_chat", "copy") == 1
    {
        let text = ring::chat_copy();
        if !text.is_empty() {
            game.clipboard.set(&text);
        }
    }
}
