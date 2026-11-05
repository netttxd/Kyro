// Keep chat: a new world or server leaves chat as it was. F3+D still clears it.
fn kyro_feat_chat_keep() -> core::ops::ControlFlow<()> {
    use crate::options_ui::kyro_ring as ring;
    if ring::on("better_chat") && ring::value("better_chat", "keep") == 1 && !ring::f3_held() {
        core::ops::ControlFlow::Break(())
    } else {
        core::ops::ControlFlow::Continue(())
    }
}
