// Sprint: holds the sprint key's state for you while the mod is on.
fn kyro_feat_sprint(game: &mut crate::game::Game<'_>) {
    use crate::game::kyro_feat as f;
    use horizon_client::options_ui::kyro_ring as ring;
    let want = ring::on("sprint") && game.native_playing() && (ring::value("sprint", "always") == 1 || f::sprint_tapped());
    if f::sprint_changed(want) || want {
        game.toggles.0[1].set_down(want, false);
        game.sync_toggle(1);
    }
}
