// Typing goes to the search box while it has focus.
fn kyro_feat_chat_type(text: &str) -> core::ops::ControlFlow<()> {
    if crate::options_ui::kyro_world::chat_focus() {
        crate::options_ui::kyro_world::chat_type(text);
        return core::ops::ControlFlow::Break(());
    }
    core::ops::ControlFlow::Continue(())
}
