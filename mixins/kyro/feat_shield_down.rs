// A shield an axe disabled: Horizon's server keeps that to itself, so the client went on raising
// it. This sends what vanilla's does: the item cooldown, and entity event 30 for everyone near.
fn kyro_feat_shield_down(
    game: &mut crate::game::Game<'_>,
    component: &horizon_client::items::BlocksAttacks,
    attacker: [f64; 2],
    fully: bool,
    disable: f32,
    at: Option<[f64; 3]>,
) -> core::ops::ControlFlow<()> {
    thread_local! {
        static NESTED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    }
    if NESTED.get() {
        return core::ops::ControlFlow::Continue(());
    }
    let held = game.using.and_then(|(slot, _)| game.stack_in(slot)).map(|s| s.item.to_string());
    NESTED.set(true);
    game.blocked_by_attacker(component, attacker, fully, disable, at);
    NESTED.set(false);
    if let Some(item) = held.filter(|_| game.using.is_none() && game.shield_cooldown > 0) {
        let ticks = game.shield_cooldown as i32;
        game.add_cooldown(std::sync::Arc::from(item.as_str()), ticks);
        if let Some(spot) = game.player.as_ref().map(|p| [p.position.x, p.position.y, p.position.z]) {
            if game.lan_acting.is_some() {
                game.guest_equipment_breaks.push((30, spot));
            } else if let Some(host) = &game.lan {
                host.hub.entity_event(host.host_entity, 30, spot);
            }
        }
    }
    core::ops::ControlFlow::Break(())
}
