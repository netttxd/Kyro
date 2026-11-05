// Typing goes to the search box while it has focus.
fn kyro_feat_chat_type(text: &str) -> core::ops::ControlFlow<()> {
    use crate::options_ui::kyro_ring as ring;
    if ring::chat_focus() {
        ring::chat_type(text);
        return core::ops::ControlFlow::Break(());
    }
    core::ops::ControlFlow::Continue(())
}
