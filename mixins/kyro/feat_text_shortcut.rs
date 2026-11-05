// Copy and paste in the ring's text rows.
fn kyro_feat_text_shortcut(shortcut: crate::clipboard::EditShortcut, clipboard: &mut dyn crate::clipboard::Clipboard) -> core::ops::ControlFlow<bool> {
    use crate::clipboard::EditShortcut as E;
    use crate::options_ui::kyro_ring as ring;
    if !ring::text_focused() {
        return core::ops::ControlFlow::Continue(());
    }
    match shortcut {
        E::Paste => ring::text_paste(&clipboard.get()),
        E::Copy => clipboard.set(&ring::text_copy()),
        _ => {}
    }
    core::ops::ControlFlow::Break(true)
}
