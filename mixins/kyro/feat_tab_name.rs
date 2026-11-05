// Ping numbers: two spaces on every name widen the list enough for three digits.
fn kyro_feat_tab_name(player: &TabPlayer, board: &Scoreboard) -> core::ops::ControlFlow<Text> {
    thread_local! {
        static NESTED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    }
    if NESTED.get() || !crate::options_ui::kyro_ring::on("tab_ping") {
        return core::ops::ControlFlow::Continue(());
    }
    NESTED.set(true);
    let name = name_for_display(player, board);
    NESTED.set(false);
    core::ops::ControlFlow::Break(name.append(Text::literal("  ")))
}
