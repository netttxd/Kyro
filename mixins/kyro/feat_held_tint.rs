// Shield status: a held shield shows its holder's state.
fn kyro_feat_held_tint(
    game: &crate::game::Game<'_>,
    mob: &Mob,
    camera: glam::DVec3,
    alpha: f64,
    light: u32,
    lit: bool,
    skin: &mut Option<&mut super::creatures::Skinned>,
    out: &mut Vec<Vertex>,
) -> core::ops::ControlFlow<()> {
    use horizon_client::options_ui::kyro_ring as ring;
    thread_local! {
        static NESTED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    }
    if NESTED.get() || !ring::on("shield_status") {
        return core::ops::ControlFlow::Continue(());
    }
    // shortcut: "own player" by position, as the cape does.
    let own = game.player.as_ref().is_some_and(|p| (p.position.x - mob.position.x).abs() < 0.5 && (p.position.z - mob.position.z).abs() < 0.5);
    NESTED.set(true);
    let tint = if own { horizon_client::options_ui::kyro_world::own_shield_color() } else { horizon_client::options_ui::kyro_world::shield_color(crate::game::kyro_feat::shield_wait(mob.id)) };
    horizon_client::options_ui::kyro_world::hold_shield_tint(Some(tint));
    game.held_item_geometry(mob, camera, alpha, light, lit, skin.as_deref_mut(), out);
    horizon_client::options_ui::kyro_world::hold_shield_tint(None);
    NESTED.set(false);
    core::ops::ControlFlow::Break(())
}
