// Your chosen cape, on your own player only: others see nothing, as in Java.
fn kyro_feat_cape(
    game: &crate::game::Game<'_>,
    m: &Mob,
    chest: Option<&str>,
    camera: glam::DVec3,
    partial: f32,
    light: u32,
    out: &mut Vec<horizon_client::mesher::Vertex>,
) -> core::ops::ControlFlow<()> {
    // shortcut: "own player" by position, until a real id check is found.
    let own = game.player.as_ref().is_some_and(|p| (p.position.x - m.position.x).abs() < 0.5 && (p.position.z - m.position.z).abs() < 0.5);
    let Some(name) = horizon_client::options_ui::kyro_ring::cape().filter(|_| own && !chest.is_some_and(|c| c.ends_with("elytra"))) else {
        return core::ops::ControlFlow::Continue(());
    };
    use horizon_client::options_ui::kyro_ring as ring;
    let Some(rect) = game.gui.atlas.sprites.get(ring::cape_name(&name).as_str()) else {
        return core::ops::ControlFlow::Continue(());
    };
    let uv = ring::cape_uv(&name, [rect.u0, rect.v0, rect.u1, rect.v1]);
    let base = horizon_client::mobs::render::base_transform(m, partial, camera);
    let start = out.len();
    horizon_client::mobs::render::cape(m, partial, base, uv, light, game.res.mob_data(), out);
    if let Some(tint) = horizon_client::options_ui::kyro_ring::cape_tint() {
        for vertex in &mut out[start..] {
            vertex.color = tint;
        }
    }
    core::ops::ControlFlow::Break(())
}
