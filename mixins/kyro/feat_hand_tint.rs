// Shield status: the shield in your first-person hand.
fn kyro_feat_hand_tint(game: &mut crate::game::Game<'_>, alpha: f64) -> core::ops::ControlFlow<Vec<Vertex>> {
    use horizon_client::options_ui::kyro_ring as ring;
    thread_local! {
        static NESTED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    }
    if NESTED.get() || !ring::on("shield_status") {
        return core::ops::ControlFlow::Continue(());
    }
    NESTED.set(true);
    ring::hold_shield_tint(Some(ring::own_shield_color()));
    let out = game.hand_geometry(alpha);
    ring::hold_shield_tint(None);
    NESTED.set(false);
    core::ops::ControlFlow::Break(out)
}
